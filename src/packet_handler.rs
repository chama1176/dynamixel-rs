use heapless::Vec;
use heapless::Deque;
use crate::DynamixelControl;
use crate::ControlTable;
use crate::Instruction;
use core::fmt;

pub const MAX_PACKET_LEN: usize = 128;
pub const BROADCAST_ID: u8 = 0xFE;

#[allow(dead_code)]
pub enum Packet {
    Header0,
    Header1,
    Header2,
    Reserved,
    Id,
    LengthL,
    LengthH,
    Instruction,
    Error,
    Parameter0,
}

#[allow(dead_code)]
impl Packet {
    pub fn to_pos(&self) -> usize {
        match self {
            Packet::Header0 => 0,
            Packet::Header1 => 1,
            Packet::Header2 => 2,
            Packet::Reserved => 3,
            Packet::Id => 4,
            Packet::LengthL => 5,
            Packet::LengthH => 6,
            Packet::Instruction => 7,
            Packet::Error => 8,
            Packet::Parameter0 => 8,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ErrorBit {
    ErrResultFail,
    ErrInstruction,
    ErrCRC,
    ErrDataRange,
    ErrDataLength,
    ErrDataLimit,
    ErrAccess,
    ErrAlert,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum CommunicationResult {
    Success,
    PortBusy,
    TxFail,
    RxFail,
    TxError,
    RxWaiting,
    RxTimeout,
    RxCorrupt,
    NotAvailable,
}
impl fmt::Display for CommunicationResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommunicationResult::Success =>
                write!(f, "[TxRxResult] Communication success."),
                CommunicationResult::PortBusy =>
                write!(f, "[TxRxResult] Port is in use!"),
                CommunicationResult::TxFail =>
                write!(f, "[TxRxResult] Failed transmit instruction packet!"),
                CommunicationResult::RxFail =>
                write!(f, "[TxRxResult] Failed get status packet from device!"),
                CommunicationResult::TxError =>
                write!(f, "[TxRxResult] Incorrect instruction packet!"),
                CommunicationResult::RxWaiting =>
                write!(f, "[TxRxResult] Now recieving status packet!"),
                CommunicationResult::RxTimeout =>
                write!(f, "[TxRxResult] There is no status packet!"),
                CommunicationResult::RxCorrupt =>
                write!(f, "[TxRxResult] Incorrect status packet!"),
                CommunicationResult::NotAvailable =>
                write!(f, "[TxRxResult] Protocol does not support This function!"),
        }
    }
}


#[allow(dead_code)]
impl<'a> DynamixelControl<'a> {

    pub fn reserve_msg_header(&self) -> [u8; 4] {
        [0x00; 4]     // Header and reserved len
    }

    fn add_stuffing(&mut self) {}
    fn remove_stuffing(&mut self) {}

    /// Set packet without crc.
    pub fn send_packet(&mut self, mut msg: Vec::<u8, MAX_PACKET_LEN>) -> CommunicationResult {
        // make header
        msg[Packet::Header0.to_pos()] = 0xFF;
        msg[Packet::Header1.to_pos()] = 0xFF;
        msg[Packet::Header2.to_pos()] = 0xFD;
        msg[Packet::Reserved.to_pos()] = 0x00;

        // msg.extend(self.make_msg_header().iter().cloned());
        // // msg.push(id).unwrap();
        // msg.extend(length.to_le_bytes().iter().cloned());       // Set length temporary
        // msg.push(Instruction::Ping.to_value()).unwrap();
        
        // add crc
        msg.extend(self.calc_crc_value(&msg).to_le_bytes().iter().cloned());

        for m in msg {
            self.uart.write_byte(m);
        }

        CommunicationResult::Success
    }
    fn receive_packet(&mut self) -> CommunicationResult {

        let mut result = CommunicationResult::TxFail;
        let mut wait_length = 11; // minimum length (HEADER0 HEADER1 HEADER2 RESERVED ID LENGTH_L LENGTH_H INST ERROR CRC16_L CRC16_H)
        let mut msg = Vec::<u8, MAX_PACKET_LEN>::new(); // VecDeque is not implemented in heapless.

        loop {
            loop {
                if msg.len() >= wait_length {
                    break;
                }
                match self.uart.read_byte() {
                    None => { break; },
                    Some(res) => {
                        msg.push(res).unwrap();
                    },
                }
            }

            if msg.len() >= wait_length {
                let mut idx = 0;
                // find packet header
                while idx < (msg.len() - 3) {
                    if msg[idx + Packet::Header0.to_pos()] == 0xFF &&
                    msg[idx + Packet::Header1.to_pos()] == 0xFF &&
                    msg[idx + Packet::Header2.to_pos()] == 0xFD &&
                    msg[idx + Packet::Reserved.to_pos()] == 0x00 {
                        break;
                    }
                    idx += 1;
                }
                
                if idx == 0 { // found at the beginning of the packet
                    if msg[Packet::Reserved.to_pos()] != 0x00 ||
                    msg[Packet::Id.to_pos()] > 0xFC ||
                    u16::from_le_bytes([msg[Packet::LengthL.to_pos()], msg[Packet::LengthL.to_pos()]]) as usize > MAX_PACKET_LEN ||
                    msg[Packet::Instruction.to_pos()] != 0x55 {
                        // remove the first byte in the packet
                        for s in 0..msg.len() - 1 {
                            msg[s] = msg[s + 1];
                        }
                        msg.truncate(msg.len() - 1);
                        continue;
                    }
                    // re-calculate the exact length of the rx packet
                    if wait_length != u16::from_le_bytes([msg[Packet::LengthL.to_pos()], msg[Packet::LengthL.to_pos()]]) as usize + Packet::LengthH.to_pos() + 1 {
                        wait_length = u16::from_le_bytes([msg[Packet::LengthL.to_pos()], msg[Packet::LengthL.to_pos()]]) as usize + Packet::LengthH.to_pos() + 1;
                        continue;
                    }
            
                    if msg.len() < wait_length {
                        // check timeout
                        // if (port.isPacketTimeout() == true) {
                        // if (rx_length == 0)
                        // {
                        //     result = COMM_RX_TIMEOUT;
                        // }
                        // else
                        // {
                        //     result = COMM_RX_CORRUPT;
                        // }
                        // break;
                        // } else {
                        //     continue;
                        // }
                        continue;
                    }
            
                    // verify CRC16
                    let crc = u16::from_le_bytes([msg[msg.len() - 2], msg[msg.len() - 1]]);
                    if self.calc_crc_value(&msg[..msg.len()-2]) == crc {
                        result = CommunicationResult::Success;
                    }
                    else
                    {
                        result = CommunicationResult::RxCorrupt;
                    }
                    break;
                } else {
                    // remove unnecessary packets
                    for s in 0..(msg.len() - idx){
                        msg[s] = msg[idx + s];
                    }
                    msg.truncate(msg.len() - idx);
                }
            }
            else
            {
                // check timeout
                // if (port->isPacketTimeout() == true)
                // {
                //     if (rx_length == 0)
                //     {
                //         result = COMM_RX_TIMEOUT;
                //     }
                //     else
                //     {
                //         result = COMM_RX_CORRUPT;
                //     }
                //     break;
                // }
            }
            // usleep(0);
        }
        self.is_using = false;
    
        if result == CommunicationResult::Success {
            // removeStuffing(rxpacket);
        }
    
        result
    }

    fn send_read_packet(&mut self, id: u8, name: ControlTable) -> CommunicationResult {

        if id >= BROADCAST_ID { 
            return CommunicationResult::NotAvailable
        }

        let address = name.to_address();
        let size = name.to_size();
        let length: u16 = 1 + 2 + 2 + 2;     // instruction + adress + data length + crc
        let mut msg = Vec::<u8, MAX_PACKET_LEN>::new();

        msg.extend(self.reserve_msg_header().iter().cloned());
        msg.push(id).unwrap();
        msg.extend(length.to_le_bytes().iter().cloned());       // Set length temporary
        msg.push(Instruction::Read.to_value()).unwrap();
        msg.extend(address.to_le_bytes().iter().cloned());
        msg.extend(size.to_le_bytes().iter().cloned());

        let result = self.send_packet(msg);
        
        if result == CommunicationResult::Success {
            // Set timeout
        }

        result
    }
    fn receive_read_packet(&mut self) {

    }

    /// TxRx
    pub fn read(&mut self, id: u8, data_name: ControlTable, data: &mut [u8]) {

        let size = data_name.to_size() as usize;
        self.send_read_packet(id, data_name);

        let mut status = Vec::<u8, 128>::new();
        let status_len = 4 + 1 + 2 + 1 + 1 + size + 2;     // header + id + length + instruction + err + param + crc
        for _i in 0..status_len {
            match self.uart.read_byte() {
                None => {},
                Some(data) => {
                    status.push(data).unwrap();
                },
            }
        }
    }

    // fn read1ByteTx(&mut self) {}
    // fn read1ByteRx(&mut self) {}
    // fn read1ByteTxRx(&mut self) {}
    // fn read2ByteTx(&mut self) {}
    // fn read2ByteRx(&mut self) {}
    // fn read2ByteTxRx(&mut self) {}
    // fn read4ByteTx(&mut self) {}
    // fn read4ByteRx(&mut self) {}
    // fn read4ByteTxRx(&mut self) {}
    
    pub fn send_write_packet(&mut self, id: u8, name: ControlTable, data: &[u8]) -> CommunicationResult {

        if id >= BROADCAST_ID { 
            return CommunicationResult::NotAvailable
        }

        let address = name.to_address();
        let size = name.to_size();
        let length: u16 = 1 + 2 + (data.len() as u16) + 2;     // instruction + adress + data + crc

        if size != data.len() as u16 {
            return CommunicationResult::NotAvailable
        }

        let mut msg = Vec::<u8, MAX_PACKET_LEN>::new();
        msg.extend(self.reserve_msg_header().iter().cloned());
        msg.push(id).unwrap();
        msg.extend(length.to_le_bytes().iter().cloned());       // Set length temporary
        msg.push(Instruction::Write.to_value()).unwrap();
        msg.extend(address.to_le_bytes().iter().cloned());

        for d in data {
            msg.push(*d).unwrap();
        }

        self.send_packet(msg)

    }

    /// TxRx
    pub fn write(&mut self, id: u8, address: u16, data: &[u8]) {
    }

    // fn write1ByteTx(&mut self) {}
    // fn write1ByteTxRx(&mut self) {}
    // fn write2ByteTx(&mut self) {}
    // fn write2ByteTxRx(&mut self) {}
    // fn write4ByteTx(&mut self) {}
    // fn write4ByteTxRx(&mut self) {}
    // regWriteTxOnly
    // regWriteTxRx
    // syncReadTx
    // syncWriteTxOnly
    // bulkReadTx
    // bulkWriteTxOnly

    pub fn calc_crc_value(&self, msg: &[u8]) -> u16 {
        
        let crc_table = [
            0x0000, 0x8005, 0x800F, 0x000A, 0x801B, 0x001E, 0x0014, 0x8011,
            0x8033, 0x0036, 0x003C, 0x8039, 0x0028, 0x802D, 0x8027, 0x0022,
            0x8063, 0x0066, 0x006C, 0x8069, 0x0078, 0x807D, 0x8077, 0x0072,
            0x0050, 0x8055, 0x805F, 0x005A, 0x804B, 0x004E, 0x0044, 0x8041,
            0x80C3, 0x00C6, 0x00CC, 0x80C9, 0x00D8, 0x80DD, 0x80D7, 0x00D2,
            0x00F0, 0x80F5, 0x80FF, 0x00FA, 0x80EB, 0x00EE, 0x00E4, 0x80E1,
            0x00A0, 0x80A5, 0x80AF, 0x00AA, 0x80BB, 0x00BE, 0x00B4, 0x80B1,
            0x8093, 0x0096, 0x009C, 0x8099, 0x0088, 0x808D, 0x8087, 0x0082,
            0x8183, 0x0186, 0x018C, 0x8189, 0x0198, 0x819D, 0x8197, 0x0192,
            0x01B0, 0x81B5, 0x81BF, 0x01BA, 0x81AB, 0x01AE, 0x01A4, 0x81A1,
            0x01E0, 0x81E5, 0x81EF, 0x01EA, 0x81FB, 0x01FE, 0x01F4, 0x81F1,
            0x81D3, 0x01D6, 0x01DC, 0x81D9, 0x01C8, 0x81CD, 0x81C7, 0x01C2,
            0x0140, 0x8145, 0x814F, 0x014A, 0x815B, 0x015E, 0x0154, 0x8151,
            0x8173, 0x0176, 0x017C, 0x8179, 0x0168, 0x816D, 0x8167, 0x0162,
            0x8123, 0x0126, 0x012C, 0x8129, 0x0138, 0x813D, 0x8137, 0x0132,
            0x0110, 0x8115, 0x811F, 0x011A, 0x810B, 0x010E, 0x0104, 0x8101,
            0x8303, 0x0306, 0x030C, 0x8309, 0x0318, 0x831D, 0x8317, 0x0312,
            0x0330, 0x8335, 0x833F, 0x033A, 0x832B, 0x032E, 0x0324, 0x8321,
            0x0360, 0x8365, 0x836F, 0x036A, 0x837B, 0x037E, 0x0374, 0x8371,
            0x8353, 0x0356, 0x035C, 0x8359, 0x0348, 0x834D, 0x8347, 0x0342,
            0x03C0, 0x83C5, 0x83CF, 0x03CA, 0x83DB, 0x03DE, 0x03D4, 0x83D1,
            0x83F3, 0x03F6, 0x03FC, 0x83F9, 0x03E8, 0x83ED, 0x83E7, 0x03E2,
            0x83A3, 0x03A6, 0x03AC, 0x83A9, 0x03B8, 0x83BD, 0x83B7, 0x03B2,
            0x0390, 0x8395, 0x839F, 0x039A, 0x838B, 0x038E, 0x0384, 0x8381,
            0x0280, 0x8285, 0x828F, 0x028A, 0x829B, 0x029E, 0x0294, 0x8291,
            0x82B3, 0x02B6, 0x02BC, 0x82B9, 0x02A8, 0x82AD, 0x82A7, 0x02A2,
            0x82E3, 0x02E6, 0x02EC, 0x82E9, 0x02F8, 0x82FD, 0x82F7, 0x02F2,
            0x02D0, 0x82D5, 0x82DF, 0x02DA, 0x82CB, 0x02CE, 0x02C4, 0x82C1,
            0x8243, 0x0246, 0x024C, 0x8249, 0x0258, 0x825D, 0x8257, 0x0252,
            0x0270, 0x8275, 0x827F, 0x027A, 0x826B, 0x026E, 0x0264, 0x8261,
            0x0220, 0x8225, 0x822F, 0x022A, 0x823B, 0x023E, 0x0234, 0x8231,
            0x8213, 0x0216, 0x021C, 0x8219, 0x0208, 0x820D, 0x8207, 0x0202
        ];

        let mut crc_accum = 0x0000;
        for j in 0..msg.len()
        {
            let i = ((((crc_accum >> 8) as u8) ^ msg[j]) & 0xFF) as usize;
            crc_accum = (crc_accum << 8) ^ crc_table[i];
        }
    
        crc_accum
    }



}

#[cfg(test)]
mod tests {
    use heapless::Deque;
    use heapless::Vec;
    use crate::DynamixelControl;
    use crate::Instruction;
    use crate::ControlTable;
    use crate::packet_handler::MAX_PACKET_LEN;

    pub struct MockSerial {
        rx_buf: Vec<u8, 256>,
        tx_buf: Deque<u8, 256>,
    }
    impl MockSerial {
        pub fn new() -> Self {
            Self { 
                rx_buf: Vec::<u8, 256>::new(), 
                tx_buf: Deque::<u8, 256>::new(), 
            }
        }
    }
    impl crate::Interface for MockSerial {
        fn write_byte(&mut self, data: u8) {
            
            self.rx_buf.push(data).unwrap();

            // For test ping
            if self.rx_buf.len() > 7 && self.rx_buf[7] == Instruction::Ping.to_value() {
                // ID1(XM430-W210) : For Model Number 1030(0x0406), Version of Firmware 38(0x26)
                // Instruction Packet ID : 1
                let res = [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x55, 0x00, 0x06, 0x04, 0x26, 0x65, 0x5D];
                for data in res{
                    self.tx_buf.push_back(data).unwrap();
                }
            }
            // For test read
            if self.rx_buf.len() > 7 && self.rx_buf[7] == Instruction::Read.to_value() {
                // ID1(XM430-W210) : Present Position(132, 0x0084, 4[byte]) = 166(0x000000A6)
                let res = [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x55, 0x00, 0x06, 0x04, 0x26, 0x65, 0x5D];
                for data in res{
                    self.tx_buf.push_back(data).unwrap();
                }
            }         
        }
        fn read_byte(&mut self) -> Option<u8> {
            self.tx_buf.pop_front()
        }
    }

    pub struct MockTimer {
        now: u32,
    }
    impl MockTimer {
        pub fn new() -> Self {
            Self { 
                now: 0,
            }
        }
        pub fn tick(&mut self) {
            self.now += 1;
        }
    }
    impl crate::Timer for MockTimer {
        fn get_current_time(&self) -> f32 {
            0.0
        } 
    }

    #[test]    
    fn crc() {
        let mut mock_uart = MockSerial::new();
        let mut mock_timer = MockTimer::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mut mock_timer);
        let mut msg = Vec::<u8, MAX_PACKET_LEN>::new();
        msg.extend([0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x55, 0x00, 0x06, 0x04, 0x26].iter().cloned());
        assert_eq!(dxl.calc_crc_value(&msg), 0x5D65);    
    }

}
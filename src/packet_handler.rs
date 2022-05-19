use heapless::Vec;
use crate::DynamixelControl;
use crate::ControlTable;
use crate::Instruction;


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
#[derive(Debug)]
pub enum CommunicationResult {
    CommSuccess,
    CommPortBusy,
    CommTxFail,
    CommRxFail,
    CommTxError,
    CommRxWaiting,
    CommRxTimeout,
    CommRxCorrupt,
    CommNotAvailable,
}


impl<'a> DynamixelControl<'a> {




    pub fn read(&mut self, id: u8, data_name: ControlTable, data: &mut [u8]) {

        let address = data_name.to_address();
        let size = data_name.to_size();
        let length: u16 = 1 + 2 + 2 + 2;     // instruction + adress + data length + crc
        let mut msg = Vec::<u8, 256>::new();
        msg.extend(self.make_msg_header().iter().cloned());
        msg.push(id).unwrap();
        msg.extend(length.to_le_bytes().iter().cloned());       // Set length temporary
        msg.push(Instruction::Read.to_value()).unwrap();
        msg.extend(address.to_le_bytes().iter().cloned());
        msg.extend(size.to_le_bytes().iter().cloned());
        msg.extend(self.calc_crc_value(&msg).to_le_bytes().iter().cloned());

        for m in msg {
            self.uart.write_byte(m);
        }

        let mut status = Vec::<u8, 128>::new();
        let status_len = 4 + 1 + 2 + 1 + 1 + size + 2;     // header + id + length + instruction + err + param + crc
        for _i in 0..status_len {
            match self.uart.read() {
                None => {},
                Some(data) => {
                    status.push(data).unwrap();
                },
            }
        }

    }


}


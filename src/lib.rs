#![no_std]
use heapless::Vec;

pub trait Interface {
    fn write_byte(&mut self, data: u8);
}

pub struct DynamixelControl<'a> {
    uart: &'a mut dyn Interface,
    is_enabled: bool,
}

impl<'a> DynamixelControl<'a> {
    pub fn new(uart: &'a mut dyn Interface,) -> Self {
        
        Self { uart, is_enabled: false }
    }

    pub fn torque_enable(&mut self, enabled: bool) {
        self.uart.write_byte(0xFF);
        self.uart.write_byte(0x6f);
        self.uart.write_byte(0x6c);
        self.uart.write_byte(0x61);
    }

    pub fn ping(&mut self, id: u8) {
        // 👺結果を返すようにする
        // For Model Number 1030(0x0406), Version of Firmware 38(0x26)
        let mut msg = self.make_msg_header();
        msg.push(id).unwrap();
        msg.extend([0x03, 0x00].iter().cloned());       // Set 0 to length temporary
        msg.push(self.instruction_value(Instruction::Ping)).unwrap();

        for m in msg {
            self.uart.write_byte(m);
        }
    }

    fn make_msg_header(&self) -> Vec<u8, 256> {
        let mut msg = Vec::<u8, 256>::new();
        msg.extend([0xFF, 0xFF, 0xFD, 0x00].iter().cloned());     // Header and reserved
        msg
    }

    fn instruction_value(&self, instruction: Instruction) -> u8{
        match instruction {
            Instruction::Ping => 0x01,
            Instruction::Read => 0x02,
            Instruction::Write => 0x03,
            Instruction::RegWrite => 0x04,
            Instruction::Action => 0x05,
            Instruction::FactoryReset => 0x06,
            Instruction::Reboot => 0x08,
            Instruction::Clear => 0x10,
            Instruction::ControlTableBackup => 0x20,
            Instruction::Status => 0x55,
            Instruction::SyncRead => 0x82,
            Instruction::SyncWrite => 0x83,
            Instruction::FastSyncRead => 0x8A,
            Instruction::BulkRead => 0x92,
            Instruction::BulkWrite => 0x93,
            Instruction::FastBulkRead => 0x9A,
        }
    }

    fn calc_crc_value(&self, msg: &Vec<u8, 256>) -> u16 {
        0x0000
    }

    fn u16_to_u8(&self, data: u16) -> [u8; 2] {
        [(data & 0x00FF) as u8, 0x00]
    }


}

pub enum Instruction {
    Ping,
    Read,
    Write,
    RegWrite,
    Action,
    FactoryReset,
    Reboot,
    Clear,
    ControlTableBackup,
    Status,
    SyncRead,
    SyncWrite,
    FastSyncRead,
    BulkRead,
    BulkWrite,
    FastBulkRead, 
}

#[cfg(test)]
mod tests {
    use heapless::Vec;
    use crate::DynamixelControl;

    pub struct MockSerial {
        buf: Vec<u8, 256>,
    }
    impl MockSerial {
        pub fn new() -> Self {
            Self { buf: Vec::<u8, 256>::new()}
        }
    }
    impl crate::Interface for MockSerial {
        fn write_byte(&mut self, data: u8) {
            self.buf.push(data).unwrap();
        }
    }
    
    #[test]    
    fn torque_enable_xc330() {
        let mut mock_uart = MockSerial::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart);
        dxl.torque_enable(true);
        assert_eq!(*mock_uart.buf, [0xFF, 0x6f, 0x6c, 0x61]);    
    }

    #[test]    
    fn ping() {
        let mut mock_uart = MockSerial::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart);
        dxl.ping(1);
        assert_eq!(*mock_uart.buf, [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x03, 0x00, 0x01, 0x19, 0x4E]);    
    }

}

#![no_std]
//! This crate is for control dynamixel.
//!
#![allow(unused_imports)]
pub mod control_table;
pub mod control_data;
pub mod packet_handler;
pub mod utils;
mod instruction;
pub use control_table::ControlTable;
pub use control_data::ControlData;
pub use packet_handler::CommunicationResult;
use packet_handler::MAX_PACKET_LEN;
pub use utils::DegRad;

use core::result::Result;
use core::time::Duration;
use heapless::Vec;
use instruction::Instruction;

pub trait Interface {
    fn write_byte(&mut self, data: u8);
    fn read_byte(&mut self) -> Option<u8>;
}
pub trait Clock {
    fn get_current_time(&self) -> Duration;
}

pub struct DynamixelControl<'a> {
    uart: &'a mut dyn Interface,
    clock: &'a dyn Clock,
    // is_enabled: bool,
    is_using: bool,
    packet_start_time: Duration,
    packet_timeout: Duration,
    baudrate: u32,
    tx_time_per_byte: u64,
}

impl<'a> DynamixelControl<'a> {
    pub fn new(uart: &'a mut dyn Interface, clock: &'a dyn Clock, baudrate: u32) -> Self {
        Self {
            uart,
            clock,
            // is_enabled: false,
            is_using: false,
            packet_start_time: Duration::new(0, 0),
            packet_timeout: Duration::new(0, 0),
            baudrate: baudrate,
            tx_time_per_byte: ((1_000_000.0 * 8.0 + (baudrate as f32 - 1.0)) / baudrate as f32) as u64,
        }
    }

    fn set_operating_mode(&mut self, id: u8, data: u8) -> Result<(), CommunicationResult> {
        self.write_1byte(id, ControlTable::OperatingMode, data)
    }

    pub fn set_led(&mut self, id: u8, data: u8) {
        self.send_write_packet(id, ControlTable::LED, &[data])
            .unwrap();
    }

    fn set_torque_enable(&mut self, id: u8, data: u8) -> Result<(), CommunicationResult> {
        self.write_1byte(id, ControlTable::TorqueEnable, 1)
    }

    pub fn torque_enable(&mut self) {
        self.uart.write_byte(0xFF);
        self.uart.write_byte(0x6f);
        self.uart.write_byte(0x6c);
        self.uart.write_byte(0x61);
    }
}

#[cfg(test)]
mod tests {
    use crate::packet_handler::CommunicationResult;
    use crate::packet_handler::Packet;
    use crate::ControlTable;
    use crate::DynamixelControl;
    use crate::Instruction;
    use core::cell::RefCell;
    use core::time::Duration;
    use heapless::Deque;
    use heapless::Vec;

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
            if self.tx_buf.len() == 0
                && self.rx_buf.len() > 8
                && self.rx_buf[Packet::Instruction.to_pos()] == Instruction::Ping.into()
                && self.rx_buf[Packet::Id.to_pos()] == 0x01
            {
                // ID1(XM430-W210) : For Model Number 1030(0x0406), Version of Firmware 38(0x26)
                // Instruction Packet ID : 1
                let res = [
                    0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x55, 0x00, 0x06, 0x04, 0x26, 0x65,
                    0x5D,
                ];
                for data in res {
                    self.tx_buf.push_back(data).unwrap();
                }
            }
            // For test read(4byte)
            if self.tx_buf.len() == 0
                && self.rx_buf.len() > 8
                && self.rx_buf[Packet::Instruction.to_pos()] == Instruction::Read.into()
                && self.rx_buf[Packet::Id.to_pos()] == 0x01
                && self.rx_buf[Packet::Parameter0.to_pos()] == 0x84
            {
                // ID1(XM430-W210) : Present Position(132, 0x0084, 4[byte]) = 166(0x000000A6)
                let res = [
                    0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x08, 0x00, 0x55, 0x00, 0xA6, 0x00, 0x00, 0x00,
                    0x8C, 0xC0,
                ];
                for data in res {
                    self.tx_buf.push_back(data).unwrap();
                }
            }
            // For test read(2byte)
            if self.tx_buf.len() == 0
                && self.rx_buf.len() > 8
                && self.rx_buf[Packet::Instruction.to_pos()] == Instruction::Read.into()
                && self.rx_buf[Packet::Id.to_pos()] == 0x01
                && self.rx_buf[Packet::Parameter0.to_pos()] == 0x26
            {
                // ID1(XC330-T181) : Current Limit(38, 0x0026, 2[byte]) = 888(0x0378)
                let res = [
                    0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x06, 0x00, 0x55, 0x00, 0x78, 0x03, 0xC9, 0x4B,
                ];
                for data in res {
                    self.tx_buf.push_back(data).unwrap();
                }
            }
            // For test read(1byte)
            if self.tx_buf.len() == 0
                && self.rx_buf.len() > 8
                && self.rx_buf[Packet::Instruction.to_pos()] == Instruction::Read.into()
                && self.rx_buf[Packet::Id.to_pos()] == 0x01
                && self.rx_buf[Packet::Parameter0.to_pos()] == 0x0B
            {
                // ID1(XC330-T181) : Operating Mode(11, 0x000B, 1[byte]) = 5(0x05)
                let res = [
                    0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x05, 0x00, 0x55, 0x00, 0x05, 0x4D, 0x21,
                ];
                for data in res {
                    self.tx_buf.push_back(data).unwrap();
                }
            }
            // For test write(4byte)
            if self.tx_buf.len() == 0
                && self.rx_buf.len() > 15
                && self.rx_buf[Packet::Instruction.to_pos()] == Instruction::Write.into()
                && self.rx_buf[Packet::Id.to_pos()] == 0x01
                && self.rx_buf[Packet::Parameter0.to_pos()] == 0x74
            {
                // ID1(XM430-W210) : Write 512(0x00000200) to Goal Position(116, 0x0074, 4[byte])
                let res = [
                    0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x04, 0x00, 0x55, 0x00, 0xA1, 0x0C,
                ];
                for data in res {
                    self.tx_buf.push_back(data).unwrap();
                }
            }
            // For test write(2byte)
            if self.tx_buf.len() == 0
                && self.rx_buf.len() > 13
                && self.rx_buf[Packet::Instruction.to_pos()] == Instruction::Write.into()
                && self.rx_buf[Packet::Id.to_pos()] == 0x01
                && self.rx_buf[Packet::Parameter0.to_pos()] == 0x26
            {
                // ID1(XC330-T181) : Current Limit(38, 0x0026, 2[byte]) = 888(0x0378)
                let res = [
                    0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x04, 0x00, 0x55, 0x00, 0xA1, 0x0C,
                ];
                for data in res {
                    self.tx_buf.push_back(data).unwrap();
                }
            }
            // For test write(1byte)
            if self.tx_buf.len() == 0
                && self.rx_buf.len() > 12
                && self.rx_buf[Packet::Instruction.to_pos()] == Instruction::Write.into()
                && self.rx_buf[Packet::Id.to_pos()] == 0x01
                && self.rx_buf[Packet::Parameter0.to_pos()] == 0x1F
            {
                // ID1(XC330-T181) : Temperature Limit(31, 0x001F, 1[byte]) = 80(0x50)
                let res = [
                    0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x04, 0x00, 0x55, 0x00, 0xA1, 0x0C,
                ];
                for data in res {
                    self.tx_buf.push_back(data).unwrap();
                }
            }
            // For test reboot and factory reset
            if self.tx_buf.len() == 0
                && self.rx_buf.len() > 8
                && self.rx_buf[Packet::Id.to_pos()] == 0x01
                && (self.rx_buf[Packet::Instruction.to_pos()] == Instruction::Reboot.into()
                    || self.rx_buf[Packet::Instruction.to_pos()]
                        == Instruction::FactoryReset.into())
            {
                let res = [
                    0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x04, 0x00, 0x55, 0x00, 0xA1, 0x0C,
                ];
                for data in res {
                    self.tx_buf.push_back(data).unwrap();
                }
            }
        }
        fn read_byte(&mut self) -> Option<u8> {
            self.tx_buf.pop_front()
        }
    }

    pub struct MockClock {
        time_elasped: RefCell<Duration>,
    }
    #[allow(dead_code)]
    impl MockClock {
        pub fn new() -> Self {
            Self {
                time_elasped: RefCell::new(Duration::new(0, 0)),
            }
        }
        pub fn tick(&self) {
            let dt = Duration::from_millis(1);
            self.time_elasped.replace_with(|&mut old| old + dt);
        }
    }
    impl crate::Clock for MockClock {
        fn get_current_time(&self) -> Duration {
            self.time_elasped.clone().into_inner()
        }
    }

    #[test]
    #[ignore]
    fn torque_enable_xc330() {
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock, 115200);
        dxl.torque_enable();
        assert_eq!(*mock_uart.rx_buf, [0xFF, 0x6f, 0x6c, 0x61]);
    }

    #[test]
    fn set_led_xc330() {
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock, 115200);
        dxl.set_led(1, 1);
        // crc以外をテスト
        assert_eq!(
            mock_uart.rx_buf[..11],
            [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x06, 0x00, 0x03, 65, 0x00, 0x01]
        );
    }

    #[test]
    fn ping() {
        // ID1(XM430-W210) : For Model Number 1030(0x0406), Version of Firmware 38(0x26)
        // Instruction Packet ID : 1
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock, 115200);

        match dxl.ping(1) {
            Ok(v) => {
                let (model_number, firmware_version) = v;
                assert_eq!(model_number, 0x0406);
                assert_eq!(firmware_version, 0x26);
            }
            Err(_) => assert!(false),
        }

        assert_eq!(
            *mock_uart.rx_buf,
            [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x03, 0x00, 0x01, 0x19, 0x4E]
        );
    }

    #[test]
    fn read() {
        // ID1(XM430-W210) : Present Position(132, 0x0084, 4[byte]) = 166(0x000000A6)
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock, 115200);
        let result = dxl.read(
            1,
            ControlTable::PresentPosition,
            ControlTable::PresentPosition.to_size(),
        );
        assert_eq!(
            *mock_uart.rx_buf,
            [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x02, 0x84, 0x00, 0x04, 0x00, 0x1D, 0x15]
        );
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), (0x000000A6 as u32).to_le_bytes());
    }

    #[test]
    fn read_4byte() {
        // ID1(XM430-W210) : Present Position(132, 0x0084, 4[byte]) = 166(0x000000A6)
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock, 115200);
        let result = dxl.read_4byte(1, ControlTable::PresentPosition);
        assert_eq!(
            *mock_uart.rx_buf,
            [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x02, 0x84, 0x00, 0x04, 0x00, 0x1D, 0x15]
        );
        assert_eq!(result.is_ok(), true);
        assert_eq!(result, Ok(0x000000A6));
    }
    
    #[test]
    fn read_2byte() {
        // ID1(XC330-T181) : Current Limit(38, 0x0026, 2[byte]) = 888(0x0378)
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock, 115200);
        let result = dxl.read_2byte(1, ControlTable::CurrentLimit);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result, Ok(0x0378));
    }

    #[test]
    fn read_1byte() {
        // ID1(XC330-T181) : Operating Mode(11, 0x000B, 1[byte]) = 5(0x05)
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock, 115200);
        let result = dxl.read_1byte(1, ControlTable::OperatingMode);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result, Ok(0x05));
    }

    #[test]
    fn write() {
        // ID1(XM430-W210) : Write 512(0x00000200) to Goal Position(116, 0x0074, 4[byte])
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock, 115200);
        let data: u32 = 0x00000200;
        let result = dxl.write(1, ControlTable::GoalPosition, &data.to_le_bytes());
        assert_eq!(
            *mock_uart.rx_buf,
            [
                0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x09, 0x00, 0x03, 0x74, 0x00, 0x00, 0x02, 0x00, 0x00,
                0xCA, 0x89
            ]
        );
        assert_eq!(result.is_ok(), true);
    }
    #[test]
    fn write_4byte() {
        // ID1(XM430-W210) : Write 512(0x00000200) to Goal Position(116, 0x0074, 4[byte])
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock, 115200);
        let result = dxl.write_4byte(1, ControlTable::GoalPosition, 0x00000200);
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            *mock_uart.rx_buf,
            [
                0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x09, 0x00, 0x03, 0x74, 0x00, 0x00, 0x02, 0x00, 0x00,
                0xCA, 0x89
            ]
        );
    }
    #[test]
    fn write_2byte() {
        // ID1(XC330-T181) : Current Limit(38, 0x0026, 2[byte]) = 888(0x0378)
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock, 115200);
        let result = dxl.write_2byte(1, ControlTable::CurrentLimit, 888);
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            *mock_uart.rx_buf,
            [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x03, 0x26, 0x00, 0x78, 0x03, 0x5A, 0x35]
        );
    }
    #[test]
    fn write_1byte() {
        // ID1(XC330-T181) : Temperature Limit(31, 0x001F, 1[byte]) = 80(0x50)
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock, 115200);
        let result = dxl.write_1byte(1, ControlTable::TemperatureLimit, 80);
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            *mock_uart.rx_buf,
            [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x06, 0x00, 0x03, 0x1F, 0x00, 0x50, 0xB2, 0xE3]
        );
    }

    #[test]
    #[ignore]
    fn read_after_write() {
        // ID1(XM430-W210) : Present Position(132, 0x0084, 4[byte]) = 166(0x000000A6)
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock, 115200);
        dxl.send_1byte_write_packet(1, ControlTable::TemperatureLimit, 80).unwrap();
        
        let result = dxl.read_4byte(1, ControlTable::PresentPosition);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result, Ok(0x000000A6));
    }


    #[test]
    fn factory_reset() {
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock, 115200);

        let result = dxl.factory_reset(1);

        assert_eq!(
            *mock_uart.rx_buf,
            [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x04, 0x00, 0x06, 0x02, 0xAB, 0xE6]
        );
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn reboot() {
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock, 115200);

        let result = dxl.reboot(1);

        assert_eq!(
            *mock_uart.rx_buf,
            [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x03, 0x00, 0x08, 0x2F, 0x4E]
        );
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn sync_read_tx() {
        // ID1(XM430-W210) : Present Position(132, 0x0084, 4[byte]) = 166(0x000000A6)
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock, 115200);
        let result = dxl.send_sync_read_packet(
            &[1, 2],
            ControlTable::PresentPosition,
            ControlTable::PresentPosition.to_size(),
        );
        assert_eq!(
            *mock_uart.rx_buf,
            [0xFF, 0xFF, 0xFD, 0x00, 0xFE, 0x09, 0x00, 0x82, 0x84, 0x00, 0x04, 0x00, 0x01, 0x02, 0xCE, 0xFA]
        );
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn u16_to_u8() {
        assert_eq!((0xFBFA as u16).to_le_bytes(), [0xFA, 0xFB]);
    }
}

#![no_std]
//! This crate is for control dynamixel.
//!
mod control_table;
mod instruction;
mod packet_handler;
use control_table::ControlTable;
use core::result::Result;
use core::time::Duration;
use heapless::Vec;
use instruction::Instruction;
use packet_handler::MAX_PACKET_LEN;

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
    is_enabled: bool,
    is_using: bool,
    packet_start_time: Duration,
    packet_timeout: Duration,
    tx_time_per_byte: u64,
}

impl<'a> DynamixelControl<'a> {

    pub fn new(uart: &'a mut dyn Interface, clock: &'a dyn Clock) -> Self {
        Self {
            uart,
            clock,
            is_enabled: false,
            is_using: false,
            packet_start_time: Duration::new(0, 0),
            packet_timeout: Duration::new(0, 0),
            tx_time_per_byte: ((1_000_000.0 * 8.0 + (115200.0 - 1.0)) / 115200.0) as u64,
        }
    }

    pub fn set_led(&mut self, id: u8, data: u8) {
        self.send_write_packet(id, ControlTable::LED, &[data]);
    }

    pub fn torque_enable(&mut self, enabled: bool) {
        self.uart.write_byte(0xFF);
        self.uart.write_byte(0x6f);
        self.uart.write_byte(0x6c);
        self.uart.write_byte(0x61);
    }

    /// 👺Broadcast is not implemented yet.
    /// 👺型をちゃんとモデルナンバーとファームバージョンにした方がいいかも
    /// 👺待ち方が不十分
    pub fn ping(&mut self, id: u8) -> (u16, u8) {
        let length: u16 = 1 + 2; // instruction + crc
        let mut msg = Vec::<u8, MAX_PACKET_LEN>::new();
        msg.extend(self.reserve_msg_header().iter().cloned());
        msg.push(id).unwrap();
        msg.extend(length.to_le_bytes().iter().cloned()); // Set length temporary
        msg.push(Instruction::Ping.to_value()).unwrap();

        self.send_packet(msg);

        let mut status = Vec::<u8, 128>::new();
        let status_len = 14;
        for _i in 0..status_len {
            match self.uart.read_byte() {
                None => {}
                Some(data) => {
                    status.push(data).unwrap();
                }
            }
        }
        if status.len() == status_len {
            let model_number = u16::from_le_bytes([status[9], status[10]]);
            let firmware_version = status[11];
            (model_number, firmware_version)
        } else {
            (0, 0)
        }
    }

    pub fn broadcast_ping(&mut self) {}
    pub fn action(&mut self) {}
    pub fn reboot(&mut self) {}
    pub fn clear_multi_turn(&mut self) {}
    pub fn factory_reset(&mut self) {}
}

#[cfg(test)]
mod tests {
    use crate::ControlTable;
    use crate::DynamixelControl;
    use crate::Instruction;
    use core::time::Duration;
    use core::cell::RefCell;
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
            if self.rx_buf.len() > 7 && self.rx_buf[7] == Instruction::Ping.to_value() {
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
            // For test read
            if self.rx_buf.len() > 7 && self.rx_buf[7] == Instruction::Read.to_value() {
                // ID1(XM430-W210) : Present Position(132, 0x0084, 4[byte]) = 166(0x000000A6)
                let res = [
                    0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x55, 0x00, 0x06, 0x04, 0x26, 0x65,
                    0x5D,
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
    fn torque_enable_xc330() {
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock);
        dxl.torque_enable(true);
        assert_eq!(*mock_uart.rx_buf, [0xFF, 0x6f, 0x6c, 0x61]);
    }

    #[test]
    fn set_led_xc330() {
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock);
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
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock);
        let (model_number, firmware_version) = dxl.ping(1);
        assert_eq!(
            *mock_uart.rx_buf,
            [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x03, 0x00, 0x01, 0x19, 0x4E]
        );
        assert_eq!(model_number, 0x0406);
        assert_eq!(firmware_version, 0x26);
    }

    #[test]
    fn read() {
        // ID1(XM430-W210) : Present Position(132, 0x0084, 4[byte]) = 166(0x000000A6)
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock);
        let mut data = [0; 1];
        dxl.read(1, ControlTable::PresentPosition, &mut data);
        assert_eq!(
            *mock_uart.rx_buf,
            [0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x07, 0x00, 0x02, 0x84, 0x00, 0x04, 0x00, 0x1D, 0x15]
        );
    }

    #[test]
    fn write() {
        // ID1(XM430-W210) : Write 512(0x00000200) to Goal Position(116, 0x0074, 4[byte])
        let mut mock_uart = MockSerial::new();
        let mock_clock = MockClock::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart, &mock_clock);
        let data: u32 = 0x00000200;
        dxl.send_write_packet(1, ControlTable::GoalPosition, &data.to_le_bytes());
        assert_eq!(
            *mock_uart.rx_buf,
            [
                0xFF, 0xFF, 0xFD, 0x00, 0x01, 0x09, 0x00, 0x03, 0x74, 0x00, 0x00, 0x02, 0x00, 0x00,
                0xCA, 0x89
            ]
        );
    }

    #[test]
    fn u16_to_u8() {
        assert_eq!((0xFBFA as u16).to_le_bytes(), [0xFA, 0xFB]);
    }
}

#![no_std]

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
    fn torque_enable() {
        let mut mock_uart = MockSerial::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart);
        dxl.torque_enable(true);
        assert_eq!(*mock_uart.buf, [0xFF, 0x6f, 0x6c, 0x61]);    
    }


}

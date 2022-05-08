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
    use core::fmt::{Error, Write};
    use heapless::Vec;
    use crate::DynamixelControl;

    fn writer<W: Write>(f: &mut W, s: &str) -> Result<(), Error> {
        f.write_str(s)
    }

    pub struct MockSerial<'a> {
        buf: &'a mut Vec<u8, 256>,
    }
    impl<'a> MockSerial<'a> {
        pub fn new(buf: &'a mut Vec<u8, 256>,) -> Self {
            Self { buf }
        }
    }
    impl<'a> crate::Interface for MockSerial<'a> {
        fn write_byte(&mut self, data: u8) {
            self.buf.push(data).unwrap();
        }
    }
    
    
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]    
    fn torque_enable() {
        // let mut port = serialport::new("/dev/ttyUSB0", 115200)
        // .stop_bits(serialport::StopBits::One)
        // .data_bits(serialport::DataBits::Eight)
        // .timeout(Duration::from_millis(10))
        // .open()
        // .unwrap_or_else(|e| {
        //     eprintln!("Failed to open \"{}\". Error: {}", "/dev/ttyUSB", e);
        //     ::std::process::exit(1);
        // });
        let mut buf = Vec::<u8, 256>::new();
        let mut mock_uart = MockSerial::new(&mut buf);
        let mut dxl = DynamixelControl::new(&mut mock_uart);
        // writer(&mut mock_uart_buf, "hola").unwrap();
        dxl.torque_enable(true);
        assert_eq!(*buf, [0xFF, 0x6f, 0x6c, 0x61]);
    
    }


}

#![no_std]
use core::fmt::{self, Error, Write};

pub struct DynamixelControl<'a> {
    uart: &'a mut dyn Write,
    is_enabled: bool,
}

impl<'a> DynamixelControl<'a> {
    pub fn new(uart: &'a mut dyn Write,) -> Self {
        
        Self { uart, is_enabled: false }
    }
    pub fn torque_enable(&mut self, enabled: bool) {
        // self.uart.write_str("hoge").unwrap();
        self.uart.write_char(0x6f as char).unwrap();
    }


}

#[cfg(test)]
mod tests {
    use core::fmt::{Error, Write};
    use heapless::String;
    use crate::DynamixelControl;

    fn writer<W: Write>(f: &mut W, s: &str) -> Result<(), Error> {
        f.write_str(s)
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
        let mut mock_uart_buf = String::<256>::new();
        let mut dxl = DynamixelControl::new(&mut mock_uart_buf);
        // writer(&mut mock_uart_buf, "hola").unwrap();
        dxl.torque_enable(true);
        assert_eq!(mock_uart_buf.as_bytes(), [0xFF, 0x6f, 0x6c, 0x61]);
    
    }


}

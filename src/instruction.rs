#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Instruction {
    Ping = 0x01,
    Read = 0x02,
    Write = 0x03,
    RegWrite = 0x04,
    Action = 0x05,
    FactoryReset = 0x06,
    Reboot = 0x08,
    Clear = 0x10,
    ControlTableBackup = 0x20,
    Status = 0x55,
    SyncRead = 0x82,
    SyncWrite = 0x83,
    FastSyncRead = 0x8A,
    BulkRead = 0x92,
    BulkWrite = 0x93,
    FastBulkRead = 0x9A,
    Unknown = 0xFF,
}

impl From<Instruction> for u8 {
    #[inline(always)]
    fn from(variant: Instruction) -> Self {
        variant as _
    }
}

impl From<u8> for Instruction {
    #[inline(always)]
    fn from(variant: u8) -> Self {
        match variant {
            0x01 => Instruction::Ping,
            0x02 => Instruction::Read,
            0x03 => Instruction::Write,
            0x04 => Instruction::RegWrite,
            0x05 => Instruction::Action,
            0x06 => Instruction::FactoryReset,
            0x08 => Instruction::Reboot,
            0x10 => Instruction::Clear,
            0x20 => Instruction::ControlTableBackup,
            0x55 => Instruction::Status,
            0x82 => Instruction::SyncRead,
            0x83 => Instruction::SyncWrite,
            0x8A => Instruction::FastSyncRead,
            0x92 => Instruction::BulkRead,
            0x93 => Instruction::BulkWrite,
            0x9A => Instruction::FastBulkRead,
            _ => Instruction::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Instruction;
    #[test]
    fn to_u8() {
        assert_eq!(Instruction::Clear as u8, 0x10);
        assert_eq!(u8::from(Instruction::Clear), 0x10);
        
        let s: u8 = Instruction::Clear.into();
        assert_eq!(s, 0x10);
    }
    // #[test]
    // fn from_u8() {
    //     let n: u8 = 0x10;
    //     let s: Instruction = n.into();
    //     assert_eq!(s, Instruction::Clear);
    // }
}


impl Instruction {
    pub fn to_value(&self) -> u8 {
        match self {
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
            Instruction::Unknown => 0xFF,
        }
    }
}

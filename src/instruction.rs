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
}

#[allow(dead_code)]
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
        }
    }
}

pub enum DynamixelModel{
    Xm430W350,
    Xc330T181,
}

#[allow(dead_code)]
pub enum ControlTable {
    ModelNumber,
    ModelInformation,
    FirmwareVersion,
    ID,
    BaudRate,
    ReturnDelayTime,
    DriveMode,
    OperatingMode,
    SecondaryID,
    ProtocolType,
    HomingOffset,
    MovingThreshold,
    TemperatureLimit,
    MaxVoltageLimit,
    MinVoltageLimit,
    PWMLimit,
    CurrentLimit,
    VelocityLimit,
    MaxPositionLimit,
    MinPositionLimit,
    StartupConfiguration,
    PWMSlope,
    Shutdown,
    TorqueEnable,
    LED,
    StatusReturnLevel,
    RegisteredInstruction,
    HardwareErrorStatus,
    VelocityIGain,
    VelocityPgain,
    PositionDGain,
    PositionIGain,
    PositionPGain,
    Feedforward2ndGain,
    Feedforward1stGain,
    BusWatchdog,
    GoalPWM,
    GoalCurrent,
    GoalVelocity,
    ProfileAccleration,
    ProfileVelocity,
    GoalPosition,
    RealtimeTick,
    Moving,
    MovingStatus,
    PresentPWM,
    PresentCurrent,
    PresentVelocity,
    PresentPosition,
    VelocityTrajectory,
    PositionTrajectory,
    PresentInputVoltage,
    PresentTemperature,
    BackupReady,
    IndirectAddress1,
    IndirectAddress2,
    IndirectAddress3,
    IndirectAddress4,
    IndirectAddress5,
    IndirectAddress6,
    IndirectAddress7,
    IndirectAddress8,
    IndirectAddress9,
    IndirectAddress10,
    IndirectAddress11,
    IndirectAddress12,
    IndirectAddress13,
    IndirectAddress14,
    IndirectAddress15,
    IndirectAddress16,
    IndirectAddress17,
    IndirectAddress18,
    IndirectAddress19,
    IndirectAddress20,
    IndirectData1,
    IndirectData2,
    IndirectData3,
    IndirectData4,
    IndirectData5,
    IndirectData6,
    IndirectData7,
    IndirectData8,
    IndirectData9,
    IndirectData10,
    IndirectData11,
    IndirectData12,
    IndirectData13,
    IndirectData14,
    IndirectData15,
    IndirectData16,
    IndirectData17,
    IndirectData18,
    IndirectData19,
    IndirectData20,
}

#[allow(dead_code)]
impl ControlTable {
    pub fn to_address(&self) -> u16 {
        match self {
            ControlTable::ModelNumber => 0,
            ControlTable::ModelInformation => 2,
            ControlTable::FirmwareVersion => 6,
            ControlTable::ID => 7,
            ControlTable::BaudRate => 8,
            ControlTable::ReturnDelayTime => 9,
            ControlTable::DriveMode => 10,
            ControlTable::OperatingMode => 11,
            ControlTable::SecondaryID => 12,
            ControlTable::ProtocolType => 13,
            ControlTable::HomingOffset => 20,
            ControlTable::MovingThreshold => 24,
            ControlTable::TemperatureLimit => 31,
            ControlTable::MaxVoltageLimit => 32,
            ControlTable::MinVoltageLimit => 34,
            ControlTable::PWMLimit => 36,
            ControlTable::CurrentLimit => 38,
            ControlTable::VelocityLimit => 44,
            ControlTable::MaxPositionLimit => 48,
            ControlTable::MinPositionLimit => 52,
            ControlTable::StartupConfiguration => 60,
            ControlTable::PWMSlope => 62,
            ControlTable::Shutdown => 63,
            ControlTable::TorqueEnable => 64,
            ControlTable::LED => 65,
            ControlTable::StatusReturnLevel => 68,
            ControlTable::RegisteredInstruction => 69,
            ControlTable::HardwareErrorStatus => 70,
            ControlTable::VelocityIGain => 76,
            ControlTable::VelocityPgain => 78,
            ControlTable::PositionDGain => 80,
            ControlTable::PositionIGain => 82,
            ControlTable::PositionPGain => 84,
            ControlTable::Feedforward2ndGain => 88,
            ControlTable::Feedforward1stGain => 90,
            ControlTable::BusWatchdog => 98,
            ControlTable::GoalPWM => 100,
            ControlTable::GoalCurrent => 102,
            ControlTable::GoalVelocity => 104,
            ControlTable::ProfileAccleration => 108,
            ControlTable::ProfileVelocity => 112,
            ControlTable::GoalPosition => 116,
            ControlTable::RealtimeTick => 120,
            ControlTable::Moving => 122,
            ControlTable::MovingStatus => 123,
            ControlTable::PresentPWM => 124,
            ControlTable::PresentCurrent => 126,
            ControlTable::PresentVelocity => 128,
            ControlTable::PresentPosition => 132,
            ControlTable::VelocityTrajectory => 136,
            ControlTable::PositionTrajectory => 140,
            ControlTable::PresentInputVoltage => 144,
            ControlTable::PresentTemperature => 146,
            ControlTable::BackupReady => 147,
            ControlTable::IndirectAddress1 => 168,
            ControlTable::IndirectAddress2 => 170,
            ControlTable::IndirectAddress3 => 172,
            ControlTable::IndirectAddress4 => 174,
            ControlTable::IndirectAddress5 => 176,
            ControlTable::IndirectAddress6 => 178,
            ControlTable::IndirectAddress7 => 180,
            ControlTable::IndirectAddress8 => 182,
            ControlTable::IndirectAddress9 => 184,
            ControlTable::IndirectAddress10 => 186,
            ControlTable::IndirectAddress11 => 188,
            ControlTable::IndirectAddress12 => 190,
            ControlTable::IndirectAddress13 => 192,
            ControlTable::IndirectAddress14 => 194,
            ControlTable::IndirectAddress15 => 196,
            ControlTable::IndirectAddress16 => 198,
            ControlTable::IndirectAddress17 => 200,
            ControlTable::IndirectAddress18 => 202,
            ControlTable::IndirectAddress19 => 204,
            ControlTable::IndirectAddress20 => 206,
            ControlTable::IndirectData1 => 208,
            ControlTable::IndirectData2 => 209,
            ControlTable::IndirectData3 => 210,
            ControlTable::IndirectData4 => 211,
            ControlTable::IndirectData5 => 212,
            ControlTable::IndirectData6 => 213,
            ControlTable::IndirectData7 => 214,
            ControlTable::IndirectData8 => 215,
            ControlTable::IndirectData9 => 216,
            ControlTable::IndirectData10 => 217,
            ControlTable::IndirectData11 => 218,
            ControlTable::IndirectData12 => 219,
            ControlTable::IndirectData13 => 220,
            ControlTable::IndirectData14 => 221,
            ControlTable::IndirectData15 => 222,
            ControlTable::IndirectData16 => 223,
            ControlTable::IndirectData17 => 224,
            ControlTable::IndirectData18 => 225,
            ControlTable::IndirectData19 => 226,
            ControlTable::IndirectData20 => 227,
        }
    }

    pub fn to_size(&self) -> u16 {
        match self {
            ControlTable::ModelNumber => 2,
            ControlTable::ModelInformation => 4,
            ControlTable::FirmwareVersion => 1,
            ControlTable::ID => 1,
            ControlTable::BaudRate => 1,
            ControlTable::ReturnDelayTime => 1,
            ControlTable::DriveMode => 1,
            ControlTable::OperatingMode => 1,
            ControlTable::SecondaryID => 1,
            ControlTable::ProtocolType => 1,
            ControlTable::HomingOffset => 4,
            ControlTable::MovingThreshold => 4,
            ControlTable::TemperatureLimit => 1,
            ControlTable::MaxVoltageLimit => 2,
            ControlTable::MinVoltageLimit => 2,
            ControlTable::PWMLimit => 2,
            ControlTable::CurrentLimit => 2,
            ControlTable::VelocityLimit => 4,
            ControlTable::MaxPositionLimit => 4,
            ControlTable::MinPositionLimit => 4,
            ControlTable::StartupConfiguration => 1,
            ControlTable::PWMSlope => 1,
            ControlTable::Shutdown => 1,
            ControlTable::TorqueEnable => 1,
            ControlTable::LED => 1,
            ControlTable::StatusReturnLevel => 1,
            ControlTable::RegisteredInstruction => 1,
            ControlTable::HardwareErrorStatus => 1,
            ControlTable::VelocityIGain => 2,
            ControlTable::VelocityPgain => 2,
            ControlTable::PositionDGain => 2,
            ControlTable::PositionIGain => 2,
            ControlTable::PositionPGain => 2,
            ControlTable::Feedforward2ndGain => 2,
            ControlTable::Feedforward1stGain => 2,
            ControlTable::BusWatchdog => 1,
            ControlTable::GoalPWM => 2,
            ControlTable::GoalCurrent => 2,
            ControlTable::GoalVelocity => 4,
            ControlTable::ProfileAccleration => 4,
            ControlTable::ProfileVelocity => 4,
            ControlTable::GoalPosition => 4,
            ControlTable::RealtimeTick => 2,
            ControlTable::Moving => 1,
            ControlTable::MovingStatus => 1,
            ControlTable::PresentPWM => 2,
            ControlTable::PresentCurrent => 2,
            ControlTable::PresentVelocity => 4,
            ControlTable::PresentPosition => 4,
            ControlTable::VelocityTrajectory => 4,
            ControlTable::PositionTrajectory => 4,
            ControlTable::PresentInputVoltage => 2,
            ControlTable::PresentTemperature => 1,
            ControlTable::BackupReady => 1,
            ControlTable::IndirectAddress1 => 2,
            ControlTable::IndirectAddress2 => 2,
            ControlTable::IndirectAddress3 => 2,
            ControlTable::IndirectAddress4 => 2,
            ControlTable::IndirectAddress5 => 2,
            ControlTable::IndirectAddress6 => 2,
            ControlTable::IndirectAddress7 => 2,
            ControlTable::IndirectAddress8 => 2,
            ControlTable::IndirectAddress9 => 2,
            ControlTable::IndirectAddress10 => 2,
            ControlTable::IndirectAddress11 => 2,
            ControlTable::IndirectAddress12 => 2,
            ControlTable::IndirectAddress13 => 2,
            ControlTable::IndirectAddress14 => 2,
            ControlTable::IndirectAddress15 => 2,
            ControlTable::IndirectAddress16 => 2,
            ControlTable::IndirectAddress17 => 2,
            ControlTable::IndirectAddress18 => 2,
            ControlTable::IndirectAddress19 => 2,
            ControlTable::IndirectAddress20 => 2,
            ControlTable::IndirectData1 => 1,
            ControlTable::IndirectData2 => 1,
            ControlTable::IndirectData3 => 1,
            ControlTable::IndirectData4 => 1,
            ControlTable::IndirectData5 => 1,
            ControlTable::IndirectData6 => 1,
            ControlTable::IndirectData7 => 1,
            ControlTable::IndirectData8 => 1,
            ControlTable::IndirectData9 => 1,
            ControlTable::IndirectData10 => 1,
            ControlTable::IndirectData11 => 1,
            ControlTable::IndirectData12 => 1,
            ControlTable::IndirectData13 => 1,
            ControlTable::IndirectData14 => 1,
            ControlTable::IndirectData15 => 1,
            ControlTable::IndirectData16 => 1,
            ControlTable::IndirectData17 => 1,
            ControlTable::IndirectData18 => 1,
            ControlTable::IndirectData19 => 1,
            ControlTable::IndirectData20 => 1,
        }
    }

    pub fn to_unit(&self, model: &DynamixelModel) -> f32 {
        match self {
            ControlTable::ModelNumber => 1.0,
            ControlTable::ModelInformation => 1.0,
            ControlTable::FirmwareVersion => 1.0,
            ControlTable::ID => 1.0,
            ControlTable::BaudRate => 1.0,
            ControlTable::ReturnDelayTime => 2.0,
            ControlTable::DriveMode => 1.0,
            ControlTable::OperatingMode => 1.0,
            ControlTable::SecondaryID => 1.0,
            ControlTable::ProtocolType => 1.0,
            ControlTable::HomingOffset => 2.0,
            ControlTable::MovingThreshold => 0.229,
            ControlTable::TemperatureLimit => 1.0,
            ControlTable::MaxVoltageLimit => 0.1,
            ControlTable::MinVoltageLimit => 0.1,
            ControlTable::PWMLimit => 0.113,
            ControlTable::CurrentLimit => 1.0,
            ControlTable::VelocityLimit => 0.229,
            ControlTable::MaxPositionLimit => 1.0,
            ControlTable::MinPositionLimit => 1.0,
            ControlTable::StartupConfiguration => 1.0,
            ControlTable::PWMSlope => 3.955,
            ControlTable::Shutdown => 1.0,
            ControlTable::TorqueEnable => 1.0,
            ControlTable::LED => 1.0,
            ControlTable::StatusReturnLevel => 1.0,
            ControlTable::RegisteredInstruction => 1.0,
            ControlTable::HardwareErrorStatus => 1.0,
            ControlTable::VelocityIGain => 1.0,
            ControlTable::VelocityPgain => 1.0,
            ControlTable::PositionDGain => 1.0,
            ControlTable::PositionIGain => 1.0,
            ControlTable::PositionPGain => 1.0,
            ControlTable::Feedforward2ndGain => 1.0,
            ControlTable::Feedforward1stGain => 1.0,
            ControlTable::BusWatchdog => 20.0,
            ControlTable::GoalPWM => 0.113,
            ControlTable::GoalCurrent => 1.0,
            ControlTable::GoalVelocity => 0.229,
            ControlTable::ProfileAccleration => 214.577,
            ControlTable::ProfileVelocity => 0.229,
            ControlTable::GoalPosition => 1.0,
            ControlTable::RealtimeTick => 1.0,
            ControlTable::Moving => 1.0,
            ControlTable::MovingStatus => 1.0,
            ControlTable::PresentPWM => 0.113,
            ControlTable::PresentCurrent => {
                match model{
                    DynamixelModel::Xm430W350 => 2.69,
                    DynamixelModel::Xc330T181 => 1.0,
                }
            },
            ControlTable::PresentVelocity => 0.229,
            ControlTable::PresentPosition => 1.0,
            ControlTable::VelocityTrajectory => 0.229,
            ControlTable::PositionTrajectory => 1.0,
            ControlTable::PresentInputVoltage => 0.1,
            ControlTable::PresentTemperature => 1.0,
            ControlTable::BackupReady => 1.0,
            ControlTable::IndirectAddress1 => 1.0,
            ControlTable::IndirectAddress2 => 1.0,
            ControlTable::IndirectAddress3 => 1.0,
            ControlTable::IndirectAddress4 => 1.0,
            ControlTable::IndirectAddress5 => 1.0,
            ControlTable::IndirectAddress6 => 1.0,
            ControlTable::IndirectAddress7 => 1.0,
            ControlTable::IndirectAddress8 => 1.0,
            ControlTable::IndirectAddress9 => 1.0,
            ControlTable::IndirectAddress10 => 1.0,
            ControlTable::IndirectAddress11 => 1.0,
            ControlTable::IndirectAddress12 => 1.0,
            ControlTable::IndirectAddress13 => 1.0,
            ControlTable::IndirectAddress14 => 1.0,
            ControlTable::IndirectAddress15 => 1.0,
            ControlTable::IndirectAddress16 => 1.0,
            ControlTable::IndirectAddress17 => 1.0,
            ControlTable::IndirectAddress18 => 1.0,
            ControlTable::IndirectAddress19 => 1.0,
            ControlTable::IndirectAddress20 => 1.0,
            ControlTable::IndirectData1 => 1.0,
            ControlTable::IndirectData2 => 1.0,
            ControlTable::IndirectData3 => 1.0,
            ControlTable::IndirectData4 => 1.0,
            ControlTable::IndirectData5 => 1.0,
            ControlTable::IndirectData6 => 1.0,
            ControlTable::IndirectData7 => 1.0,
            ControlTable::IndirectData8 => 1.0,
            ControlTable::IndirectData9 => 1.0,
            ControlTable::IndirectData10 => 1.0,
            ControlTable::IndirectData11 => 1.0,
            ControlTable::IndirectData12 => 1.0,
            ControlTable::IndirectData13 => 1.0,
            ControlTable::IndirectData14 => 1.0,
            ControlTable::IndirectData15 => 1.0,
            ControlTable::IndirectData16 => 1.0,
            ControlTable::IndirectData17 => 1.0,
            ControlTable::IndirectData18 => 1.0,
            ControlTable::IndirectData19 => 1.0,
            ControlTable::IndirectData20 => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::control_table::ControlTable;
    use crate::control_table::DynamixelModel;
    
    #[test]
    fn to_address_xc330() {
        let name = ControlTable::ModelNumber;
        assert_eq!(name.to_address(), 0);
        assert_eq!(ControlTable::TorqueEnable.to_address(), 64)
    }
    #[test]
    fn to_size_xc330() {
        let name = ControlTable::ModelNumber;
        assert_eq!(name.to_size(), 2);
        assert_eq!(ControlTable::TorqueEnable.to_size(), 1);
    }

    #[test]
    fn to_unit_xc330() {
        let name = ControlTable::ModelNumber;
        assert_eq!(name.to_unit(&DynamixelModel::Xc330T181), 1.0);
        assert_eq!(ControlTable::PresentPWM.to_unit(&DynamixelModel::Xc330T181), 0.113);
    }
}

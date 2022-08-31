#[cfg(any(feature = "xm430", feature = "xc330",))]
#[allow(dead_code)]

pub enum OperatingMode {
    CurrentControlMode,
    VelocityControlMode,
    PositionControlMode,
    ExtendedPosionControlMode,
    CurrentBasedPositionControlMode,
    PWMControMode,
}

#[allow(dead_code)]
impl OperatingMode {
    pub fn to_value(&self) -> u8 {
        match self {
            OperatingMode::CurrentControlMode => 0,
            OperatingMode::VelocityControlMode => 1,
            OperatingMode::PositionControlMode => 3,
            OperatingMode::ExtendedPosionControlMode => 4,
            OperatingMode::CurrentBasedPositionControlMode => 5,
            OperatingMode::PWMControMode => 16,
        }
    }
}



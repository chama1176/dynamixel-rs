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
    pub fn to_value(&self) -> u16 {
        match self {
            OperatingMode::CurrentControlMode => 0,
            OperatingMode::VelocityControlMode => 0,
            OperatingMode::VelocityControlMode => 0,
            OperatingMode::VelocityControlMode => 0,
            OperatingMode::VelocityControlMode => 0,
            OperatingMode::VelocityControlMode => 0,
        }
    }
}


pub enum ControlData {
    OperatingMode,

}


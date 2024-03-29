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

pub trait Pulse2Deg<T> {
    fn pulse2deg(self) -> T;
    fn deg2pulse(self) -> T;
}

#[allow(dead_code)]
pub mod dxl_consts {
    pub mod f32 {
        pub const HOME_POSITION: f32 = 2048.0;
        pub const TORQUE_CONST_XL330: f32 = 0.5; // Nm/A
    }
    pub mod f64 {
        pub const HOME_POSITION: f64 = 2048.0;
    }
}

impl Pulse2Deg<f32> for f32 {
    fn pulse2deg(self) -> f32 {
        self * 0.088
    }
    fn deg2pulse(self) -> f32 {
        self / 0.088
    }
}

impl Pulse2Deg<f64> for f64 {
    fn pulse2deg(self) -> f64 {
        self * 0.088
    }
    fn deg2pulse(self) -> f64 {
        self / 0.088
    }
}

pub trait DegRad<T> {
    fn deg2rad(self) -> T;
    fn rad2deg(self) -> T;
}

impl DegRad<f32> for f32 {
    fn deg2rad(self) -> f32 {
        self * core::f32::consts::PI / 180.0
    }
    fn rad2deg(self) -> f32 {
        self * 180.0 / core::f32::consts::PI
    }
}

impl DegRad<f64> for f64 {
    fn deg2rad(self) -> f64 {
        self * core::f64::consts::PI / 180.0
    }
    fn rad2deg(self) -> f64 {
        self * 180.0 / core::f64::consts::PI
    }
}

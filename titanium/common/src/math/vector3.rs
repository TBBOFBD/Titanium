use std::f64::consts::PI;

/// A 3D vector.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector3 {
    /// The x component of the vector.
    pub x: f32,
    /// The y component of the vector.
    pub y: f32,
    /// The z component of the vector.
    pub z: f32,
}

impl Vector3 {
    /// constructor
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    /// converts the vector to a angle
    pub fn to_angle(&self) -> Vector3 {
        Vector3 {
            x: (-self.z).atan2(self.x.hypot(self.y)) * (180.0 / PI) as f32,
            y: self.y.atan2(self.x) * (180.0 / PI) as f32,
            z: 0.0,
        }
    }

    /// checks if the vector is zero
    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, factor: f32) -> Vector3 {
        Vector3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, factor: f32) -> Vector3 {
        Vector3 {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
        }
    }
}
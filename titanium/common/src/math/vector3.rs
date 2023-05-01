use std::f64::consts::PI;
use std::ops::*;
use std::fmt::*;
use std::cmp::*;

use super::{
    IsNumber,
    IsFloat
};

/// The origin vector.
pub const ORIGIN: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);

/// A 3D vector.
#[derive(Copy, Clone, Eq)]
pub struct Vector3<T: IsFloat> {
    /// The x component of the vector.
    pub x: T,
    /// The y component of the vector.
    pub y: T,
    /// The z component of the vector.
    pub z: T,
}

impl<T: IsFloat> Vector3<T> {
    /// constructor
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// converts the vector to a angle
    #[cfg(not(feature = "less-memory"))]
    pub fn to_angle(&self) -> Vector3<impl IsFloat> {
        Vector3::new(
            (-self.z.to_f64()).atan2(self.x.to_f64().hypot(self.y.to_f64())) * (180.0 / PI),
            self.y.to_f64().atan2(self.x.to_f64()) * (180.0 / PI),
            0.0,
        )
    }

    /// converts the vector to a angle, using less memory
    #[cfg(feature = "less-memory")]
    pub fn to_angle(&self) -> Vector3<impl IsFloat> {
        Vector3::new(
            (-self.z.to_f32()).atan2(self.x.to_f32().hypot(self.y.to_f32())) * (180.0 / PI) as f32,
            self.y.to_f32().atan2(self.x.to_f32()) * (180.0 / PI) as f32,
            0.0,
        )
    }

    /// checks if the vector is zero
    #[cfg(not(feature = "less-memory"))]
    pub fn is_zero(&self) -> bool {
        self.x.to_f64() == 0.0 && self.y.to_f64() == 0.0 && self.z.to_f64() == 0.0
    }

    /// checks if the vector is zero, using less memory
    #[cfg(feature = "less-memory")]
    pub fn is_zero(&self) -> bool {
        self.x.to_f32() == 0.0 && self.y.to_f32() == 0.0 && self.z.to_f32() == 0.0
    }

    /// calculates the length of the vector
    #[cfg(not(feature = "less-memory"))]
    pub fn length(&self) -> impl IsFloat {
        self.x.to_f64().hypot(self.y.to_f64()).hypot(self.z.to_f64())
    }

    /// calculates the length of the vector, using less memory
    #[cfg(feature = "less-memory")]
    pub fn length(&self) -> impl IsFloat {
        self.x.to_f32().hypot(self.y.to_f32()).hypot(self.z.to_f32())
    }
    

    /// calculates the distance between two vectors
    #[cfg(not(feature = "less-memory"))]
    pub fn distance<T2: IsFloat, V: AsRef<Vector3<T2>>>(&self, other: V) -> impl IsFloat {
        let other = other.as_ref();
        (
            (other.x.to_f64() - self.x.to_f64()) +
            (other.y.to_f64() - self.y.to_f64()) +
            (other.z.to_f64() - self.z.to_f64())
        ).sqrt()
    }

    /// calculates the distance between two vectors, using less memory
    #[cfg(feature = "less-memory")]
    pub fn distance<T2: IsFloat, V: AsRef<Vector3<T2>>>(&self, other: V) -> impl IsFloat {
        let other = other.as_ref();
        (
            (other.x.to_f32() - self.x.to_f32()) +
            (other.y.to_f32() - self.y.to_f32()) +
            (other.z.to_f32() - self.z.to_f32())
        ).sqrt()
    }

    /// calculates the dot product of two vectors
    #[cfg(not(feature = "less-memory"))]
    pub fn dot<T2: IsFloat, V: AsRef<Vector3<T2>>>(&self, other: V) -> impl IsFloat {
        let other = other.as_ref();
        self.x.to_f64() * other.x.to_f64() + self.y.to_f64() * other.y.to_f64() + self.z.to_f64() * other.z.to_f64()
    }

    /// calculates the dot product of two vectors, using less memory
    #[cfg(feature = "less-memory")]
    pub fn dot<T2: IsFloat, V: AsRef<Vector3<T2>>>(&self, other: V) -> impl IsFloat {
        let other = other.as_ref();
        self.x.to_f32() * other.x.to_f32() + self.y.to_f32() * other.y.to_f32() + self.z.to_f32() * other.z.to_f32()
    }

    /// calculates the cross product of two vectors
    #[cfg(not(feature = "less-memory"))]
    pub fn cross<T2: IsFloat, V: AsRef<Vector3<T2>>>(&self, other: V) -> Vector3<impl IsFloat> {
        let other = other.as_ref();
        Vector3::new(
            self.y.to_f64() * other.z.to_f64() - self.z.to_f64() * other.y.to_f64(),
            self.z.to_f64() * other.x.to_f64() - self.x.to_f64() * other.z.to_f64(),
            self.x.to_f64() * other.y.to_f64() - self.y.to_f64() * other.x.to_f64(),
        )
    }

    /// calculates the cross product of two vectors, using less memory
    #[cfg(feature = "less-memory")]
    pub fn cross<T2: IsFloat, V: AsRef<Vector3<T2>>>(&self, other: V) -> Vector3<impl IsFloat> {
        let other = other.as_ref();
        Vector3::new(
            self.y.to_f32() * other.z.to_f32() - self.z.to_f32() * other.y.to_f32(),
            self.z.to_f32() * other.x.to_f32() - self.x.to_f32() * other.z.to_f32(),
            self.x.to_f32() * other.y.to_f32() - self.y.to_f32() * other.x.to_f32(),
        )
    }

    /// calculates the angle between two vectors
    #[cfg(not(feature = "less-memory"))]
    pub fn angle<T2: IsFloat, V: AsRef<Vector3<T2>>>(&self, other: V) -> impl IsFloat {
        let other = other.as_ref();
        self.dot(other).to_f64() / (self.length().to_f64() * other.length().to_f64())
    }

    /// calculates the angle between two vectors, using less memory
    #[cfg(feature = "less-memory")]
    pub fn angle<T2: IsFloat, V: AsRef<Vector3<T2>>>(&self, other: V) -> impl IsFloat {
        let other = other.as_ref();
        self.dot(other).to_f32() / (self.length().to_f32() * other.length().to_f32())
    }


    /// Converts the vector to a f64 vector.
    #[cfg(not(feature = "less-memory"))]
    pub fn as_f64_vector(&self) -> Vector3<f64> {
        Vector3::new(self.x.to_f64(), self.y.to_f64(), self.z.to_f64())
    }

    /// Converts the vector to a f32 vector.
    pub fn as_f32_vector(&self) -> Vector3<f32> {
        Vector3::new(self.x.to_f32(), self.y.to_f32(), self.z.to_f32())
    }
}

impl<T: IsFloat> Debug for Vector3<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Vector3 {{ x: {:?}, y: {:?}, z: {:?} }}", self.x, self.y, self.z)
    }
}

impl<T: IsFloat> AsRef<Vector3<T>> for Vector3<T> {
    fn as_ref(&self) -> &Vector3<T> {
        self
    }
}

impl<T: IsFloat> AsMut<Vector3<T>> for Vector3<T> {
    fn as_mut(&mut self) -> &mut Vector3<T> {
        self
    }
}

impl<T1: IsFloat, T2: IsFloat> PartialEq<Vector3<T2>> for Vector3<T1> {
    #[cfg(not(feature = "less-memory"))]
    fn eq(&self, other: &Vector3<T2>) -> bool {
        self.x.to_f64() == other.x.to_f64() &&
        self.y.to_f64() == other.y.to_f64() && 
        self.z.to_f64() == other.z.to_f64()
    }
    #[cfg(feature = "less-memory")]
    fn eq(&self, other: &Vector3<T2>) -> bool {
        self.x.to_f32() == other.x.to_f32() &&
        self.y.to_f32() == other.y.to_f32() && 
        self.z.to_f32() == other.z.to_f32()
    }
}

impl<T: IsFloat> PartialOrd for Vector3<T> {
    fn partial_cmp(&self, other: &Vector3<T>) -> Option<Ordering> {
        let d1 = self.distance(ORIGIN).to_float64();
        let d2 = other.distance(ORIGIN).to_float64();
        d1.partial_cmp(&d2)
    }

    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Less | Ordering::Equal))
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Greater | Ordering::Equal))
    }
}

impl<T: IsFloat> Add for Vector3<T> {
    type Output = Vector3<T>;
    fn add(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: IsFloat> Sub for Vector3<T> {
    type Output = Vector3<T>;
    fn sub(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: IsFloat> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;
    fn mul(self, factor: T) -> Vector3<T> {
        Vector3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

impl<T: IsFloat> Div<T> for Vector3<T> {
    type Output = Vector3<T>;
    fn div(self, factor: T) -> Vector3<T> {
        Vector3 {
            x: self.x / factor,
            y: self.y / factor,
            z: self.z / factor,
        }
    }
}

impl<T: IsFloat> Neg for Vector3<T> {
    type Output = Vector3<T>;
    fn neg(self) -> Vector3<T> {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: IsFloat> AddAssign for Vector3<T> {
    fn add_assign(&mut self, other: Vector3<T>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: IsFloat> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, other: Vector3<T>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: IsFloat> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, factor: T) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }
}

impl<T: IsFloat> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, factor: T) {
        self.x /= factor;
        self.y /= factor;
        self.z /= factor;
    }
}

impl<T: IsFloat> Index<usize> for Vector3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vector3"),
        }
    }
}

impl<T: IsFloat> IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds for Vector3"),
        }
    }
}

impl<T: IsFloat> Display for Vector3<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[{:?}, {:?}, {:?}]", self.x, self.y, self.z)
    }
}

impl<T: IsFloat> From<(T, T, T)> for Vector3<T> {
    fn from(tuple: (T, T, T)) -> Vector3<T> {
        Vector3 {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl<T: IsFloat> From<[T; 3]> for Vector3<T> {
    fn from(array: [T; 3]) -> Vector3<T> {
        Vector3 {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl<T: IsFloat> From<Vector3<T>> for (T, T, T) {
    fn from(vector: Vector3<T>) -> (T, T, T) {
        (vector.x, vector.y, vector.z)
    }
}

impl<T: IsFloat> From<Vector3<T>> for [T; 3] {
    fn from(vector: Vector3<T>) -> [T; 3] {
        [vector.x, vector.y, vector.z]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_to_angle() {
        let v = Vector3::new(1.0, 0.0, 1.0);
        let angle = v.to_angle();
        let angle = angle.as_f32_vector();
        assert_eq!(angle.x, -45.0);
        assert_eq!(angle.y, 0.0);
        assert_eq!(angle.z, 0.0);
    }

    #[test]
    fn test_is_zero() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(1.0, 0.0, 0.0);
        assert!(v1.is_zero());
        assert!(!v2.is_zero());
    }

    #[test]
    fn test_length() {
        let v = Vector3::new(1.0, 2.0, 2.0);
        assert_eq!(v.length().to_f64(), 3.0);
    }

    #[test]
    fn test_distance() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let distance = v1.distance(v2).to_f64();
        assert_eq!(distance, 3.0);
    }

    #[test]
    fn test_add() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 5.0);
        assert_eq!(v3.y, 7.0);
        assert_eq!(v3.z, 9.0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let v3 = v2 - v1;
        assert_eq!(v3.x, 3.0);
        assert_eq!(v3.y, 3.0);
        assert_eq!(v3.z, 3.0);
    }

    #[test]
    fn test_mul() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = v1 * 2.0;
        assert_eq!(v2.x, 2.0);
        assert_eq!(v2.y, 4.0);
        assert_eq!(v2.z, 6.0);
    }

    #[test]
    fn test_div() {
        let v1 = Vector3::new(2.0, 4.0, 6.0);
        let v2 = v1 / 2.0;
        assert_eq!(v2.x, 1.0);
        assert_eq!(v2.y, 2.0);
        assert_eq!(v2.z, 3.0);
    }

    #[test]
    fn test_neg() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = -v1;
        assert_eq!(v2.x, -1.0);
        assert_eq!(v2.y, -2.0);
        assert_eq!(v2.z, -3.0);
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        v1 += v2;
        assert_eq!(v1.x, 5.0);
        assert_eq!(v1.y, 7.0);
        assert_eq!(v1.z, 9.0);
    }

    #[test]
    fn test_sub_assign() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        v1 -= v2;
        assert_eq!(v1.x, -3.0);
        assert_eq!(v1.y, -3.0);
        assert_eq!(v1.z, -3.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        v1 *= 2.0;
        assert_eq!(v1.x, 2.0);
        assert_eq!(v1.y, 4.0);
        assert_eq!(v1.z, 6.0);
    }

    #[test]
    fn test_div_assign() {
        let mut v1 = Vector3::new(2.0, 4.0, 6.0);
        v1 /= 2.0;
        assert_eq!(v1.x, 1.0);
        assert_eq!(v1.y, 2.0);
        assert_eq!(v1.z, 3.0);
    }
}

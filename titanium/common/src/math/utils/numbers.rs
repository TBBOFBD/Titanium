use std::ops::*;
use std::fmt::*;

/// Trait for all number types
pub trait IsNumber: NumberWizardry +
    Copy + Clone + PartialEq + PartialOrd + Debug + Add<Output = Self> +
    Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
{
    /// Converts the number to a u8
    fn to_u8(&self) -> u8;
    /// Converts the number to a u16
    fn to_u16(&self) -> u16;
    /// Converts the number to a u32
    fn to_u32(&self) -> u32;
    /// Converts the number to a u64
    fn to_u64(&self) -> u64;
    /// Converts the number to a u128
    fn to_u128(&self) -> u128;
    /// Converts the number to a usize
    fn to_usize(&self) -> usize;
    /// Converts the number to a i8
    fn to_i8(&self) -> i8;
    /// Converts the number to a i16
    fn to_i16(&self) -> i16;
    /// Converts the number to a i32
    fn to_i32(&self) -> i32;
    /// Converts the number to a i64
    fn to_i64(&self) -> i64;
    /// Converts the number to a i128
    fn to_i128(&self) -> i128;
    /// Converts the number to a isize
    fn to_isize(&self) -> isize;
    /// Converts the number to a f32
    fn to_f32(&self) -> f32;
    /// Converts the number to a f64
    fn to_f64(&self) -> f64;
}

/// Trait for all float types
pub trait IsFloat: NumberWizardry +
    IsNumber + Neg<Output = Self> + AddAssign + SubAssign + MulAssign + DivAssign
{
    /// Converts the number to a f32
    fn to_float32(&self) -> f32;
    /// Converts the number to a f64
    fn to_float64(&self) -> f64;
}

/// perform wizardry on numbers
pub trait NumberWizardry {
    /// casts the number to an [`IsNumber`](crate::math::IsNumber)
    #[doc(hidden)]
    #[must_use = "this returns a new value instead of mutating its input"]
    fn as_isnumber(&self) -> isize;
    /// casts the number to an [`IsFloat`](crate::math::IsFloat)
    #[doc(hidden)]
    #[must_use = "this returns a new value instead of mutating its input"]
    fn as_isfloat(&self) -> f64;
}

#[doc(hidden)]
macro_rules! num_func {
    ($name:ident, $type:ty) => {
        fn $name(&self) -> $type {
            *self as $type
        }
    };
}

#[doc(hidden)]
macro_rules! impl_is_float {
    ($type:ty, $($t:tt)*) => {
        impl_is_float!($type);
        impl_is_float!($($t)*);
    };
    ($type:ty) => {
        impl IsFloat for $type {
            num_func!(to_float32,f32);
            num_func!(to_float64,f64);
        }
    };
}

#[doc(hidden)]
macro_rules! impl_is_number {
    ($type:ty, $($t:tt)*) => {
        impl_is_number!($type);
        impl_is_number!($($t)*);
    };
    ($type:ty) => {
        impl IsNumber for $type {
            num_func!(to_u8,u8);
            num_func!(to_u16,u16);
            num_func!(to_u32,u32);
            num_func!(to_u64,u64);
            num_func!(to_u128,u128);
            num_func!(to_usize,usize);
            num_func!(to_i8,i8);
            num_func!(to_i16,i16);
            num_func!(to_i32,i32);
            num_func!(to_i64,i64);
            num_func!(to_i128,i128);
            num_func!(to_isize,isize);
            num_func!(to_f32,f32);
            num_func!(to_f64,f64);
        }
    };
}

#[doc(hidden)]
macro_rules! impl_number_wizardry {
    ($type:ty, $($t:tt)*) => {
        impl_number_wizardry!($type);
        impl_number_wizardry!($($t)*);
    };
    ($type:ty) => {
        impl NumberWizardry for $type {
            fn as_isnumber(&self) -> isize {
                *self as isize
            }
            fn as_isfloat(&self) -> f64 {
                *self as f64
            }
        }
    };
}

impl_number_wizardry!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
impl_is_number!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
impl_is_float!(f32, f64);

/// Calculates the sine of a number
pub fn sin(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().sin()
}

/// Calculates the cosine of a number
pub fn cos(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().cos()
}

/// Calculates the tangent of a number
pub fn tan(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().tan()
}

/// Calculates the arcsine of a number
pub fn asin(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().asin()
}

/// Calculates the arccosine of a number
pub fn acos(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().acos()
}

/// Calculates the arctangent of a number
pub fn atan(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().atan()
}

/// Calculates the hyperbolic sine of a number
pub fn sinh(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().sinh()
}

/// Calculates the hyperbolic cosine of a number
pub fn cosh(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().cosh()
}

/// Calculates the hyperbolic tangent of a number
pub fn tanh(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().tanh()
}

/// Calculates the inverse hyperbolic sine of a number
pub fn asinh(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().asinh()
}

/// Calculates the inverse hyperbolic cosine of a number
pub fn acosh(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().acosh()
}

/// Calculates the inverse hyperbolic tangent of a number
pub fn atanh(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().atanh()
}

/// Calculates the square root of a number
pub fn sqrt(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().sqrt()
}

/// Calculates the reciprocal of a number
pub fn recip(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().recip()
}

/// Calculates the exponential of a number
pub fn exp(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().exp()
}

/// Calculates the natural logarithm of a number
pub fn ln(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().ln()
}

/// Calculates the logarithm of a number with respect to an arbitrary base
pub fn log(num: impl IsNumber, base: impl IsNumber) -> impl IsNumber {
    num.to_f64().log(base.to_f64())
}

/// Calculates the base 2 logarithm of a number
pub fn log2(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().log2()
}

/// Calculates the base 10 logarithm of a number
pub fn log10(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().log10()
}

/// Calculates the absolute value of a number
pub fn abs(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().abs()
}

/// Calculates the sign of a number
/// -1 if the number is less than 0, 1 if the number is greater than 0, 0 if the number is 0
pub fn signum(num: impl IsNumber) -> impl IsNumber {
    num.to_f64().signum().to_u8()
}

/// Calculates the minimum of two numbers
pub fn min(num1: impl IsNumber, num2: impl IsNumber) -> impl IsNumber {
    num1.to_f64().min(num2.to_f64())
}

/// Calculates the maximum of two numbers
pub fn max(num1: impl IsNumber, num2: impl IsNumber) -> impl IsNumber {
    num1.to_f64().max(num2.to_f64())
}

/// Calculates the power of a number
pub fn powf(num: impl IsNumber, power: impl IsFloat) -> impl IsNumber {
    num.to_f64().powf(power.to_f64())
}

/// Calculates the power of a number
pub fn powi(num: impl IsNumber, power: impl IsNumber) -> impl IsNumber {
    num.to_f64().powi(power.to_i32())
}

/// Calculates the power of a number
pub fn pow(num: impl IsNumber, power: impl IsNumber) -> impl IsNumber {
    num.to_f64().powf(power.to_f64())
}

/// Numeric types over which [`crate::Rect`] and [`crate::Vector2`] are defined
///
/// Types which implement this have basic operations and comparisons defined
/// that can work for all signed, unsigned numbers, integer, and floating point
/// numbers.
pub trait SpacialNumeric:
	std::cmp::PartialOrd
	+ Copy
	+ Clone
	+ std::fmt::Debug
	+ std::ops::Add<Self, Output = Self>
	+ std::ops::Sub<Self, Output = Self>
	+ std::ops::Mul<Self, Output = Self>
	+ std::ops::Div<Self, Output = Self>
	+ std::default::Default
{
}

/// Types which as well as being defined as SpacialNumeric, may be negated.
pub trait SignedSpacialNumeric: SpacialNumeric + std::ops::Neg<Output = Self> {}

/// Define the conversion between two SpacialNumeric types.
///
/// # Example
///
/// ```
/// # use riddle_math::*;
/// let a: u32 = 1;
/// let b: f32 = a.convert();
/// ```
pub trait SpacialNumericConversion<T> {
	/// Convert one SpacialNumeric value to another. This conversion can not fail.
	fn convert(self) -> T;
}

impl<T: SpacialNumeric> SpacialNumericConversion<T> for T {
	#[inline]
	fn convert(self) -> T {
		self
	}
}

macro_rules! define_spacial_numeric {
    (Conv, $f:ty, As($t:ty)) => {
        impl SpacialNumericConversion<$t> for $f {
            #[inline]
            fn convert(self) -> $t {
                self as $t
            }
        }
    };

    (Unsigned, $f:ty, ( $($ts:ident($t:ty)),* )) => {
        impl SpacialNumeric for $f {}

        $( define_spacial_numeric!(Conv, $f, $ts($t)); )*
    };
    (Signed, $f:ty, ( $($ts:ident($t:ty)),* )) => {
        impl SignedSpacialNumeric for $f {}

        define_spacial_numeric!(Unsigned, $f, ($( $ts($t) ),*));
    };
}

define_spacial_numeric!(Unsigned, u32, (As(i32), As(u64), As(i64), As(f32), As(f64)));
define_spacial_numeric!(Signed, i32, (As(u32), As(u64), As(i64), As(f32), As(f64)));
define_spacial_numeric!(Unsigned, u64, (As(u32), As(i32), As(i64), As(f32), As(f64)));
define_spacial_numeric!(Signed, i64, (As(u32), As(i32), As(u64), As(f32), As(f64)));
define_spacial_numeric!(Signed, f32, (As(u32), As(i32), As(u64), As(i64), As(f64)));
define_spacial_numeric!(Signed, f64, (As(u32), As(i32), As(u64), As(i64), As(f32)));

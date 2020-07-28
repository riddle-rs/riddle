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

pub trait SignedSpacialNumeric: SpacialNumeric + std::ops::Neg<Output = Self> {}

pub trait SpacialNumericConversion<T> {
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

define_spacial_numeric!(Unsigned, u32, (As(i32), As(f32)));
define_spacial_numeric!(Signed, i32, (As(u32), As(f32)));
define_spacial_numeric!(Signed, f32, (As(u32), As(i32)));

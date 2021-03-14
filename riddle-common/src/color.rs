/////////////////////////////////////////////////////////////////////////////
// trait ColorElement
/////////////////////////////////////////////////////////////////////////////

/// A type that represents a single channel in a color value.
pub trait ColorElement: Copy {
    const ZERO: Self;
    const SATURATED: Self;
}

/// A u8 represents a color channel value in the range 0-255
impl ColorElement for u8 {
    const ZERO: u8 = 0;
    const SATURATED: u8 = 255;
}

/// An f32 represents a color channel value in the range 0.0 - 1.0
impl ColorElement for f32 {
    const ZERO: f32 = 0.0;
    const SATURATED: f32 = 1.0;
}

/////////////////////////////////////////////////////////////////////////////
// trait ColorElementConversion
/////////////////////////////////////////////////////////////////////////////

/// Define the mapping between two ColorElement types.
///
/// Implemented by both individual color channel types, and compound types like
/// [`Color`].
pub trait ColorElementConversion<T> {
    /// Given a value that implements this trait, produce an equivalent color
    /// element of the destination type.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_common::*;
    /// // Convert a float color channel value in to a byte color channel value.
    /// let byte_val: u8 = 255;
    /// let float_val: f32 = 1.0;
    /// assert_eq!(byte_val, float_val.convert());
    /// ```
    fn convert(&self) -> T;
}

impl ColorElementConversion<f32> for u8 {
    #[inline]
    fn convert(&self) -> f32 {
        (*self as f32) / 255.0
    }
}

impl ColorElementConversion<u8> for u8 {
    #[inline]
    fn convert(&self) -> Self {
        *self
    }
}

impl ColorElementConversion<u8> for f32 {
    #[inline]
    fn convert(&self) -> u8 {
        (self * 255.0) as u8
    }
}

impl ColorElementConversion<f32> for f32 {
    #[inline]
    fn convert(&self) -> Self {
        *self
    }
}

/////////////////////////////////////////////////////////////////////////////
// struct Color
/////////////////////////////////////////////////////////////////////////////

/// An RGBA color, parameterized over color channel type.
///
/// The two supported channel types are [`u8`] and [`f32`].
///
/// # Example
///
/// ```
/// # use riddle_common::*;
/// let c = Color{ r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
/// ```
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Color<E> {
    /// Red
    pub r: E,

    /// Green
    pub g: E,

    /// Blue
    pub b: E,

    /// Alpha
    pub a: E,
}

impl<E: ColorElement> Color<E> {
    /// Opaque primary red
    pub const RED: Self = Self {
        r: E::SATURATED,
        g: E::ZERO,
        b: E::ZERO,
        a: E::SATURATED,
    };

    /// Opaque primary green
    pub const GREEN: Self = Self {
        r: E::ZERO,
        g: E::SATURATED,
        b: E::ZERO,
        a: E::SATURATED,
    };

    /// Opaque primary blue
    pub const BLUE: Self = Self {
        r: E::ZERO,
        g: E::ZERO,
        b: E::SATURATED,
        a: E::SATURATED,
    };

    /// Opaque black
    pub const BLACK: Self = Self {
        r: E::ZERO,
        g: E::ZERO,
        b: E::ZERO,
        a: E::SATURATED,
    };

    /// Opaque white
    pub const WHITE: Self = Self {
        r: E::SATURATED,
        g: E::SATURATED,
        b: E::SATURATED,
        a: E::SATURATED,
    };

    /// Transparent black
    pub const TRANSPARENT_BLACK: Self = Self {
        r: E::ZERO,
        g: E::ZERO,
        b: E::ZERO,
        a: E::ZERO,
    };

    /// Transparent black
    pub const ZERO: Self = Self {
        r: E::ZERO,
        g: E::ZERO,
        b: E::ZERO,
        a: E::ZERO,
    };

    /// Build a color using all 4 channels.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_common::*;
    /// let c = Color::rgba(1.0, 0.0, 0.0, 1.0);
    /// ```
    #[inline]
    pub fn rgba(r: E, g: E, b: E, a: E) -> Self {
        Self { r, g, b, a }
    }

    /// Build an opaque color using rgb channels.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_common::*;
    /// let c = Color::rgb(1.0, 0.0, 0.0);
    /// assert_eq!(Color::rgba(1.0, 0.0, 0.0, 1.0), c);
    /// ```
    #[inline]
    pub fn rgb(r: E, g: E, b: E) -> Self {
        Self {
            r,
            g,
            b,
            a: E::SATURATED,
        }
    }
}

impl Color<u8> {
    /// Convert the color in to an RGBA32.
    ///
    /// Note that casting this value to a &[u8] will result in platform dependent component
    /// ordering.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_common::*;
    /// let c = Color::rgb(0, 255, 0);
    /// assert_eq!(0x00FF00FF, c.into_rgba8());
    /// ```
    pub fn into_rgba8(self) -> u32 {
        (self.r as u32) << 24 | (self.g as u32) << 16 | (self.b as u32) << 8 | (self.a as u32)
    }
}

/// Support converting colors between element types
///
/// # Example
///
/// ```
/// # use riddle_common::*;
/// let a: Color<f32> = Color::RED;
/// let b: Color<u8> = Color::RED;
/// let a_converted: Color<u8> = a.convert();
/// assert_eq!(b, a_converted);
/// ```
impl<T: ColorElement, F: ColorElementConversion<T>> ColorElementConversion<Color<T>> for Color<F> {
    #[inline]
    fn convert(&self) -> Color<T> {
        Color::rgba(
            self.r.convert(),
            self.g.convert(),
            self.b.convert(),
            self.a.convert(),
        )
    }
}

impl<E: PartialEq> PartialEq for Color<E> {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}

impl<E: PartialEq> Eq for Color<E> {}

impl<E: ColorElement> From<[E; 4]> for Color<E> {
    #[inline]
    fn from(c: [E; 4]) -> Self {
        Self::rgba(c[0], c[1], c[2], c[3])
    }
}

impl<E: ColorElement> From<[E; 3]> for Color<E> {
    #[inline]
    fn from(c: [E; 3]) -> Self {
        Self::rgb(c[0], c[1], c[2])
    }
}

impl<E: Copy> From<Color<E>> for [E; 4] {
    #[inline]
    fn from(c: Color<E>) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

impl<E: Copy> From<Color<E>> for [E; 3] {
    #[inline]
    fn from(c: Color<E>) -> Self {
        [c.r, c.g, c.b]
    }
}

pub trait ColorElement: Copy {
    const ZERO: Self;
    const SATURATED: Self;
}

pub trait ColorElementConversion<T> {
    fn convert(&self) -> T;
}

impl ColorElement for u8 {
    const ZERO: u8 = 0;
    const SATURATED: u8 = 255;
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

impl ColorElement for f32 {
    const ZERO: f32 = 0.0;
    const SATURATED: f32 = 1.0;
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

#[repr(C)]
#[derive(Debug)]
pub struct Color<E> {
    pub r: E,
    pub g: E,
    pub b: E,
    pub a: E,
}

impl<E> Color<E> {
    #[inline]
    pub fn rgba(r: E, g: E, b: E, a: E) -> Self {
        Self { r, g, b, a }
    }
}

impl<E: ColorElement> Color<E> {
    pub const RED: Self = Self {
        r: E::SATURATED,
        g: E::ZERO,
        b: E::ZERO,
        a: E::SATURATED,
    };

    pub const GREEN: Self = Self {
        r: E::ZERO,
        g: E::SATURATED,
        b: E::ZERO,
        a: E::SATURATED,
    };

    pub const BLUE: Self = Self {
        r: E::ZERO,
        g: E::ZERO,
        b: E::SATURATED,
        a: E::SATURATED,
    };

    pub const BLACK: Self = Self {
        r: E::ZERO,
        g: E::ZERO,
        b: E::ZERO,
        a: E::SATURATED,
    };

    pub const WHITE: Self = Self {
        r: E::SATURATED,
        g: E::SATURATED,
        b: E::SATURATED,
        a: E::SATURATED,
    };

    pub const TRANSPARENT_BLACK: Self = Self {
        r: E::ZERO,
        g: E::ZERO,
        b: E::ZERO,
        a: E::ZERO,
    };

    pub const ZERO: Self = Self {
        r: E::ZERO,
        g: E::ZERO,
        b: E::ZERO,
        a: E::ZERO,
    };

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

impl<E: Copy> From<[E; 4]> for Color<E> {
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

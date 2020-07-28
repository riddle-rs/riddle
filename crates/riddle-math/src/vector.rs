use crate::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

#[inline]
pub fn vec2<T>(x: T, y: T) -> Vector2<T> {
    Vector2::new(x, y)
}

impl<T> Vector2<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Default> Default for Vector2<T> {
    #[inline]
    fn default() -> Self {
        Vector2 {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl<T: Copy> From<[T; 2]> for Vector2<T> {
    #[inline]
    fn from(a: [T; 2]) -> Self {
        Vector2 { x: a[0], y: a[1] }
    }
}

impl<T: Copy> From<mint::Vector2<T>> for Vector2<T> {
    #[inline]
    fn from(v: mint::Vector2<T>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl<T: SpacialNumeric> std::ops::Add<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;

    #[inline]
    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: SpacialNumeric> std::ops::Sub<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;

    #[inline]
    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SignedSpacialNumeric> std::ops::Neg for Vector2<T> {
    type Output = Vector2<T>;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: SpacialNumeric> std::ops::Mul<T> for Vector2<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> From<glam::Vec2> for Vector2<T>
where
    f32: SpacialNumericConversion<T>,
{
    #[inline]
    fn from(v: glam::Vec2) -> Self {
        Self {
            x: v.x().convert(),
            y: v.y().convert(),
        }
    }
}

impl<T: SpacialNumericConversion<f32>> From<Vector2<T>> for glam::Vec2 {
    #[inline]
    fn from(v: Vector2<T>) -> Self {
        glam::vec2(v.x.convert(), v.y.convert())
    }
}

impl<T: SpacialNumeric> Into<mint::Vector2<T>> for Vector2<T> {
    #[inline]
    fn into(self) -> mint::Vector2<T> {
        mint::Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl<T: SpacialNumericConversion<U>, U> SpacialNumericConversion<Vector2<U>> for Vector2<T> {
    #[inline]
    fn convert(self) -> Vector2<U> {
        Vector2 {
            x: self.x.convert(),
            y: self.y.convert(),
        }
    }
}

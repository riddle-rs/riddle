use crate::*;

/// A basic 2d vector, supporting a small selection of operations
///
/// Supported operations (depending on element type):
///
/// * Add
/// * Subtract
/// * Mul
/// * Neg (where T: std::ops::Neg)
/// * Converting to vectors of other element types
///
/// # Example
///
/// ```
/// # use riddle_math::*;
/// let v = vec2(1,2);
/// assert_eq!(vec2(2, 4), v + v);
/// assert_eq!(vec2(0, 0), v - v);
/// assert_eq!(vec2(3, 6), v * 3);
/// assert_eq!(vec2(-1, -2), -v);
///
/// assert_eq!(vec2(1.0, 2.0), v.convert());
/// ```
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

/// Utility function to abbreviate [`Vector2`] creation
///
/// # Example
///
/// ```
/// # use riddle_math::*;
/// assert_eq!(vec2(1,2), Vector2::new(1,2));
/// ```
#[inline]
pub fn vec2<T>(x: T, y: T) -> Vector2<T> {
    Vector2::new(x, y)
}

impl<T> Vector2<T> {
    /// Create a vector with the given coordinatates.
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: SpacialNumeric> Vector2<T> {
    /// Calculate the square of the magnitude of the vector
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_math::*;
    /// let v = Vector2::new(2, 2);
    /// assert_eq!(8, v.magnitude_squared());
    /// ```
    #[inline]
    pub fn magnitude_squared(&self) -> T {
        (self.x * self.x) + (self.y * self.y)
    }
}

impl<T: PartialEq> PartialEq for Vector2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: PartialEq> Eq for Vector2<T> {}

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

impl<T: SpacialNumeric> std::ops::AddAssign<Vector2<T>> for Vector2<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Vector2<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
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

impl<T: SpacialNumeric> std::ops::SubAssign<Vector2<T>> for Vector2<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Vector2<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
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
            x: v.x.convert(),
            y: v.y.convert(),
        }
    }
}

impl<T: SpacialNumericConversion<f32>> From<Vector2<T>> for glam::Vec2 {
    #[inline]
    fn from(v: Vector2<T>) -> Self {
        glam::vec2(v.x.convert(), v.y.convert())
    }
}

impl<T: SpacialNumeric> From<Vector2<T>> for mint::Vector2<T> {
    #[inline]
    fn from(vec: Vector2<T>) -> Self {
        mint::Vector2 { x: vec.x, y: vec.y }
    }
}

impl<T: SpacialNumeric> From<Vector2<T>> for [T; 2] {
    #[inline]
    fn from(vec: Vector2<T>) -> Self {
        [vec.x, vec.y]
    }
}

/// Vectors are convertible between numeric types
///
/// # Example
///
/// ```
/// # use riddle_math::*;
/// let v: Vector2<f32> = vec2(3.0, 4.0);
/// let w: Vector2<u32> = v.convert();
/// assert_eq!(vec2(3, 4), w);
/// ```
impl<T: SpacialNumericConversion<U>, U> SpacialNumericConversion<Vector2<U>> for Vector2<T> {
    #[inline]
    fn convert(self) -> Vector2<U> {
        Vector2 {
            x: self.x.convert(),
            y: self.y.convert(),
        }
    }
}

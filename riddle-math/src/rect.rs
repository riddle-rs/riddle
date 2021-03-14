use crate::*;

/// An axis aligned 2d rectangle with both a location and size.
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct Rect<T> {
    /// The coordinates of the min point of the rectangle.
    pub location: Vector2<T>,

    /// The size of the rectangle
    pub dimensions: Vector2<T>,
}

impl<T> Rect<T>
where
    T: SpacialNumeric,
{
    /// Create a new rect
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_math::*;
    /// let rect = Rect::new(vec2(0,0), vec2(10,10));
    /// ```
    pub fn new<V: Into<Vector2<T>>>(position: V, size: V) -> Self {
        Rect {
            location: position.into(),
            dimensions: size.into(),
        }
    }

    /// Get the min point of the rect, the same as its location
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_math::*;
    /// let rect = Rect::new(vec2(0,0), vec2(10,10));
    /// assert_eq!(vec2(0,0), rect.min_point());
    /// ```
    pub fn min_point(&self) -> Vector2<T> {
        self.location
    }

    /// Get the max point of the rect
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_math::*;
    /// let rect = Rect::new(vec2(5,5), vec2(10,10));
    /// assert_eq!(vec2(15,15), rect.max_point());
    /// ```
    pub fn max_point(&self) -> Vector2<T> {
        self.location + self.dimensions
    }

    /// Get the intersection rect of two rectangles, if one exists
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_math::*;
    /// let rect_a = Rect::new(vec2(0,0), vec2(10,10));
    /// let rect_b = Rect::new(vec2(5,5), vec2(10,10));
    /// assert_eq!(Some(Rect::new(vec2(5,5), vec2(5,5))), rect_a.intersect(&rect_b));
    ///
    /// let rect_c = Rect::new(vec2(0, 0), vec2(1,1));
    /// assert_eq!(None, rect_b.intersect(&rect_c));
    /// ```
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let Vector2 {
            x: self_min_x,
            y: self_min_y,
        } = self.min_point();
        let Vector2 {
            x: self_max_x,
            y: self_max_y,
        } = self.max_point();
        let Vector2 {
            x: other_min_x,
            y: other_min_y,
        } = other.min_point();
        let Vector2 {
            x: other_max_x,
            y: other_max_y,
        } = other.max_point();

        if self_min_x < other_max_x
            && self_max_x > other_min_x
            && self_max_y > other_min_y
            && self_min_y < other_max_y
        {
            let min_point = Vector2 {
                x: if self_min_x > other_min_x {
                    self_min_x
                } else {
                    other_min_x
                },
                y: if self_min_y > other_min_y {
                    self_min_y
                } else {
                    other_min_y
                },
            };

            let max_point = Vector2 {
                x: if self_max_x < other_max_x {
                    self_max_x
                } else {
                    other_max_x
                },
                y: if self_max_y < other_max_y {
                    self_max_y
                } else {
                    other_max_y
                },
            };

            Some(Rect {
                location: min_point,
                dimensions: max_point - min_point,
            })
        } else {
            None
        }
    }

    /// Test to see whether a point lies within the rect.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_math::*;
    /// let rect = Rect::new(vec2(0,0), vec2(1,1));
    /// assert_eq!(true, rect.contains_point(vec2(0,0)));
    /// assert_eq!(false, rect.contains_point(vec2(1,1)));
    /// ```
    pub fn contains_point(&self, point: Vector2<T>) -> bool {
        let min_point = self.min_point();
        let max_point = self.max_point();

        !(point.x < min_point.x
            || point.y < min_point.y
            || point.x >= max_point.x
            || point.y >= max_point.y)
    }
}

impl<T> Rect<T>
where
    T: SignedSpacialNumeric,
{
    /// Given the dimensions of two rects, and the relative offset of the second
    /// with respect to the first, calculate the intersection between the two rects,
    /// and return rects defining the intersection relative to each of the inputs.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_math::*;
    /// let rect_dimensions_a = vec2(4,4);
    /// let rect_dimensions_b = vec2(5,5);
    /// let b_relative_to_a = vec2(2,2);
    ///
    /// let (rect_a, rect_b) = Rect::intersect_relative_to_both(rect_dimensions_a,
    ///     rect_dimensions_b, b_relative_to_a).unwrap();
    ///
    /// assert_eq!(Rect::new(vec2(2,2), vec2(2,2)), rect_a);
    /// assert_eq!(Rect::new(vec2(0,0), vec2(2,2)), rect_b);
    /// ```
    pub fn intersect_relative_to_both<S: SpacialNumericConversion<T>>(
        size_a: Vector2<S>,
        size_b: Vector2<S>,
        b_relative_position: Vector2<T>,
    ) -> Option<(Self, Self)> {
        let mut rect_a = Rect {
            location: Default::default(),
            dimensions: size_a.convert(),
        };

        let mut rect_b = Rect {
            location: b_relative_position,
            dimensions: size_b.convert(),
        };

        let rel_a_rect = rect_a.intersect(&rect_b);

        rect_b.location = rect_a.location;
        rect_a.location = -b_relative_position;

        let rel_b_rect = rect_b.intersect(&rect_a);

        if let (Some(rel_a), Some(rel_b)) = (rel_a_rect, rel_b_rect) {
            Some((rel_a, rel_b))
        } else {
            None
        }
    }
}

impl<T: SpacialNumericConversion<U>, U> SpacialNumericConversion<Rect<U>> for Rect<T> {
    #[inline]
    fn convert(self) -> Rect<U> {
        Rect {
            location: self.location.convert(),
            dimensions: self.dimensions.convert(),
        }
    }
}

impl<T: PartialEq> PartialEq for Rect<T> {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location && self.dimensions == other.dimensions
    }
}

impl<T: PartialEq> Eq for Rect<T> {}

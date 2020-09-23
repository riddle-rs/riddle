use crate::*;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Rect<T> {
    pub location: Vector2<T>,
    pub dimensions: Vector2<T>,
}

impl<T> Rect<T>
where
    T: SpacialNumeric,
{
    pub fn new<V: Into<Vector2<T>>>(position: V, size: V) -> Self {
        Rect {
            location: position.into(),
            dimensions: size.into(),
        }
    }

    pub fn min_point(&self) -> Vector2<T> {
        self.location
    }

    pub fn max_point(&self) -> Vector2<T> {
        self.location + self.dimensions
    }

    pub fn intersect(&self, other: &Self) -> Self {
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

        Rect {
            location: min_point,
            dimensions: max_point - min_point,
        }
    }

    pub fn contains_point(&self, point: Vector2<T>) -> bool {
        let min_point = self.min_point();
        let max_point = self.max_point();

        !(point.x < min_point.x
            || point.y < min_point.y
            || point.x > max_point.x
            || point.y > max_point.y)
    }
}

impl<T> Rect<T>
where
    T: SignedSpacialNumeric,
{
    pub fn intersect_relative_to_both<S: SpacialNumericConversion<T>>(
        size_a: Vector2<S>,
        size_b: Vector2<S>,
        b_relative_position: Vector2<T>,
    ) -> (Self, Self) {
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

        (rel_a_rect, rel_b_rect)
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

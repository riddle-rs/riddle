#[repr(C)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    #[inline]
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    #[inline]
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
}

impl From<[f32; 4]> for Color {
    #[inline]
    fn from(c: [f32; 4]) -> Self {
        Self::rgba(c[0], c[1], c[2], c[3])
    }
}

impl From<[f32; 3]> for Color {
    #[inline]
    fn from(c: [f32; 3]) -> Self {
        Self::rgb(c[0], c[1], c[2])
    }
}

impl From<[u8; 4]> for Color {
    #[inline]
    fn from(c: [u8; 4]) -> Self {
        Self::rgba(
            c[0] as f32 / 255.0,
            c[1] as f32 / 255.0,
            c[2] as f32 / 255.0,
            c[3] as f32 / 255.0,
        )
    }
}

impl From<[u8; 3]> for Color {
    #[inline]
    fn from(c: [u8; 3]) -> Self {
        Self::rgb(
            c[0] as f32 / 255.0,
            c[1] as f32 / 255.0,
            c[2] as f32 / 255.0,
        )
    }
}

impl From<Color> for [f32; 4] {
    fn from(c: Color) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

impl From<Color> for [u8; 4] {
    fn from(c: Color) -> Self {
        [
            (c.r * 255.0) as u8,
            (c.g * 255.0) as u8,
            (c.b * 255.0) as u8,
            (c.a * 255.0) as u8,
        ]
    }
}

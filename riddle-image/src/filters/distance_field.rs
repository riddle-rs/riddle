use crate::image_ext::*;
use crate::*;

use ::image::Pixel;
use riddle_math::{Rect, SpacialNumericConversion, Vector2};

const GRID_INF: Vector2<i64> = Vector2 {
    x: 0xFFFF,
    y: 0xFFFF,
};

/// Generate a distance field for the given image using 8SSEDT. The scale argument roughly maps to
/// how many pixels deep the boundary region is between min and max values.
///
/// # Examples
///
/// ```
/// # use riddle_image::*; fn main() -> Result<(), ImageError> {
/// let png_bytes = include_bytes!("../../examples/image-distance-field/sample.png");
/// let png_img = Image::load(&png_bytes[..], ImageFormat::Png)?;
///
/// let processed_img = filters::distance_field(png_img, 10.0);
/// # Ok(()) }
/// ```
///
/// # Resources:
///
/// * <http://www.codersnotes.com/notes/signed-distance-fields/>
/// * <https://github.com/Lisapple/8SSEDT>
pub fn distance_field(source: Image, scale: f64) -> Image {
    let source_bounds: Rect<i64> = source.rect().convert();
    let source_img = source.image_rgbaimage();
    let mut inside_grid = vec![Vector2::new(0, 0); (source.width() * source.height()) as usize];
    let mut outside_grid = vec![Vector2::new(0, 0); (source.width() * source.height()) as usize];

    // Generate initial grids
    for y in 0..source.height() {
        for x in 0..source.width() {
            let grid_offset = (y * source.width() + x) as usize;
            let source_pixel = source_img.get_pixel(x, y).to_luma();
            if source_pixel.0[0] > 127 {
                outside_grid[grid_offset] = GRID_INF;
            } else {
                inside_grid[grid_offset] = GRID_INF;
            }
        }
    }

    do_grid_passes(&mut inside_grid, &source_bounds);
    do_grid_passes(&mut outside_grid, &source_bounds);

    let result_img = ::image::ImageBuffer::from_fn(source.width(), source.height(), |x, y| {
        let p_inside = get_grid_point(&inside_grid, &source_bounds, Vector2::new(x, y).convert());
        let p_outside = get_grid_point(&outside_grid, &source_bounds, Vector2::new(x, y).convert());

        let len_inside = (p_inside.magnitude_squared() as f64).sqrt();
        let len_outside = (p_outside.magnitude_squared() as f64).sqrt();

        let val = (((len_outside - len_inside) / scale) + 0.5).clamp(0.0, 1.0);

        ::image::Luma([(val * 255.0) as u8])
    });

    let dyn_img: ::image::DynamicImage = ::image::DynamicImage::ImageLuma8(result_img);
    Image::image_from_dynimage(dyn_img)
}

fn get_grid_point(
    grid: &[Vector2<i64>],
    bounds: &Rect<i64>,
    location: Vector2<i64>,
) -> Vector2<i64> {
    if bounds.contains_point(location) {
        grid[((location.y * bounds.dimensions.x) + location.x) as usize]
    } else {
        GRID_INF
    }
}

fn set_grid_point(
    grid: &mut Vec<Vector2<i64>>,
    bounds: &Rect<i64>,
    location: Vector2<i64>,
    value: Vector2<i64>,
) {
    grid[((location.y * bounds.dimensions.x) + location.x) as usize] = value;
}

fn do_grid_passes(grid: &mut Vec<Vector2<i64>>, bounds: &Rect<i64>) {
    do_pass(
        grid,
        bounds,
        0_i64..bounds.dimensions.y,
        0_i64..bounds.dimensions.x,
        &[
            Vector2 { x: -1, y: -1 },
            Vector2 { x: 0, y: -1 },
            Vector2 { x: 1, y: -1 },
            Vector2 { x: -1, y: 0 },
            Vector2 { x: 0, y: 0 },
        ],
    );

    do_pass(
        grid,
        bounds,
        0_i64..bounds.dimensions.y,
        (0_i64..bounds.dimensions.x).rev(),
        &[Vector2 { x: 0, y: 0 }, Vector2 { x: 1, y: 0 }],
    );

    do_pass(
        grid,
        bounds,
        (0_i64..bounds.dimensions.y).rev(),
        (0_i64..bounds.dimensions.x).rev(),
        &[
            Vector2 { x: 0, y: 0 },
            Vector2 { x: 1, y: 0 },
            Vector2 { x: -1, y: 1 },
            Vector2 { x: 0, y: 1 },
            Vector2 { x: 1, y: 1 },
        ],
    );

    do_pass(
        grid,
        bounds,
        (0_i64..bounds.dimensions.y).rev(),
        0_i64..bounds.dimensions.x,
        &[Vector2 { x: -1, y: 0 }, Vector2 { x: 0, y: 0 }],
    );
}

fn do_pass<YIter: Iterator<Item = i64>, XIter: Iterator<Item = i64> + Clone>(
    grid: &mut Vec<Vector2<i64>>,
    bounds: &Rect<i64>,
    y_range: YIter,
    x_range: XIter,
    neighbours: &[Vector2<i64>],
) {
    for y in y_range {
        for x in x_range.clone() {
            let location = Vector2::new(x, y);
            let new_val = neighbours
                .iter()
                .map(|offset| {
                    let other = get_grid_point(grid, bounds, location + *offset);
                    other + *offset
                })
                .min_by_key(|p| p.magnitude_squared())
                .unwrap();
            set_grid_point(grid, bounds, location, new_val);
        }
    }
}

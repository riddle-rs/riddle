use riddle_image::*;
use std::env;
use std::fs::File;

const OUT_FILE: &str = "image-distance-field.out.png";
const FIELD_SCALE: f64 = 20.0;

fn main() -> Result<(), ImageError> {
    let png_bytes = include_bytes!("sample.png");
    let png_img = Image::load(&png_bytes[..], ImageFormat::Png)?;
    println!("+ Image Loaded...");

    let processed_img = filters::distance_field(png_img, FIELD_SCALE);
    println!("+ Distance Field Calculated...");

    let mut out_path = env::temp_dir();
    out_path.push(OUT_FILE);

    let out_file = File::create(out_path.clone())?;
    processed_img.save(out_file, ImageFormat::Png)?;
    println!("+ Image Saved ({:?})...", out_path);
    println!("+ Done.");

    Ok(())
}

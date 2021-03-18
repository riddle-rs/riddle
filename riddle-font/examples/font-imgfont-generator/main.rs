use std::{env, fs::File};

use riddle_font::*;
use riddle_image::ImageFormat;

const IMG_OUT_FILE: &str = "font-imgfont-generator.out.png";
const CHAR_SET: &str = "abcdefghijklmnopqrstuvwxyz_ ";

fn main() -> Result<(), FontError> {
    let font_bytes = include_bytes!("../../../example_assets/Roboto-Regular.ttf");
    let ttf_font = TTFont::load(&font_bytes[..])?;
    println!("+ TTF Loaded...");

    let img_font = ImgFontGenerator::new(CHAR_SET, 32).generate(&ttf_font)?;
    println!("+ ImgFont Generated...");

    let mut out_path = env::temp_dir();
    out_path.push(IMG_OUT_FILE);
    let out_file = File::create(out_path.clone())?;
    img_font.image().save(out_file, ImageFormat::Png)?;
    println!("+ ImgFont Image Saved ({:?})...", out_path);

    println!("+ Glyph Data ");
    println!("+ +++++++++++++++++++++++++++++");
    for (character, glyph) in img_font.glyphs() {
        println!("+ '{}': {:?}", character, glyph);
    }
    println!("+ +++++++++++++++++++++++++++++");

    println!("+ Done.");
    Ok(())
}

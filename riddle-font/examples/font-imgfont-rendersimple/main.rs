use std::{env, fs::File};

use riddle_font::{FontError, ImgFontGenerator, TTFont};
use riddle_image::ImageFormat;

const FONT_OUT_FILE: &str = "font-imgfont-rendersimple.imgfont.png";
const IMG_OUT_FILE: &str = "font-imgfont-rendersimple.out.png";

const CHAR_SET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890-_=+! ";
const TEST_STRING: &str = "Hello World!";

fn main() -> Result<(), FontError> {
	let font_bytes = include_bytes!("../../../example_assets/Roboto-Regular.ttf");
	let ttf_font = TTFont::load(&font_bytes[..])?;
	println!("+ TTF Loaded...");

	let img_font = ImgFontGenerator::new(CHAR_SET, 32).generate(&ttf_font)?;
	println!("+ ImgFont Generated...");

	let mut out_path = env::temp_dir();
	out_path.push(FONT_OUT_FILE);
	let out_file = File::create(out_path.clone())?;
	img_font.image().save(out_file, ImageFormat::Png)?;
	println!("+ ImgFont Image Saved ({:?})...", out_path);

	let rendered_img = img_font.render_simple(TEST_STRING)?;
	println!("+ ImgFont String Rendered...");

	let mut out_path = env::temp_dir();
	out_path.push(IMG_OUT_FILE);
	let out_file = File::create(out_path.clone())?;
	rendered_img.save(out_file, ImageFormat::Png)?;
	println!("+ Output Image Saved ({:?})...", out_path);

	Ok(())
}

#![deny(clippy::all)]

//! Riddle crate for loading and manipulating image data in main memory.
//!
//! Built largely on the back of `::image` and its dependencies.
//!
//! # Example
//!
//! ```
//! # use riddle_image::*;
//! # fn main() -> Result<(), ImageError> {
//! // Load an image from a png
//! let png_bytes = include_bytes!("../../example_assets/image.png");
//! let png_img = Image::load(&png_bytes[..], ImageFormat::Png)?;
//!
//! // Make a blank image and blit the png on to it
//! let mut blank_img = Image::new(256, 256);
//! blank_img.blit(&png_img, [0, 0].into());
//! # Ok (()) }
//! ```

mod error;
mod image;
mod imageview;

pub mod image_ext;

pub use self::image::*;
pub use error::*;
pub use riddle_common::Color;

use self::imageview::*;

use riddle_common::CommonError;

type Result<R> = std::result::Result<R, ImageError>;

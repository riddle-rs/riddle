mod error;
mod image;
mod imageview;

pub use self::image::*;
pub use self::imageview::*;
pub use error::*;

use riddle_common::CommonError;

type Result<R> = std::result::Result<R, ImageError>;

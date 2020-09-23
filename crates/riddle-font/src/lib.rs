/*!
A crate for loading font files and rendering text to images.
*/

mod error;
mod ttfont;

pub use error::*;
pub use ttfont::TTFont;

use riddle_common::CommonError;

type Result<R> = std::result::Result<R, FontError>;

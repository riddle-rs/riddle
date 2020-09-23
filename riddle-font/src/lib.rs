/*!
Riddle crate for loading font files and rendering text to riddle_image images.

```
# use riddle_font::*;
# fn main() -> Result<(), FontError> {
// Load font from TTF file
let ttf_bytes = include_bytes!("../../example_assets/Roboto-Regular.ttf");
let font = TTFont::new(&ttf_bytes[..])?;

// Render the loaded font to a Riddle image
let image = font.render_simple("Simple String", 24)?;
# assert_eq!(24, image.height());
# assert!(image.width() > 0);
# Ok(())
# }
```
*/

mod error;
mod ttfont;

pub use error::*;
pub use ttfont::TTFont;

use riddle_common::CommonError;

type Result<R> = std::result::Result<R, FontError>;

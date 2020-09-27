/*!
# About Riddle

Riddle is a Rust media library in the vein of SDL, building as far as possible
on the most active/standard rust libraries (winit, wgpu, image, etc). Riddle
is **NOT** an engine, or a framework. It is a library devoted to exposing media
related features in a unified way while avoiding prescribing program structure
_[note 1]_.

The rough feature set is listed here, along with the crates the features are
primarily built upen. Riddle is only possible due to the massive efforts of the
greater rust community.

* **Windowing and System Event Loops**, exposed through `riddle::window`. Uses
    `winit`.
* **Input State Management**, exposed through `riddle::input`.
* **Image Loading and Basic Graphics Operations**, exposed through
    `riddle::image`. Uses `image`.
* **Font Loading and Rendering**, exposed through `riddle::font`. Uses
    `rusttype`.
* **Math**, exposed through `riddle::math`. Uses `mint` for public APIs. `glam`
    is used in other crates.
* **Audio Loading and Playing**. Uses `rodio`.
* **Basic 2D Renderer**, exposed through `riddle::renderer`. Uses `wgpu`.
* **Timers and Framerate Tracking**, exposed throug `riddle::time`.

This crate depends on an array of crates, each of which implements a specific
feature. All sub-crates can be used independently of the top level `riddle`
crate, but are best used by adding a dependency on `riddle` with appropriate
feature flags configured.

Each crate fully wraps the underlying libraries to allow for a consistent API
to be preserved between releases. Crates also contain extension traits, to
provide direct access to the underlying types. The few exceptions to this,
where types from other crates are exposed through Riddle's public API are
where those crates have become defacto standards for cross crate integration,
such as `mint`, and `raw-window-handle`.

## Cargo Features

* `renderer` - The `riddle-renderer-wgpu` renderer will be enabled.
* `audio`- The `riddle-audio`subsystem will be enabled.
* `font` - The `riddle-font` crate will be included, and reexported through `riddle::font`
* **default** - All features are enabled.

## Getting started

Add a git dependency in your cargo.toml _[note 2]_.

```text
[dependency.riddle]
version = 0.1
git = "https://github.com/vickles/riddle/"
branch = "master"
```

Place the following in main.rs:

```no_run
use riddle::{*, platform::*, renderer::*, common::Color};

fn main() -> Result<(), RiddleError> {
    // Initialize the library
    let rdl =  RiddleLib::new()?;

    // Create a window and renderer
    let window = WindowBuilder::new().build(rdl.context())?;
    let renderer = Renderer::new_from_window(&window)?;

    // Start the main event loop
    rdl.run(move |rdl| {
        match rdl.event() {
            Event::Platform(PlatformEvent::WindowClose(_)) => rdl.quit(),
            Event::ProcessFrame => {
                // Clear the window with black every frame
                let mut render_ctx = renderer.begin_render().unwrap();
                render_ctx.clear(Color::BLACK);
                render_ctx.present();
            }
            _ => (),
         }
    })
}
```

## Notes

1. A necesary exception is the top level `Riddle::run` function which must be
   used if `riddle::window` is to be used.
2. Currently Riddle depends on some patches to underlying libraries, which are
   being maintained in forked git repositories until the changes are
   integrated upstream. This means Riddle can't be uploaded to crates.io at
   the moment.

*/

mod context;
mod error;
mod event;
mod riddle_lib;
mod state;

pub use riddle_common as common;
pub use riddle_image as image;
pub use riddle_input as input;
pub use riddle_math as math;
pub use riddle_platform_winit as platform;
pub use riddle_time as time;

#[cfg(feature = "riddle-audio")]
pub use riddle_audio as audio;

#[cfg(feature = "riddle-renderer-wgpu")]
pub use riddle_renderer_wgpu as renderer;

#[cfg(feature = "riddle-font")]
pub use riddle_font as font;

pub use context::*;
pub use error::*;
pub use event::*;
pub use riddle_lib::*;
pub use state::*;

type Result<R> = std::result::Result<R, RiddleError>;

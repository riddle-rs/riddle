# About Riddle

![Build](https://github.com/vickles/riddle/workflows/Build/badge.svg)

Riddle is a Rust media library in the vein of SDL, building as far as possible
on the most active/standard rust libraries (winit, wgpu, image, etc). Riddle
is **NOT** an engine, or a framework. It is a library devoted to exposing media
related features in a unified way while avoiding prescribing program structure
_[note 1]_.

The rough feature set is listed here, along with the crates the features are
primarily built upen. Riddle is only possible due to the massive efforts of the
greater rust community.

* **Windowing and System Event Loops**
    ([Docs](https://vickles.github.io/riddle/0.1.0/riddle_platform_winit)),
    exposed through `riddle::platform`. Uses `winit`.
* **Input State Management**
    ([Docs](https://vickles.github.io/riddle/0.1.0/riddle_input)),
    exposed through `riddle::input`. Gamepad, mouse and keyboard support.
    Built on `winit` and `gilrs`
* **Image Loading and Basic Graphics Operations**
    ([Docs](https://vickles.github.io/riddle/0.1.0/riddle_image)),
    exposed through `riddle::image`. Uses `image`.
* **Font Loading and Rendering**
    ([Docs](https://vickles.github.io/riddle/0.1.0/riddle_font)),
    exposed through `riddle::font`. Uses `rusttype`.
* **Math**
    ([Docs](https://vickles.github.io/riddle/0.1.0/riddle_math)),
    exposed through `riddle::math`. Uses `mint`, and `glam`.
* **Audio Loading and Playing**.
    ([Docs](https://vickles.github.io/riddle/0.1.0/riddle_audio)),
    Uses `rodio`.
* **Basic 2D Renderer**
    ([Docs](https://vickles.github.io/riddle/0.1.0/riddle_renderer_wgpu)),
    exposed through `riddle::renderer`. Uses `wgpu`.
* **Timers and Framerate Tracking**
    ([Docs](https://vickles.github.io/riddle/0.1.0/riddle_time)),
    exposed throug `riddle::time`.

This crate depends on an array of crates, each of which implements a specific
feature. All sub-crates can be used independently of the top level `riddle`
crate, but are best used by adding a dependency on `riddle` with appropriate
feature flags configured.

Each crate fully wraps the underlying libraries to allow for a consistent API
to be preserved between releases. They also contain extension traits, to
provide direct access to the underlying types. The few exceptions to this,
where types from other crates are exposed through Riddle's public API are
where those crates have become defacto standards for cross crate integration,
such as `mint`, and `raw-window-handle`.

## Documentation

Since the crate isn't on crates.io docs are hosted on Github Pages here:
[**Rustdoc Docs**](https://vickles.github.io/riddle/0.1.0/riddle)

## Cargo Features

* `renderer` - The `riddle-renderer-wgpu` renderer will be enabled.
* `audio`- The `riddle-audio`subsystem will be enabled.
* `font` - The `riddle-font` crate will be included, and reexported through `riddle::font`
* **default** - All features are enabled.

## Getting started

Add a git dependency in your cargo.toml _[note 2]_.

```toml
[dependencies.riddle]
version = 0.1
git = "https://github.com/vickles/riddle/"
tag = "0.1.0"
```

Place the following in main.rs:

```rust
use riddle::{*, platform::*, renderer::*, common::Color};

fn main() -> Result<(), RiddleError> {
    // Initialize the library
    let rdl = RiddleLib::new()?;

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

## Examples

The rustdocs for most public APIs contain example code to illustrate how the
various riddle systems work, and how they should be used.

More complete examples live in `riddle/examples`.

The current set of examples is:

* **pong** - A simple pong game, using input, basic fill-rect rendering, and audio.
* **sandbox** - This is a scratchpad example which tries to use as much functionality
                as possible to provide quick manual verification of changes to the
                library. It is not a learning resource.

## Nightly Rust

Riddle depends heavily on `Arc::new_cyclic` which is currently only available
on rust nightly. Until this [feature](https://github.com/rust-lang/rust/issues/75861)
is stablized nightly rust will be required.

## Linux build dependencies

To build on Linux the following packages are required (on Ubuntu at least):
`libasound2-dev libudev-dev`. These are required for rodio(audio library), and gilrs(controller
support library) respectively.

They can be installed by running:

```bash
sudo apt install libasound2-dev libudev-dev
```

## Notes

1. A necesary exception is the top level `RiddleLib::run` function which must be
   used if `riddle::platform` is to be used.
2. Currently Riddle depends on some patches to underlying libraries, which are
   being maintained in forked git repositories until the changes are
   integrated upstream. This means Riddle can't be uploaded to crates.io at
   the moment. [Tracking issue](https://github.com/vickles/riddle/issues/23)

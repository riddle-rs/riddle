# Change Log

## Unreleased

* renderer: Update wgpu to 0.7.
* renderer: Convert shaders to wgsl.
* **breaking** image: Use a Vector2 arg instead of x,y arg pairs for several functions.
* image: Add `ImagePacker` to provide decent sprite packing.
* renderer: Convert `SpriteAtlasBuilder` to use `image::ImagePacker`.
* image: Add support for generating signed distance field images:  `image::filters::distance_field`.
* image: Add `Image::save`.
* font: Add support for generating `ImgFont` bitmap fonts (with basic layout support) from
    `TTFont`s.
* image: Add support for blitting part of a source image on to a destination image.
* renderer: Add `SpriteFont` to efficiently render `ImgFont` strings.
* **breaking** renderer: Make `Sprite::render` public, and `Sprite::new_from_image` takes the image
    by reference instead of by value.
* riddle: Add rustfmt.toml, setting `hard_tabs = true`. Reformat repo.
* **breaking** renderer: Remove `SpriteRenderArgs::render`, instead prefer use of `Sprite::render`.
* image: Add deriving ImageFormat from path extension.
* **breaking** renderer: Refactor `SpriteBuilder` in to `SpriteInitArgs` and make
    `Sprite::new_from_image` public.

## 0.2.0

* **breaking** image, font, audio: Standardize naming convention for asset loading, renaming methods
    to `load`, and taking an explicit format argument where applicable.
* **breaking** audio: Upgrade to rodio 0.12. Required creating `AudioMainThreadState` to store new
    `rodio::OutputStream` value.
* **breaking** riddle: rename cargo features to be consistent with crate names.
* **breaking** riddle: Update dependencies (winit, glam, gilrs, rodio).
* **breaking** riddle: Put internal system methods in to Ext traits, expose underlying types with
    crate specific Ext traits.
* **breaking** riddle: Add clippy to the CI, and fix a variety of issues. Visible change is `new`
    methods which return a handle.
* riddle: Relax bounds on error type param for `RiddleLib::run_with_err` to allow use with
    `anyhow::Error`.
* audio, riddle: add optional mp3 support behind feature `riddle-mp3`.
* image: Remove seek bound on Image::new_from_png.
* image: Add support for loading Bmp and Jpeg images.
* image, font, audio: Add `load_async` methods to asset types.
* renderer: Make renderer generic over underlying device to remove the Send + Sync restriction on
    the underlying device.
* docs: Add an example combining the riddle renderer with a custom wgpu renderer.
* input: Add `InputSystem::gamepad_axis_value` and `InputSystem::last_active_gamepad`.
* input: Add `InputSystem::is_mouse_button_down`.
* docs: Update pong example to support gamepad input.
* input: MacOS scancode normalization.
* time: Reduce the minimum frame dt, previously set equivalent max FPS as 1000FPS, now 10,000FPS

## 0.1.0 - Initial Release

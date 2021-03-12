# Change Log

## Unreleased

* image: Remove seek bound on Image::new_from_png
* image: Add support for loading Bmp and Jpeg images.
* **breaking** image, font, audio: Standardize naming convention for asset loading, renaming methods
    to `load`, and taking an explicit format argument where applicable.
* **breaking** audio: Upgrade to rodio 0.12. Required creating `AudioMainThreadState` to store new
    `rodio::OutputStream` value.
* image, font, audio: Add `load_async` methods to asset types.
* **breaking** riddle: rename cargo features to be consistent with crate names.
* audio, riddle: add optional mp3 support behind feature `riddle-mp3`.
* renderer: Make renderer generic over underlying device to remove the Send + Sync restriction on the underlying device.
* docs: Add an example combining the riddle renderer with a custom wgpu renderer.
* input: Add `InputSystem::gamepad_axis_value` and `InputSystem::last_active_gamepad`.
* docs: Update pong example to support gamepad input
* **breaking**: Put internal system methods in to Ext traits, expose underlying types with crate specific Ext traits.
* input: MacOS scancode normalization
* riddle: Relax bounds on error type param for `RiddleLib::run_with_err` to allow use with `anyhow::Error`.
* **breaking**: Add clippy to the CI, and fix a variety of issues. Visible change is `new` methods which return a handle
    type have been renamed `new_shared`.
* time: Reduce the minimum frame dt, previously set equivalent max FPS as 1000FPS, now 10,000FPS
* input: Add `InputSystem::is_mouse_button_down`
* **breaking** riddle: Update dependencies (winit, glam, gilrs, rodio)

## 0.1.0 - Initial Release

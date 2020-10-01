# Change Log

## Unreleased

* image: Remove seek bound on Image::new_from_png
* image: Add support for loading Bmp and Jpeg images.
* **breaking** image, font, audio: Standardize naming convention for asset loading, renaming methods
    to `load`, and taking an explicit format argument where applicable.
* **breaking** audio: Move rodio dependency to custom fork to support format specific decoding.
    Required creating `AudioMainThreadState` to store new `rodio::OutputStream` value.
* image, font, audio: Add `load_async` methods to asset types.
* **breaking** riddle: rename cargo features to be consistent with crate names.
* audio, riddle: add optional mp3 support behind feature `riddle-mp3`.

## 0.1.0 - Initial Release

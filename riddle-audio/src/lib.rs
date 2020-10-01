#![feature(arc_new_cyclic)]

/*!
Riddle crate for loading and playing audio data.

Built largely on the back of `::image` and its dependencies.

# Riddle Example

The **recommended** way to use this crate is through the main `riddle` crate.
Riddle exposes this crate through `riddle::audio`.

```no_run
use riddle::*;

fn main() -> Result<(), RiddleError> {
    let rdl =  RiddleLib::new()?;

    // Load the clip
    let clip_bytes = include_bytes!("../../example_assets/boop.wav");
    let clip = audio::Clip::load(&clip_bytes[..], audio::ClipFormat::Wav)?;

    // Play the clip
    let player = audio::ClipPlayerBuilder::new(&rdl.context().audio())
                    .play(&clip)?;

    let start_time = std::time::Instant::now();
    rdl.run(move |rdl| {
        if std::time::Instant::now() - start_time > std::time::Duration::from_secs(2) {
            rdl.quit()
        }
    })
}
```

# Direct Example

If you don't want to depend on `riddle`, you can use this crate directly.

```no_run
use riddle_audio::*;

fn main() -> Result<(), AudioError> {
    let (audio_system, _audio_main_thread_state) = AudioSystem::new()?;

    // Load the clip
    let clip_bytes = include_bytes!("../../example_assets/boop.wav");
    let clip = Clip::load(&clip_bytes[..], ClipFormat::Wav)?;

    // Play the clip
    let player = ClipPlayerBuilder::new(&audio_system)
                    .play(&clip)?;

    // Let it play for two seconds
    let start_time = std::time::Instant::now();
    while std::time::Instant::now() - start_time < std::time::Duration::from_secs(2) {
        audio_system.process_frame();
        std::thread::sleep_ms(100);
    }
    Ok(())
}
```
*/
mod audio_system;
mod clip;
mod clip_player;
mod error;
mod fades;

pub mod doctest;

use fades::*;
use riddle_common::*;

pub use audio_system::*;
pub use clip::*;
pub use clip_player::*;
pub use error::*;

type Result<R> = std::result::Result<R, AudioError>;

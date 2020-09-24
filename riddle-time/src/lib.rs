#![feature(arc_new_cyclic)]

/*!

Riddle crate supporting some basic game-centric time functionality.

The most significant of which is keeping track of framerate, and providing
a centralized place to access a consistent delta_t when running game logic.

# Riddle Example

The **recommended** way to use this crate is through the main `riddle` crate.
Riddle exposes this crate through `riddle::time`.

```
use riddle::*;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

fn main() -> Result<(), RiddleError> {
    let rdl = RiddleApp::new()?;

    let quit_flag = Arc::new(AtomicBool::new(false));
    rdl.state().time().register_timer(std::time::Duration::from_millis(200), {
        let quit_flag = quit_flag.clone();
        move || { quit_flag.store(true, Ordering::Relaxed); }
    });

    rdl.run(move |rdl| {
        if quit_flag.load(Ordering::Relaxed) {
            rdl.quit();
        }

        if let Event::ProcessFrame = rdl.event() {
            // FPS: rdl.time().fps()
        }
    })
}
```

# Direct Example

If you don't want to depend on `riddle`, you can use this crate directly.

```
use riddle_time::*;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

fn main() {
    let time = TimeSystem::new();

    let quit_flag = Arc::new(AtomicBool::new(false));
    time.register_timer(std::time::Duration::from_millis(200), {
        let quit_flag = quit_flag.clone();
        move || { quit_flag.store(true, Ordering::Relaxed); }
    });

    while !quit_flag.load(Ordering::Relaxed) {
        std::thread::sleep(std::time::Duration::from_millis(100));
        time.process_frame();

        // FPS: time.fps()
    }
}
```
*/

mod time_system;
mod timer;

pub mod doctest;

pub use time_system::*;
pub use timer::*;

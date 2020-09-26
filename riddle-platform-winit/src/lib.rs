#![feature(arc_new_cyclic)]

/*!

Riddle platform system implementation based on `winit`, managing the OS's main
event loop and windowing system.

# Riddle Example

The **recommended** way to use this crate is through the main `riddle` crate.
Riddle exposes this crate through `riddle::platform`.

```no_run
use riddle::{*, platform::*};

fn main() -> Result<(), RiddleError> {
    let rdl = RiddleApp::new()?;
    let window = WindowBuilder::new().build(rdl.context())?;

    rdl.run(move |rdl| {
        match rdl.event() {
            Event::Platform(PlatformEvent::WindowClose(_)) => rdl.quit(),
            _ => (),
         }
    })
}
```

# Direct Example

If you don't want to depend on `riddle`, you can use this crate directly. There isn't much point
in doing so over using `winit` directly though - the main function of this crate is to integrate
`winit` in to Riddle.

```no_run
use riddle_platform_winit::*;

fn main() -> Result<(), PlatformError> {
    let (platform_system, main_thread_state) = PlatformSystem::new();
    let window = WindowBuilder::new().build(main_thread_state.borrow_context())?;

    main_thread_state.run(move |ctx| {
        match ctx.event() {
            PlatformEvent::WindowClose(_) => { ctx.quit(); }
            _ => ()
        }
    })
}
```
*/

mod dimensions;
mod error;
mod event;
mod platform_context;
mod platform_system;
mod window;
mod window_map;

pub use error::PlatformError;
pub use platform_context::*;
pub use platform_system::*;
pub use riddle_platform_common as common;
pub use window::*;

pub use common::PlatformEvent;

use riddle_common::*;
use window_map::*;

type Result<R> = std::result::Result<R, PlatformError>;

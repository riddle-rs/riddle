#![feature(arc_new_cyclic)]

/*!

Riddle simple sprite-based renderer built on `wgpu`.

The renderer can be used by itself as the only renderer in your application,
or it can be constructed on top of existing WGPU devices to provide simple
2D rendering.

# Riddle Example

The **recommended** way to use this crate is through the main `riddle` crate.
Riddle exposes this crate through `riddle::renderer`. The following example
lets this crate take full ownership over creating the WGPU device.

```no_run
use riddle::{common::Color, platform::*, renderer::*, *};

fn main() -> Result<(), RiddleError> {
    let rdl =  RiddleLib::new()?;
    let window = WindowBuilder::new().build(rdl.context())?;
    let renderer = Renderer::new_from_window(&window)?;

    rdl.run(move |rdl| {
        match rdl.event() {
            Event::Platform(PlatformEvent::WindowClose(_)) => rdl.quit(),
            Event::ProcessFrame => {
                let mut render_ctx = renderer.begin_render().unwrap();
                render_ctx.clear(Color::RED);
                render_ctx.present();
            },
            _ => ()
         }
    })
}
```

# Direct Usage

To use this crate directly see [`ext::RendererWGPUDevice`] for how to use the renderer
with custom WGPU devices.

*/

mod buffered_renderer;
mod error;
mod render_context;
mod renderer;
mod shader;
mod sprite;
mod sprite_atlas;
mod sprite_render_target;
mod swap_chain_target;
mod texture;
mod vertex;

pub mod ext;

pub use error::*;
pub use render_context::*;
pub use renderer::*;
pub use sprite::*;
pub use sprite_atlas::*;
pub use sprite_render_target::*;
pub use texture::*;

use buffered_renderer::*;
use riddle_common::*;
use shader::*;
use swap_chain_target::*;
use vertex::*;

use riddle_image as image;
use riddle_math as math;
use riddle_platform_winit as platform;

type Result<R> = std::result::Result<R, RendererError>;

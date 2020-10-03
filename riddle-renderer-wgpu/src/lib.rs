#![feature(arc_new_cyclic)]

//! Riddle simple sprite-based renderer built on `wgpu`.
//!
//! The renderer can be used by itself as the only renderer in your application,
//! or it can be constructed on top of existing WGPU devices to provide simple
//! 2D rendering.
//!
//! # Riddle Example
//!
//! The **recommended** way to use this crate is through the main `riddle` crate.
//! Riddle exposes this crate through `riddle::renderer`. The following example
//! lets this crate take full ownership over creating the WGPU device.
//!
//! ```no_run
//! use riddle::{common::Color, platform::*, renderer::*, *};
//!
//! fn main() -> Result<(), RiddleError> {
//!     let rdl =  RiddleLib::new()?;
//!     let window = WindowBuilder::new().build(rdl.context())?;
//!     let renderer = Renderer::new_from_window(&window)?;
//!
//!     rdl.run(move |rdl| {
//!         match rdl.event() {
//!             Event::Platform(PlatformEvent::WindowClose(_)) => rdl.quit(),
//!             Event::ProcessFrame => {
//!                 let mut render_ctx = renderer.begin_render().unwrap();
//!                 render_ctx.clear(Color::RED);
//!                 render_ctx.present();
//!             },
//!             _ => ()
//!          }
//!     })
//! }
//! ```
//!
//! # Direct Usage
//!
//! To use this crate directly see [`wgpu_ext::WGPUDevice`] for how to use the renderer
//! with custom WGPU devices.

mod error;
mod sprite_utils;

pub mod wgpu_ext;

use riddle_common::*;
use riddle_image as image;
use riddle_math as math;
use riddle_platform_winit as platform;
use wgpu_ext::*;

type Result<R> = std::result::Result<R, RendererError>;

pub use error::*;
pub use sprite_utils::*;
pub use wgpu_ext::{FilterMode, RenderContext};

/// A simple 2D sprite based renderer for a riddle Window.
pub type Renderer = WGPURenderer<WindowWGPUDevice>;

/// Strong handle to a [`Renderer`].
pub type RendererHandle = WGPURendererHandle<WindowWGPUDevice>;

/// Weak handle to a [`Renderer`].
pub type RendererWeak = WGPURendererWeak<WindowWGPUDevice>;

/// A sprite for the default Window renderer.
pub type Sprite = WGPUSprite<WindowWGPUDevice>;

/// Construct a set of [`Sprite`]s from a set of `riddle_image::Image`s which share a texture atlas.
pub type SpriteAtlasBuilder<'a> = WGPUSpriteAtlasBuilder<'a, WindowWGPUDevice>;

/// A target which can be both rendered to and referenced as a [`Sprite`] for rendering.
pub type SpriteRenderTarget = WGPUSpriteRenderTarget<WindowWGPUDevice>;

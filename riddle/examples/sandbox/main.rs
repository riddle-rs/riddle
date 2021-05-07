//! Sandbox demo, using a wide range of riddle features to allow quick manual
//! testing of changes.

use riddle::{
	common::Color,
	input::{InputEvent, KeyboardModifiers, MouseButton, VirtualKey},
	math::*,
	platform::{PlatformEvent, WindowBuilder},
	renderer::*,
	*,
};

use std::sync::{Arc, Mutex};

type AppRenderer = renderer::Renderer<WindowWgpuDevice>;

struct DemoState {
	state: RiddleState,

	mouse_location: Arc<Mutex<input::LogicalPosition>>,

	clip: audio::Clip,
	music_player: audio::ClipPlayer,
	_blip_player: Option<audio::ClipPlayer>,
}

impl DemoState {
	fn new(rdl: &RiddleLib) -> Result<Self, RiddleError> {
		let window = WindowBuilder::new().build(rdl.context())?;

		let renderer = renderer::Renderer::new_from_window(&window)?;

		let img = {
			let img_bytes = include_bytes!("../../../example_assets/image.png");
			image::Image::load(&img_bytes[..], image::ImageFormat::Png)?
		};

		let font = {
			let font_bytes = include_bytes!("../../../example_assets/Roboto-Regular.ttf");
			font::TtFont::load(&font_bytes[..])?
		};

		let clip = {
			let clip_bytes = include_bytes!("../../../example_assets/boop.wav");
			audio::Clip::load(&clip_bytes[..], audio::ClipFormat::Wav)?
		};

		let music = {
			let music_bytes = include_bytes!("../../../example_assets/music.ogg");
			audio::Clip::load(&music_bytes[..], audio::ClipFormat::Vorbis)?
		};

		let label = font.render_simple("Hello World", 30)?;

		let mut label_sprite = None;
		let mut sprite = None;

		SpriteAtlasBuilder::new()
			.with_image(label, &mut label_sprite)
			.with_image(img, &mut sprite)
			.build(&renderer)?;

		let sprite = sprite.unwrap();
		let subsprite = sprite.subsprite(&Rect {
			location: [3.0, 3.0].into(),
			dimensions: [39.0, 39.0].into(),
		});
		let label_sprite = label_sprite.unwrap();

		let target = SpriteRenderTarget::new(&renderer, [50, 50].into())?;

		let music_player = audio::ClipPlayerBuilder::new(&rdl.state().audio())
			.with_mode(audio::PlayMode::Loop)
			.play(&music)?;

		let mouse_location = Arc::new(Mutex::new(input::LogicalPosition::default()));

		let img_font = font::ImgFontGenerator::new("FPS:.0123456789", 32).generate(&font)?;
		let sprite_font = SpriteFont::new(&renderer, img_font)?;

		let mut renderer_state = RendererState {
			renderer: renderer.clone(),
			sprite,
			subsprite,
			label_sprite,
			sprite_font,
			target,
			mouse_location: mouse_location.clone(),
			prev_frame_time: std::time::Instant::now(),
		};

		{
			let window = window.clone();
			std::thread::spawn(move || loop {
				println!("Window Size: {:?}", window.logical_size());
				std::thread::sleep(std::time::Duration::from_secs(5));
			});
		}

		{
			let rdlstate = rdl.state().clone();
			let clip = clip.clone();
			std::thread::spawn(move || loop {
				let _player = audio::ClipPlayerBuilder::new(&rdlstate.audio())
					.play(&clip)
					.unwrap();
				std::thread::sleep(std::time::Duration::from_secs(10));
			});
		}

		std::thread::spawn(move || renderer_state.run());

		Ok(Self {
			state: rdl.state().clone(),

			mouse_location: mouse_location,

			clip,
			music_player,
			_blip_player: None,
		})
	}

	pub fn on_mouse_down(&mut self) -> Result<(), RiddleError> {
		self._blip_player = Some(
			audio::ClipPlayerBuilder::new(&self.state.audio())
				.with_mode(audio::PlayMode::OneShot)
				.play(&self.clip)?,
		);
		Ok(())
	}
}

struct RendererState {
	renderer: AppRenderer,
	sprite: <AppRenderer as CommonRenderer>::Sprite,
	subsprite: <AppRenderer as CommonRenderer>::Sprite,
	label_sprite: <AppRenderer as CommonRenderer>::Sprite,
	sprite_font: <AppRenderer as CommonRenderer>::SpriteFont,

	target: renderer::SpriteRenderTarget<WindowWgpuDevice>,

	mouse_location: Arc<Mutex<input::LogicalPosition>>,
	prev_frame_time: std::time::Instant,
}

impl RendererState {
	fn run(&mut self) {
		loop {
			self.render_frame().unwrap();
		}
	}

	fn render_to_target(&self) -> Result<(), RiddleError> {
		self.target.render(|ctx| {
			ctx.clear(Color::rgb(0.0, 0.0, 1.0))?;
			self.sprite.render_at(ctx, [0.0, 0.0].into())
		})?;
		Ok(())
	}

	pub fn render_frame(&mut self) -> Result<(), RiddleError> {
		let frame_start_time = std::time::Instant::now();
		let delta = frame_start_time - self.prev_frame_time;
		let fps = 1.0 / delta.as_secs_f32();
		self.prev_frame_time = frame_start_time;

		self.render_to_target()?;

		self.renderer
			.render(|frame| self.render_internal(frame, fps))?;
		Ok(())
	}

	fn render_internal<R>(
		&self,
		frame: &mut R,
		fps: f32,
	) -> Result<(), <AppRenderer as CommonRenderer>::Error>
	where
		R: RenderContext<Renderer<WindowWgpuDevice>>,
	{
		frame.clear(Color::rgb(0.0, 1.0, 0.0))?;

		frame.set_transform(glam::Mat4::from_scale(glam::vec3(2.0, 2.0, 1.0)).into())?;

		frame.fill_rect(
			&Rect {
				location: [100.0, 100.0].into(),
				dimensions: [50.0, 50.0].into(),
			},
			Color::RED,
		)?;
		frame.fill_rect(
			&Rect {
				location: [102.0, 102.0].into(),
				dimensions: [46.0, 46.0].into(),
			},
			Color::WHITE,
		)?;

		self.subsprite.render_at(frame, [60.0, 60.0].into())?;
		self.label_sprite.render(
			frame,
			SpriteRenderArgs::new(vec2(10.0, 100.0)).with_color(Color::BLACK),
		)?;

		frame.set_transform(glam::Mat4::IDENTITY.into())?;

		self.target
			.sprite()
			.render_at(frame, [400.0, 400.0].into())?;

		let pos: input::LogicalPosition = self.mouse_location.lock().unwrap().clone();

		self.sprite.render_at(frame, pos.into())?;

		let fps_str = format!("FPS: {}", fps);

		self.sprite_font.render(
			frame,
			&SpriteRenderArgs::new([0.0, 0.0]).with_color(Color::BLACK),
			&fps_str,
		)?;

		Ok(())
	}
}

fn main() -> Result<(), RiddleError> {
	let rdl = RiddleLib::new()?;
	let mut state = DemoState::new(&rdl)?;

	rdl.run(move |rdl| match rdl.event() {
		Event::Platform(PlatformEvent::WindowClose(_)) => rdl.quit(),
		Event::Input(InputEvent::MouseButtonDown {
			button: MouseButton::Left,
			..
		}) => state.on_mouse_down().unwrap(),
		Event::Input(InputEvent::MouseButtonDown {
			button: MouseButton::Right,
			..
		}) => {
			rdl.time()
				.register_timer(std::time::Duration::from_secs(2), || {
					println!("Timer Done");
				});
		}
		Event::Input(InputEvent::KeyDown {
			vkey: Some(vkey),
			modifiers: KeyboardModifiers { ctrl, .. },
			..
		}) => {
			match (vkey, ctrl) {
				(VirtualKey::M, true) => {
					state.music_player.pause();
				}
				(VirtualKey::P, true) => {
					state.music_player.resume();
				}
				(VirtualKey::Down, _) => {
					state.music_player.set_volume(0.5);
				}
				(VirtualKey::Up, _) => {
					state.music_player.set_volume(1.0);
				}
				_ => (),
			}
			println!("KeyDown: {:?}", vkey);
		}
		Event::Input(InputEvent::KeyUp { vkey, .. }) => {
			println!("KeyUp: {:?}", vkey);
		}
		Event::Input(InputEvent::GamePadButtonDown { button, .. }) => {
			println!("Gamepad Down {:?}", button);
		}
		Event::Input(InputEvent::GamePadButtonUp { button, .. }) => {
			println!("Gamepad Up {:?}", button);
		}
		Event::Input(InputEvent::GamePadAxisChanged { axis, value, .. }) => {
			println!("Gamepad Up {:?} {:?}", axis, value);
		}
		Event::Input(InputEvent::CursorMove { position, .. }) => {
			*state.mouse_location.lock().unwrap() = position.clone();
		}
		_ => (),
	})
}

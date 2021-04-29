//! Minimal pong demo. Uses keyboard & gamepad input, audio, windowing, and very
//! basic usage of the 2d renderer.

use riddle::{common::Color, input::*, math::*, platform::*, renderer::*, *};

fn main() -> Result<(), RiddleError> {
	let rdl = RiddleLib::new()?;
	let mut pong = Pong::new(&rdl)?;

	rdl.run(move |rdl| match rdl.event() {
		Event::Platform(PlatformEvent::WindowClose(_)) => {
			rdl.quit();
		}
		Event::ProcessFrame => {
			pong.update().unwrap();
			pong.render().unwrap();
		}
		_ => (),
	});
}

struct Pong {
	window: Window,
	renderer: Renderer<WindowWgpuDevice>,
	state: RiddleState,

	clip: audio::Clip,
	_clip_player: Option<audio::ClipPlayer>,

	left_paddle: Rect<f32>,
	right_paddle: Rect<f32>,

	ball: Rect<f32>,
	velocity: Vector2<f32>,
}

impl Pong {
	fn new(rdl: &RiddleLib) -> Result<Self, RiddleError> {
		let window = WindowBuilder::new()
			.title("Riddle Pong")
			.dimensions(800, 600)
			.cursor_visible(false)
			.build(&rdl.context())?;
		let renderer = DefaultRenderer::new_from_window(&window)?;

		let clip = {
			let clip_bytes = include_bytes!("../../../example_assets/boop.wav");
			audio::Clip::load(&clip_bytes[..], audio::ClipFormat::Wav)?
		};

		Ok(Self {
			window,
			renderer,
			state: rdl.state().clone(),
			clip,
			_clip_player: None,
			left_paddle: Rect::new(vec2(20.0, 0.0), vec2(20.0, 60.0)),
			right_paddle: Rect::new(vec2(760.0, 0.0), vec2(20.0, 60.0)),
			ball: Rect::new(vec2(395.0, 295.0), vec2(10.0, 10.0)),
			velocity: vec2(60.0, 60.0),
		})
	}

	fn render(&self) -> Result<(), RiddleError> {
		self.renderer.render(|render_ctx| {
			render_ctx.clear(Color::BLACK)?;

			render_ctx.fill_rect(&self.left_paddle, Color::GREEN)?;
			render_ctx.fill_rect(&self.right_paddle, Color::GREEN)?;

			render_ctx.fill_rect(&self.ball, Color::GREEN)?;
			Ok(())
		})?;
		Ok(())
	}

	fn update(&mut self) -> Result<(), RiddleError> {
		let ball_delta = self.velocity * self.state.time().delta_secs();
		self.ball.location += ball_delta;

		let paddle_delta = vec2(0.0, 200.0) * self.state.time().delta_secs();

		// Paddle controls
		if self.is_input_paddle_up() {
			self.left_paddle.location -= paddle_delta;
			self.right_paddle.location -= paddle_delta;
		} else if self.is_input_paddle_down() {
			self.left_paddle.location += paddle_delta;
			self.right_paddle.location += paddle_delta;
		}

		// Ball bounces
		let mut bounced = false;

		if self.ball.location.y > 590.0 {
			self.ball.location.y = 590.0;
			self.velocity.y *= -1.0;
			bounced = true;
		}

		if self.ball.location.y < 0.0 {
			self.ball.location.y = 0.0;
			self.velocity.y *= -1.0;
			bounced = true;
		}

		if self.ball.intersect(&self.left_paddle).is_some() {
			self.ball.location.x = 40.0;
			self.velocity.x *= -1.0;
			bounced = true;
		}

		if self.ball.intersect(&self.right_paddle).is_some() {
			self.ball.location.x = 750.0;
			self.velocity.x *= -1.0;
			bounced = true;
		}

		if bounced {
			self._clip_player = Some(
				audio::ClipPlayerBuilder::new(self.state.audio())
					.with_mode(audio::PlayMode::OneShot)
					.play(&self.clip)?,
			);
		}

		// Point scoring
		if self.ball.location.x < 0.0 || self.ball.location.x > 790.0 {
			self.ball.location = vec2(395.0, 295.0);
			self.velocity = vec2(60.0, 60.0);
		}

		Ok(())
	}

	fn is_input_paddle_up(&self) -> bool {
		let input = self.state.input();
		let up_key = input.is_vkey_down(self.window.id(), VirtualKey::Up);
		let left_stick = input
			.last_active_gamepad()
			.map(|pad| input.gamepad_axis_value(pad, GamePadAxis::LeftStickY) > 0.0)
			.unwrap_or(false);
		up_key || left_stick
	}

	fn is_input_paddle_down(&self) -> bool {
		let input = self.state.input();
		let down_key = input.is_vkey_down(self.window.id(), VirtualKey::Down);
		let left_stick = input
			.last_active_gamepad()
			.map(|pad| input.gamepad_axis_value(pad, GamePadAxis::LeftStickY) < 0.0)
			.unwrap_or(false);
		down_key || left_stick
	}
}

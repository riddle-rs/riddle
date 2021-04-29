#[cfg(feature = "riddle-audio")]
use crate::audio::{ext::*, AudioMainThreadState, AudioSystem};
use crate::{
	input::{ext::*, InputMainThreadState, InputSystem},
	platform::{ext::*, PlatformMainThreadState, PlatformSystem},
	time::{ext::*, TimeSystem},
	*,
};

/// Riddle subsystem state handles
///
/// Provides access to all the thread-safe state associated with riddle systems.
#[derive(Clone)]
pub struct RiddleState {
	platform: PlatformSystem,
	input: InputSystem,
	time: TimeSystem,

	#[cfg(feature = "riddle-audio")]
	audio: AudioSystem,
}

impl RiddleState {
	pub(crate) fn new() -> Result<(Self, MainThreadState)> {
		log::debug!("Initializing platform...");
		let (platform_system, platform_main_thread) = PlatformSystem::new_system_pair();
		log::debug!("Platform initialized");

		log::debug!("Initializing input...");
		let (input_system, input_main_thread) =
			InputSystem::new_system_pair(platform_system.event_pub())?;
		log::debug!("Input initialized");

		log::debug!("Initializing time...");
		let time = TimeSystem::new();
		log::debug!("Time initialized");

		#[cfg(feature = "riddle-audio")]
		let (audio, audio_main_thread) = {
			log::debug!("Initializing audio...");
			let result = AudioSystem::new_system_pair()?;
			log::debug!("Audio initialized");
			result
		};

		let riddle_state = RiddleState {
			platform: platform_system,
			input: input_system,
			time,

			#[cfg(feature = "riddle-audio")]
			audio,
		};

		let main_thread_state = MainThreadState {
			platform: platform_main_thread,
			input: input_main_thread,

			#[cfg(feature = "riddle-audio")]
			audio: audio_main_thread,
		};

		Ok((riddle_state, main_thread_state))
	}

	/// Platform system state
	pub fn platform(&self) -> &PlatformSystem {
		&self.platform
	}

	/// Input system state
	pub fn input(&self) -> &InputSystem {
		&self.input
	}

	/// Time system state
	pub fn time(&self) -> &TimeSystem {
		&self.time
	}

	/// Audio system state
	#[cfg(feature = "riddle-audio")]
	pub fn audio(&self) -> &AudioSystem {
		&self.audio
	}
}

pub(crate) struct MainThreadState {
	pub(crate) platform: PlatformMainThreadState,
	input: InputMainThreadState,

	#[cfg(feature = "riddle-audio")]
	pub audio: AudioMainThreadState,
}

impl MainThreadState {
	#[inline]
	pub fn run<Err: std::fmt::Debug, F>(self, state: RiddleState, mut update: F) -> !
	where
		F: FnMut(&RiddleContext) -> std::result::Result<(), Err> + 'static,
	{
		let MainThreadState {
			platform,
			mut input,
			audio: _audio,
		} = self;
		platform.run::<Err, _>(move |platform_ctx| {
			if let platform::PlatformEvent::EventQueueEmpty = platform_ctx.event() {
				state.time.process_frame();

				#[cfg(feature = "riddle-audio")]
				state.audio.process_frame();
			};

			input.process_input();

			let event = match platform_ctx.event() {
				platform::PlatformEvent::EventQueueEmpty => Event::ProcessFrame,
				_ => Event::Platform(platform_ctx.event().clone()),
			};

			let mut ctx = RiddleContext {
				window_ctx: platform_ctx,
				state: &state,
				event,
			};

			update(&ctx)?;

			let input_events = state.input.take_input_events();
			for input_event in input_events {
				ctx.event = Event::Input(input_event);
				update(&ctx)?;
			}

			Ok(())
		})
	}
}

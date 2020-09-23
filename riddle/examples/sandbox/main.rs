use riddle::{
    common::Color,
    input::{InputEvent, VirtualKey},
    math::*,
    platform::{PlatformEvent, WindowBuilder},
    renderer::*,
    *,
};

use input::{KeyboardModifiers, MouseButton};
use std::sync::{Arc, Mutex};

struct DemoState {
    state: RiddleState,

    mouse_location: Arc<Mutex<input::LogicalPosition>>,

    clip: audio::Clip,
    music_player: audio::ClipPlayer,
    blip_player: Option<audio::ClipPlayer>,
}

impl DemoState {
    fn new(rdl: &RiddleApp) -> Result<Self, RiddleError> {
        let window = WindowBuilder::new().build(rdl.context())?;

        let renderer = renderer::Renderer::new_from_window(&window)?;

        let img = {
            let img_bytes = include_bytes!("../../../example_assets/image.png");
            let img_cursor = std::io::Cursor::new(&img_bytes[..]);
            image::Image::new_from_png(img_cursor)?
        };

        let font = {
            let font_bytes = include_bytes!("../../../example_assets/Roboto-Regular.ttf");
            font::TTFont::new(&font_bytes[..])?
        };

        let clip = {
            let clip_bytes = include_bytes!("../../../example_assets/boop.wav");
            audio::Clip::new(&clip_bytes[..])?
        };

        let music = {
            let music_bytes = include_bytes!("../../../example_assets/music.ogg");
            audio::Clip::new(&music_bytes[..])?
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

        let music_player = audio::ClipPlayerBuilder::new()
            .with_mode(audio::PlayMode::Loop)
            .play(&rdl.state().audio(), music)?;

        let mouse_location = Arc::new(Mutex::new(input::LogicalPosition::default()));

        let renderer_state = RendererState {
            renderer: renderer.clone(),
            sprite,
            subsprite,
            label_sprite,
            target,
            mouse_location: mouse_location.clone(),
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
                let _player = audio::ClipPlayerBuilder::new()
                    .play(&rdlstate.audio(), clip.clone())
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
            blip_player: None,
        })
    }

    pub fn on_mouse_down(&mut self) -> Result<(), RiddleError> {
        self.blip_player = Some(
            audio::ClipPlayerBuilder::new()
                .with_mode(audio::PlayMode::OneShot)
                .play(&self.state.audio(), self.clip.clone())?,
        );
        Ok(())
    }
}

struct RendererState {
    renderer: renderer::RendererHandle,
    sprite: renderer::Sprite,
    subsprite: renderer::Sprite,
    label_sprite: renderer::Sprite,

    target: renderer::SpriteRenderTarget,

    mouse_location: Arc<Mutex<input::LogicalPosition>>,
}

impl RendererState {
    fn run(&self) {
        loop {
            self.render_frame().unwrap();
        }
    }

    fn render_to_target(&self) -> Result<(), RiddleError> {
        let mut ctx = self.target.begin_render()?;
        ctx.clear(Color::rgb(0.0, 0.0, 1.0))?;

        self.sprite.render_at(&mut ctx, [0.0, 0.0])?;

        ctx.present()?;
        Ok(())
    }

    pub fn render_frame(&self) -> Result<(), RiddleError> {
        self.render_to_target()?;

        let mut frame = self.renderer.begin_render()?;
        frame.clear(Color::rgb(0.0, 1.0, 0.0))?;

        frame.push_transform(glam::Mat4::from_scale(glam::vec3(2.0, 2.0, 1.0)).into())?;

        frame.fill_rect(
            &Rect {
                location: [100.0, 100.0].into(),
                dimensions: [50.0, 50.0].into(),
            },
            [1.0, 0.0, 0.0, 1.0],
        )?;
        frame.fill_rect(
            &Rect {
                location: [102.0, 102.0].into(),
                dimensions: [46.0, 46.0].into(),
            },
            [1.0, 1.0, 1.0, 1.0],
        )?;

        self.subsprite.render_at(&mut frame, [60.0, 60.0])?;
        self.label_sprite.render(
            &mut frame,
            &SpriteRenderCommand {
                location: [10.0, 100.0].into(),
                diffuse_color: [0.0, 0.0, 1.0, 1.0],
                ..Default::default()
            },
        )?;

        frame.pop_transform()?;

        self.target.sprite().render_at(&mut frame, [400.0, 400.0])?;

        let pos: input::LogicalPosition = self.mouse_location.lock().unwrap().clone();

        self.sprite.render_at(&mut frame, pos)?;

        frame.present()?;
        Ok(())
    }
}

fn main() -> Result<(), RiddleError> {
    let rdl = RiddleApp::new()?;
    let mut state = DemoState::new(&rdl)?;

    rdl.run(move |rdl| match rdl.event() {
        Event::Platform(PlatformEvent::WindowClose(_)) => rdl.quit(),
        Event::Input(InputEvent::MouseButtonDown {
            button: MouseButton::Left,
            ..
        }) => {
            state.on_mouse_down().unwrap();
        }
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

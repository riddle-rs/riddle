use riddle::{
    common::Color,
    input::{InputEvent, VirtualKey},
    math::*,
    platform::{PlatformEvent, WindowBuilder},
    renderer::*,
    *,
};

use input::{KeyboardModifiers, MouseButton};
use std::rc::Rc;

struct DemoState {
    window: platform::WindowHandle,
    state: RiddleState,

    renderer: Rc<renderer::Renderer>,
    sprite: renderer::Sprite,
    subsprite: renderer::Sprite,
    label_sprite: renderer::Sprite,

    clip: audio::Clip,
    music_player: audio::ClipPlayer,
    blip_player: Option<audio::ClipPlayer>,
}

impl DemoState {
    fn new(rdl: &RiddleApp) -> Result<Self, RiddleError> {
        let window = WindowBuilder::new().build(rdl.context())?;

        {
            let window = window.clone();
            std::thread::spawn(move || loop {
                println!("Window Size: {:?}", window.logical_size());
                std::thread::sleep(std::time::Duration::from_secs(5));
            });
        }

        let renderer = renderer::Renderer::new_shared(&window)?;

        let img = {
            let img_bytes = include_bytes!("../example_assets/image.png");
            let img_cursor = std::io::Cursor::new(&img_bytes[..]);
            image::Image::new_from_png(img_cursor)?
        };

        let font = {
            let font_bytes = include_bytes!("../example_assets/Roboto-Regular.ttf");
            font::TTFont::new(&font_bytes[..])?
        };

        let clip = {
            let clip_bytes = include_bytes!("../example_assets/boop.wav");
            audio::Clip::new(&clip_bytes[..])?
        };

        let music = {
            let music_bytes = include_bytes!("../example_assets/music.ogg");
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

        let music_player = audio::ClipPlayerBuilder::new()
            .with_mode(audio::PlayMode::Loop)
            .play(&rdl.state().audio(), music)?;

        Ok(Self {
            window,
            state: rdl.state().clone(),

            renderer,
            sprite,
            subsprite,
            label_sprite: label_sprite.unwrap(),

            clip,
            music_player,
            blip_player: None,
        })
    }

    pub fn render_frame(&self) -> Result<(), RiddleError> {
        self.renderer.clear(Color::rgb(0.0, 1.0, 0.0))?;

        self.renderer
            .push_transform(glam::Mat4::from_scale(glam::vec3(2.0, 2.0, 1.0)).into())?;

        self.renderer.fill_rect(
            &Rect {
                location: [100.0, 100.0].into(),
                dimensions: [50.0, 50.0].into(),
            },
            [1.0, 0.0, 0.0, 1.0],
        )?;
        self.renderer.fill_rect(
            &Rect {
                location: [102.0, 102.0].into(),
                dimensions: [46.0, 46.0].into(),
            },
            [1.0, 1.0, 1.0, 1.0],
        )?;

        self.subsprite.render_at([60.0, 60.0])?;
        self.label_sprite.render(&SpriteRenderCommand {
            location: [10.0, 100.0].into(),
            diffuse_color: [0.0, 0.0, 1.0, 1.0],
            ..Default::default()
        })?;

        self.renderer.pop_transform()?;

        self.sprite
            .render_at(self.state.input().mouse_pos(self.window.window_id()))?;

        self.renderer.present()?;
        Ok(())
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
        Event::ProcessFrame => {
            state.render_frame().unwrap();
        }
        _ => (),
    })
}

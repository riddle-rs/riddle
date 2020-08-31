//use input::InputEvent;
use riddle::{math::*, renderer::*, *};
use window::WindowBuilder;

use std::rc::Rc;

struct DemoState {
    window: Rc<window::Window>,

    renderer: Rc<renderer::Renderer>,
    sprite: renderer::Sprite,
    subsprite: renderer::Sprite,
    label_sprite: renderer::Sprite,

    clip: audio::Clip,
    music_player: audio::ClipPlayer,
}

impl DemoState {
    fn new(rdl: &RiddleApp) -> Result<Self, RiddleError> {
        let window = WindowBuilder::new().build(rdl.context())?;
        let renderer = renderer::Renderer::new_shared(window.clone())?;

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
            .play(rdl.context().audio(), music)?;

        Ok(Self {
            window,

            renderer,
            sprite,
            subsprite,
            label_sprite: label_sprite.unwrap(),

            clip,
            music_player,
        })
    }

    pub fn render_frame(&self, rdl: &RiddleContext) -> Result<(), RiddleError> {
        self.renderer.clear()?;
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

        self.sprite.render_at(rdl.input().mouse_pos(&self.window))?;

        self.subsprite.render_at([60.0, 60.0])?;
        self.label_sprite.render(&SpriteRenderCommand {
            location: [10.0, 100.0].into(),
            diffuse_color: [0.0, 0.0, 1.0, 1.0],
            ..Default::default()
        })?;

        self.renderer.present()?;
        Ok(())
    }

    pub fn on_mouse_down(&self, rdl: &RiddleContext) -> Result<(), RiddleError> {
        audio::ClipPlayerBuilder::new()
            .with_mode(audio::PlayMode::OneShot)
            .play(rdl.audio(), self.clip.clone())?
            .detach();
        Ok(())
    }
}

fn main() -> Result<(), RiddleError> {
    let rdl = RiddleApp::new()?;
    let mut state = DemoState::new(&rdl)?;

    rdl.run(move |rdl| match rdl.event() {
        window::SystemEvent::Window(window::WindowEvent::WindowClose(_)) => rdl.quit(),
        window::SystemEvent::Input(window::InputEvent::MouseButtonDown { .. }) => {
            state.on_mouse_down(rdl).unwrap();
        }
        window::SystemEvent::Input(window::InputEvent::KeyDown { scancode, .. }) => {
            match scancode {
                window::Scancode::M => {
                    state.music_player.pause();
                }
                window::Scancode::P => {
                    state.music_player.resume();
                }
                window::Scancode::Down => {
                    state.music_player.set_volume(0.5);
                }
                window::Scancode::Up => {
                    state.music_player.set_volume(1.0);
                }
                _ => (),
            }
            println!("KeyDown: {:?}", scancode);
        }
        window::SystemEvent::Input(window::InputEvent::KeyUp { scancode, .. }) => {
            println!("KeyUp: {:?}", scancode);
        }
        window::SystemEvent::ProcessFrame => {
            state.render_frame(rdl).unwrap();
        }
        _ => (),
    })
}

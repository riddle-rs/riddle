use math::{vec2, Rect, SpacialNumericConversion, Vector2};

use crate::*;

/////////////////////////////////////////////////////////////////////////////
// struct Sprite
/////////////////////////////////////////////////////////////////////////////

/// A renderable region of a texture.
///
/// Multiple sprites can share a single texture. Sprites can either be built using
/// [`Sprite`], or [`SpriteAtlasBuilder`].
///
/// Use [`SpriteRenderArgs`] for access to all supported paramters when rendering
/// sprites, or use [`Sprite::render_at`] to specify only a location and use default
/// arguments for everything else.
///
/// Sprites store a reference to the [`Renderer`] which built it, which will keep
/// the renderer alive as long as the sprite is alive.
///
/// # Example
///
/// ```no_run
/// # use riddle::{common::Color, image::*, platform::*, renderer::*, math::*, *};
/// # fn main() -> Result<(), RiddleError> {
/// # let rdl =  RiddleLib::new()?;
/// # let window = WindowBuilder::new().build(rdl.context())?;
/// let renderer = Renderer::new_from_window(&window)?;
///
/// // Load an image and create a sprite from it
/// let png_bytes = include_bytes!("../../example_assets/image.png");
/// let img = Image::load(&png_bytes[..], ImageFormat::Png)?;
/// let sprite = Sprite::new_from_image(&renderer, &img, &SpriteInitArgs::new())?;
///
/// // Render the sprite at the top left corner of the screen
/// let mut render_ctx = renderer.render(|render_ctx| {
///     render_ctx.clear(Color::WHITE);
///     sprite.render_at(render_ctx, vec2(0.0, 0.0))
/// })?;
/// # Ok(()) }
/// ```
pub struct Sprite<Device: WgpuDevice> {
	renderer: Renderer<Device>,
	texture: Texture,
	source_rect: Rect<f32>,
}

impl<Device: WgpuDevice> Sprite<Device> {
	pub(crate) fn from_texture(renderer: &Renderer<Device>, texture: &Texture) -> Result<Self> {
		let dimensions = texture.internal.dimensions.convert();
		Self::from_texture_with_bounds(
			renderer,
			texture,
			Rect {
				location: Vector2 { x: 0.0, y: 0.0 },
				dimensions,
			},
		)
	}

	#[allow(clippy::unnecessary_wraps)]
	pub(crate) fn from_texture_with_bounds(
		renderer: &Renderer<Device>,
		texture: &Texture,
		source_rect: Rect<f32>,
	) -> Result<Self> {
		Ok(Sprite {
			renderer: renderer.clone(),
			texture: texture.clone(),
			source_rect,
		})
	}
}

impl<Device: WgpuDevice> CommonSprite<Renderer<Device>> for Sprite<Device> {
	fn new_from_image(
		renderer: &Renderer<Device>,
		img: &image::Image,
		init_args: &SpriteInitArgs,
	) -> std::result::Result<Self, RendererError> {
		let texture = renderer.wgpu_device().with_device_info(|info| {
			Ok(Texture::from_image(
				info.device,
				info.queue,
				&img,
				init_args.mag_filter,
				init_args.min_filter,
				TextureType::Plain,
			))
		})?;
		Ok(Self::from_texture(renderer, &texture)?)
	}

	fn subsprite(&self, source_rect: &Rect<f32>) -> Self {
		let mut translated_source = source_rect.clone();
		translated_source.location += self.source_rect.location;

		Sprite {
			renderer: self.renderer.clone(),
			texture: self.texture.clone(),
			source_rect: self
				.source_rect
				.intersect(&translated_source)
				.unwrap_or_else(|| Rect::new(self.source_rect.location, vec2(0.0, 0.0))),
		}
	}

	fn dimensions(&self) -> Vector2<f32> {
		self.source_rect.dimensions
	}

	fn render_regions<Ctx: RenderContext<Renderer<Device>> + ?Sized>(
		&self,
		render_ctx: &mut Ctx,
		args: &SpriteRenderArgs,
		parts: &[(Rect<f32>, Vector2<f32>)],
	) -> std::result::Result<(), RendererError> {
		let rot: glam::Mat2 = glam::Mat2::from_angle(args.angle);
		let scale: glam::Mat2 = glam::Mat2::from_diagonal(args.scale.into());
		let origin: glam::Vec2 = args.location.into();
		let pivot: glam::Vec2 = args.pivot.into();

		let Vector2::<f32> {
			x: tex_width,
			y: tex_height,
		} = self.texture.internal.dimensions.convert();

		let vertex_data: Vec<Vertex> = parts
			.iter()
			.flat_map(|(src_rect, location)| {
				let location = glam::Vec2::from(*location);
				let src_rect = Rect::new(
					self.source_rect.location + src_rect.location,
					src_rect.dimensions,
				);

				let pos_topleft: glam::Vec2 = location - pivot;
				let pos_topright: glam::Vec2 = pos_topleft + glam::vec2(src_rect.dimensions.x, 0.0);
				let pos_bottomleft: glam::Vec2 =
					pos_topleft + glam::vec2(0.0, src_rect.dimensions.y);
				let pos_bottomright: glam::Vec2 =
					pos_bottomleft + glam::vec2(src_rect.dimensions.x, 0.0);

				let uv_top = src_rect.location.y / (tex_height as f32);
				let uv_left = src_rect.location.x / (tex_width as f32);
				let uv_bottom = uv_top + (src_rect.dimensions.y / (tex_height as f32));
				let uv_right = uv_left + (src_rect.dimensions.x / (tex_width as f32));

				let color_arr: [f32; 4] = args.diffuse_color.clone().into();

				vec![
					Vertex::ptc(
						origin + (rot * (scale * pos_topleft)),
						[uv_left, uv_top],
						&color_arr,
					),
					Vertex::ptc(
						origin + (rot * (scale * pos_bottomleft)),
						[uv_left, uv_bottom],
						&color_arr,
					),
					Vertex::ptc(
						origin + (rot * (scale * pos_bottomright)),
						[uv_right, uv_bottom],
						&color_arr,
					),
					Vertex::ptc(
						origin + (rot * (scale * pos_topright)),
						[uv_right, uv_top],
						&color_arr,
					),
				]
			})
			.collect();

		let index_data: Vec<u16> = parts
			.iter()
			.enumerate()
			.flat_map(|(i, _)| {
				let base = (i as u16) * 4;
				vec![1 + base, 2 + base, base, 2 + base, base, 3 + base]
			})
			.collect();

		let renderable = Renderable {
			texture: self.texture.clone(),
			shader: self.renderer.standard_res().default_shader.clone(),
			verts: &vertex_data[..],
			indices: &index_data[..],
		};

		render_ctx.draw(&renderable)
	}
}

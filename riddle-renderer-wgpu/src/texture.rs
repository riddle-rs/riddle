use std::num::NonZeroU32;

use crate::*;

use riddle_math::Vector2;

pub struct Texture {
	pub(crate) internal: std::sync::Arc<TextureInternal>,
}

impl Texture {
	pub(crate) fn from_image(
		device: &wgpu::Device,
		queue: &wgpu::Queue,
		image: &image::Image,
		mag_filter: FilterMode,
		min_filter: FilterMode,
		tex_type: TextureType,
	) -> Self {
		let texture = Texture::new(device, mag_filter, min_filter, tex_type, image.dimensions());

		let texture_extent = wgpu::Extent3d {
			width: image.width(),
			height: image.height(),
			depth_or_array_layers: 1,
		};

		queue.write_texture(
			wgpu::ImageCopyTexture {
				texture: &texture.internal.texture,
				mip_level: 0,
				origin: wgpu::Origin3d::ZERO,
			},
			image.as_rgba8(),
			wgpu::ImageDataLayout {
				offset: 0,
				bytes_per_row: NonZeroU32::new(image.width() * 4),
				rows_per_image: NonZeroU32::new(image.height()),
			},
			texture_extent,
		);

		texture
	}

	pub(crate) fn new(
		device: &wgpu::Device,
		mag_filter: FilterMode,
		min_filter: FilterMode,
		tex_type: TextureType,
		dimensions: Vector2<u32>,
	) -> Texture {
		let internal = TextureInternal::new(device, mag_filter, min_filter, tex_type, dimensions);
		Self {
			internal: internal.into(),
		}
	}
}

impl Clone for Texture {
	fn clone(&self) -> Self {
		Self {
			internal: self.internal.clone(),
		}
	}
}

fn filter_to_wgpu(filter: FilterMode) -> wgpu::FilterMode {
	match filter {
		FilterMode::Nearest => wgpu::FilterMode::Nearest,
		FilterMode::Linear => wgpu::FilterMode::Linear,
	}
}

pub(crate) enum TextureType {
	Plain,
	RenderTarget,
}

pub(crate) struct TextureInternal {
	pub texture: wgpu::Texture,
	pub sampler: wgpu::Sampler,
	pub dimensions: Vector2<u32>,
}

impl TextureInternal {
	pub(crate) fn new(
		device: &wgpu::Device,
		mag_filter: FilterMode,
		min_filter: FilterMode,
		tex_type: TextureType,
		dimensions: Vector2<u32>,
	) -> Self {
		let texture_extent = wgpu::Extent3d {
			width: dimensions.x,
			height: dimensions.y,
			depth_or_array_layers: 1,
		};

		let usage = match tex_type {
			TextureType::Plain => wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
			TextureType::RenderTarget => {
				wgpu::TextureUsage::SAMPLED
					| wgpu::TextureUsage::COPY_DST
					| wgpu::TextureUsage::RENDER_ATTACHMENT
			}
		};

		let format = match tex_type {
			TextureType::Plain => wgpu::TextureFormat::Rgba8UnormSrgb,
			TextureType::RenderTarget => wgpu::TextureFormat::Bgra8UnormSrgb,
		};

		let texture = device.create_texture(&wgpu::TextureDescriptor {
			size: texture_extent,
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format,
			usage,
			label: None,
		});

		let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			address_mode_u: wgpu::AddressMode::ClampToEdge,
			address_mode_v: wgpu::AddressMode::ClampToEdge,
			address_mode_w: wgpu::AddressMode::ClampToEdge,
			mag_filter: filter_to_wgpu(mag_filter),
			min_filter: filter_to_wgpu(min_filter),
			..Default::default()
		});

		Self {
			texture,
			sampler,
			dimensions,
		}
	}
}

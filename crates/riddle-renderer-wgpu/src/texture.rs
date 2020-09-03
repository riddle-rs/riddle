use crate::*;

use riddle_math::Vector2;

pub(super) struct Texture {
    pub(crate) texture: wgpu::Texture,
    pub(crate) sampler: wgpu::Sampler,
    pub dimensions: Vector2<u32>,
}

impl Texture {
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: image::Image,
        mag_filter: FilterMode,
        min_filter: FilterMode,
    ) -> Result<Texture, RendererError> {
        let texture_extent = wgpu::Extent3d {
            width: image.width(),
            height: image.height(),
            depth: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label: None,
        });

        queue.write_texture(
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            image.as_rgba8(),
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: image.width() * 4,
                rows_per_image: image.height(),
            },
            texture_extent,
        );

        // Create other resources
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: mag_filter.into(),
            min_filter: min_filter.into(),
            ..Default::default()
        });

        Ok(Texture {
            texture,
            sampler,
            dimensions: image.dimensions(),
        }
        .into())
    }
}

pub enum FilterMode {
    Nearest,
    Linear,
}

impl From<FilterMode> for wgpu::FilterMode {
    fn from(f: FilterMode) -> Self {
        match f {
            FilterMode::Nearest => wgpu::FilterMode::Nearest,
            FilterMode::Linear => wgpu::FilterMode::Linear,
        }
    }
}

impl Default for FilterMode {
    fn default() -> Self {
        FilterMode::Nearest
    }
}

use crate::*;

use riddle_math::Vector2;

pub(crate) struct WGPUTexture {
    weak_self: WGPUTextureWeak,
    pub(crate) texture: wgpu::Texture,
    pub(crate) sampler: wgpu::Sampler,
    pub dimensions: Vector2<u32>,
}

define_handles!(<WGPUTexture>::weak_self, pub(crate) WGPUTextureHandle, pub(crate) WGPUTextureWeak);

impl WGPUTexture {
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: image::Image,
        mag_filter: FilterMode,
        min_filter: FilterMode,
        tex_type: TextureType,
    ) -> Result<WGPUTextureHandle> {
        let texture =
            WGPUTexture::new_shared(device, mag_filter, min_filter, tex_type, image.dimensions())?;

        let texture_extent = wgpu::Extent3d {
            width: image.width(),
            height: image.height(),
            depth: 1,
        };

        queue.write_texture(
            wgpu::TextureCopyView {
                texture: &texture.texture,
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

        Ok(texture)
    }

    pub fn new_shared(
        device: &wgpu::Device,
        mag_filter: FilterMode,
        min_filter: FilterMode,
        tex_type: TextureType,
        dimensions: Vector2<u32>,
    ) -> Result<WGPUTextureHandle> {
        let texture_extent = wgpu::Extent3d {
            width: dimensions.x,
            height: dimensions.y,
            depth: 1,
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
            mag_filter: mag_filter.into(),
            min_filter: min_filter.into(),
            ..Default::default()
        });

        Ok(WGPUTextureHandle::new(|weak_self| WGPUTexture {
            weak_self,
            texture,
            sampler,
            dimensions,
        }))
    }
}

/// Interpolation mode between texels when rendering
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

pub(crate) enum TextureType {
    Plain,
    RenderTarget,
}

use std::num::NonZeroU64;

use crate::*;

use math::Vector2;
use wgpu::util::DeviceExt;
use wgpu::{CommandEncoder, RenderPass, TextureView};

pub struct Shader {
	pub(crate) internal: std::sync::Arc<ShaderInternal>,
}

impl Shader {
	pub(crate) fn from_readers<SR>(
		device: &wgpu::Device,
		shader_reader: SR,
		primitive_type: wgpu::PrimitiveTopology,
	) -> Result<Self>
	where
		SR: std::io::Read + std::io::Seek,
	{
		let internal = ShaderInternal::from_readers(device, shader_reader, primitive_type)?;
		Ok(Self {
			internal: internal.into(),
		})
	}
}

impl Clone for Shader {
	fn clone(&self) -> Self {
		Self {
			internal: self.internal.clone(),
		}
	}
}

pub(crate) struct ShaderInternal {
	pub bind_group_layout: wgpu::BindGroupLayout,
	pipeline: wgpu::RenderPipeline,
}

impl ShaderInternal {
	pub(crate) fn from_readers<SR>(
		device: &wgpu::Device,
		mut shader_reader: SR,
		primitive_type: wgpu::PrimitiveTopology,
	) -> Result<Self>
	where
		SR: std::io::Read + std::io::Seek,
	{
		let mut wgsl_buf = vec![];
		shader_reader
			.read_to_end(&mut wgsl_buf)
			.map_err(CommonError::IoError)?;
		let wgsl_str =
			std::str::from_utf8(&wgsl_buf[..]).map_err(|_| WgpuRendererError::Unknown)?;
		let wgsl_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
			source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::from(wgsl_str)),
			flags: wgpu::ShaderFlags::VALIDATION,
			label: None,
		});

		let vertex_size = std::mem::size_of::<Vertex>();

		let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			entries: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStage::VERTEX,
					ty: wgpu::BindingType::Buffer {
						has_dynamic_offset: false,
						ty: wgpu::BufferBindingType::Uniform,
						min_binding_size: wgpu::BufferSize::new(64),
					},
					count: None,
				},
				wgpu::BindGroupLayoutEntry {
					binding: 1,
					visibility: wgpu::ShaderStage::FRAGMENT,
					ty: wgpu::BindingType::Texture {
						multisampled: false,
						sample_type: wgpu::TextureSampleType::Float { filterable: true },
						view_dimension: wgpu::TextureViewDimension::D2,
					},
					count: None,
				},
				wgpu::BindGroupLayoutEntry {
					binding: 2,
					visibility: wgpu::ShaderStage::FRAGMENT,
					ty: wgpu::BindingType::Sampler {
						filtering: true,
						comparison: false,
					},
					count: None,
				},
			],
			label: None,
		});
		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: None,
			bind_group_layouts: &[&bind_group_layout],
			push_constant_ranges: &[],
		});

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: None,
			layout: Some(&pipeline_layout),
			vertex: wgpu::VertexState {
				module: &wgsl_module,
				entry_point: "vs_main",
				buffers: &[wgpu::VertexBufferLayout {
					array_stride: vertex_size as wgpu::BufferAddress,
					step_mode: wgpu::InputStepMode::Vertex,
					attributes: &[
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float2,
							offset: 0,
							shader_location: 0,
						},
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float2,
							offset: (std::mem::size_of::<f32>() * 2) as u64,
							shader_location: 1,
						},
						wgpu::VertexAttribute {
							format: wgpu::VertexFormat::Float4,
							offset: (std::mem::size_of::<f32>() * 4) as u64,
							shader_location: 2,
						},
					],
				}],
			},
			fragment: Some(wgpu::FragmentState {
				module: &wgsl_module,
				entry_point: "fs_main",
				targets: &[wgpu::ColorTargetState {
					format: wgpu::TextureFormat::Bgra8UnormSrgb,
					color_blend: wgpu::BlendState {
						src_factor: wgpu::BlendFactor::SrcAlpha,
						dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
						operation: wgpu::BlendOperation::Add,
					},
					alpha_blend: wgpu::BlendState {
						src_factor: wgpu::BlendFactor::One,
						dst_factor: wgpu::BlendFactor::One,
						operation: wgpu::BlendOperation::Add,
					},
					write_mask: wgpu::ColorWrite::ALL,
				}],
			}),
			primitive: wgpu::PrimitiveState {
				topology: primitive_type,
				strip_index_format: None,
				front_face: wgpu::FrontFace::Ccw,
				cull_mode: wgpu::CullMode::None,
				polygon_mode: wgpu::PolygonMode::Fill,
			},
			depth_stencil: None,
			multisample: wgpu::MultisampleState {
				count: 1,
				mask: !0,
				alpha_to_coverage_enabled: false,
			},
		});

		Ok(Self {
			bind_group_layout,
			pipeline: render_pipeline,
		})
	}

	pub(crate) fn bind_params(
		&self,
		device: &wgpu::Device,
		camera_size: Vector2<f32>,
		view_matrix: mint::ColumnMatrix4<f32>,
		texture: &Texture,
	) -> wgpu::BindGroup {
		let ortho_matrix =
			glam::Mat4::orthographic_lh(0.0, camera_size.x, camera_size.y, 0.0, 0.0, 1.0);
		let view_matrix: glam::Mat4 = view_matrix.into();
		let result_matrix = ortho_matrix * view_matrix;
		let matrix_arr: &[f32; 16] = result_matrix.as_ref();

		let camera_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: None,
			contents: bytemuck::cast_slice(matrix_arr),
			usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
		});

		device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &self.bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::Buffer {
						buffer: &camera_uniform,
						offset: 0,
						size: NonZeroU64::new(std::mem::size_of::<f32>() as u64 * 16),
					},
				},
				wgpu::BindGroupEntry {
					binding: 1,
					resource: wgpu::BindingResource::TextureView(
						&texture
							.internal
							.texture
							.create_view(&wgpu::TextureViewDescriptor {
								..Default::default()
							}),
					),
				},
				wgpu::BindGroupEntry {
					binding: 2,
					resource: wgpu::BindingResource::Sampler(&texture.internal.sampler),
				},
			],
			label: None,
		})
	}

	pub(crate) fn begin_render_pass<'a>(
		&'a self,
		target: &'a TextureView,
		encoder: &'a mut CommandEncoder,
		load_op: wgpu::LoadOp<wgpu::Color>,
	) -> RenderPass<'a> {
		let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
				attachment: target,
				resolve_target: None,
				ops: wgpu::Operations {
					load: load_op,
					store: true,
				},
			}],
			depth_stencil_attachment: None,
			label: None,
		});
		rpass.set_pipeline(&self.pipeline);
		rpass
	}
}

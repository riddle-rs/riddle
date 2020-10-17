use crate::wgpu_ext::*;

use wgpu::util::DeviceExt;
use wgpu::{CommandEncoder, RenderPass, TextureView};

pub(crate) struct WGPUShader {
    weak_self: WGPUShaderWeak,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pipeline: wgpu::RenderPipeline,
}

define_handles!(<WGPUShader>::weak_self, pub(crate) WGPUShaderHandle, pub(crate) WGPUShaderWeak);

impl WGPUShader {
    pub(crate) fn from_readers<VR, FR>(
        device: &wgpu::Device,
        mut vs: VR,
        mut fs: FR,
        primitive_type: wgpu::PrimitiveTopology,
    ) -> Result<WGPUShaderHandle>
    where
        VR: std::io::Read + std::io::Seek,
        FR: std::io::Read + std::io::Seek,
    {
        let mut vs_buf = vec![];
        vs.read_to_end(&mut vs_buf).map_err(CommonError::IOError)?;
        let vs_module = device.create_shader_module(wgpu::ShaderModuleSource::SpirV(
            std::borrow::Cow::from(bytemuck::cast_slice(&vs_buf)),
        ));

        let mut fs_buf = vec![];
        fs.read_to_end(&mut fs_buf).map_err(CommonError::IOError)?;
        let fs_module = device.create_shader_module(wgpu::ShaderModuleSource::SpirV(
            std::borrow::Cow::from(bytemuck::cast_slice(&fs_buf)),
        ));

        let vertex_size = std::mem::size_of::<Vertex>();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        component_type: wgpu::TextureComponentType::Float,
                        dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
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
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                ..Default::default()
            }),
            primitive_topology: primitive_type,
            color_states: &[wgpu::ColorStateDescriptor {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                color_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha_blend: wgpu::BlendDescriptor {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                },
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[wgpu::VertexBufferDescriptor {
                    stride: vertex_size as wgpu::BufferAddress,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttributeDescriptor {
                            format: wgpu::VertexFormat::Float2,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttributeDescriptor {
                            format: wgpu::VertexFormat::Float2,
                            offset: (std::mem::size_of::<f32>() * 2) as u64,
                            shader_location: 1,
                        },
                        wgpu::VertexAttributeDescriptor {
                            format: wgpu::VertexFormat::Float4,
                            offset: (std::mem::size_of::<f32>() * 4) as u64,
                            shader_location: 2,
                        },
                    ],
                }],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Ok(WGPUShaderHandle::new(|weak_self| Self {
            weak_self,
            bind_group_layout,
            pipeline: render_pipeline,
        }))
    }

    pub(crate) fn bind_params(
        &self,
        device: &wgpu::Device,
        camera_size: Vector2<f32>,
        view_matrix: mint::ColumnMatrix4<f32>,
        texture: &WGPUTexture,
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
                    resource: wgpu::BindingResource::Buffer(camera_uniform.slice(..)),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture.texture.create_view(
                        &wgpu::TextureViewDescriptor {
                            ..Default::default()
                        },
                    )),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
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
        });
        rpass.set_pipeline(&self.pipeline);
        rpass
    }
}

use std::marker::PhantomData;

use matrix_engine::components::resources::Resource;
use wgpu::{
    Device, FragmentState, PipelineLayout, PrimitiveState, RenderPass, RenderPipeline,
    SurfaceConfiguration, VertexState,
};

use super::{
    buffers::{BufferContainer, Bufferable, Vertex},
    group_cluster::{BindGroupCluster, BindGroupLayoutContainerCluster},
    shaders::{MatrixShaders, ShaderConfig},
};

pub struct MatrixRenderPipelineArgs<'a> {
    pub device: &'a Device,
    pub shaders: MatrixShaders,
    pub shader_config: ShaderConfig,
    pub pipe_label: &'a str,
    pub group_label: &'a str,
    pub surface_config: &'a SurfaceConfiguration,
    pub primitive_state: PrimitiveState,
}

pub struct MatrixRenderPipeline<B: Bufferable, T: BindGroupCluster> {
    marker: PhantomData<(B, T)>,
    pipeline: RenderPipeline,
    layout: PipelineLayout,
    shaders: MatrixShaders,
}
impl<B: Bufferable, T: BindGroupCluster> Resource for MatrixRenderPipeline<B, T> {}

impl<B: Bufferable, T: BindGroupCluster> MatrixRenderPipeline<B, T> {
    pub fn apply_groups<'a>(&self, pass: &mut RenderPass<'a>, data: T::Args<'a>) {
        T::apply_to_pipeline(pass, data);
    }

    pub fn pipeline(&self) -> &RenderPipeline {
        &self.pipeline
    }

    pub fn begin<'a: 'b, 'b>(&'a self, pass: &mut RenderPass<'b>) {
        pass.set_pipeline(&self.pipeline)
    }

    pub(crate) fn apply_buffer<'a>(
        &self,
        pass: &mut RenderPass<'a>,
        buffer: &'a BufferContainer<Vertex>,
    ) {
        if let Some(indexes) = buffer.index_buffer() {
            pass.set_index_buffer(indexes.slice(..), wgpu::IndexFormat::Uint16);
        }

        pass.set_vertex_buffer(0, buffer.buffer().slice(..));
    }

    pub fn new(
        MatrixRenderPipelineArgs {
            device,
            group_label,
            pipe_label,
            shader_config: shader_conf,
            shaders,
            surface_config,
            primitive_state,
        }: MatrixRenderPipelineArgs<'_>,
    ) -> Self {
        let ls = T::create_bind_group_layouts(group_label, device);
        let ls = Box::new(ls.iter_groups().collect::<Vec<_>>());

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(pipe_label),
            bind_group_layouts: &ls,
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(pipe_label),
            layout: Some(&layout),
            vertex: VertexState {
                module: shaders.module(),
                buffers: &[B::describe()],
                entry_point: shader_conf.vertex_entry(),
            },
            fragment: Some(FragmentState {
                module: shaders.module(),
                entry_point: shader_conf.fragment_entry(),
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: primitive_state,
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            marker: PhantomData,
            pipeline,
            shaders,
            layout,
        }
    }

    pub(crate) fn draw(
        &self,
        pass: &mut RenderPass<'_>,
        range: std::ops::Range<u32>,
        instances: std::ops::Range<u32>,
    ) {
        pass.draw_indexed(range, 0, instances);
    }
}

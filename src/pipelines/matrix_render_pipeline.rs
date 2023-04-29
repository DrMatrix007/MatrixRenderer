use std::marker::PhantomData;

use wgpu::{Device, PipelineLayout, PipelineLayoutDescriptor, RenderPipeline};

use super::group_cluster::{BindGroupCluster, BindGroupLayoutContainerCluster};

pub struct MatrixRenderPipline<T: BindGroupCluster> {
    marker: PhantomData<T>,
    pipeline: RenderPipeline,
    layout: PipelineLayout,
}

impl<T: BindGroupCluster> MatrixRenderPipline<T> {
    pub fn new(device: &Device, pipe_label: &str, group_label: &str) -> Self {
        let ls = T::create_bind_group_layouts(group_label, device);
        let ls = Box::new(ls.iter_groups().collect::<Vec<_>>());

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(pipe_label),
            bind_group_layouts: &ls,
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(pipe_label),
            layout: Some(&pipeline_layout),
            vertex: (),
            primitive: (),
            depth_stencil: (),
            multisample: (),
            fragment: (),
            multiview: (),
        });

        let pipeline;
        Self {
            marker: PhantomData,
            pipeline,
        }
    }
}

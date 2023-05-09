use std::{fs, io};

use image::{GenericImageView, ImageError};

pub struct MatrixTexture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

#[derive(Debug)]
pub enum MatrixTextureLoadError {
    ImageError(ImageError),
    IOError(io::Error),
}

impl MatrixTexture {
    pub fn from_name(
        img: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: &str,
    ) -> Result<Self, MatrixTextureLoadError> {
        let img = match fs::read_to_string(img) {
            Ok(data) => data,
            Err(e) => return Err(MatrixTextureLoadError::IOError(e)),
        }
        .into_bytes();

        Self::from_bytes(&img, device, queue, label)
    }

    pub fn from_bytes(
        img: &[u8],
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: &str,
    ) -> Result<Self, MatrixTextureLoadError> {
        let img = match image::load_from_memory(img) {
            Ok(data) => data,
            Err(e) => return Err(MatrixTextureLoadError::ImageError(e)),
        };
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }
    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}

#[macro_export]
macro_rules! texture {
    ($path:expr,$device:expr,$queue:expr,$label:expr) => {
        $crate::pipelines::texture::MatrixTexture::from_bytes(
            $device,
            $queue,
            include_bytes!($path),
            $label,
        )
    };
}

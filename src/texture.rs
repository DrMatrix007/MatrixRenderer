use std::num::NonZeroU32;

use image::GenericImageView;
use wgpu::{
    Device, Extent3d, ImageCopyTexture, ImageDataLayout, Origin3d, Queue, Texture,
    TextureDescriptor, TextureFormat, TextureUsages, TextureView,
};

pub struct TextureData {
    texture: Texture,
    view: TextureView,
}

impl TextureData {
    pub fn view(&self) -> &TextureView  {
        &self.view
    }
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn new(image_bytes: &[u8], device: &Device, queue: &Queue) -> Self {
        let image = image::load_from_memory(image_bytes).unwrap();
        let image_rgba = image.to_rgba8();
        let size = image.dimensions();

        let ext_size = Extent3d {
            depth_or_array_layers: 1,
            height: size.1,
            width: size.0,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: ext_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                mip_level: 0,
                origin: Origin3d::ZERO,
                texture: &texture,
            },
            &image_rgba,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * size.0),
                rows_per_image: NonZeroU32::new(size.1),
            },
            ext_size,
        );

        let view = texture.create_view(&Default::default());

        Self { texture, view }
    }
}

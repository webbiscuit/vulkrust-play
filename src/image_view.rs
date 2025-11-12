use ash::vk::{ComponentMapping, ComponentSwizzle, Format, Image, ImageAspectFlags, ImageSubresourceRange, ImageViewType};
use anyhow::Result;

use crate::LogicalDevice;

pub struct ImageView {
    raw: ash::vk::ImageView,
    device: ash::Device
}

impl ImageView {
    pub fn new(device: &LogicalDevice, image: &Image, image_format: &Format) -> Result<Self> {

        let create_info = ash::vk::ImageViewCreateInfo {
            image: *image,
            view_type: ImageViewType::TYPE_2D,
            format: *image_format,
            // Can map the channels for i.e. monochrome textures
            components: ComponentMapping {
                r: ComponentSwizzle::IDENTITY,
                g: ComponentSwizzle::IDENTITY,
                b: ComponentSwizzle::IDENTITY,
                a: ComponentSwizzle::IDENTITY,
            },
            // Can do stereographic stuff here, i.e. different layers for different eyes
            subresource_range: ImageSubresourceRange {
                aspect_mask: ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1
            },
            ..Default::default()
        };

        let image_view = unsafe { device.raw().create_image_view(&create_info, None)? };

        Ok(Self {
            raw: image_view,
            device: device.raw().clone()
        })
    }
}

impl Drop for ImageView {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_image_view(self.raw, None);
        }
    }
}
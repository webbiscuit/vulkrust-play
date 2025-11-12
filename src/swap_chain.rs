use ash::vk::{CompositeAlphaFlagsKHR, ImageUsageFlags, PhysicalDevice, SharingMode, SwapchainKHR};
use crate::{LogicalDevice, Surface, instance::Instance, logical_device::find_queue_families};
use anyhow::Result;

pub struct SwapChain {
    swapchain: SwapchainKHR,
    swapchain_loader: ash::khr::swapchain::Device
}

impl SwapChain {
    pub fn new(instance: &Instance, physical_device: &PhysicalDevice, logical_device: &LogicalDevice, surface: &Surface, width: u32, height: u32) -> Result<Self>{
        let surface_capabilities = surface.query_surface_capabilities(*physical_device)?;

        let best_surface_format = surface_capabilities.find_best_format().expect("Could not find a good surface");
        println!("Chosen surface: {:?}", best_surface_format);
        let best_present_mode = surface_capabilities.find_best_present_mode().expect("Could not find present mode");
        println!("Chosen present mode: {:?}", best_present_mode);
        let extent_2d = surface_capabilities.find_swap_extent(width, height);
        println!("Extent extent2D: {:?}", extent_2d);
        
        let mut create_info = ash::vk::SwapchainCreateInfoKHR {
            surface: *surface.raw(),
            min_image_count: surface_capabilities.image_count(),
            image_format: best_surface_format.format,
            image_color_space: best_surface_format.color_space,
            image_extent: extent_2d,
            image_array_layers: 1,
            image_usage: ImageUsageFlags::COLOR_ATTACHMENT,
            ..Default::default()
        };

        let indices = find_queue_families(instance, physical_device, surface)?;
        let queue_family_indicies = [indices.graphics_family.unwrap(), indices.present_family.unwrap()];

        if indices.graphics_family != indices.present_family {
            create_info.image_sharing_mode = SharingMode::CONCURRENT;
            create_info.queue_family_index_count = queue_family_indicies.len() as u32;
            create_info.p_queue_family_indices = queue_family_indicies.as_ptr();
        } else {
            // Should be case on most hardware
            create_info.image_sharing_mode = SharingMode::EXCLUSIVE;
            create_info.queue_family_index_count = 0;
        }

        create_info.pre_transform = surface_capabilities.capabilities().current_transform;
        // Almost always want to ignore other windows
        create_info.composite_alpha = CompositeAlphaFlagsKHR::OPAQUE;
        create_info.present_mode = best_present_mode;
        create_info.clipped = 1;
        create_info.old_swapchain = SwapchainKHR::null();

        let swapchain_loader = ash::khr::swapchain::Device::new(instance.raw(), logical_device.raw());
        let swapchain = unsafe {
            swapchain_loader
                .create_swapchain(&create_info, None)?
        };

        Ok(SwapChain {
            swapchain_loader,
            swapchain: SwapchainKHR::null()
        })
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        unsafe {
            println!("Dropping SwapChain");


            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
        }
    }
}
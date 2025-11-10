use ash::vk::PhysicalDevice;
use crate::{instance::Instance};

pub struct SwapChainSupportDetails {
    
}

impl SwapChainSupportDetails {
    pub fn is_adequate(&self) -> bool {
        true
    }
}

// pub fn query_swap_chain_support(surface: &ash::khr::surface::Instance, instance: &Instance, physical_device: &PhysicalDevice) -> SwapChainSupportDetails {
//     // let capabilities = instance.raw().get_phy

//     // surface.get_physical_device_surface_capabilities(physical_device, surface)
    
// }
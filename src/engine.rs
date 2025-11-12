use ash::vk::{PhysicalDevice, PhysicalDeviceType, SurfaceKHR};
use ash_window::enumerate_required_extensions;
use raw_window_handle::{HasDisplayHandle};
use winit::window::Window;
use crate::{Instance, LogicalDevice, Surface, logical_device::find_queue_families, surface, swap_chain::{self, SwapChain}, utils::vk_str_to_string};
use anyhow::{Error, Result};

pub struct VulkanEngine {
    logical_device: LogicalDevice,
    surface: Surface,
    instance: Instance, // Must be last
}

impl VulkanEngine {
    pub fn new(app_name: &str, enable_validation: bool, window: &Window) -> Result<Self> {
        let wsi_exts  = enumerate_required_extensions(window.display_handle()?.into())?;
        let window_dims = window.inner_size();

        let instance = Instance::new(app_name, enable_validation, Some(wsi_exts))?;
        let surface = Surface::new(&instance, window)?;
        assert_ne!(*surface.raw(), SurfaceKHR::null());
        let physical_device = Self::pick_suitable_device(&instance, &surface)?;
        let logical_device = LogicalDevice::new(&instance, &physical_device,  &surface, &Self::required_device_prop_names())?;
        let swap_chain = SwapChain::new(&instance, &physical_device, &logical_device, &surface, window_dims.width, window_dims.height)?;

        Ok(VulkanEngine { 
            surface,
            instance, 
            logical_device
        })
    }

    fn required_device_prop_names() -> Vec<String> {
        let swapchain = ash::khr::swapchain::NAME;
        let swapchain = swapchain.to_str().unwrap().to_string();

        vec![swapchain]
    }

    pub fn pick_suitable_device(instance: &Instance, surface: &Surface) -> Result<PhysicalDevice> {
        let physical_devices = unsafe { instance.raw().enumerate_physical_devices()? };

        let mut best_score = 0;
        let mut best_device: Option<PhysicalDevice> = None;

        for device in physical_devices.iter() {
            if !Self::is_physical_device_suitable(instance, device, surface)? {
                continue;
            }

            let score = Self::rate_physical_device_suitability(instance, device);

            if score > best_score {
                best_device = Some(*device);
                best_score = score;
            }
        }

        if let Some(best_device) = best_device {
            return Ok(best_device)
        }

        Err(Error::msg("Failed to find a suitable GPU with vulkan support"))
    }

    fn is_physical_device_suitable(instance: &Instance, device: &PhysicalDevice, surface: &Surface) -> Result<bool> {
        let indices = find_queue_families(instance, device, surface)?;

        let extensions_supported = check_physical_device_extension_support(instance, device, &Self::required_device_prop_names())?;

        Ok(indices.is_complete() && extensions_supported)
    }

    fn rate_physical_device_suitability(instance: &Instance, device: &PhysicalDevice) -> u32 {
        let mut score = 0;

        let device_properties  = unsafe {instance.raw().get_physical_device_properties(*device) };
        let device_features  = unsafe {instance.raw().get_physical_device_features(*device) };

        // Discrete GPUs have a significant performance advantage
        if device_properties.device_type == PhysicalDeviceType::DISCRETE_GPU {
            score += 1000;
        }

        // Maximum possible size of textures affects graphics quality
        score += device_properties.limits.max_image_dimension2_d;

        // Application can't function without geometry shaders
        if device_features.geometry_shader == 0 {
            return 0;
        }

        let name = unsafe {std::ffi::CStr::from_ptr(device_properties.device_name.as_ptr()) };
        println!("{name:?} - Scores {score}");

        score
    }
}

fn check_physical_device_extension_support(instance: &Instance, physical_device: &PhysicalDevice, required_props_names: &[String]) -> Result<bool> {
    let extension_props = unsafe { instance.raw().enumerate_device_extension_properties(*physical_device)? };
    let mut required_props_names = required_props_names.to_vec();

    for prop in extension_props {
        let ix = required_props_names.iter().position(|p| p == &vk_str_to_string(&prop.extension_name));

        if let Some(ix) = ix {
            required_props_names.remove(ix); 

            if required_props_names.is_empty() {
                return Ok(true);
            }
        }
    }

    Ok(required_props_names.is_empty())
}
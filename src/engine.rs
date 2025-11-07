use ash::vk::{PhysicalDevice, PhysicalDeviceType, SurfaceKHR};
use ash_window::enumerate_required_extensions;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::{self, Window};
use crate::{LogicalDevice, Instance, Surface, logical_device::find_queue_families, surface};
use anyhow::{Error, Result};

pub struct VulkanEngine {
    surface: Surface,
    device: LogicalDevice,
    instance: Instance, // Must be last
}

impl VulkanEngine {
    pub fn new(app_name: &str, enable_validation: bool, window: &Window) -> Result<Self> {
        
        let wsi_exts  = enumerate_required_extensions(window.display_handle()?.into())?;

        let instance = Instance::new(app_name, enable_validation, Some(wsi_exts))?;
        let physical_device = Self::pick_suitable_device(&instance)?;
        let device = LogicalDevice::new(&instance, &physical_device)?;
        let surface = Surface::new(&instance, window)?;

        println!("Surface handle: {:?}", surface.raw());
        assert_ne!(*surface.raw(), SurfaceKHR::null());


        Ok(VulkanEngine { 
            surface,
            instance, 
            device
        })
    }

    pub fn pick_suitable_device(instance: &Instance) -> Result<PhysicalDevice> {
        let physical_devices = unsafe { instance.raw().enumerate_physical_devices()? };

        let mut best_score = 0;
        let mut best_device: Option<PhysicalDevice> = None;

        for device in physical_devices.iter() {
            if !Self::is_device_suitable(instance, device) {
                continue;
            }

            let score = Self::rate_device_suitability(instance, device);

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

    fn is_device_suitable(instance: &Instance, device: &PhysicalDevice) -> bool {
        let indices = find_queue_families(instance, device);
        indices.is_complete()
    }

    fn rate_device_suitability(instance: &Instance, device: &PhysicalDevice) -> u32 {
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
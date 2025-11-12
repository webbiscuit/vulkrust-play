use ash::vk::{self, PhysicalDevice, QueueFlags};
use crate::{Surface, instance::Instance, utils::VkStringArray};
use anyhow::Result;

pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
    pub present_family: Option<u32> 
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}

pub struct LogicalDevice {
    raw: ash::Device,
    graphics_queue: vk::Queue
}

impl LogicalDevice {
    pub fn new(instance: &Instance, physical_device: &PhysicalDevice, surface: &Surface, required_props_names: &[String]) -> Result<Self> {
        let family_indicies = find_queue_families(instance, physical_device, surface)?;

        let queue_priority = 1.0f32;

        let queue_create_info = vk::DeviceQueueCreateInfo {
            queue_family_index: family_indicies.graphics_family.unwrap(),
            queue_count: 1,
            p_queue_priorities: &queue_priority,
            ..Default::default()
        };
        let device_features = vk::PhysicalDeviceFeatures {
            ..Default::default()
        };

        let prepared_required_props_names = VkStringArray::new(required_props_names);
        
        let device_create_info = vk::DeviceCreateInfo {
            p_queue_create_infos: &queue_create_info,
            queue_create_info_count: 1,
            p_enabled_features: &device_features,
            pp_enabled_extension_names: prepared_required_props_names.as_ptrs(),
            enabled_extension_count: required_props_names.len() as u32,
            ..Default::default()
        };
        // Can set validation layers here

        let device = unsafe { instance.raw().create_device(*physical_device, &device_create_info, None)? };

        let graphics_queue = unsafe { device.get_device_queue(family_indicies.graphics_family.unwrap(), 0) };

        Ok(Self {
            raw: device,
            graphics_queue
        })
    }

    pub fn raw(&self) -> &ash::Device {
        &self.raw
    }
}

impl Drop for LogicalDevice {
    fn drop(&mut self) {
        println!("Dropping LogicalDevice");
        unsafe {
            self.raw.device_wait_idle().ok();
            self.raw.destroy_device(None);
        }
    }
}

pub fn find_queue_families(instance: &Instance, physical_device: &PhysicalDevice, surface: &Surface) -> Result<QueueFamilyIndices> {
    let props = unsafe { instance.raw().get_physical_device_queue_family_properties(*physical_device) };

    let mut graphics_index = None;
    let mut present_index = None;

    for (ix, prop) in props.iter().enumerate() {
        if prop.queue_flags.contains(QueueFlags::GRAPHICS) {
            graphics_index = Some(ix as u32);
        }

        let has_support = unsafe {surface.surface_instance().get_physical_device_surface_support(*physical_device, ix as u32, *surface.raw())? };

        if has_support {
            present_index = Some(ix as u32);
        }

        break;
    }

    Ok(QueueFamilyIndices { graphics_family: graphics_index, present_family: present_index })
}

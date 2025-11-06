use ash::{Entry, vk::{self, PhysicalDevice, PhysicalDeviceType, Queue, QueueFlags}};
use crate::debug::{DebugState, debug_callback};
use anyhow::{Error, Result};

struct QueueFamilyIndices {
    graphics_family: Option<u32> 
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some()
    }
}

pub struct VulkanEngine {
    _entry: ash::Entry,
    instance: ash::Instance,
    debug: Option<DebugState>,
    device: Option<ash::Device>,
    graphics_queue: vk::Queue
}

impl VulkanEngine {
    pub fn new(app_name: &str, enable_validation: bool) -> Result<Self> {
        let entry = unsafe { Entry::load()? };

        let app_name = std::ffi::CString::new(app_name)?;
        let engine_name = std::ffi::CString::new("Dan's on Vulkan Engine")?;
        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 3, 0),
            p_application_name: app_name.as_ptr(),
            p_engine_name: engine_name.as_ptr(),
            ..Default::default()
        };

        let validation_layer = std::ffi::CString::new("VK_LAYER_KHRONOS_validation")?;
        let mut enabled_layers = Vec::new();
        let mut enabled_exts = Vec::new(); 
        let mut debug_ci_opt: Option<vk::DebugUtilsMessengerCreateInfoEXT> = None;

        if enable_validation {
            enabled_layers.push(validation_layer.as_ptr());
            enabled_exts.push(ash::ext::debug_utils::NAME.as_ptr()); 

            let debug_messenger_create_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
                .message_severity(
                    vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                    // | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE,
                )
                .message_type(
                    vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                )
                .pfn_user_callback(Some(debug_callback));

            debug_ci_opt = Some(debug_messenger_create_info);
        }

        let mut create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            pp_enabled_layer_names: enabled_layers.as_ptr(),
            enabled_layer_count: enabled_layers.len() as u32,
            pp_enabled_extension_names: enabled_exts.as_ptr(),
            enabled_extension_count: enabled_exts.len() as u32,
            ..Default::default()
        };

        if let Some(ref mut debug_ci) = debug_ci_opt {
            create_info = create_info.push_next(debug_ci);
        }

        let instance = unsafe { entry.create_instance(&create_info, None)? };

        let mut debug_state: Option<DebugState> = None;

        if let Some(ref debug_ci) = debug_ci_opt {
            let debug_utils = ash::ext::debug_utils::Instance::new(&entry, &instance);
            let debug_messenger = unsafe { debug_utils.create_debug_utils_messenger(debug_ci, None)? };

            debug_state = Some(
                DebugState::new(debug_utils, debug_messenger)
            )
        }

        Ok(VulkanEngine { 
            _entry: entry, 
            instance, 
            debug: debug_state,
            device: None,
            graphics_queue: Queue::null()
        })
    }

    pub fn phase2(&mut self) -> Result<()> {
        let physical_device = self.pick_suitable_device()?;
        let family_indicies = self.find_queue_families(&physical_device);

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
        let device_create_info = vk::DeviceCreateInfo {
            p_queue_create_infos: &queue_create_info,
            queue_create_info_count: 1,
            p_enabled_features: &device_features,
            ..Default::default()
        };
        // Can set validation layers here

        let device = unsafe { self.instance.create_device(physical_device, &device_create_info, None)? };

        let queue = unsafe { device.get_device_queue(family_indicies.graphics_family.unwrap(), 0) };

        self.device = Some(device);
        self.graphics_queue = queue;

        Ok(())
    }

    fn find_queue_families(&self, physical_device: &PhysicalDevice) -> QueueFamilyIndices {
        let props = unsafe { self.instance.get_physical_device_queue_family_properties(*physical_device) };

        let mut graphics_index = None;

        for (ix, prop) in props.iter().enumerate() {
            if prop.queue_flags.contains(QueueFlags::GRAPHICS) {
                graphics_index = Some(ix as u32);
                break;
            }
        }

        QueueFamilyIndices { graphics_family: graphics_index }
    }

    pub fn list_physical_devices(&self) -> Result<()> {
        let physical_devices = unsafe { self.instance.enumerate_physical_devices()? };
        for (i, p) in physical_devices.iter().enumerate() {
            let props  = unsafe {self.instance.get_physical_device_properties(*p) };
            let name = unsafe {std::ffi::CStr::from_ptr(props.device_name.as_ptr()) };

            println!("#{i}: {name:?}")
        }

        Ok(())
    }

    pub fn pick_suitable_device(&self) -> Result<PhysicalDevice> {
        let physical_devices = unsafe { self.instance.enumerate_physical_devices()? };

        let mut best_score = 0;
        let mut best_device: Option<PhysicalDevice> = None;

        for device in physical_devices.iter() {
            if !self.is_device_suitable(device) {
                continue;
            }

            let score = self.rate_device_suitability(device);

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

    fn is_device_suitable(&self, device: &PhysicalDevice) -> bool {
        let indices = self.find_queue_families(device);
        indices.is_complete()
    }

    fn rate_device_suitability(&self, device: &PhysicalDevice) -> u32 {
        let mut score = 0;

        let device_properties  = unsafe {self.instance.get_physical_device_properties(*device) };
        let device_features  = unsafe {self.instance.get_physical_device_features(*device) };

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

impl Drop for VulkanEngine {
    fn drop(&mut self) {
        unsafe { 
            if let Some(debug_state)= &mut self.debug {
                debug_state.destroy();
            }
            if let Some(device) = &mut self.device {
                device.destroy_device(None);
            }
            self.instance.destroy_instance(None) 
        }
    }
}
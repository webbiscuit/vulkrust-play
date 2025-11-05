use ash::{vk, Entry};
use crate::debug::{DebugState, debug_callback};
use anyhow::Result;

pub struct VulkanApp {
    _entry: ash::Entry,
    instance: ash::Instance,
    debug: Option<DebugState>
}

impl VulkanApp {
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

        Ok(VulkanApp { 
            _entry: entry, 
            instance, 
            debug: debug_state
        })
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
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe { 
            if let Some(debug_state)= &mut self.debug {
                debug_state.destroy();
            }
            self.instance.destroy_instance(None) 
        }
    }
}
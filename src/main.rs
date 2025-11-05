use ash::{vk, Entry};
use anyhow::Result;

extern "system" fn debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    types: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user: *mut std::ffi::c_void,
) -> vk::Bool32 {
    unsafe {
        let msg = std::ffi::CStr::from_ptr((*data).p_message).to_string_lossy();
        eprintln!("[{:?} {:?}] {}", severity, types, msg);
    }
    vk::FALSE
}

struct VulkanApp {
    _entry: ash::Entry,
    instance: ash::Instance,
    debug_utils: Option<ash::ext::debug_utils::Instance>,
    debug_messenger: Option<vk::DebugUtilsMessengerEXT>,
}

impl VulkanApp {
    pub fn new(app_name: &str) -> Result<Self> {
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
        let enabled_layers = [validation_layer.as_ptr()];
        let enabled_exts = [ash::ext::debug_utils::NAME.as_ptr()];

        let mut debug_messenger_create_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
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

        let mut create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            pp_enabled_layer_names: enabled_layers.as_ptr(),
            enabled_layer_count: enabled_layers.len() as u32,
            pp_enabled_extension_names: enabled_exts.as_ptr(),
            enabled_extension_count: enabled_exts.len() as u32,
            ..Default::default()
        };
        create_info = create_info.push_next(&mut debug_messenger_create_info);

        let instance = unsafe { entry.create_instance(&create_info, None)? };

        let debug_utils = ash::ext::debug_utils::Instance::new(&entry, &instance);
        let debug_messenger = unsafe { debug_utils.create_debug_utils_messenger(&debug_messenger_create_info, None)? };

        Ok(VulkanApp { 
            _entry: entry, 
            instance, 
            debug_utils: Some(debug_utils), 
            debug_messenger: Some(debug_messenger) 
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
            if let (Some(du), Some(m)) = (&self.debug_utils, self.debug_messenger) {
                du.destroy_debug_utils_messenger(m, None);
            }
            self.instance.destroy_instance(None) 
        }
    }
}

fn main() -> Result<()>{
    let app = VulkanApp::new("Vulkan Play")?;
    app.list_physical_devices()?;
    
    Ok(())
}

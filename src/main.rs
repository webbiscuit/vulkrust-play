use ash::{vk, Entry};
use anyhow::Result;

struct VulkanApp {
    _entry: ash::Entry,
    instance: ash::Instance
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
        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            ..Default::default()
        };
        let instance = unsafe { entry.create_instance(&create_info, None)? };

        Ok(VulkanApp { _entry: entry, instance })
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
        unsafe { self.instance.destroy_instance(None) };
    }
}

fn main() -> Result<()>{
    let app = VulkanApp::new("Vulkan Play")?;
    app.list_physical_devices()?;
    
    Ok(())
}

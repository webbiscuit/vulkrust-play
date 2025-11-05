use ash::{vk, Entry};
use anyhow::Result;

fn main() -> Result<()>{
    let entry = unsafe { Entry::load()? };

    let app_name = std::ffi::CString::new("Hello Vulkan")?;
    let app_info = vk::ApplicationInfo {
        api_version: vk::make_api_version(0, 1, 3, 0),
        p_application_name: app_name.as_ptr(),
        ..Default::default()
    };
    let create_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        ..Default::default()
    };
    let instance = unsafe { entry.create_instance(&create_info, None)? };

    let physical_devices = unsafe { instance.enumerate_physical_devices()? };
    for (i, p) in physical_devices.iter().enumerate() {
        let props  = unsafe {instance.get_physical_device_properties(*p) };
        let name = unsafe {std::ffi::CStr::from_ptr(props.device_name.as_ptr()) };

        println!("#{i}: {name:?}")
    }

    unsafe { instance.destroy_instance(None) };

    Ok(())
}

use ash::vk;

pub extern "system" fn debug_callback(
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

pub struct DebugState {
    utils: ash::ext::debug_utils::Instance,
    messenger: vk::DebugUtilsMessengerEXT,
}

impl DebugState {
    pub fn new(utils: ash::ext::debug_utils::Instance, messenger: vk::DebugUtilsMessengerEXT) -> Self {
        Self {
            utils,
            messenger
        }
    }

    pub unsafe fn destroy(&mut self) {
        unsafe {
            self.utils.destroy_debug_utils_messenger(self.messenger, None);
        }
    }
}

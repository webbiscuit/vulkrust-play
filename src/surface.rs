use anyhow::Result;
use ash::{vk};
use ash_window::create_surface;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::Instance;

pub struct Surface {
    handle: vk::SurfaceKHR,
    surface_instance: ash::khr::surface::Instance
}

impl Surface {
    pub fn new(instance: &Instance, window: &winit::window::Window) -> Result<Self> {
        let surface_instance: ash::khr::surface::Instance = ash::khr::surface::Instance::new(instance.entry(), instance.raw());
        let handle = unsafe { create_surface(instance.entry(), instance.raw(), window.display_handle()?.into(), window.window_handle()?.into(), None)? };

        Ok(Self {
            handle,
            surface_instance
        })
    }

    #[inline]
    pub fn raw(&self) -> &vk::SurfaceKHR {
        &self.handle
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.surface_instance.destroy_surface(self.handle, None);
        }
    }
}
use anyhow::Result;
use ash::vk::{self, ColorSpaceKHR, Extent2D, Format, PhysicalDevice, PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR};
use ash_window::create_surface;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use crate::Instance;

pub struct SurfaceCapabilities {
    capabilities: SurfaceCapabilitiesKHR,
    physical_device_surface_formats: Vec<SurfaceFormatKHR>,
    physical_device_surface_present_modes: Vec<PresentModeKHR>,
}

impl SurfaceCapabilities {
    pub fn is_adequate(&self) -> bool {
        !(self.physical_device_surface_formats.is_empty() && self.physical_device_surface_present_modes.is_empty())
    }

    pub fn find_best_format(&self) -> Option<&SurfaceFormatKHR> {
        if self.physical_device_surface_formats.is_empty() {
            return None;
        }

        for format in &self.physical_device_surface_formats {
            if format.format == Format::B8G8R8A8_SRGB && format.color_space == ColorSpaceKHR::SRGB_NONLINEAR {
                return Some(format);
            }
        }

        return Some(&self.physical_device_surface_formats[0])
    }

    pub fn find_best_present_mode(&self) -> Option<PresentModeKHR> {
        if self.physical_device_surface_present_modes.is_empty() {
            return None;
        }

        // println!("{:?}", self.physical_device_surface_present_modes);

        for mode in &self.physical_device_surface_present_modes {
            if mode == &PresentModeKHR::MAILBOX {
                return Some(PresentModeKHR::MAILBOX);
            }
        }

        return Some(PresentModeKHR::FIFO)
    }

    pub fn find_swap_extent(&self) -> Extent2D {
        self.capabilities.current_extent
    }
}

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

    pub fn query_surface_capabilities(&self, physical_device: PhysicalDevice) -> Result<SurfaceCapabilities> {
        let surface_capabilities = unsafe {
            self.surface_instance.get_physical_device_surface_capabilities(physical_device, self.handle)?
        };
        let physical_device_surface_formats = unsafe {
            self.surface_instance.get_physical_device_surface_formats(physical_device, self.handle)?
        };
        let physical_device_surface_present_modes = unsafe {
            self.surface_instance.get_physical_device_surface_present_modes(physical_device, self.handle)?
        };

        Ok(SurfaceCapabilities { 
            capabilities: surface_capabilities,
            physical_device_surface_formats,
            physical_device_surface_present_modes
         })
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.surface_instance.destroy_surface(self.handle, None);
        }
    }
}
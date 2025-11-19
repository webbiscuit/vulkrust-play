use crate::LogicalDevice;
use anyhow::Result;

pub struct ShaderModule {
    logical_device: ash::Device,
    raw: ash::vk::ShaderModule
}

impl ShaderModule {
    pub fn new(logical_device: &LogicalDevice, shader_data: &Vec<u8>) -> Result<Self>{
        let create_info= ash::vk::ShaderModuleCreateInfo {
            code_size: shader_data.len(),
            p_code: shader_data.as_ptr() as *const u32,
            ..Default::default()
        };

        let shader_module = unsafe { logical_device.raw().create_shader_module(&create_info, None)? };

        Ok(Self {
            logical_device: logical_device.raw().clone(),
            raw: shader_module
        })
    }

    pub fn raw(&self) -> &ash::vk::ShaderModule {
        &self.raw
    }
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        unsafe {
            println!("Dropping ShaderModule");

            self.logical_device.destroy_shader_module(self.raw, None);
        }
    }
}
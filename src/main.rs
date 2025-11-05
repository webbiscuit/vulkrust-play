use anyhow::Result;

use crate::app::VulkanApp;

mod app;
mod debug;

fn main() -> Result<()>{
    let app = VulkanApp::new("Vulkan Play", true)?;
    app.list_physical_devices()?;
    
    Ok(())
}

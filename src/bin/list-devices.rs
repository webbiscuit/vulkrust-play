use anyhow::Result;
use vulkrust_play::engine::VulkanEngine;

fn main() -> Result<()>{
    let app = VulkanEngine::new("Vulkan Play", true)?;
    app.list_physical_devices()?;
    
    Ok(())
}

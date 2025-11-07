use anyhow::Result;
use vulkrust_play::Instance;

fn main() -> Result<()>{
    let instance = Instance::new("List Devices", true, None)?;
    let physical_devices = instance.physical_devices()?;

    for (ix, device) in physical_devices.iter().enumerate() {
        println!("#{ix}: {device:?}")
    }
    
    Ok(())
}

use std::{fs::{self, File}, io::Read};

use ash::vk::{Bool32, CullModeFlags, FrontFace, Offset2D, PhysicalDevice, PhysicalDeviceType, PipelineInputAssemblyStateCreateInfo, PipelineRasterizationStateCreateInfo, PipelineViewportStateCreateInfo, PolygonMode, PrimitiveTopology, ShaderStageFlags, SurfaceKHR, VertexInputAttributeDescription, Viewport};
use ash_window::enumerate_required_extensions;
use raw_window_handle::{HasDisplayHandle};
use winit::window::Window;
use crate::{Instance, LogicalDevice, Surface, image_view::ImageView, logical_device::find_queue_families, shader_module::ShaderModule, surface, swap_chain::{self, SwapChain}, utils::vk_str_to_string};
use anyhow::{Error, Result};

pub struct VulkanEngine {
    logical_device: LogicalDevice,
    surface: Surface,
    instance: Instance, // Must be last
}

impl VulkanEngine {
    pub fn new(app_name: &str, enable_validation: bool, window: &Window) -> Result<Self> {
        let wsi_exts  = enumerate_required_extensions(window.display_handle()?.into())?;
        let window_dims = window.inner_size();

        let instance = Instance::new(app_name, enable_validation, Some(wsi_exts))?;
        let surface = Surface::new(&instance, window)?;
        assert_ne!(*surface.raw(), SurfaceKHR::null());
        let physical_device = Self::pick_suitable_device(&instance, &surface)?;
        let logical_device = LogicalDevice::new(&instance, &physical_device,  &surface, &Self::required_device_prop_names())?;
        let swap_chain = SwapChain::new(&instance, &physical_device, &logical_device, &surface, window_dims.width, window_dims.height)?;
        let image_views = swap_chain.images().iter().map(|i| ImageView::new(
            &logical_device, i, swap_chain.image_format()
        )).collect::<Result<Vec<_>, _>>()?;

        create_graphics_pipeline(&logical_device, &swap_chain)?;

        Ok(VulkanEngine { 
            surface,
            instance, 
            logical_device
        })
    }

    fn required_device_prop_names() -> Vec<String> {
        let swapchain = ash::khr::swapchain::NAME;
        let swapchain = swapchain.to_str().unwrap().to_string();

        vec![swapchain]
    }

    pub fn pick_suitable_device(instance: &Instance, surface: &Surface) -> Result<PhysicalDevice> {
        let physical_devices = unsafe { instance.raw().enumerate_physical_devices()? };

        let mut best_score = 0;
        let mut best_device: Option<PhysicalDevice> = None;

        for device in physical_devices.iter() {
            if !Self::is_physical_device_suitable(instance, device, surface)? {
                continue;
            }

            let score = Self::rate_physical_device_suitability(instance, device);

            if score > best_score {
                best_device = Some(*device);
                best_score = score;
            }
        }

        if let Some(best_device) = best_device {
            return Ok(best_device)
        }

        Err(Error::msg("Failed to find a suitable GPU with vulkan support"))
    }

    fn is_physical_device_suitable(instance: &Instance, device: &PhysicalDevice, surface: &Surface) -> Result<bool> {
        let indices = find_queue_families(instance, device, surface)?;

        let extensions_supported = check_physical_device_extension_support(instance, device, &Self::required_device_prop_names())?;

        Ok(indices.is_complete() && extensions_supported)
    }

    fn rate_physical_device_suitability(instance: &Instance, device: &PhysicalDevice) -> u32 {
        let mut score = 0;

        let device_properties  = unsafe {instance.raw().get_physical_device_properties(*device) };
        let device_features  = unsafe {instance.raw().get_physical_device_features(*device) };

        // Discrete GPUs have a significant performance advantage
        if device_properties.device_type == PhysicalDeviceType::DISCRETE_GPU {
            score += 1000;
        }

        // Maximum possible size of textures affects graphics quality
        score += device_properties.limits.max_image_dimension2_d;

        // Application can't function without geometry shaders
        if device_features.geometry_shader == 0 {
            return 0;
        }

        let name = unsafe {std::ffi::CStr::from_ptr(device_properties.device_name.as_ptr()) };
        println!("{name:?} - Scores {score}");

        score
    }
}

fn check_physical_device_extension_support(instance: &Instance, physical_device: &PhysicalDevice, required_props_names: &[String]) -> Result<bool> {
    let extension_props = unsafe { instance.raw().enumerate_device_extension_properties(*physical_device)? };
    let mut required_props_names = required_props_names.to_vec();

    for prop in extension_props {
        let ix = required_props_names.iter().position(|p| p == &vk_str_to_string(&prop.extension_name));

        if let Some(ix) = ix {
            required_props_names.remove(ix); 

            if required_props_names.is_empty() {
                return Ok(true);
            }
        }
    }

    Ok(required_props_names.is_empty())
}

fn create_graphics_pipeline(logical_device: &LogicalDevice, swap_chain: &SwapChain) -> Result<()> {
    let vertex_shader = read_file("shaders/out/vert.spv")?;
    let fragment_shader = read_file("shaders/out/frag.spv")?;

    let vertex_shader_module = ShaderModule::new(logical_device, &vertex_shader)?;
    let fragment_shader_module = ShaderModule::new(logical_device, &fragment_shader)?;

    let vertex_shader_stage_info = ash::vk::PipelineShaderStageCreateInfo {
        stage: ShaderStageFlags::VERTEX,
        module: *vertex_shader_module.raw(),
        p_name: "main".as_ptr() as *const i8,
        ..Default::default()
    };
    let fragment_shader_module_info = ash::vk::PipelineShaderStageCreateInfo {
        stage: ShaderStageFlags::VERTEX,
        module: *fragment_shader_module.raw(),
        p_name: "main".as_ptr() as *const i8,
        ..Default::default()
    };

    let shader_stages = [vertex_shader_stage_info, fragment_shader_module_info];

    let dynamic_states: Vec<ash::vk::DynamicState> = vec![
        ash::vk::DynamicState::VIEWPORT,
        ash::vk::DynamicState::SCISSOR
    ];

    let dynamic_state_create_info = ash::vk::PipelineDynamicStateCreateInfo {
        dynamic_state_count: dynamic_states.len() as u32,
        p_dynamic_states: dynamic_states.as_ptr(),
        ..Default::default()
    };

    // VkPipelineVertexInputStateCreateInfo 
    let pipeline_vertex_input_create_info = ash::vk::PipelineVertexInputStateCreateInfo {
        vertex_attribute_description_count: 0,
        p_vertex_attribute_descriptions: ::core::ptr::null(),
        vertex_binding_description_count: 0,
        p_vertex_binding_descriptions: ::core::ptr::null(),
        ..Default::default()
    };

    // VkPipelineInputAssemblyStateCreateInfo 
    let pipeline_input_assembly_state_create_info = PipelineInputAssemblyStateCreateInfo {
        topology: PrimitiveTopology::TRIANGLE_LIST,
        primitive_restart_enable: ash::vk::FALSE,
        ..Default::default()
    };

    let viewport = Viewport {
        x: 0.0f32,
        y: 0.0f32,
        width: swap_chain.extent().width as f32, 
        height: swap_chain.extent().height as f32,
        min_depth: 0.0f32,
        max_depth: 1.0f32
    };

    let scissor = ash::vk::Rect2D {
        offset: Offset2D { x: 0, y: 0},
        extent: *swap_chain.extent()
    };

    // VkPipelineViewportStateCreateInfo 
    let pipeline_viewport_state_create_info = PipelineViewportStateCreateInfo {
        viewport_count: 1,
        scissor_count: 1,
        // p_scissors: &scissor, // I think this makes the scissor immutable
        // p_viewports: &viewport, // I think this makes the viewport immutable
        ..Default::default()
    };

    // VkPipelineRasterizationStateCreateInfo 
    let pipeline_rasterization_state_create_info = PipelineRasterizationStateCreateInfo {
        depth_clamp_enable: ash::vk::FALSE, // Maybe useful in shadow maps
        rasterizer_discard_enable: ash::vk::FALSE,
        polygon_mode: PolygonMode::FILL,
        line_width: 1.0f32,
        cull_mode: CullModeFlags::BACK,
        front_face: FrontFace::CLOCKWISE,
        ..Default::default()
    };


    Ok(())
}

fn read_file(path: &str) -> Result<Vec<u8>> {
    Ok(fs::read(path)?)
}
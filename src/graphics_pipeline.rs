use crate::{LogicalDevice, shader_module::ShaderModule, swap_chain::SwapChain, utils::read_file};
use anyhow::Result;
use ash::vk::{CullModeFlags, FrontFace, Offset2D, PipelineColorBlendAttachmentState, PipelineColorBlendStateCreateInfo, PipelineInputAssemblyStateCreateInfo, PipelineLayout, PipelineLayoutCreateInfo, PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo, PipelineViewportStateCreateInfo, PolygonMode, PrimitiveTopology, ShaderStageFlags, Viewport};


pub struct GraphicsPipeline {
    pipeline_layout: PipelineLayout,
    device: ash::Device
}

impl GraphicsPipeline {
    pub fn new(logical_device: &LogicalDevice, swap_chain: &SwapChain) -> Result<Self> {
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

        // VkPipelineMultisampleStateCreateInfo 
        // Disabled for now
        let pipeline_multisample_state_create_info = PipelineMultisampleStateCreateInfo {
            sample_shading_enable: ash::vk::FALSE,
            ..Default::default()
        };

        // VkPipelineColorBlendAttachmentState 
        // Disabled for now
        let pipeline_colour_blend_attachment_state = PipelineColorBlendAttachmentState {
            blend_enable: ash::vk::FALSE,
            ..Default::default()
        };

        // VkPipelineColorBlendStateCreateInfo 
        let pipeline_colour_blend_state_create_info = PipelineColorBlendStateCreateInfo {
            logic_op_enable: ash::vk::FALSE,
            attachment_count: 1,
            p_attachments: &pipeline_colour_blend_attachment_state,
            ..Default::default()
        };

        // VkPipelineLayout 
        let pipeline_layout = PipelineLayout::null();

        // VkPipelineLayoutCreateInfo 
        let pipeline_layout_create_info = PipelineLayoutCreateInfo {
            ..Default::default()
        };

        let pipeline_layout = unsafe { logical_device.raw().create_pipeline_layout(&pipeline_layout_create_info, None)? };

        Ok(Self {
            pipeline_layout,
            device: logical_device.raw().clone()
        })
    }
}

impl Drop for GraphicsPipeline {
    fn drop(&mut self) {
        unsafe {
            println!("Dropping GraphicsPipeline");

            self.device.destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}
use crate::{LogicalDevice, shader_module::ShaderModule, swap_chain::SwapChain, utils::read_file};
use anyhow::Result;
use ash::vk::{AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp, CullModeFlags, FrontFace, ImageLayout, Offset2D, PipelineBindPoint, PipelineColorBlendAttachmentState, PipelineColorBlendStateCreateInfo, PipelineInputAssemblyStateCreateInfo, PipelineLayout, PipelineLayoutCreateInfo, PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo, PipelineViewportStateCreateInfo, PolygonMode, PrimitiveTopology, RenderPassCreateInfo, SampleCountFlags, ShaderStageFlags, SubpassDescription, Viewport};


pub struct RenderPass {
    raw: ash::vk::RenderPass,
    device: ash::Device
}

impl RenderPass {
    pub fn new(logical_device: &LogicalDevice, swap_chain: &SwapChain) -> Result<Self> {

        // VkAttachmentDescription 
        let colour_attachment_description = AttachmentDescription {
            format: *swap_chain.image_format(),
            samples: SampleCountFlags::TYPE_1,
            load_op: AttachmentLoadOp::CLEAR,
            store_op: AttachmentStoreOp::STORE,
            stencil_load_op: AttachmentLoadOp::DONT_CARE,
            stencil_store_op: AttachmentStoreOp::DONT_CARE,
            initial_layout: ImageLayout::UNDEFINED,
            final_layout: ImageLayout::PRESENT_SRC_KHR,
            ..Default::default()
        };

        // VkAttachmentReference 
        let colour_attachment_ref = AttachmentReference {
            attachment: 0,
            layout: ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            ..Default::default()
        };

        let subpass_description = SubpassDescription {
            pipeline_bind_point: PipelineBindPoint::GRAPHICS,
            color_attachment_count: 1,
            p_color_attachments: &colour_attachment_ref,
            ..Default::default()
        };

        // VkRenderPassCreateInfo 
        let render_pass_create_info = RenderPassCreateInfo {
            attachment_count: 1,
            p_attachments: &colour_attachment_description,
            subpass_count: 1,
            p_subpasses: &subpass_description,
            ..Default::default()
        };

        let render_pass = unsafe { logical_device.raw().create_render_pass(&render_pass_create_info, None)? };
        
        Ok(Self {
            raw: render_pass,
            device: logical_device.raw().clone()
        })
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe {
            println!("Dropping RenderPass");

            self.device.destroy_render_pass(self.raw, None);
        }
    }
}
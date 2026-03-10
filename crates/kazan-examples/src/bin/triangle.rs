#![warn(unused_qualifications)]

use kazan::util::Align;
use kazan::vk;
use kazan_examples::*;
use std::default::Default;
use std::error::Error;
use std::io::Cursor;
use std::mem;

#[derive(Clone, Debug, Copy)]
struct Vertex {
    pos: [f32; 4],
    color: [f32; 4],
}

fn main() -> Result<(), Box<dyn Error>> {
    unsafe {
        let base = ExampleBase::new(1920, 1080)?;
        let renderpass_attachments = [
            vk::AttachmentDescription {
                format: base.surface_format.format,
                samples: vk::SampleCountFlagBits::_1,
                load_op: vk::AttachmentLoadOp::CLEAR,
                store_op: vk::AttachmentStoreOp::STORE,
                final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                ..Default::default()
            },
            vk::AttachmentDescription {
                format: vk::Format::D16_UNORM,
                samples: vk::SampleCountFlagBits::_1,
                load_op: vk::AttachmentLoadOp::CLEAR,
                initial_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                ..Default::default()
            },
        ];
        let color_attachment_refs = [vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];
        let depth_attachment_ref = vk::AttachmentReference {
            attachment: 1,
            layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };
        let dependencies = [vk::SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            src_stage_mask: vk::PipelineStageFlagBits::COLOR_ATTACHMENT_OUTPUT.into(),
            dst_access_mask: vk::AccessFlagBits::COLOR_ATTACHMENT_READ
                | vk::AccessFlagBits::COLOR_ATTACHMENT_WRITE,
            dst_stage_mask: vk::PipelineStageFlagBits::COLOR_ATTACHMENT_OUTPUT.into(),
            ..Default::default()
        }];

        let subpass = vk::SubpassDescription::default()
            .color_attachments(&color_attachment_refs, None)
            .depth_stencil_attachment(&depth_attachment_ref)
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS);

        let renderpass_create_info = vk::RenderPassCreateInfo::default()
            .attachments(&renderpass_attachments)
            .subpasses(std::slice::from_ref(&subpass))
            .dependencies(&dependencies);

        let renderpass = base
            .device_fn
            .create_render_pass(base.device, &renderpass_create_info, None)
            .unwrap();

        let framebuffers: Vec<vk::Framebuffer> = base
            .present_image_views
            .iter()
            .map(|&present_image_view| {
                let framebuffer_attachments = [present_image_view, base.depth_image_view];
                let frame_buffer_create_info = vk::FramebufferCreateInfo::default()
                    .render_pass(renderpass)
                    .attachments(&framebuffer_attachments)
                    .width(base.surface_resolution.width)
                    .height(base.surface_resolution.height)
                    .layers(1);

                base.device_fn
                    .create_framebuffer(base.device, &frame_buffer_create_info, None)
                    .unwrap()
            })
            .collect();

        let index_buffer_data = [0u32, 1, 2];
        let index_buffer_info = vk::BufferCreateInfo::default()
            .size(size_of_val(&index_buffer_data) as u64)
            .usage(vk::BufferUsageFlagBits::INDEX_BUFFER.into())
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let index_buffer = base
            .device_fn
            .create_buffer(base.device, &index_buffer_info, None)
            .unwrap();
        let index_buffer_memory_req = base
            .device_fn
            .get_buffer_memory_requirements(base.device, index_buffer);
        let index_buffer_memory_index = find_memorytype_index(
            &index_buffer_memory_req,
            &base.device_memory_properties,
            vk::MemoryPropertyFlagBits::HOST_VISIBLE | vk::MemoryPropertyFlagBits::HOST_COHERENT,
        )
        .expect("Unable to find suitable memorytype for the index buffer.");

        let index_allocate_info = vk::MemoryAllocateInfo {
            allocation_size: index_buffer_memory_req.size,
            memory_type_index: index_buffer_memory_index,
            ..Default::default()
        };
        let index_buffer_memory = base
            .device_fn
            .allocate_memory(base.device, &index_allocate_info, None)
            .unwrap();
        let index_ptr = base
            .device_fn
            .map_memory(
                base.device,
                index_buffer_memory,
                0,
                index_buffer_memory_req.size,
                vk::MemoryMapFlags::empty(),
            )
            .unwrap();
        let mut index_slice = Align::new(
            index_ptr,
            align_of::<u32>() as u64,
            index_buffer_memory_req.size,
        );
        index_slice.copy_from_slice(&index_buffer_data);
        base.device_fn
            .unmap_memory(base.device, index_buffer_memory);
        base.device_fn
            .bind_buffer_memory(base.device, index_buffer, index_buffer_memory, 0)
            .unwrap();

        let vertex_input_buffer_info = vk::BufferCreateInfo {
            size: 3 * size_of::<Vertex>() as u64,
            usage: vk::BufferUsageFlagBits::VERTEX_BUFFER.into(),
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            ..Default::default()
        };

        let vertex_input_buffer = base
            .device_fn
            .create_buffer(base.device, &vertex_input_buffer_info, None)
            .unwrap();

        let vertex_input_buffer_memory_req = base
            .device_fn
            .get_buffer_memory_requirements(base.device, vertex_input_buffer);

        let vertex_input_buffer_memory_index = find_memorytype_index(
            &vertex_input_buffer_memory_req,
            &base.device_memory_properties,
            vk::MemoryPropertyFlagBits::HOST_VISIBLE | vk::MemoryPropertyFlagBits::HOST_COHERENT,
        )
        .expect("Unable to find suitable memorytype for the vertex buffer.");

        let vertex_buffer_allocate_info = vk::MemoryAllocateInfo {
            allocation_size: vertex_input_buffer_memory_req.size,
            memory_type_index: vertex_input_buffer_memory_index,
            ..Default::default()
        };

        let vertex_input_buffer_memory = base
            .device_fn
            .allocate_memory(base.device, &vertex_buffer_allocate_info, None)
            .unwrap();

        let vertices = [
            Vertex {
                pos: [-1.0, 1.0, 0.0, 1.0],
                color: [0.0, 1.0, 0.0, 1.0],
            },
            Vertex {
                pos: [1.0, 1.0, 0.0, 1.0],
                color: [0.0, 0.0, 1.0, 1.0],
            },
            Vertex {
                pos: [0.0, -1.0, 0.0, 1.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
        ];

        let vert_ptr = base
            .device_fn
            .map_memory(
                base.device,
                vertex_input_buffer_memory,
                0,
                vertex_input_buffer_memory_req.size,
                vk::MemoryMapFlags::empty(),
            )
            .unwrap();

        let mut vert_align = Align::new(
            vert_ptr,
            align_of::<Vertex>() as u64,
            vertex_input_buffer_memory_req.size,
        );
        vert_align.copy_from_slice(&vertices);
        base.device_fn
            .unmap_memory(base.device, vertex_input_buffer_memory);
        base.device_fn
            .bind_buffer_memory(
                base.device,
                vertex_input_buffer,
                vertex_input_buffer_memory,
                0,
            )
            .unwrap();

        let mut vertex_spv_file =
            Cursor::new(&include_bytes!("../../shader/triangle/vert.spv")[..]);
        let mut frag_spv_file = Cursor::new(&include_bytes!("../../shader/triangle/frag.spv")[..]);

        let vertex_code =
            read_spv(&mut vertex_spv_file).expect("Failed to read vertex shader spv file");
        let vertex_shader_info = vk::ShaderModuleCreateInfo::default().code(&vertex_code);

        let frag_code =
            read_spv(&mut frag_spv_file).expect("Failed to read fragment shader spv file");
        let frag_shader_info = vk::ShaderModuleCreateInfo::default().code(&frag_code);

        let vertex_shader_module = base
            .device_fn
            .create_shader_module(base.device, &vertex_shader_info, None)
            .expect("Vertex shader module error");

        let fragment_shader_module = base
            .device_fn
            .create_shader_module(base.device, &frag_shader_info, None)
            .expect("Fragment shader module error");

        let layout_create_info = vk::PipelineLayoutCreateInfo::default();

        let pipeline_layout = base
            .device_fn
            .create_pipeline_layout(base.device, &layout_create_info, None)
            .unwrap();

        let shader_entry_name = c"main";
        let shader_stage_create_infos = [
            vk::PipelineShaderStageCreateInfo {
                module: vertex_shader_module,
                p_name: shader_entry_name.as_ptr(),
                stage: vk::ShaderStageFlagBits::VERTEX,
                ..Default::default()
            },
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                module: fragment_shader_module,
                p_name: shader_entry_name.as_ptr(),
                stage: vk::ShaderStageFlagBits::FRAGMENT,
                ..Default::default()
            },
        ];
        let vertex_input_binding_descriptions = [vk::VertexInputBindingDescription {
            binding: 0,
            stride: size_of::<Vertex>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }];
        let vertex_input_attribute_descriptions = [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32B32A32_SFLOAT,
                offset: offset_of!(Vertex, pos) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::Format::R32G32B32A32_SFLOAT,
                offset: offset_of!(Vertex, color) as u32,
            },
        ];

        let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_attribute_descriptions(&vertex_input_attribute_descriptions)
            .vertex_binding_descriptions(&vertex_input_binding_descriptions);
        let vertex_input_assembly_state_info = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            ..Default::default()
        };
        let viewports = [vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: base.surface_resolution.width as f32,
            height: base.surface_resolution.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }];
        let scissors = [vk::Rect2D::from(base.surface_resolution)];
        let viewport_state_info = vk::PipelineViewportStateCreateInfo::default()
            .scissors(&scissors)
            .viewports(&viewports);

        let rasterization_info = vk::PipelineRasterizationStateCreateInfo {
            front_face: vk::FrontFace::COUNTER_CLOCKWISE,
            line_width: 1.0,
            polygon_mode: vk::PolygonMode::FILL,
            ..Default::default()
        };
        let multisample_state_info = vk::PipelineMultisampleStateCreateInfo {
            rasterization_samples: vk::SampleCountFlagBits::_1,
            ..Default::default()
        };
        let noop_stencil_state = vk::StencilOpState {
            fail_op: vk::StencilOp::KEEP,
            pass_op: vk::StencilOp::KEEP,
            depth_fail_op: vk::StencilOp::KEEP,
            compare_op: vk::CompareOp::ALWAYS,
            ..Default::default()
        };
        let depth_state_info = vk::PipelineDepthStencilStateCreateInfo {
            depth_test_enable: 1,
            depth_write_enable: 1,
            depth_compare_op: vk::CompareOp::LESS_OR_EQUAL,
            front: noop_stencil_state,
            back: noop_stencil_state,
            max_depth_bounds: 1.0,
            ..Default::default()
        };
        let color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState {
            blend_enable: 0,
            src_color_blend_factor: vk::BlendFactor::SRC_COLOR,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_DST_COLOR,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ZERO,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlagBits::R
                | vk::ColorComponentFlagBits::G
                | vk::ColorComponentFlagBits::B
                | vk::ColorComponentFlagBits::A,
        }];
        let color_blend_state = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op(vk::LogicOp::CLEAR)
            .attachments(&color_blend_attachment_states);

        let dynamic_state = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state_info =
            vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_state);

        let graphic_pipeline_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stage_create_infos)
            .vertex_input_state(&vertex_input_state_info)
            .input_assembly_state(&vertex_input_assembly_state_info)
            .viewport_state(&viewport_state_info)
            .rasterization_state(&rasterization_info)
            .multisample_state(&multisample_state_info)
            .depth_stencil_state(&depth_state_info)
            .color_blend_state(&color_blend_state)
            .dynamic_state(&dynamic_state_info)
            .layout(pipeline_layout)
            .render_pass(renderpass);

        let mut graphics_pipelines = [vk::Pipeline::null()];
        base.device_fn
            .create_graphics_pipelines(
                base.device,
                vk::PipelineCache::null(),
                &[graphic_pipeline_info],
                None,
                &mut graphics_pipelines,
            )
            .expect("Unable to create graphics pipeline");

        let graphic_pipeline = graphics_pipelines[0];

        let _ = base.render_loop(|frame_index| {
            let present_complete_semaphore =
                base.present_complete_semaphores[frame_index % MAX_FRAME_LATENCY];
            let draw_commands_reuse_fence =
                base.draw_commands_reuse_fences[frame_index % MAX_FRAME_LATENCY];
            let draw_command_buffer = base.draw_command_buffers[frame_index % MAX_FRAME_LATENCY];

            let (present_index, _) = base
                .swapchain_fn
                .acquire_next_image_khr(
                    base.device,
                    base.swapchain,
                    u64::MAX,
                    present_complete_semaphore,
                    vk::Fence::null(),
                )
                .unwrap();
            let clear_values = [
                vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 0.0],
                    },
                },
                vk::ClearValue {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth: 1.0,
                        stencil: 0,
                    },
                },
            ];

            let rendering_complete_semaphore =
                base.rendering_complete_semaphores[present_index as usize];

            let render_pass_begin_info = vk::RenderPassBeginInfo::default()
                .render_pass(renderpass)
                .framebuffer(framebuffers[present_index as usize])
                .render_area(base.surface_resolution.into())
                .clear_values(&clear_values);

            record_submit_commandbuffer(
                &base.device_fn,
                base.device,
                draw_command_buffer,
                draw_commands_reuse_fence,
                base.present_queue,
                &[vk::PipelineStageFlagBits::COLOR_ATTACHMENT_OUTPUT.into()],
                &[present_complete_semaphore],
                &[rendering_complete_semaphore],
                |device_fn, draw_command_buffer| {
                    device_fn.cmd_begin_render_pass(
                        draw_command_buffer,
                        &render_pass_begin_info,
                        vk::SubpassContents::INLINE,
                    );
                    device_fn.cmd_bind_pipeline(
                        draw_command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        graphic_pipeline,
                    );
                    device_fn.cmd_set_viewport(draw_command_buffer, 0, &viewports);
                    device_fn.cmd_set_scissor(draw_command_buffer, 0, &scissors);
                    device_fn.cmd_bind_vertex_buffers(
                        draw_command_buffer,
                        0,
                        &[vertex_input_buffer],
                        &[0],
                    );
                    device_fn.cmd_bind_index_buffer(
                        draw_command_buffer,
                        index_buffer,
                        0,
                        vk::IndexType::UINT32,
                    );
                    device_fn.cmd_draw_indexed(
                        draw_command_buffer,
                        index_buffer_data.len() as u32,
                        1,
                        0,
                        0,
                        1,
                    );
                    device_fn.cmd_end_render_pass(draw_command_buffer);
                },
            );
            let wait_semaphores = [rendering_complete_semaphore];
            let swapchains = [base.swapchain];
            let image_indices = [present_index];
            let present_info = vk::PresentInfoKHR::default()
                .wait_semaphores(&wait_semaphores)
                .swapchains(&swapchains, &image_indices, None);

            base.swapchain_fn
                .queue_present_khr(base.present_queue, &present_info)
                .unwrap();
        });

        base.device_fn.device_wait_idle(base.device).unwrap();
        for pipeline in graphics_pipelines {
            base.device_fn.destroy_pipeline(base.device, pipeline, None);
        }
        base.device_fn
            .destroy_pipeline_layout(base.device, pipeline_layout, None);
        base.device_fn
            .destroy_shader_module(base.device, vertex_shader_module, None);
        base.device_fn
            .destroy_shader_module(base.device, fragment_shader_module, None);
        base.device_fn
            .free_memory(base.device, index_buffer_memory, None);
        base.device_fn
            .destroy_buffer(base.device, index_buffer, None);
        base.device_fn
            .free_memory(base.device, vertex_input_buffer_memory, None);
        base.device_fn
            .destroy_buffer(base.device, vertex_input_buffer, None);
        for framebuffer in framebuffers {
            base.device_fn
                .destroy_framebuffer(base.device, framebuffer, None);
        }
        base.device_fn
            .destroy_render_pass(base.device, renderpass, None);
    }

    Ok(())
}

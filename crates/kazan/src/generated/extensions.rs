use crate::{define_extension_set, vk::*};

define_extension_set!(
    InstanceExtensionSet,
    [
        (khr_surface, khr::surface::EXTENSION_NAME),
        (khr_display, khr::display::EXTENSION_NAME),
        (khr_xlib_surface, khr::xlib_surface::EXTENSION_NAME),
        (khr_xcb_surface, khr::xcb_surface::EXTENSION_NAME),
        (khr_wayland_surface, khr::wayland_surface::EXTENSION_NAME),
        (khr_android_surface, khr::android_surface::EXTENSION_NAME),
        (khr_win32_surface, khr::win32_surface::EXTENSION_NAME),
        (ext_debug_report, ext::debug_report::EXTENSION_NAME),
        (
            ggp_stream_descriptor_surface,
            ggp::stream_descriptor_surface::EXTENSION_NAME
        ),
        (
            nv_external_memory_capabilities,
            nv::external_memory_capabilities::EXTENSION_NAME
        ),
        (
            khr_get_physical_device_properties2,
            khr::get_physical_device_properties2::EXTENSION_NAME
        ),
        (ext_validation_flags, ext::validation_flags::EXTENSION_NAME),
        (nn_vi_surface, nn::vi_surface::EXTENSION_NAME),
        (
            khr_device_group_creation,
            khr::device_group_creation::EXTENSION_NAME
        ),
        (
            khr_external_memory_capabilities,
            khr::external_memory_capabilities::EXTENSION_NAME
        ),
        (
            khr_external_semaphore_capabilities,
            khr::external_semaphore_capabilities::EXTENSION_NAME
        ),
        (
            ext_direct_mode_display,
            ext::direct_mode_display::EXTENSION_NAME
        ),
        (
            ext_acquire_xlib_display,
            ext::acquire_xlib_display::EXTENSION_NAME
        ),
        (
            ext_display_surface_counter,
            ext::display_surface_counter::EXTENSION_NAME
        ),
        (
            ext_swapchain_colorspace,
            ext::swapchain_colorspace::EXTENSION_NAME
        ),
        (
            khr_external_fence_capabilities,
            khr::external_fence_capabilities::EXTENSION_NAME
        ),
        (
            khr_get_surface_capabilities2,
            khr::get_surface_capabilities2::EXTENSION_NAME
        ),
        (
            khr_get_display_properties2,
            khr::get_display_properties2::EXTENSION_NAME
        ),
        (mvk_ios_surface, mvk::ios_surface::EXTENSION_NAME),
        (mvk_macos_surface, mvk::macos_surface::EXTENSION_NAME),
        (ext_debug_utils, ext::debug_utils::EXTENSION_NAME),
        (
            fuchsia_imagepipe_surface,
            fuchsia::imagepipe_surface::EXTENSION_NAME
        ),
        (ext_metal_surface, ext::metal_surface::EXTENSION_NAME),
        (
            khr_surface_protected_capabilities,
            khr::surface_protected_capabilities::EXTENSION_NAME
        ),
        (
            ext_validation_features,
            ext::validation_features::EXTENSION_NAME
        ),
        (ext_headless_surface, ext::headless_surface::EXTENSION_NAME),
        (
            ext_surface_maintenance1,
            ext::surface_maintenance1::EXTENSION_NAME
        ),
        (
            ext_acquire_drm_display,
            ext::acquire_drm_display::EXTENSION_NAME
        ),
        (ext_directfb_surface, ext::directfb_surface::EXTENSION_NAME),
        (qnx_screen_surface, qnx::screen_surface::EXTENSION_NAME),
        (
            khr_portability_enumeration,
            khr::portability_enumeration::EXTENSION_NAME
        ),
        (
            google_surfaceless_query,
            google::surfaceless_query::EXTENSION_NAME
        ),
        (
            lunarg_direct_driver_loading,
            lunarg::direct_driver_loading::EXTENSION_NAME
        ),
        (
            khr_surface_maintenance1,
            khr::surface_maintenance1::EXTENSION_NAME
        ),
        (ext_layer_settings, ext::layer_settings::EXTENSION_NAME),
        (nv_display_stereo, nv::display_stereo::EXTENSION_NAME),
        (ohos_surface, ohos::surface::EXTENSION_NAME),
        (sec_ubm_surface, sec::ubm_surface::EXTENSION_NAME),
    ]
);

define_extension_set!(
    DeviceExtensionSet,
    [
        (khr_swapchain, khr::swapchain::EXTENSION_NAME),
        (
            khr_display_swapchain,
            khr::display_swapchain::EXTENSION_NAME
        ),
        (nv_glsl_shader, nv::glsl_shader::EXTENSION_NAME),
        (
            ext_depth_range_unrestricted,
            ext::depth_range_unrestricted::EXTENSION_NAME
        ),
        (
            khr_sampler_mirror_clamp_to_edge,
            khr::sampler_mirror_clamp_to_edge::EXTENSION_NAME
        ),
        (img_filter_cubic, img::filter_cubic::EXTENSION_NAME),
        (
            amd_rasterization_order,
            amd::rasterization_order::EXTENSION_NAME
        ),
        (
            amd_shader_trinary_minmax,
            amd::shader_trinary_minmax::EXTENSION_NAME
        ),
        (
            amd_shader_explicit_vertex_parameter,
            amd::shader_explicit_vertex_parameter::EXTENSION_NAME
        ),
        (ext_debug_marker, ext::debug_marker::EXTENSION_NAME),
        (khr_video_queue, khr::video_queue::EXTENSION_NAME),
        (
            khr_video_decode_queue,
            khr::video_decode_queue::EXTENSION_NAME
        ),
        (amd_gcn_shader, amd::gcn_shader::EXTENSION_NAME),
        (
            nv_dedicated_allocation,
            nv::dedicated_allocation::EXTENSION_NAME
        ),
        (
            ext_transform_feedback,
            ext::transform_feedback::EXTENSION_NAME
        ),
        (nvx_binary_import, nvx::binary_import::EXTENSION_NAME),
        (
            nvx_image_view_handle,
            nvx::image_view_handle::EXTENSION_NAME
        ),
        (
            amd_draw_indirect_count,
            amd::draw_indirect_count::EXTENSION_NAME
        ),
        (
            amd_negative_viewport_height,
            amd::negative_viewport_height::EXTENSION_NAME
        ),
        (
            amd_gpu_shader_half_float,
            amd::gpu_shader_half_float::EXTENSION_NAME
        ),
        (amd_shader_ballot, amd::shader_ballot::EXTENSION_NAME),
        (
            khr_video_encode_h264,
            khr::video_encode_h264::EXTENSION_NAME
        ),
        (
            khr_video_encode_h265,
            khr::video_encode_h265::EXTENSION_NAME
        ),
        (
            khr_video_decode_h264,
            khr::video_decode_h264::EXTENSION_NAME
        ),
        (
            amd_texture_gather_bias_lod,
            amd::texture_gather_bias_lod::EXTENSION_NAME
        ),
        (amd_shader_info, amd::shader_info::EXTENSION_NAME),
        (
            khr_dynamic_rendering,
            khr::dynamic_rendering::EXTENSION_NAME
        ),
        (
            amd_shader_image_load_store_lod,
            amd::shader_image_load_store_lod::EXTENSION_NAME
        ),
        (
            nv_corner_sampled_image,
            nv::corner_sampled_image::EXTENSION_NAME
        ),
        (khr_multiview, khr::multiview::EXTENSION_NAME),
        (img_format_pvrtc, img::format_pvrtc::EXTENSION_NAME),
        (nv_external_memory, nv::external_memory::EXTENSION_NAME),
        (
            nv_external_memory_win32,
            nv::external_memory_win32::EXTENSION_NAME
        ),
        (nv_win32_keyed_mutex, nv::win32_keyed_mutex::EXTENSION_NAME),
        (khr_device_group, khr::device_group::EXTENSION_NAME),
        (
            khr_shader_draw_parameters,
            khr::shader_draw_parameters::EXTENSION_NAME
        ),
        (
            ext_shader_subgroup_ballot,
            ext::shader_subgroup_ballot::EXTENSION_NAME
        ),
        (
            ext_shader_subgroup_vote,
            ext::shader_subgroup_vote::EXTENSION_NAME
        ),
        (
            ext_texture_compression_astc_hdr,
            ext::texture_compression_astc_hdr::EXTENSION_NAME
        ),
        (ext_astc_decode_mode, ext::astc_decode_mode::EXTENSION_NAME),
        (
            ext_pipeline_robustness,
            ext::pipeline_robustness::EXTENSION_NAME
        ),
        (khr_maintenance1, khr::maintenance1::EXTENSION_NAME),
        (khr_external_memory, khr::external_memory::EXTENSION_NAME),
        (
            khr_external_memory_win32,
            khr::external_memory_win32::EXTENSION_NAME
        ),
        (
            khr_external_memory_fd,
            khr::external_memory_fd::EXTENSION_NAME
        ),
        (
            khr_win32_keyed_mutex,
            khr::win32_keyed_mutex::EXTENSION_NAME
        ),
        (
            khr_external_semaphore,
            khr::external_semaphore::EXTENSION_NAME
        ),
        (
            khr_external_semaphore_win32,
            khr::external_semaphore_win32::EXTENSION_NAME
        ),
        (
            khr_external_semaphore_fd,
            khr::external_semaphore_fd::EXTENSION_NAME
        ),
        (khr_push_descriptor, khr::push_descriptor::EXTENSION_NAME),
        (
            ext_conditional_rendering,
            ext::conditional_rendering::EXTENSION_NAME
        ),
        (
            khr_shader_float16_int8,
            khr::shader_float16_int8::EXTENSION_NAME
        ),
        (khr__16bit_storage, khr::_16bit_storage::EXTENSION_NAME),
        (
            khr_incremental_present,
            khr::incremental_present::EXTENSION_NAME
        ),
        (
            khr_descriptor_update_template,
            khr::descriptor_update_template::EXTENSION_NAME
        ),
        (
            nv_clip_space_w_scaling,
            nv::clip_space_w_scaling::EXTENSION_NAME
        ),
        (ext_display_control, ext::display_control::EXTENSION_NAME),
        (
            google_display_timing,
            google::display_timing::EXTENSION_NAME
        ),
        (
            nv_sample_mask_override_coverage,
            nv::sample_mask_override_coverage::EXTENSION_NAME
        ),
        (
            nv_geometry_shader_passthrough,
            nv::geometry_shader_passthrough::EXTENSION_NAME
        ),
        (nv_viewport_array2, nv::viewport_array2::EXTENSION_NAME),
        (
            nvx_multiview_per_view_attributes,
            nvx::multiview_per_view_attributes::EXTENSION_NAME
        ),
        (nv_viewport_swizzle, nv::viewport_swizzle::EXTENSION_NAME),
        (
            ext_discard_rectangles,
            ext::discard_rectangles::EXTENSION_NAME
        ),
        (
            ext_conservative_rasterization,
            ext::conservative_rasterization::EXTENSION_NAME
        ),
        (
            ext_depth_clip_enable,
            ext::depth_clip_enable::EXTENSION_NAME
        ),
        (ext_hdr_metadata, ext::hdr_metadata::EXTENSION_NAME),
        (
            khr_imageless_framebuffer,
            khr::imageless_framebuffer::EXTENSION_NAME
        ),
        (
            khr_create_renderpass2,
            khr::create_renderpass2::EXTENSION_NAME
        ),
        (
            img_relaxed_line_rasterization,
            img::relaxed_line_rasterization::EXTENSION_NAME
        ),
        (
            khr_shared_presentable_image,
            khr::shared_presentable_image::EXTENSION_NAME
        ),
        (khr_external_fence, khr::external_fence::EXTENSION_NAME),
        (
            khr_external_fence_win32,
            khr::external_fence_win32::EXTENSION_NAME
        ),
        (
            khr_external_fence_fd,
            khr::external_fence_fd::EXTENSION_NAME
        ),
        (
            khr_performance_query,
            khr::performance_query::EXTENSION_NAME
        ),
        (khr_maintenance2, khr::maintenance2::EXTENSION_NAME),
        (
            khr_variable_pointers,
            khr::variable_pointers::EXTENSION_NAME
        ),
        (
            ext_external_memory_dma_buf,
            ext::external_memory_dma_buf::EXTENSION_NAME
        ),
        (
            ext_queue_family_foreign,
            ext::queue_family_foreign::EXTENSION_NAME
        ),
        (
            khr_dedicated_allocation,
            khr::dedicated_allocation::EXTENSION_NAME
        ),
        (
            android_external_memory_android_hardware_buffer,
            android::external_memory_android_hardware_buffer::EXTENSION_NAME
        ),
        (
            ext_sampler_filter_minmax,
            ext::sampler_filter_minmax::EXTENSION_NAME
        ),
        (
            khr_storage_buffer_storage_class,
            khr::storage_buffer_storage_class::EXTENSION_NAME
        ),
        (amd_gpu_shader_int16, amd::gpu_shader_int16::EXTENSION_NAME),
        #[cfg(feature = "provisional")]
        (amdx_shader_enqueue, amdx::shader_enqueue::EXTENSION_NAME),
        (ext_descriptor_heap, ext::descriptor_heap::EXTENSION_NAME),
        (
            amd_mixed_attachment_samples,
            amd::mixed_attachment_samples::EXTENSION_NAME
        ),
        (
            amd_shader_fragment_mask,
            amd::shader_fragment_mask::EXTENSION_NAME
        ),
        (
            ext_inline_uniform_block,
            ext::inline_uniform_block::EXTENSION_NAME
        ),
        (
            ext_shader_stencil_export,
            ext::shader_stencil_export::EXTENSION_NAME
        ),
        (khr_shader_bfloat16, khr::shader_bfloat16::EXTENSION_NAME),
        (ext_sample_locations, ext::sample_locations::EXTENSION_NAME),
        (
            khr_relaxed_block_layout,
            khr::relaxed_block_layout::EXTENSION_NAME
        ),
        (
            khr_get_memory_requirements2,
            khr::get_memory_requirements2::EXTENSION_NAME
        ),
        (
            khr_image_format_list,
            khr::image_format_list::EXTENSION_NAME
        ),
        (
            ext_blend_operation_advanced,
            ext::blend_operation_advanced::EXTENSION_NAME
        ),
        (
            nv_fragment_coverage_to_color,
            nv::fragment_coverage_to_color::EXTENSION_NAME
        ),
        (
            khr_acceleration_structure,
            khr::acceleration_structure::EXTENSION_NAME
        ),
        (
            khr_ray_tracing_pipeline,
            khr::ray_tracing_pipeline::EXTENSION_NAME
        ),
        (khr_ray_query, khr::ray_query::EXTENSION_NAME),
        (
            nv_framebuffer_mixed_samples,
            nv::framebuffer_mixed_samples::EXTENSION_NAME
        ),
        (nv_fill_rectangle, nv::fill_rectangle::EXTENSION_NAME),
        (
            nv_shader_sm_builtins,
            nv::shader_sm_builtins::EXTENSION_NAME
        ),
        (
            ext_post_depth_coverage,
            ext::post_depth_coverage::EXTENSION_NAME
        ),
        (
            khr_sampler_ycbcr_conversion,
            khr::sampler_ycbcr_conversion::EXTENSION_NAME
        ),
        (khr_bind_memory2, khr::bind_memory2::EXTENSION_NAME),
        (
            ext_image_drm_format_modifier,
            ext::image_drm_format_modifier::EXTENSION_NAME
        ),
        (ext_validation_cache, ext::validation_cache::EXTENSION_NAME),
        (
            ext_descriptor_indexing,
            ext::descriptor_indexing::EXTENSION_NAME
        ),
        (
            ext_shader_viewport_index_layer,
            ext::shader_viewport_index_layer::EXTENSION_NAME
        ),
        #[cfg(feature = "provisional")]
        (
            khr_portability_subset,
            khr::portability_subset::EXTENSION_NAME
        ),
        (
            nv_shading_rate_image,
            nv::shading_rate_image::EXTENSION_NAME
        ),
        (nv_ray_tracing, nv::ray_tracing::EXTENSION_NAME),
        (
            nv_representative_fragment_test,
            nv::representative_fragment_test::EXTENSION_NAME
        ),
        (khr_maintenance3, khr::maintenance3::EXTENSION_NAME),
        (
            khr_draw_indirect_count,
            khr::draw_indirect_count::EXTENSION_NAME
        ),
        (ext_filter_cubic, ext::filter_cubic::EXTENSION_NAME),
        (
            qcom_render_pass_shader_resolve,
            qcom::render_pass_shader_resolve::EXTENSION_NAME
        ),
        (
            qcom_cooperative_matrix_conversion,
            qcom::cooperative_matrix_conversion::EXTENSION_NAME
        ),
        (ext_global_priority, ext::global_priority::EXTENSION_NAME),
        (
            khr_shader_subgroup_extended_types,
            khr::shader_subgroup_extended_types::EXTENSION_NAME
        ),
        (khr__8bit_storage, khr::_8bit_storage::EXTENSION_NAME),
        (
            ext_external_memory_host,
            ext::external_memory_host::EXTENSION_NAME
        ),
        (amd_buffer_marker, amd::buffer_marker::EXTENSION_NAME),
        (
            khr_shader_atomic_int64,
            khr::shader_atomic_int64::EXTENSION_NAME
        ),
        (khr_shader_clock, khr::shader_clock::EXTENSION_NAME),
        (
            amd_pipeline_compiler_control,
            amd::pipeline_compiler_control::EXTENSION_NAME
        ),
        (
            ext_calibrated_timestamps,
            ext::calibrated_timestamps::EXTENSION_NAME
        ),
        (
            amd_shader_core_properties,
            amd::shader_core_properties::EXTENSION_NAME
        ),
        (
            khr_video_decode_h265,
            khr::video_decode_h265::EXTENSION_NAME
        ),
        (khr_global_priority, khr::global_priority::EXTENSION_NAME),
        (
            amd_memory_overallocation_behavior,
            amd::memory_overallocation_behavior::EXTENSION_NAME
        ),
        (
            ext_vertex_attribute_divisor,
            ext::vertex_attribute_divisor::EXTENSION_NAME
        ),
        (ggp_frame_token, ggp::frame_token::EXTENSION_NAME),
        (
            ext_pipeline_creation_feedback,
            ext::pipeline_creation_feedback::EXTENSION_NAME
        ),
        (
            khr_driver_properties,
            khr::driver_properties::EXTENSION_NAME
        ),
        (
            khr_shader_float_controls,
            khr::shader_float_controls::EXTENSION_NAME
        ),
        (
            nv_shader_subgroup_partitioned,
            nv::shader_subgroup_partitioned::EXTENSION_NAME
        ),
        (
            khr_depth_stencil_resolve,
            khr::depth_stencil_resolve::EXTENSION_NAME
        ),
        (
            khr_swapchain_mutable_format,
            khr::swapchain_mutable_format::EXTENSION_NAME
        ),
        (
            nv_compute_shader_derivatives,
            nv::compute_shader_derivatives::EXTENSION_NAME
        ),
        (nv_mesh_shader, nv::mesh_shader::EXTENSION_NAME),
        (
            nv_fragment_shader_barycentric,
            nv::fragment_shader_barycentric::EXTENSION_NAME
        ),
        (
            nv_shader_image_footprint,
            nv::shader_image_footprint::EXTENSION_NAME
        ),
        (nv_scissor_exclusive, nv::scissor_exclusive::EXTENSION_NAME),
        (
            nv_device_diagnostic_checkpoints,
            nv::device_diagnostic_checkpoints::EXTENSION_NAME
        ),
        (
            khr_timeline_semaphore,
            khr::timeline_semaphore::EXTENSION_NAME
        ),
        (ext_present_timing, ext::present_timing::EXTENSION_NAME),
        (
            intel_shader_integer_functions2,
            intel::shader_integer_functions2::EXTENSION_NAME
        ),
        (
            intel_performance_query,
            intel::performance_query::EXTENSION_NAME
        ),
        (
            khr_vulkan_memory_model,
            khr::vulkan_memory_model::EXTENSION_NAME
        ),
        (ext_pci_bus_info, ext::pci_bus_info::EXTENSION_NAME),
        (
            amd_display_native_hdr,
            amd::display_native_hdr::EXTENSION_NAME
        ),
        (
            khr_shader_terminate_invocation,
            khr::shader_terminate_invocation::EXTENSION_NAME
        ),
        (
            ext_fragment_density_map,
            ext::fragment_density_map::EXTENSION_NAME
        ),
        (
            ext_scalar_block_layout,
            ext::scalar_block_layout::EXTENSION_NAME
        ),
        (
            google_hlsl_functionality1,
            google::hlsl_functionality1::EXTENSION_NAME
        ),
        (
            google_decorate_string,
            google::decorate_string::EXTENSION_NAME
        ),
        (
            ext_subgroup_size_control,
            ext::subgroup_size_control::EXTENSION_NAME
        ),
        (
            khr_fragment_shading_rate,
            khr::fragment_shading_rate::EXTENSION_NAME
        ),
        (
            amd_shader_core_properties2,
            amd::shader_core_properties2::EXTENSION_NAME
        ),
        (
            amd_device_coherent_memory,
            amd::device_coherent_memory::EXTENSION_NAME
        ),
        (
            khr_shader_constant_data,
            khr::shader_constant_data::EXTENSION_NAME
        ),
        (
            khr_dynamic_rendering_local_read,
            khr::dynamic_rendering_local_read::EXTENSION_NAME
        ),
        (khr_shader_abort, khr::shader_abort::EXTENSION_NAME),
        (
            ext_shader_image_atomic_int64,
            ext::shader_image_atomic_int64::EXTENSION_NAME
        ),
        (
            khr_shader_quad_control,
            khr::shader_quad_control::EXTENSION_NAME
        ),
        (khr_spirv_1_4, khr::spirv_1_4::EXTENSION_NAME),
        (ext_memory_budget, ext::memory_budget::EXTENSION_NAME),
        (ext_memory_priority, ext::memory_priority::EXTENSION_NAME),
        (
            nv_dedicated_allocation_image_aliasing,
            nv::dedicated_allocation_image_aliasing::EXTENSION_NAME
        ),
        (
            khr_separate_depth_stencil_layouts,
            khr::separate_depth_stencil_layouts::EXTENSION_NAME
        ),
        (
            ext_buffer_device_address,
            ext::buffer_device_address::EXTENSION_NAME
        ),
        (ext_tooling_info, ext::tooling_info::EXTENSION_NAME),
        (
            ext_separate_stencil_usage,
            ext::separate_stencil_usage::EXTENSION_NAME
        ),
        (khr_present_wait, khr::present_wait::EXTENSION_NAME),
        (
            nv_cooperative_matrix,
            nv::cooperative_matrix::EXTENSION_NAME
        ),
        (
            nv_coverage_reduction_mode,
            nv::coverage_reduction_mode::EXTENSION_NAME
        ),
        (
            ext_fragment_shader_interlock,
            ext::fragment_shader_interlock::EXTENSION_NAME
        ),
        (
            ext_ycbcr_image_arrays,
            ext::ycbcr_image_arrays::EXTENSION_NAME
        ),
        (
            khr_uniform_buffer_standard_layout,
            khr::uniform_buffer_standard_layout::EXTENSION_NAME
        ),
        (ext_provoking_vertex, ext::provoking_vertex::EXTENSION_NAME),
        (
            ext_full_screen_exclusive,
            ext::full_screen_exclusive::EXTENSION_NAME
        ),
        (
            khr_buffer_device_address,
            khr::buffer_device_address::EXTENSION_NAME
        ),
        (
            ext_line_rasterization,
            ext::line_rasterization::EXTENSION_NAME
        ),
        (
            ext_shader_atomic_float,
            ext::shader_atomic_float::EXTENSION_NAME
        ),
        (ext_host_query_reset, ext::host_query_reset::EXTENSION_NAME),
        (ext_index_type_uint8, ext::index_type_uint8::EXTENSION_NAME),
        (
            ext_extended_dynamic_state,
            ext::extended_dynamic_state::EXTENSION_NAME
        ),
        (
            khr_deferred_host_operations,
            khr::deferred_host_operations::EXTENSION_NAME
        ),
        (
            khr_pipeline_executable_properties,
            khr::pipeline_executable_properties::EXTENSION_NAME
        ),
        (ext_host_image_copy, ext::host_image_copy::EXTENSION_NAME),
        (khr_map_memory2, khr::map_memory2::EXTENSION_NAME),
        (
            ext_map_memory_placed,
            ext::map_memory_placed::EXTENSION_NAME
        ),
        (
            ext_shader_atomic_float2,
            ext::shader_atomic_float2::EXTENSION_NAME
        ),
        (
            ext_swapchain_maintenance1,
            ext::swapchain_maintenance1::EXTENSION_NAME
        ),
        (
            ext_shader_demote_to_helper_invocation,
            ext::shader_demote_to_helper_invocation::EXTENSION_NAME
        ),
        (
            nv_device_generated_commands,
            nv::device_generated_commands::EXTENSION_NAME
        ),
        (
            nv_inherited_viewport_scissor,
            nv::inherited_viewport_scissor::EXTENSION_NAME
        ),
        (
            khr_shader_integer_dot_product,
            khr::shader_integer_dot_product::EXTENSION_NAME
        ),
        (
            ext_texel_buffer_alignment,
            ext::texel_buffer_alignment::EXTENSION_NAME
        ),
        (
            qcom_render_pass_transform,
            qcom::render_pass_transform::EXTENSION_NAME
        ),
        (
            ext_depth_bias_control,
            ext::depth_bias_control::EXTENSION_NAME
        ),
        (
            ext_device_memory_report,
            ext::device_memory_report::EXTENSION_NAME
        ),
        (ext_robustness2, ext::robustness2::EXTENSION_NAME),
        (
            ext_custom_border_color,
            ext::custom_border_color::EXTENSION_NAME
        ),
        (
            ext_texture_compression_astc_3d,
            ext::texture_compression_astc_3d::EXTENSION_NAME
        ),
        (google_user_type, google::user_type::EXTENSION_NAME),
        (khr_pipeline_library, khr::pipeline_library::EXTENSION_NAME),
        (nv_present_barrier, nv::present_barrier::EXTENSION_NAME),
        (
            khr_shader_non_semantic_info,
            khr::shader_non_semantic_info::EXTENSION_NAME
        ),
        (khr_present_id, khr::present_id::EXTENSION_NAME),
        (ext_private_data, ext::private_data::EXTENSION_NAME),
        (
            ext_pipeline_creation_cache_control,
            ext::pipeline_creation_cache_control::EXTENSION_NAME
        ),
        (
            khr_video_encode_queue,
            khr::video_encode_queue::EXTENSION_NAME
        ),
        (
            nv_device_diagnostics_config,
            nv::device_diagnostics_config::EXTENSION_NAME
        ),
        (
            qcom_render_pass_store_ops,
            qcom::render_pass_store_ops::EXTENSION_NAME
        ),
        (qcom_queue_perf_hint, qcom::queue_perf_hint::EXTENSION_NAME),
        #[cfg(feature = "provisional")]
        (
            nv_cuda_kernel_launch,
            nv::cuda_kernel_launch::EXTENSION_NAME
        ),
        (qcom_tile_shading, qcom::tile_shading::EXTENSION_NAME),
        (nv_low_latency, nv::low_latency::EXTENSION_NAME),
        (ext_metal_objects, ext::metal_objects::EXTENSION_NAME),
        (khr_synchronization2, khr::synchronization2::EXTENSION_NAME),
        (
            ext_descriptor_buffer,
            ext::descriptor_buffer::EXTENSION_NAME
        ),
        (
            khr_device_address_commands,
            khr::device_address_commands::EXTENSION_NAME
        ),
        (
            ext_graphics_pipeline_library,
            ext::graphics_pipeline_library::EXTENSION_NAME
        ),
        (
            amd_shader_early_and_late_fragment_tests,
            amd::shader_early_and_late_fragment_tests::EXTENSION_NAME
        ),
        (
            khr_fragment_shader_barycentric,
            khr::fragment_shader_barycentric::EXTENSION_NAME
        ),
        (
            khr_shader_subgroup_uniform_control_flow,
            khr::shader_subgroup_uniform_control_flow::EXTENSION_NAME
        ),
        (
            khr_zero_initialize_workgroup_memory,
            khr::zero_initialize_workgroup_memory::EXTENSION_NAME
        ),
        (
            nv_fragment_shading_rate_enums,
            nv::fragment_shading_rate_enums::EXTENSION_NAME
        ),
        (
            nv_ray_tracing_motion_blur,
            nv::ray_tracing_motion_blur::EXTENSION_NAME
        ),
        (ext_mesh_shader, ext::mesh_shader::EXTENSION_NAME),
        (
            ext_ycbcr_2plane_444_formats,
            ext::ycbcr_2plane_444_formats::EXTENSION_NAME
        ),
        (
            ext_fragment_density_map2,
            ext::fragment_density_map2::EXTENSION_NAME
        ),
        (
            qcom_rotated_copy_commands,
            qcom::rotated_copy_commands::EXTENSION_NAME
        ),
        (ext_image_robustness, ext::image_robustness::EXTENSION_NAME),
        (
            khr_workgroup_memory_explicit_layout,
            khr::workgroup_memory_explicit_layout::EXTENSION_NAME
        ),
        (khr_copy_commands2, khr::copy_commands2::EXTENSION_NAME),
        (
            ext_image_compression_control,
            ext::image_compression_control::EXTENSION_NAME
        ),
        (
            ext_attachment_feedback_loop_layout,
            ext::attachment_feedback_loop_layout::EXTENSION_NAME
        ),
        (ext__4444_formats, ext::_4444_formats::EXTENSION_NAME),
        (ext_device_fault, ext::device_fault::EXTENSION_NAME),
        (
            arm_rasterization_order_attachment_access,
            arm::rasterization_order_attachment_access::EXTENSION_NAME
        ),
        (ext_rgba10x6_formats, ext::rgba10x6_formats::EXTENSION_NAME),
        (
            nv_acquire_winrt_display,
            nv::acquire_winrt_display::EXTENSION_NAME
        ),
        (
            valve_mutable_descriptor_type,
            valve::mutable_descriptor_type::EXTENSION_NAME
        ),
        (
            ext_vertex_input_dynamic_state,
            ext::vertex_input_dynamic_state::EXTENSION_NAME
        ),
        (
            ext_physical_device_drm,
            ext::physical_device_drm::EXTENSION_NAME
        ),
        (
            ext_device_address_binding_report,
            ext::device_address_binding_report::EXTENSION_NAME
        ),
        (
            ext_depth_clip_control,
            ext::depth_clip_control::EXTENSION_NAME
        ),
        (
            ext_primitive_topology_list_restart,
            ext::primitive_topology_list_restart::EXTENSION_NAME
        ),
        (
            khr_format_feature_flags2,
            khr::format_feature_flags2::EXTENSION_NAME
        ),
        (
            ext_present_mode_fifo_latest_ready,
            ext::present_mode_fifo_latest_ready::EXTENSION_NAME
        ),
        (
            fuchsia_external_memory,
            fuchsia::external_memory::EXTENSION_NAME
        ),
        (
            fuchsia_external_semaphore,
            fuchsia::external_semaphore::EXTENSION_NAME
        ),
        (
            fuchsia_buffer_collection,
            fuchsia::buffer_collection::EXTENSION_NAME
        ),
        (
            huawei_subpass_shading,
            huawei::subpass_shading::EXTENSION_NAME
        ),
        (
            huawei_invocation_mask,
            huawei::invocation_mask::EXTENSION_NAME
        ),
        (
            nv_external_memory_rdma,
            nv::external_memory_rdma::EXTENSION_NAME
        ),
        (
            ext_pipeline_properties,
            ext::pipeline_properties::EXTENSION_NAME
        ),
        (ext_frame_boundary, ext::frame_boundary::EXTENSION_NAME),
        (
            ext_multisampled_render_to_single_sampled,
            ext::multisampled_render_to_single_sampled::EXTENSION_NAME
        ),
        (
            ext_extended_dynamic_state2,
            ext::extended_dynamic_state2::EXTENSION_NAME
        ),
        (
            ext_color_write_enable,
            ext::color_write_enable::EXTENSION_NAME
        ),
        (
            ext_primitives_generated_query,
            ext::primitives_generated_query::EXTENSION_NAME
        ),
        (
            khr_ray_tracing_maintenance1,
            khr::ray_tracing_maintenance1::EXTENSION_NAME
        ),
        (
            khr_shader_untyped_pointers,
            khr::shader_untyped_pointers::EXTENSION_NAME
        ),
        (
            ext_global_priority_query,
            ext::global_priority_query::EXTENSION_NAME
        ),
        (
            valve_video_encode_rgb_conversion,
            valve::video_encode_rgb_conversion::EXTENSION_NAME
        ),
        (
            ext_image_view_min_lod,
            ext::image_view_min_lod::EXTENSION_NAME
        ),
        (ext_multi_draw, ext::multi_draw::EXTENSION_NAME),
        (
            ext_image_2d_view_of_3d,
            ext::image_2d_view_of_3d::EXTENSION_NAME
        ),
        (
            ext_shader_tile_image,
            ext::shader_tile_image::EXTENSION_NAME
        ),
        (ext_opacity_micromap, ext::opacity_micromap::EXTENSION_NAME),
        #[cfg(feature = "provisional")]
        (
            nv_displacement_micromap,
            nv::displacement_micromap::EXTENSION_NAME
        ),
        (
            ext_load_store_op_none,
            ext::load_store_op_none::EXTENSION_NAME
        ),
        (
            huawei_cluster_culling_shader,
            huawei::cluster_culling_shader::EXTENSION_NAME
        ),
        (
            ext_border_color_swizzle,
            ext::border_color_swizzle::EXTENSION_NAME
        ),
        (
            ext_pageable_device_local_memory,
            ext::pageable_device_local_memory::EXTENSION_NAME
        ),
        (khr_maintenance4, khr::maintenance4::EXTENSION_NAME),
        (
            arm_shader_core_properties,
            arm::shader_core_properties::EXTENSION_NAME
        ),
        (
            khr_shader_subgroup_rotate,
            khr::shader_subgroup_rotate::EXTENSION_NAME
        ),
        (
            arm_scheduling_controls,
            arm::scheduling_controls::EXTENSION_NAME
        ),
        (
            ext_image_sliced_view_of_3d,
            ext::image_sliced_view_of_3d::EXTENSION_NAME
        ),
        (
            valve_descriptor_set_host_mapping,
            valve::descriptor_set_host_mapping::EXTENSION_NAME
        ),
        (
            ext_depth_clamp_zero_one,
            ext::depth_clamp_zero_one::EXTENSION_NAME
        ),
        (
            ext_non_seamless_cube_map,
            ext::non_seamless_cube_map::EXTENSION_NAME
        ),
        (
            arm_render_pass_striped,
            arm::render_pass_striped::EXTENSION_NAME
        ),
        (
            qcom_fragment_density_map_offset,
            qcom::fragment_density_map_offset::EXTENSION_NAME
        ),
        (
            nv_copy_memory_indirect,
            nv::copy_memory_indirect::EXTENSION_NAME
        ),
        (
            nv_memory_decompression,
            nv::memory_decompression::EXTENSION_NAME
        ),
        (
            nv_device_generated_commands_compute,
            nv::device_generated_commands_compute::EXTENSION_NAME
        ),
        (
            nv_ray_tracing_linear_swept_spheres,
            nv::ray_tracing_linear_swept_spheres::EXTENSION_NAME
        ),
        (
            nv_linear_color_attachment,
            nv::linear_color_attachment::EXTENSION_NAME
        ),
        (
            khr_shader_maximal_reconvergence,
            khr::shader_maximal_reconvergence::EXTENSION_NAME
        ),
        (
            ext_image_compression_control_swapchain,
            ext::image_compression_control_swapchain::EXTENSION_NAME
        ),
        (
            qcom_image_processing,
            qcom::image_processing::EXTENSION_NAME
        ),
        (
            ext_nested_command_buffer,
            ext::nested_command_buffer::EXTENSION_NAME
        ),
        (ohos_external_memory, ohos::external_memory::EXTENSION_NAME),
        (
            ext_external_memory_acquire_unmodified,
            ext::external_memory_acquire_unmodified::EXTENSION_NAME
        ),
        (
            ext_extended_dynamic_state3,
            ext::extended_dynamic_state3::EXTENSION_NAME
        ),
        (
            ext_subpass_merge_feedback,
            ext::subpass_merge_feedback::EXTENSION_NAME
        ),
        (arm_tensors, arm::tensors::EXTENSION_NAME),
        (
            ext_shader_module_identifier,
            ext::shader_module_identifier::EXTENSION_NAME
        ),
        (
            ext_rasterization_order_attachment_access,
            ext::rasterization_order_attachment_access::EXTENSION_NAME
        ),
        (nv_optical_flow, nv::optical_flow::EXTENSION_NAME),
        (ext_legacy_dithering, ext::legacy_dithering::EXTENSION_NAME),
        (
            ext_pipeline_protected_access,
            ext::pipeline_protected_access::EXTENSION_NAME
        ),
        (
            android_external_format_resolve,
            android::external_format_resolve::EXTENSION_NAME
        ),
        (khr_maintenance5, khr::maintenance5::EXTENSION_NAME),
        (amd_anti_lag, amd::anti_lag::EXTENSION_NAME),
        #[cfg(feature = "provisional")]
        (
            amdx_dense_geometry_format,
            amdx::dense_geometry_format::EXTENSION_NAME
        ),
        (khr_present_id2, khr::present_id2::EXTENSION_NAME),
        (khr_present_wait2, khr::present_wait2::EXTENSION_NAME),
        (
            khr_ray_tracing_position_fetch,
            khr::ray_tracing_position_fetch::EXTENSION_NAME
        ),
        (ext_shader_object, ext::shader_object::EXTENSION_NAME),
        (khr_pipeline_binary, khr::pipeline_binary::EXTENSION_NAME),
        (qcom_tile_properties, qcom::tile_properties::EXTENSION_NAME),
        (sec_amigo_profiling, sec::amigo_profiling::EXTENSION_NAME),
        (
            khr_swapchain_maintenance1,
            khr::swapchain_maintenance1::EXTENSION_NAME
        ),
        (
            qcom_multiview_per_view_viewports,
            qcom::multiview_per_view_viewports::EXTENSION_NAME
        ),
        (
            nv_ray_tracing_invocation_reorder,
            nv::ray_tracing_invocation_reorder::EXTENSION_NAME
        ),
        (
            nv_cooperative_vector,
            nv::cooperative_vector::EXTENSION_NAME
        ),
        (
            nv_extended_sparse_address_space,
            nv::extended_sparse_address_space::EXTENSION_NAME
        ),
        (
            ext_mutable_descriptor_type,
            ext::mutable_descriptor_type::EXTENSION_NAME
        ),
        (
            ext_legacy_vertex_attributes,
            ext::legacy_vertex_attributes::EXTENSION_NAME
        ),
        (
            arm_shader_core_builtins,
            arm::shader_core_builtins::EXTENSION_NAME
        ),
        (
            ext_pipeline_library_group_handles,
            ext::pipeline_library_group_handles::EXTENSION_NAME
        ),
        (
            ext_dynamic_rendering_unused_attachments,
            ext::dynamic_rendering_unused_attachments::EXTENSION_NAME
        ),
        (
            khr_internally_synchronized_queues,
            khr::internally_synchronized_queues::EXTENSION_NAME
        ),
        (nv_low_latency2, nv::low_latency2::EXTENSION_NAME),
        (
            khr_cooperative_matrix,
            khr::cooperative_matrix::EXTENSION_NAME
        ),
        (arm_data_graph, arm::data_graph::EXTENSION_NAME),
        (
            arm_data_graph_instruction_set_tosa,
            arm::data_graph_instruction_set_tosa::EXTENSION_NAME
        ),
        (
            qcom_multiview_per_view_render_areas,
            qcom::multiview_per_view_render_areas::EXTENSION_NAME
        ),
        (
            khr_compute_shader_derivatives,
            khr::compute_shader_derivatives::EXTENSION_NAME
        ),
        (khr_video_decode_av1, khr::video_decode_av1::EXTENSION_NAME),
        (khr_video_encode_av1, khr::video_encode_av1::EXTENSION_NAME),
        (khr_video_decode_vp9, khr::video_decode_vp9::EXTENSION_NAME),
        (
            khr_video_maintenance1,
            khr::video_maintenance1::EXTENSION_NAME
        ),
        (
            nv_per_stage_descriptor_set,
            nv::per_stage_descriptor_set::EXTENSION_NAME
        ),
        (
            qcom_image_processing2,
            qcom::image_processing2::EXTENSION_NAME
        ),
        (
            qcom_filter_cubic_weights,
            qcom::filter_cubic_weights::EXTENSION_NAME
        ),
        (qcom_ycbcr_degamma, qcom::ycbcr_degamma::EXTENSION_NAME),
        (
            qcom_filter_cubic_clamp,
            qcom::filter_cubic_clamp::EXTENSION_NAME
        ),
        (
            ext_attachment_feedback_loop_dynamic_state,
            ext::attachment_feedback_loop_dynamic_state::EXTENSION_NAME
        ),
        (
            khr_vertex_attribute_divisor,
            khr::vertex_attribute_divisor::EXTENSION_NAME
        ),
        (
            khr_load_store_op_none,
            khr::load_store_op_none::EXTENSION_NAME
        ),
        (
            khr_unified_image_layouts,
            khr::unified_image_layouts::EXTENSION_NAME
        ),
        (
            khr_shader_float_controls2,
            khr::shader_float_controls2::EXTENSION_NAME
        ),
        (
            qnx_external_memory_screen_buffer,
            qnx::external_memory_screen_buffer::EXTENSION_NAME
        ),
        (msft_layered_driver, msft::layered_driver::EXTENSION_NAME),
        (khr_index_type_uint8, khr::index_type_uint8::EXTENSION_NAME),
        (
            khr_line_rasterization,
            khr::line_rasterization::EXTENSION_NAME
        ),
        (
            khr_calibrated_timestamps,
            khr::calibrated_timestamps::EXTENSION_NAME
        ),
        (
            khr_shader_expect_assume,
            khr::shader_expect_assume::EXTENSION_NAME
        ),
        (khr_maintenance6, khr::maintenance6::EXTENSION_NAME),
        (
            nv_descriptor_pool_overallocation,
            nv::descriptor_pool_overallocation::EXTENSION_NAME
        ),
        (
            qcom_tile_memory_heap,
            qcom::tile_memory_heap::EXTENSION_NAME
        ),
        (
            khr_copy_memory_indirect,
            khr::copy_memory_indirect::EXTENSION_NAME
        ),
        (
            ext_memory_decompression,
            ext::memory_decompression::EXTENSION_NAME
        ),
        (
            khr_video_encode_intra_refresh,
            khr::video_encode_intra_refresh::EXTENSION_NAME
        ),
        (
            khr_video_encode_quantization_map,
            khr::video_encode_quantization_map::EXTENSION_NAME
        ),
        (nv_raw_access_chains, nv::raw_access_chains::EXTENSION_NAME),
        (
            nv_external_compute_queue,
            nv::external_compute_queue::EXTENSION_NAME
        ),
        (
            khr_shader_relaxed_extended_instruction,
            khr::shader_relaxed_extended_instruction::EXTENSION_NAME
        ),
        (
            nv_command_buffer_inheritance,
            nv::command_buffer_inheritance::EXTENSION_NAME
        ),
        (khr_maintenance7, khr::maintenance7::EXTENSION_NAME),
        (
            nv_shader_atomic_float16_vector,
            nv::shader_atomic_float16_vector::EXTENSION_NAME
        ),
        (
            ext_shader_replicated_composites,
            ext::shader_replicated_composites::EXTENSION_NAME
        ),
        (ext_shader_float8, ext::shader_float8::EXTENSION_NAME),
        (
            nv_ray_tracing_validation,
            nv::ray_tracing_validation::EXTENSION_NAME
        ),
        (
            nv_cluster_acceleration_structure,
            nv::cluster_acceleration_structure::EXTENSION_NAME
        ),
        (
            nv_partitioned_acceleration_structure,
            nv::partitioned_acceleration_structure::EXTENSION_NAME
        ),
        (
            ext_device_generated_commands,
            ext::device_generated_commands::EXTENSION_NAME
        ),
        (khr_device_fault, khr::device_fault::EXTENSION_NAME),
        (khr_maintenance8, khr::maintenance8::EXTENSION_NAME),
        (
            mesa_image_alignment_control,
            mesa::image_alignment_control::EXTENSION_NAME
        ),
        (khr_shader_fma, khr::shader_fma::EXTENSION_NAME),
        (
            nv_push_constant_bank,
            nv::push_constant_bank::EXTENSION_NAME
        ),
        (
            ext_ray_tracing_invocation_reorder,
            ext::ray_tracing_invocation_reorder::EXTENSION_NAME
        ),
        (
            ext_depth_clamp_control,
            ext::depth_clamp_control::EXTENSION_NAME
        ),
        (khr_maintenance9, khr::maintenance9::EXTENSION_NAME),
        (
            khr_video_maintenance2,
            khr::video_maintenance2::EXTENSION_NAME
        ),
        (huawei_hdr_vivid, huawei::hdr_vivid::EXTENSION_NAME),
        (
            nv_cooperative_matrix2,
            nv::cooperative_matrix2::EXTENSION_NAME
        ),
        (
            arm_pipeline_opacity_micromap,
            arm::pipeline_opacity_micromap::EXTENSION_NAME
        ),
        (
            ext_external_memory_metal,
            ext::external_memory_metal::EXTENSION_NAME
        ),
        (
            khr_depth_clamp_zero_one,
            khr::depth_clamp_zero_one::EXTENSION_NAME
        ),
        (
            arm_performance_counters_by_region,
            arm::performance_counters_by_region::EXTENSION_NAME
        ),
        (
            arm_shader_instrumentation,
            arm::shader_instrumentation::EXTENSION_NAME
        ),
        (
            ext_vertex_attribute_robustness,
            ext::vertex_attribute_robustness::EXTENSION_NAME
        ),
        (arm_format_pack, arm::format_pack::EXTENSION_NAME),
        (
            valve_fragment_density_map_layered,
            valve::fragment_density_map_layered::EXTENSION_NAME
        ),
        (khr_robustness2, khr::robustness2::EXTENSION_NAME),
        (nv_present_metering, nv::present_metering::EXTENSION_NAME),
        (
            ext_fragment_density_map_offset,
            ext::fragment_density_map_offset::EXTENSION_NAME
        ),
        (
            ext_zero_initialize_device_memory,
            ext::zero_initialize_device_memory::EXTENSION_NAME
        ),
        (
            khr_present_mode_fifo_latest_ready,
            khr::present_mode_fifo_latest_ready::EXTENSION_NAME
        ),
        (
            ext_shader_64bit_indexing,
            ext::shader_64bit_indexing::EXTENSION_NAME
        ),
        (ext_custom_resolve, ext::custom_resolve::EXTENSION_NAME),
        (
            qcom_data_graph_model,
            qcom::data_graph_model::EXTENSION_NAME
        ),
        (khr_maintenance10, khr::maintenance10::EXTENSION_NAME),
        (
            arm_data_graph_optical_flow,
            arm::data_graph_optical_flow::EXTENSION_NAME
        ),
        (
            ext_shader_long_vector,
            ext::shader_long_vector::EXTENSION_NAME
        ),
        (
            sec_pipeline_cache_incremental_mode,
            sec::pipeline_cache_incremental_mode::EXTENSION_NAME
        ),
        (
            ext_shader_uniform_buffer_unsized_array,
            ext::shader_uniform_buffer_unsized_array::EXTENSION_NAME
        ),
        (
            nv_compute_occupancy_priority,
            nv::compute_occupancy_priority::EXTENSION_NAME
        ),
        (khr_maintenance11, khr::maintenance11::EXTENSION_NAME),
        (
            ext_shader_subgroup_partitioned,
            ext::shader_subgroup_partitioned::EXTENSION_NAME
        ),
        (
            valve_shader_mixed_float_dot_product,
            valve::shader_mixed_float_dot_product::EXTENSION_NAME
        ),
        (sec_throttle_hint, sec::throttle_hint::EXTENSION_NAME),
        (
            arm_data_graph_neural_accelerator_statistics,
            arm::data_graph_neural_accelerator_statistics::EXTENSION_NAME
        ),
        (
            ext_primitive_restart_index,
            ext::primitive_restart_index::EXTENSION_NAME
        ),
    ]
);

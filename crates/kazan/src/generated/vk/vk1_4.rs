#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    pub const MAX_GLOBAL_PRIORITY_SIZE: u32 = 16;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BufferUsageFlags2CreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub usage: BufferUsageFlags2,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for BufferUsageFlags2CreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::BUFFER_USAGE_FLAGS_2_CREATE_INFO,
                p_next: core::ptr::null(),
                usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BufferUsageFlags2CreateInfo<'a> {
        pub fn usage(mut self, usage: BufferUsageFlags2) -> Self {
            self.usage = usage;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineCreateFlags2CreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineCreateFlags2,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PipelineCreateFlags2CreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PIPELINE_CREATE_FLAGS_2_CREATE_INFO,
                p_next: core::ptr::null(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineCreateFlags2CreateInfo<'a> {
        pub fn flags(mut self, flags: PipelineCreateFlags2) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePushDescriptorProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_push_descriptors: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDevicePushDescriptorProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES,
                p_next: core::ptr::null_mut(),
                max_push_descriptors: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePushDescriptorProperties<'a> {
        pub fn max_push_descriptors(mut self, max_push_descriptors: u32) -> Self {
            self.max_push_descriptors = max_push_descriptors;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMaintenance5Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub maintenance5: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceMaintenance5Features<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES,
                p_next: core::ptr::null_mut(),
                maintenance5: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMaintenance5Features<'a> {
        pub fn maintenance5(mut self, maintenance5: Bool32) -> Self {
            self.maintenance5 = maintenance5;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMaintenance5Properties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub early_fragment_multisample_coverage_after_sample_counting: Bool32,
        pub early_fragment_sample_mask_test_before_sample_counting: Bool32,
        pub depth_stencil_swizzle_one_support: Bool32,
        pub polygon_mode_point_size: Bool32,
        pub non_strict_single_pixel_wide_lines_use_parallelogram: Bool32,
        pub non_strict_wide_lines_use_parallelogram: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceMaintenance5Properties<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES,
                p_next: core::ptr::null_mut(),
                early_fragment_multisample_coverage_after_sample_counting: Default::default(),
                early_fragment_sample_mask_test_before_sample_counting: Default::default(),
                depth_stencil_swizzle_one_support: Default::default(),
                polygon_mode_point_size: Default::default(),
                non_strict_single_pixel_wide_lines_use_parallelogram: Default::default(),
                non_strict_wide_lines_use_parallelogram: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMaintenance5Properties<'a> {
        pub fn early_fragment_multisample_coverage_after_sample_counting(
            mut self,
            early_fragment_multisample_coverage_after_sample_counting: Bool32,
        ) -> Self {
            self.early_fragment_multisample_coverage_after_sample_counting =
                early_fragment_multisample_coverage_after_sample_counting;
            self
        }
        pub fn early_fragment_sample_mask_test_before_sample_counting(
            mut self,
            early_fragment_sample_mask_test_before_sample_counting: Bool32,
        ) -> Self {
            self.early_fragment_sample_mask_test_before_sample_counting =
                early_fragment_sample_mask_test_before_sample_counting;
            self
        }
        pub fn depth_stencil_swizzle_one_support(
            mut self,
            depth_stencil_swizzle_one_support: Bool32,
        ) -> Self {
            self.depth_stencil_swizzle_one_support = depth_stencil_swizzle_one_support;
            self
        }
        pub fn polygon_mode_point_size(mut self, polygon_mode_point_size: Bool32) -> Self {
            self.polygon_mode_point_size = polygon_mode_point_size;
            self
        }
        pub fn non_strict_single_pixel_wide_lines_use_parallelogram(
            mut self,
            non_strict_single_pixel_wide_lines_use_parallelogram: Bool32,
        ) -> Self {
            self.non_strict_single_pixel_wide_lines_use_parallelogram =
                non_strict_single_pixel_wide_lines_use_parallelogram;
            self
        }
        pub fn non_strict_wide_lines_use_parallelogram(
            mut self,
            non_strict_wide_lines_use_parallelogram: Bool32,
        ) -> Self {
            self.non_strict_wide_lines_use_parallelogram = non_strict_wide_lines_use_parallelogram;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMaintenance6Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub maintenance6: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceMaintenance6Features<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES,
                p_next: core::ptr::null_mut(),
                maintenance6: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMaintenance6Features<'a> {
        pub fn maintenance6(mut self, maintenance6: Bool32) -> Self {
            self.maintenance6 = maintenance6;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMaintenance6Properties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub block_texel_view_compatible_multiple_layers: Bool32,
        pub max_combined_image_sampler_descriptor_count: u32,
        pub fragment_shading_rate_clamp_combiner_inputs: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceMaintenance6Properties<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES,
                p_next: core::ptr::null_mut(),
                block_texel_view_compatible_multiple_layers: Default::default(),
                max_combined_image_sampler_descriptor_count: Default::default(),
                fragment_shading_rate_clamp_combiner_inputs: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMaintenance6Properties<'a> {
        pub fn block_texel_view_compatible_multiple_layers(
            mut self,
            block_texel_view_compatible_multiple_layers: Bool32,
        ) -> Self {
            self.block_texel_view_compatible_multiple_layers =
                block_texel_view_compatible_multiple_layers;
            self
        }
        pub fn max_combined_image_sampler_descriptor_count(
            mut self,
            max_combined_image_sampler_descriptor_count: u32,
        ) -> Self {
            self.max_combined_image_sampler_descriptor_count =
                max_combined_image_sampler_descriptor_count;
            self
        }
        pub fn fragment_shading_rate_clamp_combiner_inputs(
            mut self,
            fragment_shading_rate_clamp_combiner_inputs: Bool32,
        ) -> Self {
            self.fragment_shading_rate_clamp_combiner_inputs =
                fragment_shading_rate_clamp_combiner_inputs;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderingAreaInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub view_mask: u32,
        pub color_attachment_count: u32,
        pub p_color_attachment_formats: *const Format,
        pub depth_attachment_format: Format,
        pub stencil_attachment_format: Format,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for RenderingAreaInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::RENDERING_AREA_INFO,
                p_next: core::ptr::null(),
                view_mask: Default::default(),
                color_attachment_count: Default::default(),
                p_color_attachment_formats: core::ptr::null(),
                depth_attachment_format: Default::default(),
                stencil_attachment_format: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderingAreaInfo<'a> {
        pub fn view_mask(mut self, view_mask: u32) -> Self {
            self.view_mask = view_mask;
            self
        }
        pub fn color_attachment_formats(mut self, color_attachment_formats: &'a [Format]) -> Self {
            self.color_attachment_count = color_attachment_formats.len().try_into().unwrap();
            self.p_color_attachment_formats = color_attachment_formats.as_ptr();
            self
        }
        pub fn depth_attachment_format(mut self, depth_attachment_format: Format) -> Self {
            self.depth_attachment_format = depth_attachment_format;
            self
        }
        pub fn stencil_attachment_format(mut self, stencil_attachment_format: Format) -> Self {
            self.stencil_attachment_format = stencil_attachment_format;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceQueueGlobalPriorityCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub global_priority: QueueGlobalPriority,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for DeviceQueueGlobalPriorityCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO,
                p_next: core::ptr::null(),
                global_priority: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceQueueGlobalPriorityCreateInfo<'a> {
        pub fn global_priority(mut self, global_priority: QueueGlobalPriority) -> Self {
            self.global_priority = global_priority;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceGlobalPriorityQueryFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub global_priority_query: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceGlobalPriorityQueryFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES,
                p_next: core::ptr::null_mut(),
                global_priority_query: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceGlobalPriorityQueryFeatures<'a> {
        pub fn global_priority_query(mut self, global_priority_query: Bool32) -> Self {
            self.global_priority_query = global_priority_query;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct QueueFamilyGlobalPriorityProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub priority_count: u32,
        pub priorities: [QueueGlobalPriority; MAX_GLOBAL_PRIORITY_SIZE as usize],
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for QueueFamilyGlobalPriorityProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES,
                p_next: core::ptr::null_mut(),
                priority_count: Default::default(),
                priorities: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }
    impl<'a> QueueFamilyGlobalPriorityProperties<'a> {
        pub fn priorities(mut self, priorities: &[QueueGlobalPriority]) -> Self {
            self.priority_count = priorities.len().try_into().unwrap();
            self.priorities[..priorities.len()].copy_from_slice(priorities);
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct VertexInputBindingDivisorDescription {
        pub binding: u32,
        pub divisor: u32,
    }
    impl VertexInputBindingDivisorDescription {
        pub fn binding(mut self, binding: u32) -> Self {
            self.binding = binding;
            self
        }
        pub fn divisor(mut self, divisor: u32) -> Self {
            self.divisor = divisor;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineVertexInputDivisorStateCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub vertex_binding_divisor_count: u32,
        pub p_vertex_binding_divisors: *const VertexInputBindingDivisorDescription,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PipelineVertexInputDivisorStateCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO,
                p_next: core::ptr::null(),
                vertex_binding_divisor_count: Default::default(),
                p_vertex_binding_divisors: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineVertexInputDivisorStateCreateInfo<'a> {
        pub fn vertex_binding_divisors(
            mut self,
            vertex_binding_divisors: &'a [VertexInputBindingDivisorDescription],
        ) -> Self {
            self.vertex_binding_divisor_count = vertex_binding_divisors.len().try_into().unwrap();
            self.p_vertex_binding_divisors = vertex_binding_divisors.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVertexAttributeDivisorProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_vertex_attrib_divisor: u32,
        pub supports_non_zero_first_instance: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceVertexAttributeDivisorProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES,
                p_next: core::ptr::null_mut(),
                max_vertex_attrib_divisor: Default::default(),
                supports_non_zero_first_instance: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceVertexAttributeDivisorProperties<'a> {
        pub fn max_vertex_attrib_divisor(mut self, max_vertex_attrib_divisor: u32) -> Self {
            self.max_vertex_attrib_divisor = max_vertex_attrib_divisor;
            self
        }
        pub fn supports_non_zero_first_instance(
            mut self,
            supports_non_zero_first_instance: Bool32,
        ) -> Self {
            self.supports_non_zero_first_instance = supports_non_zero_first_instance;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVertexAttributeDivisorFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub vertex_attribute_instance_rate_divisor: Bool32,
        pub vertex_attribute_instance_rate_zero_divisor: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceVertexAttributeDivisorFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES,
                p_next: core::ptr::null_mut(),
                vertex_attribute_instance_rate_divisor: Default::default(),
                vertex_attribute_instance_rate_zero_divisor: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceVertexAttributeDivisorFeatures<'a> {
        pub fn vertex_attribute_instance_rate_divisor(
            mut self,
            vertex_attribute_instance_rate_divisor: Bool32,
        ) -> Self {
            self.vertex_attribute_instance_rate_divisor = vertex_attribute_instance_rate_divisor;
            self
        }
        pub fn vertex_attribute_instance_rate_zero_divisor(
            mut self,
            vertex_attribute_instance_rate_zero_divisor: Bool32,
        ) -> Self {
            self.vertex_attribute_instance_rate_zero_divisor =
                vertex_attribute_instance_rate_zero_divisor;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceIndexTypeUint8Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub index_type_uint8: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceIndexTypeUint8Features<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES,
                p_next: core::ptr::null_mut(),
                index_type_uint8: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceIndexTypeUint8Features<'a> {
        pub fn index_type_uint8(mut self, index_type_uint8: Bool32) -> Self {
            self.index_type_uint8 = index_type_uint8;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceLineRasterizationFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub rectangular_lines: Bool32,
        pub bresenham_lines: Bool32,
        pub smooth_lines: Bool32,
        pub stippled_rectangular_lines: Bool32,
        pub stippled_bresenham_lines: Bool32,
        pub stippled_smooth_lines: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceLineRasterizationFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES,
                p_next: core::ptr::null_mut(),
                rectangular_lines: Default::default(),
                bresenham_lines: Default::default(),
                smooth_lines: Default::default(),
                stippled_rectangular_lines: Default::default(),
                stippled_bresenham_lines: Default::default(),
                stippled_smooth_lines: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceLineRasterizationFeatures<'a> {
        pub fn rectangular_lines(mut self, rectangular_lines: Bool32) -> Self {
            self.rectangular_lines = rectangular_lines;
            self
        }
        pub fn bresenham_lines(mut self, bresenham_lines: Bool32) -> Self {
            self.bresenham_lines = bresenham_lines;
            self
        }
        pub fn smooth_lines(mut self, smooth_lines: Bool32) -> Self {
            self.smooth_lines = smooth_lines;
            self
        }
        pub fn stippled_rectangular_lines(mut self, stippled_rectangular_lines: Bool32) -> Self {
            self.stippled_rectangular_lines = stippled_rectangular_lines;
            self
        }
        pub fn stippled_bresenham_lines(mut self, stippled_bresenham_lines: Bool32) -> Self {
            self.stippled_bresenham_lines = stippled_bresenham_lines;
            self
        }
        pub fn stippled_smooth_lines(mut self, stippled_smooth_lines: Bool32) -> Self {
            self.stippled_smooth_lines = stippled_smooth_lines;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceLineRasterizationProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub line_sub_pixel_precision_bits: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceLineRasterizationProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES,
                p_next: core::ptr::null_mut(),
                line_sub_pixel_precision_bits: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceLineRasterizationProperties<'a> {
        pub fn line_sub_pixel_precision_bits(mut self, line_sub_pixel_precision_bits: u32) -> Self {
            self.line_sub_pixel_precision_bits = line_sub_pixel_precision_bits;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineRasterizationLineStateCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub line_rasterization_mode: LineRasterizationMode,
        pub stippled_line_enable: Bool32,
        pub line_stipple_factor: u32,
        pub line_stipple_pattern: u16,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PipelineRasterizationLineStateCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO,
                p_next: core::ptr::null(),
                line_rasterization_mode: Default::default(),
                stippled_line_enable: Default::default(),
                line_stipple_factor: Default::default(),
                line_stipple_pattern: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineRasterizationLineStateCreateInfo<'a> {
        pub fn line_rasterization_mode(
            mut self,
            line_rasterization_mode: LineRasterizationMode,
        ) -> Self {
            self.line_rasterization_mode = line_rasterization_mode;
            self
        }
        pub fn stippled_line_enable(mut self, stippled_line_enable: Bool32) -> Self {
            self.stippled_line_enable = stippled_line_enable;
            self
        }
        pub fn line_stipple_factor(mut self, line_stipple_factor: u32) -> Self {
            self.line_stipple_factor = line_stipple_factor;
            self
        }
        pub fn line_stipple_pattern(mut self, line_stipple_pattern: u16) -> Self {
            self.line_stipple_pattern = line_stipple_pattern;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVulkan14Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub global_priority_query: Bool32,
        pub shader_subgroup_rotate: Bool32,
        pub shader_subgroup_rotate_clustered: Bool32,
        pub shader_float_controls2: Bool32,
        pub shader_expect_assume: Bool32,
        pub rectangular_lines: Bool32,
        pub bresenham_lines: Bool32,
        pub smooth_lines: Bool32,
        pub stippled_rectangular_lines: Bool32,
        pub stippled_bresenham_lines: Bool32,
        pub stippled_smooth_lines: Bool32,
        pub vertex_attribute_instance_rate_divisor: Bool32,
        pub vertex_attribute_instance_rate_zero_divisor: Bool32,
        pub index_type_uint8: Bool32,
        pub dynamic_rendering_local_read: Bool32,
        pub maintenance5: Bool32,
        pub maintenance6: Bool32,
        pub pipeline_protected_access: Bool32,
        pub pipeline_robustness: Bool32,
        pub host_image_copy: Bool32,
        pub push_descriptor: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceVulkan14Features<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_VULKAN_1_4_FEATURES,
                p_next: core::ptr::null_mut(),
                global_priority_query: Default::default(),
                shader_subgroup_rotate: Default::default(),
                shader_subgroup_rotate_clustered: Default::default(),
                shader_float_controls2: Default::default(),
                shader_expect_assume: Default::default(),
                rectangular_lines: Default::default(),
                bresenham_lines: Default::default(),
                smooth_lines: Default::default(),
                stippled_rectangular_lines: Default::default(),
                stippled_bresenham_lines: Default::default(),
                stippled_smooth_lines: Default::default(),
                vertex_attribute_instance_rate_divisor: Default::default(),
                vertex_attribute_instance_rate_zero_divisor: Default::default(),
                index_type_uint8: Default::default(),
                dynamic_rendering_local_read: Default::default(),
                maintenance5: Default::default(),
                maintenance6: Default::default(),
                pipeline_protected_access: Default::default(),
                pipeline_robustness: Default::default(),
                host_image_copy: Default::default(),
                push_descriptor: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceVulkan14Features<'a> {
        pub fn global_priority_query(mut self, global_priority_query: Bool32) -> Self {
            self.global_priority_query = global_priority_query;
            self
        }
        pub fn shader_subgroup_rotate(mut self, shader_subgroup_rotate: Bool32) -> Self {
            self.shader_subgroup_rotate = shader_subgroup_rotate;
            self
        }
        pub fn shader_subgroup_rotate_clustered(
            mut self,
            shader_subgroup_rotate_clustered: Bool32,
        ) -> Self {
            self.shader_subgroup_rotate_clustered = shader_subgroup_rotate_clustered;
            self
        }
        pub fn shader_float_controls2(mut self, shader_float_controls2: Bool32) -> Self {
            self.shader_float_controls2 = shader_float_controls2;
            self
        }
        pub fn shader_expect_assume(mut self, shader_expect_assume: Bool32) -> Self {
            self.shader_expect_assume = shader_expect_assume;
            self
        }
        pub fn rectangular_lines(mut self, rectangular_lines: Bool32) -> Self {
            self.rectangular_lines = rectangular_lines;
            self
        }
        pub fn bresenham_lines(mut self, bresenham_lines: Bool32) -> Self {
            self.bresenham_lines = bresenham_lines;
            self
        }
        pub fn smooth_lines(mut self, smooth_lines: Bool32) -> Self {
            self.smooth_lines = smooth_lines;
            self
        }
        pub fn stippled_rectangular_lines(mut self, stippled_rectangular_lines: Bool32) -> Self {
            self.stippled_rectangular_lines = stippled_rectangular_lines;
            self
        }
        pub fn stippled_bresenham_lines(mut self, stippled_bresenham_lines: Bool32) -> Self {
            self.stippled_bresenham_lines = stippled_bresenham_lines;
            self
        }
        pub fn stippled_smooth_lines(mut self, stippled_smooth_lines: Bool32) -> Self {
            self.stippled_smooth_lines = stippled_smooth_lines;
            self
        }
        pub fn vertex_attribute_instance_rate_divisor(
            mut self,
            vertex_attribute_instance_rate_divisor: Bool32,
        ) -> Self {
            self.vertex_attribute_instance_rate_divisor = vertex_attribute_instance_rate_divisor;
            self
        }
        pub fn vertex_attribute_instance_rate_zero_divisor(
            mut self,
            vertex_attribute_instance_rate_zero_divisor: Bool32,
        ) -> Self {
            self.vertex_attribute_instance_rate_zero_divisor =
                vertex_attribute_instance_rate_zero_divisor;
            self
        }
        pub fn index_type_uint8(mut self, index_type_uint8: Bool32) -> Self {
            self.index_type_uint8 = index_type_uint8;
            self
        }
        pub fn dynamic_rendering_local_read(
            mut self,
            dynamic_rendering_local_read: Bool32,
        ) -> Self {
            self.dynamic_rendering_local_read = dynamic_rendering_local_read;
            self
        }
        pub fn maintenance5(mut self, maintenance5: Bool32) -> Self {
            self.maintenance5 = maintenance5;
            self
        }
        pub fn maintenance6(mut self, maintenance6: Bool32) -> Self {
            self.maintenance6 = maintenance6;
            self
        }
        pub fn pipeline_protected_access(mut self, pipeline_protected_access: Bool32) -> Self {
            self.pipeline_protected_access = pipeline_protected_access;
            self
        }
        pub fn pipeline_robustness(mut self, pipeline_robustness: Bool32) -> Self {
            self.pipeline_robustness = pipeline_robustness;
            self
        }
        pub fn host_image_copy(mut self, host_image_copy: Bool32) -> Self {
            self.host_image_copy = host_image_copy;
            self
        }
        pub fn push_descriptor(mut self, push_descriptor: Bool32) -> Self {
            self.push_descriptor = push_descriptor;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVulkan14Properties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub line_sub_pixel_precision_bits: u32,
        pub max_vertex_attrib_divisor: u32,
        pub supports_non_zero_first_instance: Bool32,
        pub max_push_descriptors: u32,
        pub dynamic_rendering_local_read_depth_stencil_attachments: Bool32,
        pub dynamic_rendering_local_read_multisampled_attachments: Bool32,
        pub early_fragment_multisample_coverage_after_sample_counting: Bool32,
        pub early_fragment_sample_mask_test_before_sample_counting: Bool32,
        pub depth_stencil_swizzle_one_support: Bool32,
        pub polygon_mode_point_size: Bool32,
        pub non_strict_single_pixel_wide_lines_use_parallelogram: Bool32,
        pub non_strict_wide_lines_use_parallelogram: Bool32,
        pub block_texel_view_compatible_multiple_layers: Bool32,
        pub max_combined_image_sampler_descriptor_count: u32,
        pub fragment_shading_rate_clamp_combiner_inputs: Bool32,
        pub default_robustness_storage_buffers: PipelineRobustnessBufferBehavior,
        pub default_robustness_uniform_buffers: PipelineRobustnessBufferBehavior,
        pub default_robustness_vertex_inputs: PipelineRobustnessBufferBehavior,
        pub default_robustness_images: PipelineRobustnessImageBehavior,
        pub copy_src_layout_count: u32,
        pub p_copy_src_layouts: *mut ImageLayout,
        pub copy_dst_layout_count: u32,
        pub p_copy_dst_layouts: *mut ImageLayout,
        pub optimal_tiling_layout_uuid: [u8; UUID_SIZE as usize],
        pub identical_memory_type_requirements: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceVulkan14Properties<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES,
                p_next: core::ptr::null_mut(),
                line_sub_pixel_precision_bits: Default::default(),
                max_vertex_attrib_divisor: Default::default(),
                supports_non_zero_first_instance: Default::default(),
                max_push_descriptors: Default::default(),
                dynamic_rendering_local_read_depth_stencil_attachments: Default::default(),
                dynamic_rendering_local_read_multisampled_attachments: Default::default(),
                early_fragment_multisample_coverage_after_sample_counting: Default::default(),
                early_fragment_sample_mask_test_before_sample_counting: Default::default(),
                depth_stencil_swizzle_one_support: Default::default(),
                polygon_mode_point_size: Default::default(),
                non_strict_single_pixel_wide_lines_use_parallelogram: Default::default(),
                non_strict_wide_lines_use_parallelogram: Default::default(),
                block_texel_view_compatible_multiple_layers: Default::default(),
                max_combined_image_sampler_descriptor_count: Default::default(),
                fragment_shading_rate_clamp_combiner_inputs: Default::default(),
                default_robustness_storage_buffers: Default::default(),
                default_robustness_uniform_buffers: Default::default(),
                default_robustness_vertex_inputs: Default::default(),
                default_robustness_images: Default::default(),
                copy_src_layout_count: Default::default(),
                p_copy_src_layouts: core::ptr::null_mut(),
                copy_dst_layout_count: Default::default(),
                p_copy_dst_layouts: core::ptr::null_mut(),
                optimal_tiling_layout_uuid: [Default::default(); _],
                identical_memory_type_requirements: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceVulkan14Properties<'a> {
        pub fn line_sub_pixel_precision_bits(mut self, line_sub_pixel_precision_bits: u32) -> Self {
            self.line_sub_pixel_precision_bits = line_sub_pixel_precision_bits;
            self
        }
        pub fn max_vertex_attrib_divisor(mut self, max_vertex_attrib_divisor: u32) -> Self {
            self.max_vertex_attrib_divisor = max_vertex_attrib_divisor;
            self
        }
        pub fn supports_non_zero_first_instance(
            mut self,
            supports_non_zero_first_instance: Bool32,
        ) -> Self {
            self.supports_non_zero_first_instance = supports_non_zero_first_instance;
            self
        }
        pub fn max_push_descriptors(mut self, max_push_descriptors: u32) -> Self {
            self.max_push_descriptors = max_push_descriptors;
            self
        }
        pub fn dynamic_rendering_local_read_depth_stencil_attachments(
            mut self,
            dynamic_rendering_local_read_depth_stencil_attachments: Bool32,
        ) -> Self {
            self.dynamic_rendering_local_read_depth_stencil_attachments =
                dynamic_rendering_local_read_depth_stencil_attachments;
            self
        }
        pub fn dynamic_rendering_local_read_multisampled_attachments(
            mut self,
            dynamic_rendering_local_read_multisampled_attachments: Bool32,
        ) -> Self {
            self.dynamic_rendering_local_read_multisampled_attachments =
                dynamic_rendering_local_read_multisampled_attachments;
            self
        }
        pub fn early_fragment_multisample_coverage_after_sample_counting(
            mut self,
            early_fragment_multisample_coverage_after_sample_counting: Bool32,
        ) -> Self {
            self.early_fragment_multisample_coverage_after_sample_counting =
                early_fragment_multisample_coverage_after_sample_counting;
            self
        }
        pub fn early_fragment_sample_mask_test_before_sample_counting(
            mut self,
            early_fragment_sample_mask_test_before_sample_counting: Bool32,
        ) -> Self {
            self.early_fragment_sample_mask_test_before_sample_counting =
                early_fragment_sample_mask_test_before_sample_counting;
            self
        }
        pub fn depth_stencil_swizzle_one_support(
            mut self,
            depth_stencil_swizzle_one_support: Bool32,
        ) -> Self {
            self.depth_stencil_swizzle_one_support = depth_stencil_swizzle_one_support;
            self
        }
        pub fn polygon_mode_point_size(mut self, polygon_mode_point_size: Bool32) -> Self {
            self.polygon_mode_point_size = polygon_mode_point_size;
            self
        }
        pub fn non_strict_single_pixel_wide_lines_use_parallelogram(
            mut self,
            non_strict_single_pixel_wide_lines_use_parallelogram: Bool32,
        ) -> Self {
            self.non_strict_single_pixel_wide_lines_use_parallelogram =
                non_strict_single_pixel_wide_lines_use_parallelogram;
            self
        }
        pub fn non_strict_wide_lines_use_parallelogram(
            mut self,
            non_strict_wide_lines_use_parallelogram: Bool32,
        ) -> Self {
            self.non_strict_wide_lines_use_parallelogram = non_strict_wide_lines_use_parallelogram;
            self
        }
        pub fn block_texel_view_compatible_multiple_layers(
            mut self,
            block_texel_view_compatible_multiple_layers: Bool32,
        ) -> Self {
            self.block_texel_view_compatible_multiple_layers =
                block_texel_view_compatible_multiple_layers;
            self
        }
        pub fn max_combined_image_sampler_descriptor_count(
            mut self,
            max_combined_image_sampler_descriptor_count: u32,
        ) -> Self {
            self.max_combined_image_sampler_descriptor_count =
                max_combined_image_sampler_descriptor_count;
            self
        }
        pub fn fragment_shading_rate_clamp_combiner_inputs(
            mut self,
            fragment_shading_rate_clamp_combiner_inputs: Bool32,
        ) -> Self {
            self.fragment_shading_rate_clamp_combiner_inputs =
                fragment_shading_rate_clamp_combiner_inputs;
            self
        }
        pub fn default_robustness_storage_buffers(
            mut self,
            default_robustness_storage_buffers: PipelineRobustnessBufferBehavior,
        ) -> Self {
            self.default_robustness_storage_buffers = default_robustness_storage_buffers;
            self
        }
        pub fn default_robustness_uniform_buffers(
            mut self,
            default_robustness_uniform_buffers: PipelineRobustnessBufferBehavior,
        ) -> Self {
            self.default_robustness_uniform_buffers = default_robustness_uniform_buffers;
            self
        }
        pub fn default_robustness_vertex_inputs(
            mut self,
            default_robustness_vertex_inputs: PipelineRobustnessBufferBehavior,
        ) -> Self {
            self.default_robustness_vertex_inputs = default_robustness_vertex_inputs;
            self
        }
        pub fn default_robustness_images(
            mut self,
            default_robustness_images: PipelineRobustnessImageBehavior,
        ) -> Self {
            self.default_robustness_images = default_robustness_images;
            self
        }
        pub fn copy_src_layouts(mut self, copy_src_layouts: &'a mut [ImageLayout]) -> Self {
            self.copy_src_layout_count = copy_src_layouts.len().try_into().unwrap();
            self.p_copy_src_layouts = copy_src_layouts.as_mut_ptr();
            self
        }
        pub fn copy_dst_layouts(mut self, copy_dst_layouts: &'a mut [ImageLayout]) -> Self {
            self.copy_dst_layout_count = copy_dst_layouts.len().try_into().unwrap();
            self.p_copy_dst_layouts = copy_dst_layouts.as_mut_ptr();
            self
        }
        pub fn optimal_tiling_layout_uuid(
            mut self,
            optimal_tiling_layout_uuid: [u8; UUID_SIZE as usize],
        ) -> Self {
            self.optimal_tiling_layout_uuid = optimal_tiling_layout_uuid;
            self
        }
        pub fn identical_memory_type_requirements(
            mut self,
            identical_memory_type_requirements: Bool32,
        ) -> Self {
            self.identical_memory_type_requirements = identical_memory_type_requirements;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceHostImageCopyFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub host_image_copy: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceHostImageCopyFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES,
                p_next: core::ptr::null_mut(),
                host_image_copy: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceHostImageCopyFeatures<'a> {
        pub fn host_image_copy(mut self, host_image_copy: Bool32) -> Self {
            self.host_image_copy = host_image_copy;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceHostImageCopyProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub copy_src_layout_count: u32,
        pub p_copy_src_layouts: *mut ImageLayout,
        pub copy_dst_layout_count: u32,
        pub p_copy_dst_layouts: *mut ImageLayout,
        pub optimal_tiling_layout_uuid: [u8; UUID_SIZE as usize],
        pub identical_memory_type_requirements: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceHostImageCopyProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES,
                p_next: core::ptr::null_mut(),
                copy_src_layout_count: Default::default(),
                p_copy_src_layouts: core::ptr::null_mut(),
                copy_dst_layout_count: Default::default(),
                p_copy_dst_layouts: core::ptr::null_mut(),
                optimal_tiling_layout_uuid: [Default::default(); _],
                identical_memory_type_requirements: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceHostImageCopyProperties<'a> {
        pub fn copy_src_layouts(mut self, copy_src_layouts: &'a mut [ImageLayout]) -> Self {
            self.copy_src_layout_count = copy_src_layouts.len().try_into().unwrap();
            self.p_copy_src_layouts = copy_src_layouts.as_mut_ptr();
            self
        }
        pub fn copy_dst_layouts(mut self, copy_dst_layouts: &'a mut [ImageLayout]) -> Self {
            self.copy_dst_layout_count = copy_dst_layouts.len().try_into().unwrap();
            self.p_copy_dst_layouts = copy_dst_layouts.as_mut_ptr();
            self
        }
        pub fn optimal_tiling_layout_uuid(
            mut self,
            optimal_tiling_layout_uuid: [u8; UUID_SIZE as usize],
        ) -> Self {
            self.optimal_tiling_layout_uuid = optimal_tiling_layout_uuid;
            self
        }
        pub fn identical_memory_type_requirements(
            mut self,
            identical_memory_type_requirements: Bool32,
        ) -> Self {
            self.identical_memory_type_requirements = identical_memory_type_requirements;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryToImageCopy<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_host_pointer: *const c_void,
        pub memory_row_length: u32,
        pub memory_image_height: u32,
        pub image_subresource: ImageSubresourceLayers,
        pub image_offset: Offset3D,
        pub image_extent: Extent3D,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for MemoryToImageCopy<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::MEMORY_TO_IMAGE_COPY,
                p_next: core::ptr::null(),
                p_host_pointer: core::ptr::null(),
                memory_row_length: Default::default(),
                memory_image_height: Default::default(),
                image_subresource: Default::default(),
                image_offset: Default::default(),
                image_extent: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryToImageCopy<'a> {
        pub fn host_pointer(mut self, host_pointer: &'a c_void) -> Self {
            self.p_host_pointer = host_pointer;
            self
        }
        pub fn memory_row_length(mut self, memory_row_length: u32) -> Self {
            self.memory_row_length = memory_row_length;
            self
        }
        pub fn memory_image_height(mut self, memory_image_height: u32) -> Self {
            self.memory_image_height = memory_image_height;
            self
        }
        pub fn image_subresource(mut self, image_subresource: ImageSubresourceLayers) -> Self {
            self.image_subresource = image_subresource;
            self
        }
        pub fn image_offset(mut self, image_offset: Offset3D) -> Self {
            self.image_offset = image_offset;
            self
        }
        pub fn image_extent(mut self, image_extent: Extent3D) -> Self {
            self.image_extent = image_extent;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageToMemoryCopy<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_host_pointer: *mut c_void,
        pub memory_row_length: u32,
        pub memory_image_height: u32,
        pub image_subresource: ImageSubresourceLayers,
        pub image_offset: Offset3D,
        pub image_extent: Extent3D,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for ImageToMemoryCopy<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::IMAGE_TO_MEMORY_COPY,
                p_next: core::ptr::null(),
                p_host_pointer: core::ptr::null_mut(),
                memory_row_length: Default::default(),
                memory_image_height: Default::default(),
                image_subresource: Default::default(),
                image_offset: Default::default(),
                image_extent: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageToMemoryCopy<'a> {
        pub fn host_pointer(mut self, host_pointer: &'a mut c_void) -> Self {
            self.p_host_pointer = host_pointer;
            self
        }
        pub fn memory_row_length(mut self, memory_row_length: u32) -> Self {
            self.memory_row_length = memory_row_length;
            self
        }
        pub fn memory_image_height(mut self, memory_image_height: u32) -> Self {
            self.memory_image_height = memory_image_height;
            self
        }
        pub fn image_subresource(mut self, image_subresource: ImageSubresourceLayers) -> Self {
            self.image_subresource = image_subresource;
            self
        }
        pub fn image_offset(mut self, image_offset: Offset3D) -> Self {
            self.image_offset = image_offset;
            self
        }
        pub fn image_extent(mut self, image_extent: Extent3D) -> Self {
            self.image_extent = image_extent;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CopyMemoryToImageInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: HostImageCopyFlags,
        pub dst_image: Image,
        pub dst_image_layout: ImageLayout,
        pub region_count: u32,
        pub p_regions: *const MemoryToImageCopy<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for CopyMemoryToImageInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::COPY_MEMORY_TO_IMAGE_INFO,
                p_next: core::ptr::null(),
                flags: Default::default(),
                dst_image: Default::default(),
                dst_image_layout: Default::default(),
                region_count: Default::default(),
                p_regions: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CopyMemoryToImageInfo<'a> {
        pub fn flags(mut self, flags: HostImageCopyFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn dst_image(mut self, dst_image: Image) -> Self {
            self.dst_image = dst_image;
            self
        }
        pub fn dst_image_layout(mut self, dst_image_layout: ImageLayout) -> Self {
            self.dst_image_layout = dst_image_layout;
            self
        }
        pub fn regions(mut self, regions: &'a [MemoryToImageCopy<'a>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CopyImageToMemoryInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: HostImageCopyFlags,
        pub src_image: Image,
        pub src_image_layout: ImageLayout,
        pub region_count: u32,
        pub p_regions: *const ImageToMemoryCopy<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for CopyImageToMemoryInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::COPY_IMAGE_TO_MEMORY_INFO,
                p_next: core::ptr::null(),
                flags: Default::default(),
                src_image: Default::default(),
                src_image_layout: Default::default(),
                region_count: Default::default(),
                p_regions: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CopyImageToMemoryInfo<'a> {
        pub fn flags(mut self, flags: HostImageCopyFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn src_image(mut self, src_image: Image) -> Self {
            self.src_image = src_image;
            self
        }
        pub fn src_image_layout(mut self, src_image_layout: ImageLayout) -> Self {
            self.src_image_layout = src_image_layout;
            self
        }
        pub fn regions(mut self, regions: &'a [ImageToMemoryCopy<'a>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CopyImageToImageInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: HostImageCopyFlags,
        pub src_image: Image,
        pub src_image_layout: ImageLayout,
        pub dst_image: Image,
        pub dst_image_layout: ImageLayout,
        pub region_count: u32,
        pub p_regions: *const ImageCopy2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for CopyImageToImageInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::COPY_IMAGE_TO_IMAGE_INFO,
                p_next: core::ptr::null(),
                flags: Default::default(),
                src_image: Default::default(),
                src_image_layout: Default::default(),
                dst_image: Default::default(),
                dst_image_layout: Default::default(),
                region_count: Default::default(),
                p_regions: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CopyImageToImageInfo<'a> {
        pub fn flags(mut self, flags: HostImageCopyFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn src_image(mut self, src_image: Image) -> Self {
            self.src_image = src_image;
            self
        }
        pub fn src_image_layout(mut self, src_image_layout: ImageLayout) -> Self {
            self.src_image_layout = src_image_layout;
            self
        }
        pub fn dst_image(mut self, dst_image: Image) -> Self {
            self.dst_image = dst_image;
            self
        }
        pub fn dst_image_layout(mut self, dst_image_layout: ImageLayout) -> Self {
            self.dst_image_layout = dst_image_layout;
            self
        }
        pub fn regions(mut self, regions: &'a [ImageCopy2<'a>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct HostImageLayoutTransitionInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image: Image,
        pub old_layout: ImageLayout,
        pub new_layout: ImageLayout,
        pub subresource_range: ImageSubresourceRange,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for HostImageLayoutTransitionInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::HOST_IMAGE_LAYOUT_TRANSITION_INFO,
                p_next: core::ptr::null(),
                image: Default::default(),
                old_layout: Default::default(),
                new_layout: Default::default(),
                subresource_range: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> HostImageLayoutTransitionInfo<'a> {
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }
        pub fn old_layout(mut self, old_layout: ImageLayout) -> Self {
            self.old_layout = old_layout;
            self
        }
        pub fn new_layout(mut self, new_layout: ImageLayout) -> Self {
            self.new_layout = new_layout;
            self
        }
        pub fn subresource_range(mut self, subresource_range: ImageSubresourceRange) -> Self {
            self.subresource_range = subresource_range;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SubresourceHostMemcpySize<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for SubresourceHostMemcpySize<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::SUBRESOURCE_HOST_MEMCPY_SIZE,
                p_next: core::ptr::null_mut(),
                size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SubresourceHostMemcpySize<'a> {
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct HostImageCopyDevicePerformanceQuery<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub optimal_device_access: Bool32,
        pub identical_memory_layout: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for HostImageCopyDevicePerformanceQuery<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY,
                p_next: core::ptr::null_mut(),
                optimal_device_access: Default::default(),
                identical_memory_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> HostImageCopyDevicePerformanceQuery<'a> {
        pub fn optimal_device_access(mut self, optimal_device_access: Bool32) -> Self {
            self.optimal_device_access = optimal_device_access;
            self
        }
        pub fn identical_memory_layout(mut self, identical_memory_layout: Bool32) -> Self {
            self.identical_memory_layout = identical_memory_layout;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePipelineProtectedAccessFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_protected_access: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDevicePipelineProtectedAccessFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES,
                p_next: core::ptr::null_mut(),
                pipeline_protected_access: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePipelineProtectedAccessFeatures<'a> {
        pub fn pipeline_protected_access(mut self, pipeline_protected_access: Bool32) -> Self {
            self.pipeline_protected_access = pipeline_protected_access;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageSubresource2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image_subresource: ImageSubresource,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for ImageSubresource2<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::IMAGE_SUBRESOURCE_2,
                p_next: core::ptr::null_mut(),
                image_subresource: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageSubresource2<'a> {
        pub fn image_subresource(mut self, image_subresource: ImageSubresource) -> Self {
            self.image_subresource = image_subresource;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SubresourceLayout2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub subresource_layout: SubresourceLayout,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for SubresourceLayout2<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::SUBRESOURCE_LAYOUT_2,
                p_next: core::ptr::null_mut(),
                subresource_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SubresourceLayout2<'a> {
        pub fn subresource_layout(mut self, subresource_layout: SubresourceLayout) -> Self {
            self.subresource_layout = subresource_layout;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePipelineRobustnessFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_robustness: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDevicePipelineRobustnessFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES,
                p_next: core::ptr::null_mut(),
                pipeline_robustness: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePipelineRobustnessFeatures<'a> {
        pub fn pipeline_robustness(mut self, pipeline_robustness: Bool32) -> Self {
            self.pipeline_robustness = pipeline_robustness;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineRobustnessCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub storage_buffers: PipelineRobustnessBufferBehavior,
        pub uniform_buffers: PipelineRobustnessBufferBehavior,
        pub vertex_inputs: PipelineRobustnessBufferBehavior,
        pub images: PipelineRobustnessImageBehavior,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PipelineRobustnessCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PIPELINE_ROBUSTNESS_CREATE_INFO,
                p_next: core::ptr::null(),
                storage_buffers: Default::default(),
                uniform_buffers: Default::default(),
                vertex_inputs: Default::default(),
                images: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineRobustnessCreateInfo<'a> {
        pub fn storage_buffers(
            mut self,
            storage_buffers: PipelineRobustnessBufferBehavior,
        ) -> Self {
            self.storage_buffers = storage_buffers;
            self
        }
        pub fn uniform_buffers(
            mut self,
            uniform_buffers: PipelineRobustnessBufferBehavior,
        ) -> Self {
            self.uniform_buffers = uniform_buffers;
            self
        }
        pub fn vertex_inputs(mut self, vertex_inputs: PipelineRobustnessBufferBehavior) -> Self {
            self.vertex_inputs = vertex_inputs;
            self
        }
        pub fn images(mut self, images: PipelineRobustnessImageBehavior) -> Self {
            self.images = images;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePipelineRobustnessProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub default_robustness_storage_buffers: PipelineRobustnessBufferBehavior,
        pub default_robustness_uniform_buffers: PipelineRobustnessBufferBehavior,
        pub default_robustness_vertex_inputs: PipelineRobustnessBufferBehavior,
        pub default_robustness_images: PipelineRobustnessImageBehavior,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDevicePipelineRobustnessProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES,
                p_next: core::ptr::null_mut(),
                default_robustness_storage_buffers: Default::default(),
                default_robustness_uniform_buffers: Default::default(),
                default_robustness_vertex_inputs: Default::default(),
                default_robustness_images: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePipelineRobustnessProperties<'a> {
        pub fn default_robustness_storage_buffers(
            mut self,
            default_robustness_storage_buffers: PipelineRobustnessBufferBehavior,
        ) -> Self {
            self.default_robustness_storage_buffers = default_robustness_storage_buffers;
            self
        }
        pub fn default_robustness_uniform_buffers(
            mut self,
            default_robustness_uniform_buffers: PipelineRobustnessBufferBehavior,
        ) -> Self {
            self.default_robustness_uniform_buffers = default_robustness_uniform_buffers;
            self
        }
        pub fn default_robustness_vertex_inputs(
            mut self,
            default_robustness_vertex_inputs: PipelineRobustnessBufferBehavior,
        ) -> Self {
            self.default_robustness_vertex_inputs = default_robustness_vertex_inputs;
            self
        }
        pub fn default_robustness_images(
            mut self,
            default_robustness_images: PipelineRobustnessImageBehavior,
        ) -> Self {
            self.default_robustness_images = default_robustness_images;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceImageSubresourceInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_create_info: *const ImageCreateInfo<'a>,
        pub p_subresource: *const ImageSubresource2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for DeviceImageSubresourceInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::DEVICE_IMAGE_SUBRESOURCE_INFO,
                p_next: core::ptr::null(),
                p_create_info: core::ptr::null(),
                p_subresource: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceImageSubresourceInfo<'a> {
        pub fn create_info(mut self, create_info: &'a ImageCreateInfo<'a>) -> Self {
            self.p_create_info = create_info;
            self
        }
        pub fn subresource(mut self, subresource: &'a ImageSubresource2<'a>) -> Self {
            self.p_subresource = subresource;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryMapInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: MemoryMapFlags,
        pub memory: DeviceMemory,
        pub offset: DeviceSize,
        pub size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for MemoryMapInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::MEMORY_MAP_INFO,
                p_next: core::ptr::null(),
                flags: Default::default(),
                memory: Default::default(),
                offset: Default::default(),
                size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryMapInfo<'a> {
        pub fn flags(mut self, flags: MemoryMapFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
        pub fn offset(mut self, offset: DeviceSize) -> Self {
            self.offset = offset;
            self
        }
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryUnmapInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: MemoryUnmapFlags,
        pub memory: DeviceMemory,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for MemoryUnmapInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::MEMORY_UNMAP_INFO,
                p_next: core::ptr::null(),
                flags: Default::default(),
                memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryUnmapInfo<'a> {
        pub fn flags(mut self, flags: MemoryUnmapFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BindMemoryStatus<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_result: *mut vk::Result,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for BindMemoryStatus<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::BIND_MEMORY_STATUS,
                p_next: core::ptr::null(),
                p_result: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BindMemoryStatus<'a> {
        pub fn result(mut self, result: &'a mut vk::Result) -> Self {
            self.p_result = result;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BindDescriptorSetsInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub stage_flags: ShaderStageFlags,
        pub layout: PipelineLayout,
        pub first_set: u32,
        pub descriptor_set_count: u32,
        pub p_descriptor_sets: *const DescriptorSet,
        pub dynamic_offset_count: u32,
        pub p_dynamic_offsets: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for BindDescriptorSetsInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::BIND_DESCRIPTOR_SETS_INFO,
                p_next: core::ptr::null(),
                stage_flags: Default::default(),
                layout: Default::default(),
                first_set: Default::default(),
                descriptor_set_count: Default::default(),
                p_descriptor_sets: core::ptr::null(),
                dynamic_offset_count: Default::default(),
                p_dynamic_offsets: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BindDescriptorSetsInfo<'a> {
        pub fn stage_flags(mut self, stage_flags: ShaderStageFlags) -> Self {
            self.stage_flags = stage_flags;
            self
        }
        pub fn layout(mut self, layout: PipelineLayout) -> Self {
            self.layout = layout;
            self
        }
        pub fn first_set(mut self, first_set: u32) -> Self {
            self.first_set = first_set;
            self
        }
        pub fn descriptor_sets(mut self, descriptor_sets: &'a [DescriptorSet]) -> Self {
            self.descriptor_set_count = descriptor_sets.len().try_into().unwrap();
            self.p_descriptor_sets = descriptor_sets.as_ptr();
            self
        }
        pub fn dynamic_offsets(mut self, dynamic_offsets: &'a [u32]) -> Self {
            self.dynamic_offset_count = dynamic_offsets.len().try_into().unwrap();
            self.p_dynamic_offsets = dynamic_offsets.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PushConstantsInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub layout: PipelineLayout,
        pub stage_flags: ShaderStageFlags,
        pub offset: u32,
        pub size: u32,
        pub p_values: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PushConstantsInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PUSH_CONSTANTS_INFO,
                p_next: core::ptr::null(),
                layout: Default::default(),
                stage_flags: Default::default(),
                offset: Default::default(),
                size: Default::default(),
                p_values: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PushConstantsInfo<'a> {
        pub fn layout(mut self, layout: PipelineLayout) -> Self {
            self.layout = layout;
            self
        }
        pub fn stage_flags(mut self, stage_flags: ShaderStageFlags) -> Self {
            self.stage_flags = stage_flags;
            self
        }
        pub fn offset(mut self, offset: u32) -> Self {
            self.offset = offset;
            self
        }
        pub fn values(mut self, values: &'a [u8]) -> Self {
            self.size = values.len().try_into().unwrap();
            self.p_values = values.as_ptr() as _;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PushDescriptorSetInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub stage_flags: ShaderStageFlags,
        pub layout: PipelineLayout,
        pub set: u32,
        pub descriptor_write_count: u32,
        pub p_descriptor_writes: *const WriteDescriptorSet<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PushDescriptorSetInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PUSH_DESCRIPTOR_SET_INFO,
                p_next: core::ptr::null(),
                stage_flags: Default::default(),
                layout: Default::default(),
                set: Default::default(),
                descriptor_write_count: Default::default(),
                p_descriptor_writes: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PushDescriptorSetInfo<'a> {
        pub fn stage_flags(mut self, stage_flags: ShaderStageFlags) -> Self {
            self.stage_flags = stage_flags;
            self
        }
        pub fn layout(mut self, layout: PipelineLayout) -> Self {
            self.layout = layout;
            self
        }
        pub fn set(mut self, set: u32) -> Self {
            self.set = set;
            self
        }
        pub fn descriptor_writes(
            mut self,
            descriptor_writes: &'a [WriteDescriptorSet<'a>],
        ) -> Self {
            self.descriptor_write_count = descriptor_writes.len().try_into().unwrap();
            self.p_descriptor_writes = descriptor_writes.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PushDescriptorSetWithTemplateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub descriptor_update_template: DescriptorUpdateTemplate,
        pub layout: PipelineLayout,
        pub set: u32,
        pub p_data: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PushDescriptorSetWithTemplateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO,
                p_next: core::ptr::null(),
                descriptor_update_template: Default::default(),
                layout: Default::default(),
                set: Default::default(),
                p_data: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PushDescriptorSetWithTemplateInfo<'a> {
        pub fn descriptor_update_template(
            mut self,
            descriptor_update_template: DescriptorUpdateTemplate,
        ) -> Self {
            self.descriptor_update_template = descriptor_update_template;
            self
        }
        pub fn layout(mut self, layout: PipelineLayout) -> Self {
            self.layout = layout;
            self
        }
        pub fn set(mut self, set: u32) -> Self {
            self.set = set;
            self
        }
        pub fn data(mut self, data: &'a c_void) -> Self {
            self.p_data = data;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderSubgroupRotateFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_subgroup_rotate: Bool32,
        pub shader_subgroup_rotate_clustered: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceShaderSubgroupRotateFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES,
                p_next: core::ptr::null_mut(),
                shader_subgroup_rotate: Default::default(),
                shader_subgroup_rotate_clustered: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderSubgroupRotateFeatures<'a> {
        pub fn shader_subgroup_rotate(mut self, shader_subgroup_rotate: Bool32) -> Self {
            self.shader_subgroup_rotate = shader_subgroup_rotate;
            self
        }
        pub fn shader_subgroup_rotate_clustered(
            mut self,
            shader_subgroup_rotate_clustered: Bool32,
        ) -> Self {
            self.shader_subgroup_rotate_clustered = shader_subgroup_rotate_clustered;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderExpectAssumeFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_expect_assume: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceShaderExpectAssumeFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES,
                p_next: core::ptr::null_mut(),
                shader_expect_assume: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderExpectAssumeFeatures<'a> {
        pub fn shader_expect_assume(mut self, shader_expect_assume: Bool32) -> Self {
            self.shader_expect_assume = shader_expect_assume;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderFloatControls2Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_float_controls2: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceShaderFloatControls2Features<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES,
                p_next: core::ptr::null_mut(),
                shader_float_controls2: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderFloatControls2Features<'a> {
        pub fn shader_float_controls2(mut self, shader_float_controls2: Bool32) -> Self {
            self.shader_float_controls2 = shader_float_controls2;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDynamicRenderingLocalReadFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub dynamic_rendering_local_read: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceDynamicRenderingLocalReadFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES,
                p_next: core::ptr::null_mut(),
                dynamic_rendering_local_read: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDynamicRenderingLocalReadFeatures<'a> {
        pub fn dynamic_rendering_local_read(
            mut self,
            dynamic_rendering_local_read: Bool32,
        ) -> Self {
            self.dynamic_rendering_local_read = dynamic_rendering_local_read;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderingAttachmentLocationInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub color_attachment_count: u32,
        pub p_color_attachment_locations: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for RenderingAttachmentLocationInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::RENDERING_ATTACHMENT_LOCATION_INFO,
                p_next: core::ptr::null(),
                color_attachment_count: Default::default(),
                p_color_attachment_locations: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderingAttachmentLocationInfo<'a> {
        pub fn color_attachment_locations(mut self, color_attachment_locations: &'a [u32]) -> Self {
            self.color_attachment_count = color_attachment_locations.len().try_into().unwrap();
            self.p_color_attachment_locations = color_attachment_locations.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderingInputAttachmentIndexInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub color_attachment_count: u32,
        pub p_color_attachment_input_indices: *const u32,
        pub p_depth_input_attachment_index: *const u32,
        pub p_stencil_input_attachment_index: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for RenderingInputAttachmentIndexInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::RENDERING_INPUT_ATTACHMENT_INDEX_INFO,
                p_next: core::ptr::null(),
                color_attachment_count: Default::default(),
                p_color_attachment_input_indices: core::ptr::null(),
                p_depth_input_attachment_index: core::ptr::null(),
                p_stencil_input_attachment_index: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderingInputAttachmentIndexInfo<'a> {
        pub fn color_attachment_input_indices(
            mut self,
            color_attachment_input_indices: &'a [u32],
        ) -> Self {
            self.color_attachment_count = color_attachment_input_indices.len().try_into().unwrap();
            self.p_color_attachment_input_indices = color_attachment_input_indices.as_ptr();
            self
        }
        pub fn depth_input_attachment_index(
            mut self,
            depth_input_attachment_index: &'a u32,
        ) -> Self {
            self.p_depth_input_attachment_index = depth_input_attachment_index;
            self
        }
        pub fn stencil_input_attachment_index(
            mut self,
            stencil_input_attachment_index: &'a u32,
        ) -> Self {
            self.p_stencil_input_attachment_index = stencil_input_attachment_index;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct QueueGlobalPriority(i32);
    impl QueueGlobalPriority {
        pub const LOW: Self = Self(128);
        pub const MEDIUM: Self = Self(256);
        pub const HIGH: Self = Self(512);
        pub const REALTIME: Self = Self(1024);
        pub const HIGH_EXT: Self = Self::HIGH;
        pub const HIGH_KHR: Self = Self::HIGH;
        pub const LOW_EXT: Self = Self::LOW;
        pub const LOW_KHR: Self = Self::LOW;
        pub const MEDIUM_EXT: Self = Self::MEDIUM;
        pub const MEDIUM_KHR: Self = Self::MEDIUM;
        pub const REALTIME_EXT: Self = Self::REALTIME;
        pub const REALTIME_KHR: Self = Self::REALTIME;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LineRasterizationMode(i32);
    impl LineRasterizationMode {
        pub const DEFAULT: Self = Self(0);
        pub const RECTANGULAR: Self = Self(1);
        pub const BRESENHAM: Self = Self(2);
        pub const RECTANGULAR_SMOOTH: Self = Self(3);
        pub const BRESENHAM_EXT: Self = Self::BRESENHAM;
        pub const BRESENHAM_KHR: Self = Self::BRESENHAM;
        pub const DEFAULT_EXT: Self = Self::DEFAULT;
        pub const DEFAULT_KHR: Self = Self::DEFAULT;
        pub const RECTANGULAR_EXT: Self = Self::RECTANGULAR;
        pub const RECTANGULAR_KHR: Self = Self::RECTANGULAR;
        pub const RECTANGULAR_SMOOTH_EXT: Self = Self::RECTANGULAR_SMOOTH;
        pub const RECTANGULAR_SMOOTH_KHR: Self = Self::RECTANGULAR_SMOOTH;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineRobustnessBufferBehavior(i32);
    impl PipelineRobustnessBufferBehavior {
        pub const DEVICE_DEFAULT: Self = Self(0);
        pub const DISABLED: Self = Self(1);
        pub const ROBUST_BUFFER_ACCESS: Self = Self(2);
        pub const ROBUST_BUFFER_ACCESS_2: Self = Self(3);
        pub const DEVICE_DEFAULT_EXT: Self = Self::DEVICE_DEFAULT;
        pub const DISABLED_EXT: Self = Self::DISABLED;
        pub const ROBUST_BUFFER_ACCESS_2_EXT: Self = Self::ROBUST_BUFFER_ACCESS_2;
        pub const ROBUST_BUFFER_ACCESS_EXT: Self = Self::ROBUST_BUFFER_ACCESS;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineRobustnessImageBehavior(i32);
    impl PipelineRobustnessImageBehavior {
        pub const DEVICE_DEFAULT: Self = Self(0);
        pub const DISABLED: Self = Self(1);
        pub const ROBUST_IMAGE_ACCESS: Self = Self(2);
        pub const ROBUST_IMAGE_ACCESS_2: Self = Self(3);
        pub const DEVICE_DEFAULT_EXT: Self = Self::DEVICE_DEFAULT;
        pub const DISABLED_EXT: Self = Self::DISABLED;
        pub const ROBUST_IMAGE_ACCESS_2_EXT: Self = Self::ROBUST_IMAGE_ACCESS_2;
        pub const ROBUST_IMAGE_ACCESS_EXT: Self = Self::ROBUST_IMAGE_ACCESS;
    }
    bitflags! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct MemoryUnmapFlags: Flags {
            const RESERVE_EXT = MemoryUnmapFlagBits::RESERVE_EXT.0;
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MemoryUnmapFlagBits(u32);
    impl MemoryUnmapFlagBits {
        pub const RESERVE_EXT: Self = Self(1 << 0);
    }
    bitflags! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct PipelineCreateFlags2: Flags64 {
            const DISABLE_OPTIMIZATION = PipelineCreateFlagBits2::DISABLE_OPTIMIZATION.0;
            const ALLOW_DERIVATIVES = PipelineCreateFlagBits2::ALLOW_DERIVATIVES.0;
            const DERIVATIVE = PipelineCreateFlagBits2::DERIVATIVE.0;
            const VIEW_INDEX_FROM_DEVICE_INDEX = PipelineCreateFlagBits2::VIEW_INDEX_FROM_DEVICE_INDEX.0;
            const DISPATCH_BASE = PipelineCreateFlagBits2::DISPATCH_BASE.0;
            const DEFER_COMPILE_NV = PipelineCreateFlagBits2::DEFER_COMPILE_NV.0;
            const CAPTURE_STATISTICS_KHR = PipelineCreateFlagBits2::CAPTURE_STATISTICS_KHR.0;
            const CAPTURE_INTERNAL_REPRESENTATIONS_KHR = PipelineCreateFlagBits2::CAPTURE_INTERNAL_REPRESENTATIONS_KHR.0;
            const FAIL_ON_PIPELINE_COMPILE_REQUIRED = PipelineCreateFlagBits2::FAIL_ON_PIPELINE_COMPILE_REQUIRED.0;
            const EARLY_RETURN_ON_FAILURE = PipelineCreateFlagBits2::EARLY_RETURN_ON_FAILURE.0;
            const LINK_TIME_OPTIMIZATION_EXT = PipelineCreateFlagBits2::LINK_TIME_OPTIMIZATION_EXT.0;
            const LIBRARY_KHR = PipelineCreateFlagBits2::LIBRARY_KHR.0;
            const RAY_TRACING_SKIP_TRIANGLES_KHR = PipelineCreateFlagBits2::RAY_TRACING_SKIP_TRIANGLES_KHR.0;
            const RAY_TRACING_SKIP_AABBS_KHR = PipelineCreateFlagBits2::RAY_TRACING_SKIP_AABBS_KHR.0;
            const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR = PipelineCreateFlagBits2::RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR.0;
            const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR = PipelineCreateFlagBits2::RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR.0;
            const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR = PipelineCreateFlagBits2::RAY_TRACING_NO_NULL_MISS_SHADERS_KHR.0;
            const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR = PipelineCreateFlagBits2::RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR.0;
            const INDIRECT_BINDABLE_NV = PipelineCreateFlagBits2::INDIRECT_BINDABLE_NV.0;
            const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR = PipelineCreateFlagBits2::RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR.0;
            const RAY_TRACING_ALLOW_MOTION_NV = PipelineCreateFlagBits2::RAY_TRACING_ALLOW_MOTION_NV.0;
            const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = PipelineCreateFlagBits2::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0;
            const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT = PipelineCreateFlagBits2::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT.0;
            const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT = PipelineCreateFlagBits2::RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT.0;
            const RAY_TRACING_OPACITY_MICROMAP_EXT = PipelineCreateFlagBits2::RAY_TRACING_OPACITY_MICROMAP_EXT.0;
            const COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT = PipelineCreateFlagBits2::COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT.0;
            const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT = PipelineCreateFlagBits2::DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT.0;
            const NO_PROTECTED_ACCESS = PipelineCreateFlagBits2::NO_PROTECTED_ACCESS.0;
            const RAY_TRACING_DISPLACEMENT_MICROMAP_NV = PipelineCreateFlagBits2::RAY_TRACING_DISPLACEMENT_MICROMAP_NV.0;
            const DESCRIPTOR_BUFFER_EXT = PipelineCreateFlagBits2::DESCRIPTOR_BUFFER_EXT.0;
            const PROTECTED_ACCESS_ONLY = PipelineCreateFlagBits2::PROTECTED_ACCESS_ONLY.0;
            const CAPTURE_DATA_KHR = PipelineCreateFlagBits2::CAPTURE_DATA_KHR.0;
            const EXECUTION_GRAPH_AMDX = PipelineCreateFlagBits2::EXECUTION_GRAPH_AMDX.0;
            const RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV = PipelineCreateFlagBits2::RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV.0;
            const ENABLE_LEGACY_DITHERING_EXT = PipelineCreateFlagBits2::ENABLE_LEGACY_DITHERING_EXT.0;
            const DESCRIPTOR_HEAP_EXT = PipelineCreateFlagBits2::DESCRIPTOR_HEAP_EXT.0;
            const DISALLOW_OPACITY_MICROMAP_ARM = PipelineCreateFlagBits2::DISALLOW_OPACITY_MICROMAP_ARM.0;
            const INDIRECT_BINDABLE_EXT = PipelineCreateFlagBits2::INDIRECT_BINDABLE_EXT.0;
            const PER_LAYER_FRAGMENT_DENSITY_VALVE = PipelineCreateFlagBits2::PER_LAYER_FRAGMENT_DENSITY_VALVE.0;
            const _64_INDEXING_EXT = PipelineCreateFlagBits2::_64_INDEXING_EXT.0;
            const ALLOW_DERIVATIVES_KHR = Self::ALLOW_DERIVATIVES.bits();
            const DERIVATIVE_KHR = Self::DERIVATIVE.bits();
            const DISABLE_OPTIMIZATION_KHR = Self::DISABLE_OPTIMIZATION.bits();
            const DISPATCH_BASE_KHR = Self::DISPATCH_BASE.bits();
            const EARLY_RETURN_ON_FAILURE_KHR = Self::EARLY_RETURN_ON_FAILURE.bits();
            const FAIL_ON_PIPELINE_COMPILE_REQUIRED_KHR = Self::FAIL_ON_PIPELINE_COMPILE_REQUIRED.bits();
            const NO_PROTECTED_ACCESS_EXT = Self::NO_PROTECTED_ACCESS.bits();
            const PROTECTED_ACCESS_ONLY_EXT = Self::PROTECTED_ACCESS_ONLY.bits();
            const RAY_TRACING_SKIP_BUILT_IN_PRIMITIVES_KHR = Self::RAY_TRACING_SKIP_TRIANGLES_KHR.bits();
            const VIEW_INDEX_FROM_DEVICE_INDEX_KHR = Self::VIEW_INDEX_FROM_DEVICE_INDEX.bits();
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineCreateFlagBits2(u64);
    impl PipelineCreateFlagBits2 {
        pub const DISABLE_OPTIMIZATION: Self = Self(1 << 0);
        pub const ALLOW_DERIVATIVES: Self = Self(1 << 1);
        pub const DERIVATIVE: Self = Self(1 << 2);
        pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self(1 << 3);
        pub const DISPATCH_BASE: Self = Self(1 << 4);
        pub const DEFER_COMPILE_NV: Self = Self(1 << 5);
        pub const CAPTURE_STATISTICS_KHR: Self = Self(1 << 6);
        pub const CAPTURE_INTERNAL_REPRESENTATIONS_KHR: Self = Self(1 << 7);
        pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self = Self(1 << 8);
        pub const EARLY_RETURN_ON_FAILURE: Self = Self(1 << 9);
        pub const LINK_TIME_OPTIMIZATION_EXT: Self = Self(1 << 10);
        pub const LIBRARY_KHR: Self = Self(1 << 11);
        pub const RAY_TRACING_SKIP_TRIANGLES_KHR: Self = Self(1 << 12);
        pub const RAY_TRACING_SKIP_AABBS_KHR: Self = Self(1 << 13);
        pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR: Self = Self(1 << 14);
        pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR: Self = Self(1 << 15);
        pub const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR: Self = Self(1 << 16);
        pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR: Self = Self(1 << 17);
        pub const INDIRECT_BINDABLE_NV: Self = Self(1 << 18);
        pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR: Self = Self(1 << 19);
        pub const RAY_TRACING_ALLOW_MOTION_NV: Self = Self(1 << 20);
        pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 21);
        pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self = Self(1 << 22);
        pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT: Self = Self(1 << 23);
        pub const RAY_TRACING_OPACITY_MICROMAP_EXT: Self = Self(1 << 24);
        pub const COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(1 << 25);
        pub const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(1 << 26);
        pub const NO_PROTECTED_ACCESS: Self = Self(1 << 27);
        pub const RAY_TRACING_DISPLACEMENT_MICROMAP_NV: Self = Self(1 << 28);
        pub const DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 29);
        pub const PROTECTED_ACCESS_ONLY: Self = Self(1 << 30);
        pub const CAPTURE_DATA_KHR: Self = Self(1 << 31);
        pub const EXECUTION_GRAPH_AMDX: Self = Self(1 << 32);
        pub const RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV: Self = Self(1 << 33);
        pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(1 << 34);
        pub const DESCRIPTOR_HEAP_EXT: Self = Self(1 << 36);
        pub const DISALLOW_OPACITY_MICROMAP_ARM: Self = Self(1 << 37);
        pub const INDIRECT_BINDABLE_EXT: Self = Self(1 << 38);
        pub const PER_LAYER_FRAGMENT_DENSITY_VALVE: Self = Self(1 << 40);
        pub const _64_INDEXING_EXT: Self = Self(1 << 43);
        pub const ALLOW_DERIVATIVES_KHR: Self = Self::ALLOW_DERIVATIVES;
        pub const DERIVATIVE_KHR: Self = Self::DERIVATIVE;
        pub const DISABLE_OPTIMIZATION_KHR: Self = Self::DISABLE_OPTIMIZATION;
        pub const DISPATCH_BASE_KHR: Self = Self::DISPATCH_BASE;
        pub const EARLY_RETURN_ON_FAILURE_KHR: Self = Self::EARLY_RETURN_ON_FAILURE;
        pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_KHR: Self =
            Self::FAIL_ON_PIPELINE_COMPILE_REQUIRED;
        pub const NO_PROTECTED_ACCESS_EXT: Self = Self::NO_PROTECTED_ACCESS;
        pub const PROTECTED_ACCESS_ONLY_EXT: Self = Self::PROTECTED_ACCESS_ONLY;
        pub const RAY_TRACING_SKIP_BUILT_IN_PRIMITIVES_KHR: Self =
            Self::RAY_TRACING_SKIP_TRIANGLES_KHR;
        pub const VIEW_INDEX_FROM_DEVICE_INDEX_KHR: Self = Self::VIEW_INDEX_FROM_DEVICE_INDEX;
    }
    bitflags! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct BufferUsageFlags2: Flags64 {
            const TRANSFER_SRC = BufferUsageFlagBits2::TRANSFER_SRC.0;
            const TRANSFER_DST = BufferUsageFlagBits2::TRANSFER_DST.0;
            const UNIFORM_TEXEL_BUFFER = BufferUsageFlagBits2::UNIFORM_TEXEL_BUFFER.0;
            const STORAGE_TEXEL_BUFFER = BufferUsageFlagBits2::STORAGE_TEXEL_BUFFER.0;
            const UNIFORM_BUFFER = BufferUsageFlagBits2::UNIFORM_BUFFER.0;
            const STORAGE_BUFFER = BufferUsageFlagBits2::STORAGE_BUFFER.0;
            const INDEX_BUFFER = BufferUsageFlagBits2::INDEX_BUFFER.0;
            const VERTEX_BUFFER = BufferUsageFlagBits2::VERTEX_BUFFER.0;
            const INDIRECT_BUFFER = BufferUsageFlagBits2::INDIRECT_BUFFER.0;
            const CONDITIONAL_RENDERING_EXT = BufferUsageFlagBits2::CONDITIONAL_RENDERING_EXT.0;
            const SHADER_BINDING_TABLE_KHR = BufferUsageFlagBits2::SHADER_BINDING_TABLE_KHR.0;
            const TRANSFORM_FEEDBACK_BUFFER_EXT = BufferUsageFlagBits2::TRANSFORM_FEEDBACK_BUFFER_EXT.0;
            const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT = BufferUsageFlagBits2::TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT.0;
            const VIDEO_DECODE_SRC_KHR = BufferUsageFlagBits2::VIDEO_DECODE_SRC_KHR.0;
            const VIDEO_DECODE_DST_KHR = BufferUsageFlagBits2::VIDEO_DECODE_DST_KHR.0;
            const VIDEO_ENCODE_DST_KHR = BufferUsageFlagBits2::VIDEO_ENCODE_DST_KHR.0;
            const VIDEO_ENCODE_SRC_KHR = BufferUsageFlagBits2::VIDEO_ENCODE_SRC_KHR.0;
            const SHADER_DEVICE_ADDRESS = BufferUsageFlagBits2::SHADER_DEVICE_ADDRESS.0;
            const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR = BufferUsageFlagBits2::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR.0;
            const ACCELERATION_STRUCTURE_STORAGE_KHR = BufferUsageFlagBits2::ACCELERATION_STRUCTURE_STORAGE_KHR.0;
            const SAMPLER_DESCRIPTOR_BUFFER_EXT = BufferUsageFlagBits2::SAMPLER_DESCRIPTOR_BUFFER_EXT.0;
            const RESOURCE_DESCRIPTOR_BUFFER_EXT = BufferUsageFlagBits2::RESOURCE_DESCRIPTOR_BUFFER_EXT.0;
            const MICROMAP_BUILD_INPUT_READ_ONLY_EXT = BufferUsageFlagBits2::MICROMAP_BUILD_INPUT_READ_ONLY_EXT.0;
            const MICROMAP_STORAGE_EXT = BufferUsageFlagBits2::MICROMAP_STORAGE_EXT.0;
            const EXECUTION_GRAPH_SCRATCH_AMDX = BufferUsageFlagBits2::EXECUTION_GRAPH_SCRATCH_AMDX.0;
            const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT = BufferUsageFlagBits2::PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT.0;
            const TILE_MEMORY_QCOM = BufferUsageFlagBits2::TILE_MEMORY_QCOM.0;
            const DESCRIPTOR_HEAP_EXT = BufferUsageFlagBits2::DESCRIPTOR_HEAP_EXT.0;
            const DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM = BufferUsageFlagBits2::DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM.0;
            const PREPROCESS_BUFFER_EXT = BufferUsageFlagBits2::PREPROCESS_BUFFER_EXT.0;
            const MEMORY_DECOMPRESSION_EXT = BufferUsageFlagBits2::MEMORY_DECOMPRESSION_EXT.0;
            const COMPRESSED_DATA_DGF1_AMDX = BufferUsageFlagBits2::COMPRESSED_DATA_DGF1_AMDX.0;
            const INDEX_BUFFER_KHR = Self::INDEX_BUFFER.bits();
            const INDIRECT_BUFFER_KHR = Self::INDIRECT_BUFFER.bits();
            const RAY_TRACING_NV = Self::SHADER_BINDING_TABLE_KHR.bits();
            const SHADER_DEVICE_ADDRESS_KHR = Self::SHADER_DEVICE_ADDRESS.bits();
            const STORAGE_BUFFER_KHR = Self::STORAGE_BUFFER.bits();
            const STORAGE_TEXEL_BUFFER_KHR = Self::STORAGE_TEXEL_BUFFER.bits();
            const TRANSFER_DST_KHR = Self::TRANSFER_DST.bits();
            const TRANSFER_SRC_KHR = Self::TRANSFER_SRC.bits();
            const UNIFORM_BUFFER_KHR = Self::UNIFORM_BUFFER.bits();
            const UNIFORM_TEXEL_BUFFER_KHR = Self::UNIFORM_TEXEL_BUFFER.bits();
            const VERTEX_BUFFER_KHR = Self::VERTEX_BUFFER.bits();
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BufferUsageFlagBits2(u64);
    impl BufferUsageFlagBits2 {
        pub const TRANSFER_SRC: Self = Self(1 << 0);
        pub const TRANSFER_DST: Self = Self(1 << 1);
        pub const UNIFORM_TEXEL_BUFFER: Self = Self(1 << 2);
        pub const STORAGE_TEXEL_BUFFER: Self = Self(1 << 3);
        pub const UNIFORM_BUFFER: Self = Self(1 << 4);
        pub const STORAGE_BUFFER: Self = Self(1 << 5);
        pub const INDEX_BUFFER: Self = Self(1 << 6);
        pub const VERTEX_BUFFER: Self = Self(1 << 7);
        pub const INDIRECT_BUFFER: Self = Self(1 << 8);
        pub const CONDITIONAL_RENDERING_EXT: Self = Self(1 << 9);
        pub const SHADER_BINDING_TABLE_KHR: Self = Self(1 << 10);
        pub const TRANSFORM_FEEDBACK_BUFFER_EXT: Self = Self(1 << 11);
        pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT: Self = Self(1 << 12);
        pub const VIDEO_DECODE_SRC_KHR: Self = Self(1 << 13);
        pub const VIDEO_DECODE_DST_KHR: Self = Self(1 << 14);
        pub const VIDEO_ENCODE_DST_KHR: Self = Self(1 << 15);
        pub const VIDEO_ENCODE_SRC_KHR: Self = Self(1 << 16);
        pub const SHADER_DEVICE_ADDRESS: Self = Self(1 << 17);
        pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR: Self = Self(1 << 19);
        pub const ACCELERATION_STRUCTURE_STORAGE_KHR: Self = Self(1 << 20);
        pub const SAMPLER_DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 21);
        pub const RESOURCE_DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 22);
        pub const MICROMAP_BUILD_INPUT_READ_ONLY_EXT: Self = Self(1 << 23);
        pub const MICROMAP_STORAGE_EXT: Self = Self(1 << 24);
        pub const EXECUTION_GRAPH_SCRATCH_AMDX: Self = Self(1 << 25);
        pub const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 26);
        pub const TILE_MEMORY_QCOM: Self = Self(1 << 27);
        pub const DESCRIPTOR_HEAP_EXT: Self = Self(1 << 28);
        pub const DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM: Self = Self(1 << 29);
        pub const PREPROCESS_BUFFER_EXT: Self = Self(1 << 31);
        pub const MEMORY_DECOMPRESSION_EXT: Self = Self(1 << 32);
        pub const COMPRESSED_DATA_DGF1_AMDX: Self = Self(1 << 33);
        pub const INDEX_BUFFER_KHR: Self = Self::INDEX_BUFFER;
        pub const INDIRECT_BUFFER_KHR: Self = Self::INDIRECT_BUFFER;
        pub const RAY_TRACING_NV: Self = Self::SHADER_BINDING_TABLE_KHR;
        pub const SHADER_DEVICE_ADDRESS_KHR: Self = Self::SHADER_DEVICE_ADDRESS;
        pub const STORAGE_BUFFER_KHR: Self = Self::STORAGE_BUFFER;
        pub const STORAGE_TEXEL_BUFFER_KHR: Self = Self::STORAGE_TEXEL_BUFFER;
        pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
        pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
        pub const UNIFORM_BUFFER_KHR: Self = Self::UNIFORM_BUFFER;
        pub const UNIFORM_TEXEL_BUFFER_KHR: Self = Self::UNIFORM_TEXEL_BUFFER;
        pub const VERTEX_BUFFER_KHR: Self = Self::VERTEX_BUFFER;
    }
    bitflags! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct HostImageCopyFlags: Flags {
            const MEMCPY = HostImageCopyFlagBits::MEMCPY.0;
            const MEMCPY_EXT = Self::MEMCPY.bits();
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct HostImageCopyFlagBits(u32);
    impl HostImageCopyFlagBits {
        pub const MEMCPY: Self = Self(1 << 0);
        pub const MEMCPY_EXT: Self = Self::MEMCPY;
    }
    pub type PFN_vkGetRenderingAreaGranularity = unsafe extern "system" fn(
        device: Device,
        p_rendering_area_info: *const RenderingAreaInfo<'_>,
        p_granularity: *mut Extent2D,
    );
    pub type PFN_vkCmdPushDescriptorSet = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet<'_>,
    );
    pub type PFN_vkCmdPushDescriptorSetWithTemplate = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        p_data: *const c_void,
    );
    pub type PFN_vkCmdSetLineStipple = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    );
    pub type PFN_vkCmdBindIndexBuffer2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    );
    pub type PFN_vkCopyMemoryToImage = unsafe extern "system" fn(
        device: Device,
        p_copy_memory_to_image_info: *const CopyMemoryToImageInfo<'_>,
    ) -> vk::Result;
    pub type PFN_vkCopyImageToMemory = unsafe extern "system" fn(
        device: Device,
        p_copy_image_to_memory_info: *const CopyImageToMemoryInfo<'_>,
    ) -> vk::Result;
    pub type PFN_vkCopyImageToImage = unsafe extern "system" fn(
        device: Device,
        p_copy_image_to_image_info: *const CopyImageToImageInfo<'_>,
    ) -> vk::Result;
    pub type PFN_vkTransitionImageLayout = unsafe extern "system" fn(
        device: Device,
        transition_count: u32,
        p_transitions: *const HostImageLayoutTransitionInfo<'_>,
    ) -> vk::Result;
    pub type PFN_vkGetImageSubresourceLayout2 = unsafe extern "system" fn(
        device: Device,
        image: Image,
        p_subresource: *const ImageSubresource2<'_>,
        p_layout: *mut SubresourceLayout2<'_>,
    );
    pub type PFN_vkGetDeviceImageSubresourceLayout = unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceImageSubresourceInfo<'_>,
        p_layout: *mut SubresourceLayout2<'_>,
    );
    pub type PFN_vkMapMemory2 = unsafe extern "system" fn(
        device: Device,
        p_memory_map_info: *const MemoryMapInfo<'_>,
        pp_data: *mut *mut c_void,
    ) -> vk::Result;
    pub type PFN_vkUnmapMemory2 = unsafe extern "system" fn(
        device: Device,
        p_memory_unmap_info: *const MemoryUnmapInfo<'_>,
    ) -> vk::Result;
    pub type PFN_vkCmdBindDescriptorSets2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_bind_descriptor_sets_info: *const BindDescriptorSetsInfo<'_>,
    );
    pub type PFN_vkCmdPushConstants2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_push_constants_info: *const PushConstantsInfo<'_>,
    );
    pub type PFN_vkCmdPushDescriptorSet2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_push_descriptor_set_info: *const PushDescriptorSetInfo<'_>,
    );
    pub type PFN_vkCmdPushDescriptorSetWithTemplate2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo<'_>,
    );
    pub type PFN_vkCmdSetRenderingAttachmentLocations = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_location_info: *const RenderingAttachmentLocationInfo<'_>,
    );
    pub type PFN_vkCmdSetRenderingInputAttachmentIndices = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_input_attachment_index_info: *const RenderingInputAttachmentIndexInfo<'_>,
    );
}
pub struct DeviceFn {
    map_memory2: PFN_vkMapMemory2,
    unmap_memory2: PFN_vkUnmapMemory2,
    get_device_image_subresource_layout: PFN_vkGetDeviceImageSubresourceLayout,
    get_image_subresource_layout2: PFN_vkGetImageSubresourceLayout2,
    copy_memory_to_image: PFN_vkCopyMemoryToImage,
    copy_image_to_memory: PFN_vkCopyImageToMemory,
    copy_image_to_image: PFN_vkCopyImageToImage,
    transition_image_layout: PFN_vkTransitionImageLayout,
    cmd_push_descriptor_set: PFN_vkCmdPushDescriptorSet,
    cmd_push_descriptor_set_with_template: PFN_vkCmdPushDescriptorSetWithTemplate,
    cmd_bind_descriptor_sets2: PFN_vkCmdBindDescriptorSets2,
    cmd_push_constants2: PFN_vkCmdPushConstants2,
    cmd_push_descriptor_set2: PFN_vkCmdPushDescriptorSet2,
    cmd_push_descriptor_set_with_template2: PFN_vkCmdPushDescriptorSetWithTemplate2,
    cmd_set_line_stipple: PFN_vkCmdSetLineStipple,
    cmd_bind_index_buffer2: PFN_vkCmdBindIndexBuffer2,
    get_rendering_area_granularity: PFN_vkGetRenderingAreaGranularity,
    cmd_set_rendering_attachment_locations: PFN_vkCmdSetRenderingAttachmentLocations,
    cmd_set_rendering_input_attachment_indices: PFN_vkCmdSetRenderingInputAttachmentIndices,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                map_memory2: transmute(load(c"vkMapMemory2").ok_or(LoadingError)?),
                unmap_memory2: transmute(load(c"vkUnmapMemory2").ok_or(LoadingError)?),
                get_device_image_subresource_layout: transmute(
                    load(c"vkGetDeviceImageSubresourceLayout").ok_or(LoadingError)?,
                ),
                get_image_subresource_layout2: transmute(
                    load(c"vkGetImageSubresourceLayout2").ok_or(LoadingError)?,
                ),
                copy_memory_to_image: transmute(load(c"vkCopyMemoryToImage").ok_or(LoadingError)?),
                copy_image_to_memory: transmute(load(c"vkCopyImageToMemory").ok_or(LoadingError)?),
                copy_image_to_image: transmute(load(c"vkCopyImageToImage").ok_or(LoadingError)?),
                transition_image_layout: transmute(
                    load(c"vkTransitionImageLayout").ok_or(LoadingError)?,
                ),
                cmd_push_descriptor_set: transmute(
                    load(c"vkCmdPushDescriptorSet").ok_or(LoadingError)?,
                ),
                cmd_push_descriptor_set_with_template: transmute(
                    load(c"vkCmdPushDescriptorSetWithTemplate").ok_or(LoadingError)?,
                ),
                cmd_bind_descriptor_sets2: transmute(
                    load(c"vkCmdBindDescriptorSets2").ok_or(LoadingError)?,
                ),
                cmd_push_constants2: transmute(load(c"vkCmdPushConstants2").ok_or(LoadingError)?),
                cmd_push_descriptor_set2: transmute(
                    load(c"vkCmdPushDescriptorSet2").ok_or(LoadingError)?,
                ),
                cmd_push_descriptor_set_with_template2: transmute(
                    load(c"vkCmdPushDescriptorSetWithTemplate2").ok_or(LoadingError)?,
                ),
                cmd_set_line_stipple: transmute(load(c"vkCmdSetLineStipple").ok_or(LoadingError)?),
                cmd_bind_index_buffer2: transmute(
                    load(c"vkCmdBindIndexBuffer2").ok_or(LoadingError)?,
                ),
                get_rendering_area_granularity: transmute(
                    load(c"vkGetRenderingAreaGranularity").ok_or(LoadingError)?,
                ),
                cmd_set_rendering_attachment_locations: transmute(
                    load(c"vkCmdSetRenderingAttachmentLocations").ok_or(LoadingError)?,
                ),
                cmd_set_rendering_input_attachment_indices: transmute(
                    load(c"vkCmdSetRenderingInputAttachmentIndices").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn map_memory2(
        &self,
        device: Device,
        memory_map_info: &MemoryMapInfo<'_>,
        data: &mut *mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.map_memory2)(device, memory_map_info, data);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn unmap_memory2(
        &self,
        device: Device,
        memory_unmap_info: &MemoryUnmapInfo<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.unmap_memory2)(device, memory_unmap_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_device_image_subresource_layout(
        &self,
        device: Device,
        info: &DeviceImageSubresourceInfo<'_>,
    ) -> SubresourceLayout2<'_> {
        unsafe {
            let mut layout = core::mem::MaybeUninit::uninit();
            (self.get_device_image_subresource_layout)(device, info, layout.as_mut_ptr());
            layout.assume_init()
        }
    }
    pub unsafe fn get_image_subresource_layout2(
        &self,
        device: Device,
        image: Image,
        subresource: &ImageSubresource2<'_>,
    ) -> SubresourceLayout2<'_> {
        unsafe {
            let mut layout = core::mem::MaybeUninit::uninit();
            (self.get_image_subresource_layout2)(device, image, subresource, layout.as_mut_ptr());
            layout.assume_init()
        }
    }
    pub unsafe fn copy_memory_to_image(
        &self,
        device: Device,
        copy_memory_to_image_info: &CopyMemoryToImageInfo<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.copy_memory_to_image)(device, copy_memory_to_image_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn copy_image_to_memory(
        &self,
        device: Device,
        copy_image_to_memory_info: &CopyImageToMemoryInfo<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.copy_image_to_memory)(device, copy_image_to_memory_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn copy_image_to_image(
        &self,
        device: Device,
        copy_image_to_image_info: &CopyImageToImageInfo<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.copy_image_to_image)(device, copy_image_to_image_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn transition_image_layout(
        &self,
        device: Device,
        transitions: &[HostImageLayoutTransitionInfo<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.transition_image_layout)(
                device,
                transitions.len().try_into().unwrap(),
                transitions.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn cmd_push_descriptor_set(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_writes: &[WriteDescriptorSet<'_>],
    ) {
        unsafe {
            (self.cmd_push_descriptor_set)(
                command_buffer,
                pipeline_bind_point,
                layout,
                set,
                descriptor_writes.len().try_into().unwrap(),
                descriptor_writes.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_push_descriptor_set_with_template(
        &self,
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        data: &c_void,
    ) {
        unsafe {
            (self.cmd_push_descriptor_set_with_template)(
                command_buffer,
                descriptor_update_template,
                layout,
                set,
                data,
            )
        }
    }
    pub unsafe fn cmd_bind_descriptor_sets2(
        &self,
        command_buffer: CommandBuffer,
        bind_descriptor_sets_info: &BindDescriptorSetsInfo<'_>,
    ) {
        unsafe { (self.cmd_bind_descriptor_sets2)(command_buffer, bind_descriptor_sets_info) }
    }
    pub unsafe fn cmd_push_constants2(
        &self,
        command_buffer: CommandBuffer,
        push_constants_info: &PushConstantsInfo<'_>,
    ) {
        unsafe { (self.cmd_push_constants2)(command_buffer, push_constants_info) }
    }
    pub unsafe fn cmd_push_descriptor_set2(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_info: &PushDescriptorSetInfo<'_>,
    ) {
        unsafe { (self.cmd_push_descriptor_set2)(command_buffer, push_descriptor_set_info) }
    }
    pub unsafe fn cmd_push_descriptor_set_with_template2(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo<'_>,
    ) {
        unsafe {
            (self.cmd_push_descriptor_set_with_template2)(
                command_buffer,
                push_descriptor_set_with_template_info,
            )
        }
    }
    pub unsafe fn cmd_set_line_stipple(
        &self,
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        unsafe {
            (self.cmd_set_line_stipple)(command_buffer, line_stipple_factor, line_stipple_pattern)
        }
    }
    pub unsafe fn cmd_bind_index_buffer2(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ) {
        unsafe { (self.cmd_bind_index_buffer2)(command_buffer, buffer, offset, size, index_type) }
    }
    pub unsafe fn get_rendering_area_granularity(
        &self,
        device: Device,
        rendering_area_info: &RenderingAreaInfo<'_>,
    ) -> Extent2D {
        unsafe {
            let mut granularity = core::mem::MaybeUninit::uninit();
            (self.get_rendering_area_granularity)(
                device,
                rendering_area_info,
                granularity.as_mut_ptr(),
            );
            granularity.assume_init()
        }
    }
    pub unsafe fn cmd_set_rendering_attachment_locations(
        &self,
        command_buffer: CommandBuffer,
        location_info: &RenderingAttachmentLocationInfo<'_>,
    ) {
        unsafe { (self.cmd_set_rendering_attachment_locations)(command_buffer, location_info) }
    }
    pub unsafe fn cmd_set_rendering_input_attachment_indices(
        &self,
        command_buffer: CommandBuffer,
        input_attachment_index_info: &RenderingInputAttachmentIndexInfo<'_>,
    ) {
        unsafe {
            (self.cmd_set_rendering_input_attachment_indices)(
                command_buffer,
                input_attachment_index_info,
            )
        }
    }
}

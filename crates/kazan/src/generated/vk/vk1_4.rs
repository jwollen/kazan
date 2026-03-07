#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    pub const MAX_GLOBAL_PRIORITY_SIZE: u32 = 16;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferUsageFlags2CreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BufferUsageFlags2CreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub usage: BufferUsageFlags2,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BufferUsageFlags2CreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BufferUsageFlags2CreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("usage", &self.usage)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BufferUsageFlags2CreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_USAGE_FLAGS_2_CREATE_INFO;
    }

    unsafe impl<'a> Extends<BufferViewCreateInfo<'a>> for BufferUsageFlags2CreateInfo<'a> {}
    unsafe impl<'a> Extends<BufferCreateInfo<'a>> for BufferUsageFlags2CreateInfo<'a> {}
    unsafe impl<'a> Extends<PhysicalDeviceExternalBufferInfo<'a>> for BufferUsageFlags2CreateInfo<'a> {}
    unsafe impl<'a> Extends<DescriptorBufferBindingInfoEXT<'a>> for BufferUsageFlags2CreateInfo<'a> {}

    impl Default for BufferUsageFlags2CreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCreateFlags2CreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineCreateFlags2CreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineCreateFlags2,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineCreateFlags2CreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineCreateFlags2CreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineCreateFlags2CreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_CREATE_FLAGS_2_CREATE_INFO;
    }

    unsafe impl<'a> Extends<ComputePipelineCreateInfo<'a>> for PipelineCreateFlags2CreateInfo<'a> {}
    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>> for PipelineCreateFlags2CreateInfo<'a> {}
    unsafe impl<'a> Extends<RayTracingPipelineCreateInfoNV<'a>> for PipelineCreateFlags2CreateInfo<'a> {}
    unsafe impl<'a> Extends<RayTracingPipelineCreateInfoKHR<'a>>
        for PipelineCreateFlags2CreateInfo<'a>
    {
    }

    impl Default for PipelineCreateFlags2CreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePushDescriptorProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePushDescriptorProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_push_descriptors: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePushDescriptorProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePushDescriptorProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_push_descriptors", &self.max_push_descriptors)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePushDescriptorProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDevicePushDescriptorProperties<'a>
    {
    }

    impl Default for PhysicalDevicePushDescriptorProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance5Features.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMaintenance5Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub maintenance5: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMaintenance5Features<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMaintenance5Features")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("maintenance5", &self.maintenance5)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance5Features<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceMaintenance5Features<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceMaintenance5Features<'a> {}

    impl Default for PhysicalDeviceMaintenance5Features<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                maintenance5: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMaintenance5Features<'a> {
        pub fn maintenance5(mut self, maintenance5: bool) -> Self {
            self.maintenance5 = maintenance5.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance5Properties.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMaintenance5Properties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMaintenance5Properties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "early_fragment_multisample_coverage_after_sample_counting",
                    &self.early_fragment_multisample_coverage_after_sample_counting,
                )
                .field(
                    "early_fragment_sample_mask_test_before_sample_counting",
                    &self.early_fragment_sample_mask_test_before_sample_counting,
                )
                .field(
                    "depth_stencil_swizzle_one_support",
                    &self.depth_stencil_swizzle_one_support,
                )
                .field("polygon_mode_point_size", &self.polygon_mode_point_size)
                .field(
                    "non_strict_single_pixel_wide_lines_use_parallelogram",
                    &self.non_strict_single_pixel_wide_lines_use_parallelogram,
                )
                .field(
                    "non_strict_wide_lines_use_parallelogram",
                    &self.non_strict_wide_lines_use_parallelogram,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance5Properties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceMaintenance5Properties<'a>
    {
    }

    impl Default for PhysicalDeviceMaintenance5Properties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
            early_fragment_multisample_coverage_after_sample_counting: bool,
        ) -> Self {
            self.early_fragment_multisample_coverage_after_sample_counting =
                early_fragment_multisample_coverage_after_sample_counting.into();
            self
        }

        pub fn early_fragment_sample_mask_test_before_sample_counting(
            mut self,
            early_fragment_sample_mask_test_before_sample_counting: bool,
        ) -> Self {
            self.early_fragment_sample_mask_test_before_sample_counting =
                early_fragment_sample_mask_test_before_sample_counting.into();
            self
        }

        pub fn depth_stencil_swizzle_one_support(
            mut self,
            depth_stencil_swizzle_one_support: bool,
        ) -> Self {
            self.depth_stencil_swizzle_one_support = depth_stencil_swizzle_one_support.into();
            self
        }

        pub fn polygon_mode_point_size(mut self, polygon_mode_point_size: bool) -> Self {
            self.polygon_mode_point_size = polygon_mode_point_size.into();
            self
        }

        pub fn non_strict_single_pixel_wide_lines_use_parallelogram(
            mut self,
            non_strict_single_pixel_wide_lines_use_parallelogram: bool,
        ) -> Self {
            self.non_strict_single_pixel_wide_lines_use_parallelogram =
                non_strict_single_pixel_wide_lines_use_parallelogram.into();
            self
        }

        pub fn non_strict_wide_lines_use_parallelogram(
            mut self,
            non_strict_wide_lines_use_parallelogram: bool,
        ) -> Self {
            self.non_strict_wide_lines_use_parallelogram =
                non_strict_wide_lines_use_parallelogram.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance6Features.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMaintenance6Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub maintenance6: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMaintenance6Features<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMaintenance6Features")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("maintenance6", &self.maintenance6)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance6Features<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceMaintenance6Features<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceMaintenance6Features<'a> {}

    impl Default for PhysicalDeviceMaintenance6Features<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                maintenance6: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMaintenance6Features<'a> {
        pub fn maintenance6(mut self, maintenance6: bool) -> Self {
            self.maintenance6 = maintenance6.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance6Properties.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMaintenance6Properties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMaintenance6Properties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "block_texel_view_compatible_multiple_layers",
                    &self.block_texel_view_compatible_multiple_layers,
                )
                .field(
                    "max_combined_image_sampler_descriptor_count",
                    &self.max_combined_image_sampler_descriptor_count,
                )
                .field(
                    "fragment_shading_rate_clamp_combiner_inputs",
                    &self.fragment_shading_rate_clamp_combiner_inputs,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance6Properties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceMaintenance6Properties<'a>
    {
    }

    impl Default for PhysicalDeviceMaintenance6Properties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
            block_texel_view_compatible_multiple_layers: bool,
        ) -> Self {
            self.block_texel_view_compatible_multiple_layers =
                block_texel_view_compatible_multiple_layers.into();
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
            fragment_shading_rate_clamp_combiner_inputs: bool,
        ) -> Self {
            self.fragment_shading_rate_clamp_combiner_inputs =
                fragment_shading_rate_clamp_combiner_inputs.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingAreaInfo.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderingAreaInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderingAreaInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("view_mask", &self.view_mask)
                .field("color_attachment_count", &self.color_attachment_count)
                .field(
                    "p_color_attachment_formats",
                    &self.p_color_attachment_formats,
                )
                .field("depth_attachment_format", &self.depth_attachment_format)
                .field("stencil_attachment_format", &self.stencil_attachment_format)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderingAreaInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDERING_AREA_INFO;
    }

    impl Default for RenderingAreaInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceQueueGlobalPriorityCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceQueueGlobalPriorityCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub global_priority: QueueGlobalPriority,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceQueueGlobalPriorityCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceQueueGlobalPriorityCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("global_priority", &self.global_priority)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceQueueGlobalPriorityCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO;
    }

    unsafe impl<'a> Extends<DeviceQueueCreateInfo<'a>> for DeviceQueueGlobalPriorityCreateInfo<'a> {}

    impl Default for DeviceQueueGlobalPriorityCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceGlobalPriorityQueryFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceGlobalPriorityQueryFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub global_priority_query: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceGlobalPriorityQueryFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceGlobalPriorityQueryFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("global_priority_query", &self.global_priority_query)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceGlobalPriorityQueryFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceGlobalPriorityQueryFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceGlobalPriorityQueryFeatures<'a> {}

    impl Default for PhysicalDeviceGlobalPriorityQueryFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                global_priority_query: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceGlobalPriorityQueryFeatures<'a> {
        pub fn global_priority_query(mut self, global_priority_query: bool) -> Self {
            self.global_priority_query = global_priority_query.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueFamilyGlobalPriorityProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct QueueFamilyGlobalPriorityProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub priority_count: u32,
        pub priorities: [QueueGlobalPriority; MAX_GLOBAL_PRIORITY_SIZE as usize],
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for QueueFamilyGlobalPriorityProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("QueueFamilyGlobalPriorityProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("priority_count", &self.priority_count)
                .field("priorities", &self.priorities)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for QueueFamilyGlobalPriorityProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES;
    }

    unsafe impl<'a> Extends<QueueFamilyProperties2<'a>> for QueueFamilyGlobalPriorityProperties<'a> {}

    impl Default for QueueFamilyGlobalPriorityProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVertexInputBindingDivisorDescription.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineVertexInputDivisorStateCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineVertexInputDivisorStateCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub vertex_binding_divisor_count: u32,
        pub p_vertex_binding_divisors: *const VertexInputBindingDivisorDescription,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineVertexInputDivisorStateCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineVertexInputDivisorStateCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "vertex_binding_divisor_count",
                    &self.vertex_binding_divisor_count,
                )
                .field("p_vertex_binding_divisors", &self.p_vertex_binding_divisors)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineVertexInputDivisorStateCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO;
    }

    unsafe impl<'a> Extends<PipelineVertexInputStateCreateInfo<'a>>
        for PipelineVertexInputDivisorStateCreateInfo<'a>
    {
    }

    impl Default for PipelineVertexInputDivisorStateCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVertexAttributeDivisorProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVertexAttributeDivisorProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_vertex_attrib_divisor: u32,
        pub supports_non_zero_first_instance: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceVertexAttributeDivisorProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVertexAttributeDivisorProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_vertex_attrib_divisor", &self.max_vertex_attrib_divisor)
                .field(
                    "supports_non_zero_first_instance",
                    &self.supports_non_zero_first_instance,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVertexAttributeDivisorProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceVertexAttributeDivisorProperties<'a>
    {
    }

    impl Default for PhysicalDeviceVertexAttributeDivisorProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
            supports_non_zero_first_instance: bool,
        ) -> Self {
            self.supports_non_zero_first_instance = supports_non_zero_first_instance.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVertexAttributeDivisorFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVertexAttributeDivisorFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub vertex_attribute_instance_rate_divisor: Bool32,
        pub vertex_attribute_instance_rate_zero_divisor: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceVertexAttributeDivisorFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVertexAttributeDivisorFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "vertex_attribute_instance_rate_divisor",
                    &self.vertex_attribute_instance_rate_divisor,
                )
                .field(
                    "vertex_attribute_instance_rate_zero_divisor",
                    &self.vertex_attribute_instance_rate_zero_divisor,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVertexAttributeDivisorFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceVertexAttributeDivisorFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceVertexAttributeDivisorFeatures<'a> {}

    impl Default for PhysicalDeviceVertexAttributeDivisorFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
            vertex_attribute_instance_rate_divisor: bool,
        ) -> Self {
            self.vertex_attribute_instance_rate_divisor =
                vertex_attribute_instance_rate_divisor.into();
            self
        }

        pub fn vertex_attribute_instance_rate_zero_divisor(
            mut self,
            vertex_attribute_instance_rate_zero_divisor: bool,
        ) -> Self {
            self.vertex_attribute_instance_rate_zero_divisor =
                vertex_attribute_instance_rate_zero_divisor.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceIndexTypeUint8Features.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceIndexTypeUint8Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub index_type_uint8: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceIndexTypeUint8Features<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceIndexTypeUint8Features")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("index_type_uint8", &self.index_type_uint8)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceIndexTypeUint8Features<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceIndexTypeUint8Features<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceIndexTypeUint8Features<'a> {}

    impl Default for PhysicalDeviceIndexTypeUint8Features<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                index_type_uint8: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceIndexTypeUint8Features<'a> {
        pub fn index_type_uint8(mut self, index_type_uint8: bool) -> Self {
            self.index_type_uint8 = index_type_uint8.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceLineRasterizationFeatures.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceLineRasterizationFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceLineRasterizationFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("rectangular_lines", &self.rectangular_lines)
                .field("bresenham_lines", &self.bresenham_lines)
                .field("smooth_lines", &self.smooth_lines)
                .field(
                    "stippled_rectangular_lines",
                    &self.stippled_rectangular_lines,
                )
                .field("stippled_bresenham_lines", &self.stippled_bresenham_lines)
                .field("stippled_smooth_lines", &self.stippled_smooth_lines)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceLineRasterizationFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceLineRasterizationFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceLineRasterizationFeatures<'a> {}

    impl Default for PhysicalDeviceLineRasterizationFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
        pub fn rectangular_lines(mut self, rectangular_lines: bool) -> Self {
            self.rectangular_lines = rectangular_lines.into();
            self
        }

        pub fn bresenham_lines(mut self, bresenham_lines: bool) -> Self {
            self.bresenham_lines = bresenham_lines.into();
            self
        }

        pub fn smooth_lines(mut self, smooth_lines: bool) -> Self {
            self.smooth_lines = smooth_lines.into();
            self
        }

        pub fn stippled_rectangular_lines(mut self, stippled_rectangular_lines: bool) -> Self {
            self.stippled_rectangular_lines = stippled_rectangular_lines.into();
            self
        }

        pub fn stippled_bresenham_lines(mut self, stippled_bresenham_lines: bool) -> Self {
            self.stippled_bresenham_lines = stippled_bresenham_lines.into();
            self
        }

        pub fn stippled_smooth_lines(mut self, stippled_smooth_lines: bool) -> Self {
            self.stippled_smooth_lines = stippled_smooth_lines.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceLineRasterizationProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceLineRasterizationProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub line_sub_pixel_precision_bits: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceLineRasterizationProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceLineRasterizationProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "line_sub_pixel_precision_bits",
                    &self.line_sub_pixel_precision_bits,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceLineRasterizationProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceLineRasterizationProperties<'a>
    {
    }

    impl Default for PhysicalDeviceLineRasterizationProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRasterizationLineStateCreateInfo.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineRasterizationLineStateCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineRasterizationLineStateCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("line_rasterization_mode", &self.line_rasterization_mode)
                .field("stippled_line_enable", &self.stippled_line_enable)
                .field("line_stipple_factor", &self.line_stipple_factor)
                .field("line_stipple_pattern", &self.line_stipple_pattern)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineRasterizationLineStateCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO;
    }

    unsafe impl<'a> Extends<PipelineRasterizationStateCreateInfo<'a>>
        for PipelineRasterizationLineStateCreateInfo<'a>
    {
    }

    impl Default for PipelineRasterizationLineStateCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

        pub fn stippled_line_enable(mut self, stippled_line_enable: bool) -> Self {
            self.stippled_line_enable = stippled_line_enable.into();
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVulkan14Features.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceVulkan14Features<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVulkan14Features")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("global_priority_query", &self.global_priority_query)
                .field("shader_subgroup_rotate", &self.shader_subgroup_rotate)
                .field(
                    "shader_subgroup_rotate_clustered",
                    &self.shader_subgroup_rotate_clustered,
                )
                .field("shader_float_controls2", &self.shader_float_controls2)
                .field("shader_expect_assume", &self.shader_expect_assume)
                .field("rectangular_lines", &self.rectangular_lines)
                .field("bresenham_lines", &self.bresenham_lines)
                .field("smooth_lines", &self.smooth_lines)
                .field(
                    "stippled_rectangular_lines",
                    &self.stippled_rectangular_lines,
                )
                .field("stippled_bresenham_lines", &self.stippled_bresenham_lines)
                .field("stippled_smooth_lines", &self.stippled_smooth_lines)
                .field(
                    "vertex_attribute_instance_rate_divisor",
                    &self.vertex_attribute_instance_rate_divisor,
                )
                .field(
                    "vertex_attribute_instance_rate_zero_divisor",
                    &self.vertex_attribute_instance_rate_zero_divisor,
                )
                .field("index_type_uint8", &self.index_type_uint8)
                .field(
                    "dynamic_rendering_local_read",
                    &self.dynamic_rendering_local_read,
                )
                .field("maintenance5", &self.maintenance5)
                .field("maintenance6", &self.maintenance6)
                .field("pipeline_protected_access", &self.pipeline_protected_access)
                .field("pipeline_robustness", &self.pipeline_robustness)
                .field("host_image_copy", &self.host_image_copy)
                .field("push_descriptor", &self.push_descriptor)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVulkan14Features<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VULKAN_1_4_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceVulkan14Features<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceVulkan14Features<'a> {}

    impl Default for PhysicalDeviceVulkan14Features<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
        pub fn global_priority_query(mut self, global_priority_query: bool) -> Self {
            self.global_priority_query = global_priority_query.into();
            self
        }

        pub fn shader_subgroup_rotate(mut self, shader_subgroup_rotate: bool) -> Self {
            self.shader_subgroup_rotate = shader_subgroup_rotate.into();
            self
        }

        pub fn shader_subgroup_rotate_clustered(
            mut self,
            shader_subgroup_rotate_clustered: bool,
        ) -> Self {
            self.shader_subgroup_rotate_clustered = shader_subgroup_rotate_clustered.into();
            self
        }

        pub fn shader_float_controls2(mut self, shader_float_controls2: bool) -> Self {
            self.shader_float_controls2 = shader_float_controls2.into();
            self
        }

        pub fn shader_expect_assume(mut self, shader_expect_assume: bool) -> Self {
            self.shader_expect_assume = shader_expect_assume.into();
            self
        }

        pub fn rectangular_lines(mut self, rectangular_lines: bool) -> Self {
            self.rectangular_lines = rectangular_lines.into();
            self
        }

        pub fn bresenham_lines(mut self, bresenham_lines: bool) -> Self {
            self.bresenham_lines = bresenham_lines.into();
            self
        }

        pub fn smooth_lines(mut self, smooth_lines: bool) -> Self {
            self.smooth_lines = smooth_lines.into();
            self
        }

        pub fn stippled_rectangular_lines(mut self, stippled_rectangular_lines: bool) -> Self {
            self.stippled_rectangular_lines = stippled_rectangular_lines.into();
            self
        }

        pub fn stippled_bresenham_lines(mut self, stippled_bresenham_lines: bool) -> Self {
            self.stippled_bresenham_lines = stippled_bresenham_lines.into();
            self
        }

        pub fn stippled_smooth_lines(mut self, stippled_smooth_lines: bool) -> Self {
            self.stippled_smooth_lines = stippled_smooth_lines.into();
            self
        }

        pub fn vertex_attribute_instance_rate_divisor(
            mut self,
            vertex_attribute_instance_rate_divisor: bool,
        ) -> Self {
            self.vertex_attribute_instance_rate_divisor =
                vertex_attribute_instance_rate_divisor.into();
            self
        }

        pub fn vertex_attribute_instance_rate_zero_divisor(
            mut self,
            vertex_attribute_instance_rate_zero_divisor: bool,
        ) -> Self {
            self.vertex_attribute_instance_rate_zero_divisor =
                vertex_attribute_instance_rate_zero_divisor.into();
            self
        }

        pub fn index_type_uint8(mut self, index_type_uint8: bool) -> Self {
            self.index_type_uint8 = index_type_uint8.into();
            self
        }

        pub fn dynamic_rendering_local_read(mut self, dynamic_rendering_local_read: bool) -> Self {
            self.dynamic_rendering_local_read = dynamic_rendering_local_read.into();
            self
        }

        pub fn maintenance5(mut self, maintenance5: bool) -> Self {
            self.maintenance5 = maintenance5.into();
            self
        }

        pub fn maintenance6(mut self, maintenance6: bool) -> Self {
            self.maintenance6 = maintenance6.into();
            self
        }

        pub fn pipeline_protected_access(mut self, pipeline_protected_access: bool) -> Self {
            self.pipeline_protected_access = pipeline_protected_access.into();
            self
        }

        pub fn pipeline_robustness(mut self, pipeline_robustness: bool) -> Self {
            self.pipeline_robustness = pipeline_robustness.into();
            self
        }

        pub fn host_image_copy(mut self, host_image_copy: bool) -> Self {
            self.host_image_copy = host_image_copy.into();
            self
        }

        pub fn push_descriptor(mut self, push_descriptor: bool) -> Self {
            self.push_descriptor = push_descriptor.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVulkan14Properties.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceVulkan14Properties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVulkan14Properties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "line_sub_pixel_precision_bits",
                    &self.line_sub_pixel_precision_bits,
                )
                .field("max_vertex_attrib_divisor", &self.max_vertex_attrib_divisor)
                .field(
                    "supports_non_zero_first_instance",
                    &self.supports_non_zero_first_instance,
                )
                .field("max_push_descriptors", &self.max_push_descriptors)
                .field(
                    "dynamic_rendering_local_read_depth_stencil_attachments",
                    &self.dynamic_rendering_local_read_depth_stencil_attachments,
                )
                .field(
                    "dynamic_rendering_local_read_multisampled_attachments",
                    &self.dynamic_rendering_local_read_multisampled_attachments,
                )
                .field(
                    "early_fragment_multisample_coverage_after_sample_counting",
                    &self.early_fragment_multisample_coverage_after_sample_counting,
                )
                .field(
                    "early_fragment_sample_mask_test_before_sample_counting",
                    &self.early_fragment_sample_mask_test_before_sample_counting,
                )
                .field(
                    "depth_stencil_swizzle_one_support",
                    &self.depth_stencil_swizzle_one_support,
                )
                .field("polygon_mode_point_size", &self.polygon_mode_point_size)
                .field(
                    "non_strict_single_pixel_wide_lines_use_parallelogram",
                    &self.non_strict_single_pixel_wide_lines_use_parallelogram,
                )
                .field(
                    "non_strict_wide_lines_use_parallelogram",
                    &self.non_strict_wide_lines_use_parallelogram,
                )
                .field(
                    "block_texel_view_compatible_multiple_layers",
                    &self.block_texel_view_compatible_multiple_layers,
                )
                .field(
                    "max_combined_image_sampler_descriptor_count",
                    &self.max_combined_image_sampler_descriptor_count,
                )
                .field(
                    "fragment_shading_rate_clamp_combiner_inputs",
                    &self.fragment_shading_rate_clamp_combiner_inputs,
                )
                .field(
                    "default_robustness_storage_buffers",
                    &self.default_robustness_storage_buffers,
                )
                .field(
                    "default_robustness_uniform_buffers",
                    &self.default_robustness_uniform_buffers,
                )
                .field(
                    "default_robustness_vertex_inputs",
                    &self.default_robustness_vertex_inputs,
                )
                .field("default_robustness_images", &self.default_robustness_images)
                .field("copy_src_layout_count", &self.copy_src_layout_count)
                .field("p_copy_src_layouts", &self.p_copy_src_layouts)
                .field("copy_dst_layout_count", &self.copy_dst_layout_count)
                .field("p_copy_dst_layouts", &self.p_copy_dst_layouts)
                .field(
                    "optimal_tiling_layout_uuid",
                    &self.optimal_tiling_layout_uuid,
                )
                .field(
                    "identical_memory_type_requirements",
                    &self.identical_memory_type_requirements,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVulkan14Properties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>> for PhysicalDeviceVulkan14Properties<'a> {}

    impl Default for PhysicalDeviceVulkan14Properties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
            supports_non_zero_first_instance: bool,
        ) -> Self {
            self.supports_non_zero_first_instance = supports_non_zero_first_instance.into();
            self
        }

        pub fn max_push_descriptors(mut self, max_push_descriptors: u32) -> Self {
            self.max_push_descriptors = max_push_descriptors;
            self
        }

        pub fn dynamic_rendering_local_read_depth_stencil_attachments(
            mut self,
            dynamic_rendering_local_read_depth_stencil_attachments: bool,
        ) -> Self {
            self.dynamic_rendering_local_read_depth_stencil_attachments =
                dynamic_rendering_local_read_depth_stencil_attachments.into();
            self
        }

        pub fn dynamic_rendering_local_read_multisampled_attachments(
            mut self,
            dynamic_rendering_local_read_multisampled_attachments: bool,
        ) -> Self {
            self.dynamic_rendering_local_read_multisampled_attachments =
                dynamic_rendering_local_read_multisampled_attachments.into();
            self
        }

        pub fn early_fragment_multisample_coverage_after_sample_counting(
            mut self,
            early_fragment_multisample_coverage_after_sample_counting: bool,
        ) -> Self {
            self.early_fragment_multisample_coverage_after_sample_counting =
                early_fragment_multisample_coverage_after_sample_counting.into();
            self
        }

        pub fn early_fragment_sample_mask_test_before_sample_counting(
            mut self,
            early_fragment_sample_mask_test_before_sample_counting: bool,
        ) -> Self {
            self.early_fragment_sample_mask_test_before_sample_counting =
                early_fragment_sample_mask_test_before_sample_counting.into();
            self
        }

        pub fn depth_stencil_swizzle_one_support(
            mut self,
            depth_stencil_swizzle_one_support: bool,
        ) -> Self {
            self.depth_stencil_swizzle_one_support = depth_stencil_swizzle_one_support.into();
            self
        }

        pub fn polygon_mode_point_size(mut self, polygon_mode_point_size: bool) -> Self {
            self.polygon_mode_point_size = polygon_mode_point_size.into();
            self
        }

        pub fn non_strict_single_pixel_wide_lines_use_parallelogram(
            mut self,
            non_strict_single_pixel_wide_lines_use_parallelogram: bool,
        ) -> Self {
            self.non_strict_single_pixel_wide_lines_use_parallelogram =
                non_strict_single_pixel_wide_lines_use_parallelogram.into();
            self
        }

        pub fn non_strict_wide_lines_use_parallelogram(
            mut self,
            non_strict_wide_lines_use_parallelogram: bool,
        ) -> Self {
            self.non_strict_wide_lines_use_parallelogram =
                non_strict_wide_lines_use_parallelogram.into();
            self
        }

        pub fn block_texel_view_compatible_multiple_layers(
            mut self,
            block_texel_view_compatible_multiple_layers: bool,
        ) -> Self {
            self.block_texel_view_compatible_multiple_layers =
                block_texel_view_compatible_multiple_layers.into();
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
            fragment_shading_rate_clamp_combiner_inputs: bool,
        ) -> Self {
            self.fragment_shading_rate_clamp_combiner_inputs =
                fragment_shading_rate_clamp_combiner_inputs.into();
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
            identical_memory_type_requirements: bool,
        ) -> Self {
            self.identical_memory_type_requirements = identical_memory_type_requirements.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceHostImageCopyFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceHostImageCopyFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub host_image_copy: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceHostImageCopyFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceHostImageCopyFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("host_image_copy", &self.host_image_copy)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceHostImageCopyFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceHostImageCopyFeatures<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceHostImageCopyFeatures<'a> {}

    impl Default for PhysicalDeviceHostImageCopyFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                host_image_copy: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceHostImageCopyFeatures<'a> {
        pub fn host_image_copy(mut self, host_image_copy: bool) -> Self {
            self.host_image_copy = host_image_copy.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceHostImageCopyProperties.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceHostImageCopyProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceHostImageCopyProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("copy_src_layout_count", &self.copy_src_layout_count)
                .field("p_copy_src_layouts", &self.p_copy_src_layouts)
                .field("copy_dst_layout_count", &self.copy_dst_layout_count)
                .field("p_copy_dst_layouts", &self.p_copy_dst_layouts)
                .field(
                    "optimal_tiling_layout_uuid",
                    &self.optimal_tiling_layout_uuid,
                )
                .field(
                    "identical_memory_type_requirements",
                    &self.identical_memory_type_requirements,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceHostImageCopyProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceHostImageCopyProperties<'a>
    {
    }

    impl Default for PhysicalDeviceHostImageCopyProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
            identical_memory_type_requirements: bool,
        ) -> Self {
            self.identical_memory_type_requirements = identical_memory_type_requirements.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryToImageCopy.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryToImageCopy<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryToImageCopy")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_host_pointer", &self.p_host_pointer)
                .field("memory_row_length", &self.memory_row_length)
                .field("memory_image_height", &self.memory_image_height)
                .field("image_subresource", &self.image_subresource)
                .field("image_offset", &self.image_offset)
                .field("image_extent", &self.image_extent)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryToImageCopy<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_TO_IMAGE_COPY;
    }

    impl Default for MemoryToImageCopy<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
        pub fn host_pointer(mut self, host_pointer: *const c_void) -> Self {
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageToMemoryCopy.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageToMemoryCopy<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageToMemoryCopy")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_host_pointer", &self.p_host_pointer)
                .field("memory_row_length", &self.memory_row_length)
                .field("memory_image_height", &self.memory_image_height)
                .field("image_subresource", &self.image_subresource)
                .field("image_offset", &self.image_offset)
                .field("image_extent", &self.image_extent)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageToMemoryCopy<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_TO_MEMORY_COPY;
    }

    impl Default for ImageToMemoryCopy<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
        pub fn host_pointer(mut self, host_pointer: *mut c_void) -> Self {
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyMemoryToImageInfo.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyMemoryToImageInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyMemoryToImageInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("dst_image", &self.dst_image)
                .field("dst_image_layout", &self.dst_image_layout)
                .field("region_count", &self.region_count)
                .field("p_regions", &self.p_regions)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyMemoryToImageInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_MEMORY_TO_IMAGE_INFO;
    }

    impl Default for CopyMemoryToImageInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyImageToMemoryInfo.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyImageToMemoryInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyImageToMemoryInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("src_image", &self.src_image)
                .field("src_image_layout", &self.src_image_layout)
                .field("region_count", &self.region_count)
                .field("p_regions", &self.p_regions)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyImageToMemoryInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_IMAGE_TO_MEMORY_INFO;
    }

    impl Default for CopyImageToMemoryInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyImageToImageInfo.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyImageToImageInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyImageToImageInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("src_image", &self.src_image)
                .field("src_image_layout", &self.src_image_layout)
                .field("dst_image", &self.dst_image)
                .field("dst_image_layout", &self.dst_image_layout)
                .field("region_count", &self.region_count)
                .field("p_regions", &self.p_regions)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyImageToImageInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_IMAGE_TO_IMAGE_INFO;
    }

    impl Default for CopyImageToImageInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkHostImageLayoutTransitionInfo.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for HostImageLayoutTransitionInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("HostImageLayoutTransitionInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image", &self.image)
                .field("old_layout", &self.old_layout)
                .field("new_layout", &self.new_layout)
                .field("subresource_range", &self.subresource_range)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for HostImageLayoutTransitionInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::HOST_IMAGE_LAYOUT_TRANSITION_INFO;
    }

    impl Default for HostImageLayoutTransitionInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubresourceHostMemcpySize.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SubresourceHostMemcpySize<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SubresourceHostMemcpySize<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SubresourceHostMemcpySize")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("size", &self.size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SubresourceHostMemcpySize<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SUBRESOURCE_HOST_MEMCPY_SIZE;
    }

    unsafe impl<'a> Extends<SubresourceLayout2<'a>> for SubresourceHostMemcpySize<'a> {}

    impl Default for SubresourceHostMemcpySize<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkHostImageCopyDevicePerformanceQuery.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct HostImageCopyDevicePerformanceQuery<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub optimal_device_access: Bool32,
        pub identical_memory_layout: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for HostImageCopyDevicePerformanceQuery<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("HostImageCopyDevicePerformanceQuery")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("optimal_device_access", &self.optimal_device_access)
                .field("identical_memory_layout", &self.identical_memory_layout)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for HostImageCopyDevicePerformanceQuery<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY;
    }

    unsafe impl<'a> Extends<ImageFormatProperties2<'a>> for HostImageCopyDevicePerformanceQuery<'a> {}

    impl Default for HostImageCopyDevicePerformanceQuery<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                optimal_device_access: Default::default(),
                identical_memory_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> HostImageCopyDevicePerformanceQuery<'a> {
        pub fn optimal_device_access(mut self, optimal_device_access: bool) -> Self {
            self.optimal_device_access = optimal_device_access.into();
            self
        }

        pub fn identical_memory_layout(mut self, identical_memory_layout: bool) -> Self {
            self.identical_memory_layout = identical_memory_layout.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePipelineProtectedAccessFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePipelineProtectedAccessFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_protected_access: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePipelineProtectedAccessFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePipelineProtectedAccessFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("pipeline_protected_access", &self.pipeline_protected_access)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePipelineProtectedAccessFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePipelineProtectedAccessFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDevicePipelineProtectedAccessFeatures<'a>
    {
    }

    impl Default for PhysicalDevicePipelineProtectedAccessFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                pipeline_protected_access: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePipelineProtectedAccessFeatures<'a> {
        pub fn pipeline_protected_access(mut self, pipeline_protected_access: bool) -> Self {
            self.pipeline_protected_access = pipeline_protected_access.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageSubresource2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageSubresource2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image_subresource: ImageSubresource,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageSubresource2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageSubresource2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image_subresource", &self.image_subresource)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageSubresource2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_SUBRESOURCE_2;
    }

    impl Default for ImageSubresource2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubresourceLayout2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SubresourceLayout2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub subresource_layout: SubresourceLayout,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SubresourceLayout2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SubresourceLayout2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("subresource_layout", &self.subresource_layout)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SubresourceLayout2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SUBRESOURCE_LAYOUT_2;
    }

    impl Default for SubresourceLayout2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePipelineRobustnessFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePipelineRobustnessFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_robustness: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePipelineRobustnessFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePipelineRobustnessFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("pipeline_robustness", &self.pipeline_robustness)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePipelineRobustnessFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePipelineRobustnessFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDevicePipelineRobustnessFeatures<'a> {}

    impl Default for PhysicalDevicePipelineRobustnessFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                pipeline_robustness: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePipelineRobustnessFeatures<'a> {
        pub fn pipeline_robustness(mut self, pipeline_robustness: bool) -> Self {
            self.pipeline_robustness = pipeline_robustness.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRobustnessCreateInfo.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineRobustnessCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineRobustnessCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("storage_buffers", &self.storage_buffers)
                .field("uniform_buffers", &self.uniform_buffers)
                .field("vertex_inputs", &self.vertex_inputs)
                .field("images", &self.images)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineRobustnessCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_ROBUSTNESS_CREATE_INFO;
    }

    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>> for PipelineRobustnessCreateInfo<'a> {}
    unsafe impl<'a> Extends<ComputePipelineCreateInfo<'a>> for PipelineRobustnessCreateInfo<'a> {}
    unsafe impl<'a> Extends<PipelineShaderStageCreateInfo<'a>> for PipelineRobustnessCreateInfo<'a> {}
    unsafe impl<'a> Extends<RayTracingPipelineCreateInfoKHR<'a>> for PipelineRobustnessCreateInfo<'a> {}

    impl Default for PipelineRobustnessCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePipelineRobustnessProperties.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePipelineRobustnessProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePipelineRobustnessProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "default_robustness_storage_buffers",
                    &self.default_robustness_storage_buffers,
                )
                .field(
                    "default_robustness_uniform_buffers",
                    &self.default_robustness_uniform_buffers,
                )
                .field(
                    "default_robustness_vertex_inputs",
                    &self.default_robustness_vertex_inputs,
                )
                .field("default_robustness_images", &self.default_robustness_images)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePipelineRobustnessProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDevicePipelineRobustnessProperties<'a>
    {
    }

    impl Default for PhysicalDevicePipelineRobustnessProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceImageSubresourceInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceImageSubresourceInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_create_info: *const ImageCreateInfo<'a>,
        pub p_subresource: *const ImageSubresource2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceImageSubresourceInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceImageSubresourceInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_create_info", &self.p_create_info)
                .field("p_subresource", &self.p_subresource)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceImageSubresourceInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_IMAGE_SUBRESOURCE_INFO;
    }

    impl Default for DeviceImageSubresourceInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryMapInfo.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryMapInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryMapInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("memory", &self.memory)
                .field("offset", &self.offset)
                .field("size", &self.size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryMapInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_MAP_INFO;
    }

    impl Default for MemoryMapInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryUnmapInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryUnmapInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: MemoryUnmapFlags,
        pub memory: DeviceMemory,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryUnmapInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryUnmapInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("memory", &self.memory)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryUnmapInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_UNMAP_INFO;
    }

    impl Default for MemoryUnmapInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindMemoryStatus.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BindMemoryStatus<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_result: *mut vk::Result,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BindMemoryStatus<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BindMemoryStatus")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_result", &self.p_result)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BindMemoryStatus<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_MEMORY_STATUS;
    }

    unsafe impl<'a> Extends<BindBufferMemoryInfo<'a>> for BindMemoryStatus<'a> {}
    unsafe impl<'a> Extends<BindImageMemoryInfo<'a>> for BindMemoryStatus<'a> {}

    impl Default for BindMemoryStatus<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindDescriptorSetsInfo.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for BindDescriptorSetsInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BindDescriptorSetsInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("stage_flags", &self.stage_flags)
                .field("layout", &self.layout)
                .field("first_set", &self.first_set)
                .field("descriptor_set_count", &self.descriptor_set_count)
                .field("p_descriptor_sets", &self.p_descriptor_sets)
                .field("dynamic_offset_count", &self.dynamic_offset_count)
                .field("p_dynamic_offsets", &self.p_dynamic_offsets)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BindDescriptorSetsInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_DESCRIPTOR_SETS_INFO;
    }

    impl Default for BindDescriptorSetsInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPushConstantsInfo.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for PushConstantsInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PushConstantsInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("layout", &self.layout)
                .field("stage_flags", &self.stage_flags)
                .field("offset", &self.offset)
                .field("size", &self.size)
                .field("p_values", &self.p_values)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PushConstantsInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PUSH_CONSTANTS_INFO;
    }

    impl Default for PushConstantsInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPushDescriptorSetInfo.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for PushDescriptorSetInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PushDescriptorSetInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("stage_flags", &self.stage_flags)
                .field("layout", &self.layout)
                .field("set", &self.set)
                .field("descriptor_write_count", &self.descriptor_write_count)
                .field("p_descriptor_writes", &self.p_descriptor_writes)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PushDescriptorSetInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PUSH_DESCRIPTOR_SET_INFO;
    }

    impl Default for PushDescriptorSetInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPushDescriptorSetWithTemplateInfo.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for PushDescriptorSetWithTemplateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PushDescriptorSetWithTemplateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "descriptor_update_template",
                    &self.descriptor_update_template,
                )
                .field("layout", &self.layout)
                .field("set", &self.set)
                .field("p_data", &self.p_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PushDescriptorSetWithTemplateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO;
    }

    impl Default for PushDescriptorSetWithTemplateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

        pub fn data(mut self, data: *const c_void) -> Self {
            self.p_data = data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderSubgroupRotateFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderSubgroupRotateFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_subgroup_rotate: Bool32,
        pub shader_subgroup_rotate_clustered: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderSubgroupRotateFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderSubgroupRotateFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_subgroup_rotate", &self.shader_subgroup_rotate)
                .field(
                    "shader_subgroup_rotate_clustered",
                    &self.shader_subgroup_rotate_clustered,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderSubgroupRotateFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderSubgroupRotateFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShaderSubgroupRotateFeatures<'a> {}

    impl Default for PhysicalDeviceShaderSubgroupRotateFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_subgroup_rotate: Default::default(),
                shader_subgroup_rotate_clustered: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderSubgroupRotateFeatures<'a> {
        pub fn shader_subgroup_rotate(mut self, shader_subgroup_rotate: bool) -> Self {
            self.shader_subgroup_rotate = shader_subgroup_rotate.into();
            self
        }

        pub fn shader_subgroup_rotate_clustered(
            mut self,
            shader_subgroup_rotate_clustered: bool,
        ) -> Self {
            self.shader_subgroup_rotate_clustered = shader_subgroup_rotate_clustered.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderExpectAssumeFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderExpectAssumeFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_expect_assume: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderExpectAssumeFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderExpectAssumeFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_expect_assume", &self.shader_expect_assume)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderExpectAssumeFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderExpectAssumeFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShaderExpectAssumeFeatures<'a> {}

    impl Default for PhysicalDeviceShaderExpectAssumeFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_expect_assume: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderExpectAssumeFeatures<'a> {
        pub fn shader_expect_assume(mut self, shader_expect_assume: bool) -> Self {
            self.shader_expect_assume = shader_expect_assume.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderFloatControls2Features.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderFloatControls2Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_float_controls2: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderFloatControls2Features<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderFloatControls2Features")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_float_controls2", &self.shader_float_controls2)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderFloatControls2Features<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderFloatControls2Features<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShaderFloatControls2Features<'a> {}

    impl Default for PhysicalDeviceShaderFloatControls2Features<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_float_controls2: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderFloatControls2Features<'a> {
        pub fn shader_float_controls2(mut self, shader_float_controls2: bool) -> Self {
            self.shader_float_controls2 = shader_float_controls2.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDynamicRenderingLocalReadFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDynamicRenderingLocalReadFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub dynamic_rendering_local_read: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDynamicRenderingLocalReadFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDynamicRenderingLocalReadFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "dynamic_rendering_local_read",
                    &self.dynamic_rendering_local_read,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDynamicRenderingLocalReadFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDynamicRenderingLocalReadFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceDynamicRenderingLocalReadFeatures<'a>
    {
    }

    impl Default for PhysicalDeviceDynamicRenderingLocalReadFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                dynamic_rendering_local_read: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDynamicRenderingLocalReadFeatures<'a> {
        pub fn dynamic_rendering_local_read(mut self, dynamic_rendering_local_read: bool) -> Self {
            self.dynamic_rendering_local_read = dynamic_rendering_local_read.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingAttachmentLocationInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderingAttachmentLocationInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub color_attachment_count: u32,
        pub p_color_attachment_locations: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderingAttachmentLocationInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderingAttachmentLocationInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("color_attachment_count", &self.color_attachment_count)
                .field(
                    "p_color_attachment_locations",
                    &self.p_color_attachment_locations,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderingAttachmentLocationInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDERING_ATTACHMENT_LOCATION_INFO;
    }

    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>> for RenderingAttachmentLocationInfo<'a> {}
    unsafe impl<'a> Extends<CommandBufferInheritanceInfo<'a>> for RenderingAttachmentLocationInfo<'a> {}

    impl Default for RenderingAttachmentLocationInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingInputAttachmentIndexInfo.html>
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderingInputAttachmentIndexInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderingInputAttachmentIndexInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("color_attachment_count", &self.color_attachment_count)
                .field(
                    "p_color_attachment_input_indices",
                    &self.p_color_attachment_input_indices,
                )
                .field(
                    "p_depth_input_attachment_index",
                    &self.p_depth_input_attachment_index,
                )
                .field(
                    "p_stencil_input_attachment_index",
                    &self.p_stencil_input_attachment_index,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderingInputAttachmentIndexInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDERING_INPUT_ATTACHMENT_INDEX_INFO;
    }

    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>> for RenderingInputAttachmentIndexInfo<'a> {}
    unsafe impl<'a> Extends<CommandBufferInheritanceInfo<'a>>
        for RenderingInputAttachmentIndexInfo<'a>
    {
    }

    impl Default for RenderingInputAttachmentIndexInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueGlobalPriority.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct QueueGlobalPriority(i32);

    impl QueueGlobalPriority {
        pub const LOW: Self = Self(128);
        pub const MEDIUM: Self = Self(256);
        pub const HIGH: Self = Self(512);
        pub const REALTIME: Self = Self(1024);
        // VK_EXT_global_priority
        pub const LOW_EXT: Self = Self::LOW;
        pub const MEDIUM_EXT: Self = Self::MEDIUM;
        pub const HIGH_EXT: Self = Self::HIGH;
        pub const REALTIME_EXT: Self = Self::REALTIME;

        // VK_KHR_global_priority
        pub const LOW_KHR: Self = Self::LOW;
        pub const MEDIUM_KHR: Self = Self::MEDIUM;
        pub const HIGH_KHR: Self = Self::HIGH;
        pub const REALTIME_KHR: Self = Self::REALTIME;
    }

    impl fmt::Debug for QueueGlobalPriority {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::LOW => Some("LOW"),
                Self::MEDIUM => Some("MEDIUM"),
                Self::HIGH => Some("HIGH"),
                Self::REALTIME => Some("REALTIME"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkLineRasterizationMode.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LineRasterizationMode(i32);

    impl LineRasterizationMode {
        pub const DEFAULT: Self = Self(0);
        pub const RECTANGULAR: Self = Self(1);
        pub const BRESENHAM: Self = Self(2);
        pub const RECTANGULAR_SMOOTH: Self = Self(3);
        // VK_EXT_line_rasterization
        pub const DEFAULT_EXT: Self = Self::DEFAULT;
        pub const RECTANGULAR_EXT: Self = Self::RECTANGULAR;
        pub const BRESENHAM_EXT: Self = Self::BRESENHAM;
        pub const RECTANGULAR_SMOOTH_EXT: Self = Self::RECTANGULAR_SMOOTH;

        // VK_KHR_line_rasterization
        pub const DEFAULT_KHR: Self = Self::DEFAULT;
        pub const RECTANGULAR_KHR: Self = Self::RECTANGULAR;
        pub const BRESENHAM_KHR: Self = Self::BRESENHAM;
        pub const RECTANGULAR_SMOOTH_KHR: Self = Self::RECTANGULAR_SMOOTH;
    }

    impl fmt::Debug for LineRasterizationMode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEFAULT => Some("DEFAULT"),
                Self::RECTANGULAR => Some("RECTANGULAR"),
                Self::BRESENHAM => Some("BRESENHAM"),
                Self::RECTANGULAR_SMOOTH => Some("RECTANGULAR_SMOOTH"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRobustnessBufferBehavior.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineRobustnessBufferBehavior(i32);

    impl PipelineRobustnessBufferBehavior {
        pub const DEVICE_DEFAULT: Self = Self(0);
        pub const DISABLED: Self = Self(1);
        pub const ROBUST_BUFFER_ACCESS: Self = Self(2);
        pub const ROBUST_BUFFER_ACCESS_2: Self = Self(3);
        // VK_EXT_pipeline_robustness
        pub const DEVICE_DEFAULT_EXT: Self = Self::DEVICE_DEFAULT;
        pub const DISABLED_EXT: Self = Self::DISABLED;
        pub const ROBUST_BUFFER_ACCESS_EXT: Self = Self::ROBUST_BUFFER_ACCESS;
        pub const ROBUST_BUFFER_ACCESS_2_EXT: Self = Self::ROBUST_BUFFER_ACCESS_2;
    }

    impl fmt::Debug for PipelineRobustnessBufferBehavior {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEVICE_DEFAULT => Some("DEVICE_DEFAULT"),
                Self::DISABLED => Some("DISABLED"),
                Self::ROBUST_BUFFER_ACCESS => Some("ROBUST_BUFFER_ACCESS"),
                Self::ROBUST_BUFFER_ACCESS_2 => Some("ROBUST_BUFFER_ACCESS_2"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRobustnessImageBehavior.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineRobustnessImageBehavior(i32);

    impl PipelineRobustnessImageBehavior {
        pub const DEVICE_DEFAULT: Self = Self(0);
        pub const DISABLED: Self = Self(1);
        pub const ROBUST_IMAGE_ACCESS: Self = Self(2);
        pub const ROBUST_IMAGE_ACCESS_2: Self = Self(3);
        // VK_EXT_pipeline_robustness
        pub const DEVICE_DEFAULT_EXT: Self = Self::DEVICE_DEFAULT;
        pub const DISABLED_EXT: Self = Self::DISABLED;
        pub const ROBUST_IMAGE_ACCESS_EXT: Self = Self::ROBUST_IMAGE_ACCESS;
        pub const ROBUST_IMAGE_ACCESS_2_EXT: Self = Self::ROBUST_IMAGE_ACCESS_2;
    }

    impl fmt::Debug for PipelineRobustnessImageBehavior {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEVICE_DEFAULT => Some("DEVICE_DEFAULT"),
                Self::DISABLED => Some("DISABLED"),
                Self::ROBUST_IMAGE_ACCESS => Some("ROBUST_IMAGE_ACCESS"),
                Self::ROBUST_IMAGE_ACCESS_2 => Some("ROBUST_IMAGE_ACCESS_2"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryUnmapFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MemoryUnmapFlags(Flags);
    vk_bitflags_wrapped!(MemoryUnmapFlags, Flags);

    impl MemoryUnmapFlags {
        // VK_EXT_map_memory_placed
        pub const RESERVE_EXT: Self = Self(MemoryUnmapFlagBits::RESERVE_EXT.0);
    }

    impl fmt::Debug for MemoryUnmapFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[(MemoryUnmapFlags::RESERVE_EXT.0, "RESERVE_EXT")];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryUnmapFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct MemoryUnmapFlagBits(u32);

    impl MemoryUnmapFlagBits {
        // VK_EXT_map_memory_placed
        pub const RESERVE_EXT: Self = Self(1 << 0);
    }

    impl fmt::Debug for MemoryUnmapFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::RESERVE_EXT => Some("RESERVE_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCreateFlags2.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineCreateFlags2(Flags64);
    vk_bitflags_wrapped!(PipelineCreateFlags2, Flags64);

    impl PipelineCreateFlags2 {
        pub const DISABLE_OPTIMIZATION: Self =
            Self(PipelineCreateFlagBits2::DISABLE_OPTIMIZATION.0);
        pub const ALLOW_DERIVATIVES: Self = Self(PipelineCreateFlagBits2::ALLOW_DERIVATIVES.0);
        pub const DERIVATIVE: Self = Self(PipelineCreateFlagBits2::DERIVATIVE.0);
        pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self =
            Self(PipelineCreateFlagBits2::VIEW_INDEX_FROM_DEVICE_INDEX.0);
        pub const DISPATCH_BASE: Self = Self(PipelineCreateFlagBits2::DISPATCH_BASE.0);
        pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self =
            Self(PipelineCreateFlagBits2::FAIL_ON_PIPELINE_COMPILE_REQUIRED.0);
        pub const EARLY_RETURN_ON_FAILURE: Self =
            Self(PipelineCreateFlagBits2::EARLY_RETURN_ON_FAILURE.0);
        pub const NO_PROTECTED_ACCESS: Self = Self(PipelineCreateFlagBits2::NO_PROTECTED_ACCESS.0);
        pub const PROTECTED_ACCESS_ONLY: Self =
            Self(PipelineCreateFlagBits2::PROTECTED_ACCESS_ONLY.0);
        // VK_AMDX_shader_enqueue
        pub const EXECUTION_GRAPH_AMDX: Self =
            Self(PipelineCreateFlagBits2::EXECUTION_GRAPH_AMDX.0);

        // VK_ARM_pipeline_opacity_micromap
        pub const DISALLOW_OPACITY_MICROMAP_ARM: Self =
            Self(PipelineCreateFlagBits2::DISALLOW_OPACITY_MICROMAP_ARM.0);

        // VK_EXT_descriptor_heap
        pub const DESCRIPTOR_HEAP_EXT: Self = Self(PipelineCreateFlagBits2::DESCRIPTOR_HEAP_EXT.0);

        // VK_EXT_device_generated_commands
        pub const INDIRECT_BINDABLE_EXT: Self =
            Self(PipelineCreateFlagBits2::INDIRECT_BINDABLE_EXT.0);

        // VK_EXT_legacy_dithering
        pub const ENABLE_LEGACY_DITHERING_EXT: Self =
            Self(PipelineCreateFlagBits2::ENABLE_LEGACY_DITHERING_EXT.0);

        // VK_EXT_shader_64bit_indexing
        pub const _64_INDEXING_EXT: Self = Self(PipelineCreateFlagBits2::_64_INDEXING_EXT.0);

        // VK_KHR_maintenance5
        pub const DEFER_COMPILE_NV: Self = Self(PipelineCreateFlagBits2::DEFER_COMPILE_NV.0);
        pub const CAPTURE_STATISTICS_KHR: Self =
            Self(PipelineCreateFlagBits2::CAPTURE_STATISTICS_KHR.0);
        pub const CAPTURE_INTERNAL_REPRESENTATIONS_KHR: Self =
            Self(PipelineCreateFlagBits2::CAPTURE_INTERNAL_REPRESENTATIONS_KHR.0);
        pub const LINK_TIME_OPTIMIZATION_EXT: Self =
            Self(PipelineCreateFlagBits2::LINK_TIME_OPTIMIZATION_EXT.0);
        pub const LIBRARY_KHR: Self = Self(PipelineCreateFlagBits2::LIBRARY_KHR.0);
        pub const RAY_TRACING_SKIP_TRIANGLES_KHR: Self =
            Self(PipelineCreateFlagBits2::RAY_TRACING_SKIP_TRIANGLES_KHR.0);
        pub const RAY_TRACING_SKIP_AABBS_KHR: Self =
            Self(PipelineCreateFlagBits2::RAY_TRACING_SKIP_AABBS_KHR.0);
        pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR: Self =
            Self(PipelineCreateFlagBits2::RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR.0);
        pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR: Self =
            Self(PipelineCreateFlagBits2::RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR.0);
        pub const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR: Self =
            Self(PipelineCreateFlagBits2::RAY_TRACING_NO_NULL_MISS_SHADERS_KHR.0);
        pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR: Self =
            Self(PipelineCreateFlagBits2::RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR.0);
        pub const INDIRECT_BINDABLE_NV: Self =
            Self(PipelineCreateFlagBits2::INDIRECT_BINDABLE_NV.0);
        pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR: Self =
            Self(PipelineCreateFlagBits2::RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR.0);
        pub const RAY_TRACING_ALLOW_MOTION_NV: Self =
            Self(PipelineCreateFlagBits2::RAY_TRACING_ALLOW_MOTION_NV.0);
        pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
            Self(PipelineCreateFlagBits2::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0);
        pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self =
            Self(PipelineCreateFlagBits2::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT.0);
        pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT: Self =
            Self(PipelineCreateFlagBits2::RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT.0);
        pub const RAY_TRACING_OPACITY_MICROMAP_EXT: Self =
            Self(PipelineCreateFlagBits2::RAY_TRACING_OPACITY_MICROMAP_EXT.0);
        pub const COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT: Self =
            Self(PipelineCreateFlagBits2::COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT.0);
        pub const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT: Self =
            Self(PipelineCreateFlagBits2::DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT.0);
        pub const RAY_TRACING_DISPLACEMENT_MICROMAP_NV: Self =
            Self(PipelineCreateFlagBits2::RAY_TRACING_DISPLACEMENT_MICROMAP_NV.0);
        pub const DESCRIPTOR_BUFFER_EXT: Self =
            Self(PipelineCreateFlagBits2::DESCRIPTOR_BUFFER_EXT.0);
        pub const DISABLE_OPTIMIZATION_KHR: Self = Self::DISABLE_OPTIMIZATION;
        pub const ALLOW_DERIVATIVES_KHR: Self = Self::ALLOW_DERIVATIVES;
        pub const DERIVATIVE_KHR: Self = Self::DERIVATIVE;
        pub const VIEW_INDEX_FROM_DEVICE_INDEX_KHR: Self = Self::VIEW_INDEX_FROM_DEVICE_INDEX;
        pub const DISPATCH_BASE_KHR: Self = Self::DISPATCH_BASE;
        pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_KHR: Self =
            Self::FAIL_ON_PIPELINE_COMPILE_REQUIRED;
        pub const EARLY_RETURN_ON_FAILURE_KHR: Self = Self::EARLY_RETURN_ON_FAILURE;
        pub const NO_PROTECTED_ACCESS_EXT: Self = Self::NO_PROTECTED_ACCESS;
        pub const PROTECTED_ACCESS_ONLY_EXT: Self = Self::PROTECTED_ACCESS_ONLY;

        // VK_KHR_pipeline_binary
        pub const CAPTURE_DATA_KHR: Self = Self(PipelineCreateFlagBits2::CAPTURE_DATA_KHR.0);

        // VK_KHR_ray_tracing_pipeline
        pub const RAY_TRACING_SKIP_BUILT_IN_PRIMITIVES_KHR: Self =
            Self::RAY_TRACING_SKIP_TRIANGLES_KHR;

        // VK_NV_ray_tracing_linear_swept_spheres
        pub const RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV: Self =
            Self(PipelineCreateFlagBits2::RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV.0);

        // VK_VALVE_fragment_density_map_layered
        pub const PER_LAYER_FRAGMENT_DENSITY_VALVE: Self =
            Self(PipelineCreateFlagBits2::PER_LAYER_FRAGMENT_DENSITY_VALVE.0);
    }

    impl fmt::Debug for PipelineCreateFlags2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags64, &str)] = &[
                (
                    PipelineCreateFlags2::DISABLE_OPTIMIZATION.0,
                    "DISABLE_OPTIMIZATION",
                ),
                (
                    PipelineCreateFlags2::ALLOW_DERIVATIVES.0,
                    "ALLOW_DERIVATIVES",
                ),
                (PipelineCreateFlags2::DERIVATIVE.0, "DERIVATIVE"),
                (
                    PipelineCreateFlags2::VIEW_INDEX_FROM_DEVICE_INDEX.0,
                    "VIEW_INDEX_FROM_DEVICE_INDEX",
                ),
                (PipelineCreateFlags2::DISPATCH_BASE.0, "DISPATCH_BASE"),
                (
                    PipelineCreateFlags2::FAIL_ON_PIPELINE_COMPILE_REQUIRED.0,
                    "FAIL_ON_PIPELINE_COMPILE_REQUIRED",
                ),
                (
                    PipelineCreateFlags2::EARLY_RETURN_ON_FAILURE.0,
                    "EARLY_RETURN_ON_FAILURE",
                ),
                (
                    PipelineCreateFlags2::NO_PROTECTED_ACCESS.0,
                    "NO_PROTECTED_ACCESS",
                ),
                (
                    PipelineCreateFlags2::PROTECTED_ACCESS_ONLY.0,
                    "PROTECTED_ACCESS_ONLY",
                ),
                (
                    PipelineCreateFlags2::EXECUTION_GRAPH_AMDX.0,
                    "EXECUTION_GRAPH_AMDX",
                ),
                (
                    PipelineCreateFlags2::DISALLOW_OPACITY_MICROMAP_ARM.0,
                    "DISALLOW_OPACITY_MICROMAP_ARM",
                ),
                (
                    PipelineCreateFlags2::DESCRIPTOR_HEAP_EXT.0,
                    "DESCRIPTOR_HEAP_EXT",
                ),
                (
                    PipelineCreateFlags2::INDIRECT_BINDABLE_EXT.0,
                    "INDIRECT_BINDABLE_EXT",
                ),
                (
                    PipelineCreateFlags2::ENABLE_LEGACY_DITHERING_EXT.0,
                    "ENABLE_LEGACY_DITHERING_EXT",
                ),
                (PipelineCreateFlags2::_64_INDEXING_EXT.0, "_64_INDEXING_EXT"),
                (PipelineCreateFlags2::DEFER_COMPILE_NV.0, "DEFER_COMPILE_NV"),
                (
                    PipelineCreateFlags2::CAPTURE_STATISTICS_KHR.0,
                    "CAPTURE_STATISTICS_KHR",
                ),
                (
                    PipelineCreateFlags2::CAPTURE_INTERNAL_REPRESENTATIONS_KHR.0,
                    "CAPTURE_INTERNAL_REPRESENTATIONS_KHR",
                ),
                (
                    PipelineCreateFlags2::LINK_TIME_OPTIMIZATION_EXT.0,
                    "LINK_TIME_OPTIMIZATION_EXT",
                ),
                (PipelineCreateFlags2::LIBRARY_KHR.0, "LIBRARY_KHR"),
                (
                    PipelineCreateFlags2::RAY_TRACING_SKIP_TRIANGLES_KHR.0,
                    "RAY_TRACING_SKIP_TRIANGLES_KHR",
                ),
                (
                    PipelineCreateFlags2::RAY_TRACING_SKIP_AABBS_KHR.0,
                    "RAY_TRACING_SKIP_AABBS_KHR",
                ),
                (
                    PipelineCreateFlags2::RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR.0,
                    "RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR",
                ),
                (
                    PipelineCreateFlags2::RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR.0,
                    "RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR",
                ),
                (
                    PipelineCreateFlags2::RAY_TRACING_NO_NULL_MISS_SHADERS_KHR.0,
                    "RAY_TRACING_NO_NULL_MISS_SHADERS_KHR",
                ),
                (
                    PipelineCreateFlags2::RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR.0,
                    "RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR",
                ),
                (
                    PipelineCreateFlags2::INDIRECT_BINDABLE_NV.0,
                    "INDIRECT_BINDABLE_NV",
                ),
                (
                    PipelineCreateFlags2::RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR.0,
                    "RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR",
                ),
                (
                    PipelineCreateFlags2::RAY_TRACING_ALLOW_MOTION_NV.0,
                    "RAY_TRACING_ALLOW_MOTION_NV",
                ),
                (
                    PipelineCreateFlags2::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0,
                    "RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR",
                ),
                (
                    PipelineCreateFlags2::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT.0,
                    "RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT",
                ),
                (
                    PipelineCreateFlags2::RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT.0,
                    "RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT",
                ),
                (
                    PipelineCreateFlags2::RAY_TRACING_OPACITY_MICROMAP_EXT.0,
                    "RAY_TRACING_OPACITY_MICROMAP_EXT",
                ),
                (
                    PipelineCreateFlags2::COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT.0,
                    "COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT",
                ),
                (
                    PipelineCreateFlags2::DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT.0,
                    "DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT",
                ),
                (
                    PipelineCreateFlags2::RAY_TRACING_DISPLACEMENT_MICROMAP_NV.0,
                    "RAY_TRACING_DISPLACEMENT_MICROMAP_NV",
                ),
                (
                    PipelineCreateFlags2::DESCRIPTOR_BUFFER_EXT.0,
                    "DESCRIPTOR_BUFFER_EXT",
                ),
                (PipelineCreateFlags2::CAPTURE_DATA_KHR.0, "CAPTURE_DATA_KHR"),
                (
                    PipelineCreateFlags2::RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV.0,
                    "RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV",
                ),
                (
                    PipelineCreateFlags2::PER_LAYER_FRAGMENT_DENSITY_VALVE.0,
                    "PER_LAYER_FRAGMENT_DENSITY_VALVE",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCreateFlagBits2.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PipelineCreateFlagBits2(u64);

    impl PipelineCreateFlagBits2 {
        pub const DISABLE_OPTIMIZATION: Self = Self(1 << 0);
        pub const ALLOW_DERIVATIVES: Self = Self(1 << 1);
        pub const DERIVATIVE: Self = Self(1 << 2);
        pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self(1 << 3);
        pub const DISPATCH_BASE: Self = Self(1 << 4);
        pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self = Self(1 << 8);
        pub const EARLY_RETURN_ON_FAILURE: Self = Self(1 << 9);
        pub const NO_PROTECTED_ACCESS: Self = Self(1 << 27);
        pub const PROTECTED_ACCESS_ONLY: Self = Self(1 << 30);
        // VK_AMDX_shader_enqueue
        pub const EXECUTION_GRAPH_AMDX: Self = Self(1 << 32);

        // VK_ARM_pipeline_opacity_micromap
        pub const DISALLOW_OPACITY_MICROMAP_ARM: Self = Self(1 << 37);

        // VK_EXT_descriptor_heap
        pub const DESCRIPTOR_HEAP_EXT: Self = Self(1 << 36);

        // VK_EXT_device_generated_commands
        pub const INDIRECT_BINDABLE_EXT: Self = Self(1 << 38);

        // VK_EXT_legacy_dithering
        pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(1 << 34);

        // VK_EXT_shader_64bit_indexing
        pub const _64_INDEXING_EXT: Self = Self(1 << 43);

        // VK_KHR_maintenance5
        pub const DEFER_COMPILE_NV: Self = Self(1 << 5);
        pub const CAPTURE_STATISTICS_KHR: Self = Self(1 << 6);
        pub const CAPTURE_INTERNAL_REPRESENTATIONS_KHR: Self = Self(1 << 7);
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
        pub const RAY_TRACING_DISPLACEMENT_MICROMAP_NV: Self = Self(1 << 28);
        pub const DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 29);
        pub const DISABLE_OPTIMIZATION_KHR: Self = Self::DISABLE_OPTIMIZATION;
        pub const ALLOW_DERIVATIVES_KHR: Self = Self::ALLOW_DERIVATIVES;
        pub const DERIVATIVE_KHR: Self = Self::DERIVATIVE;
        pub const VIEW_INDEX_FROM_DEVICE_INDEX_KHR: Self = Self::VIEW_INDEX_FROM_DEVICE_INDEX;
        pub const DISPATCH_BASE_KHR: Self = Self::DISPATCH_BASE;
        pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_KHR: Self =
            Self::FAIL_ON_PIPELINE_COMPILE_REQUIRED;
        pub const EARLY_RETURN_ON_FAILURE_KHR: Self = Self::EARLY_RETURN_ON_FAILURE;
        pub const NO_PROTECTED_ACCESS_EXT: Self = Self::NO_PROTECTED_ACCESS;
        pub const PROTECTED_ACCESS_ONLY_EXT: Self = Self::PROTECTED_ACCESS_ONLY;

        // VK_KHR_pipeline_binary
        pub const CAPTURE_DATA_KHR: Self = Self(1 << 31);

        // VK_KHR_ray_tracing_pipeline
        pub const RAY_TRACING_SKIP_BUILT_IN_PRIMITIVES_KHR: Self =
            Self::RAY_TRACING_SKIP_TRIANGLES_KHR;

        // VK_NV_ray_tracing_linear_swept_spheres
        pub const RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV: Self = Self(1 << 33);

        // VK_VALVE_fragment_density_map_layered
        pub const PER_LAYER_FRAGMENT_DENSITY_VALVE: Self = Self(1 << 40);
    }

    impl fmt::Debug for PipelineCreateFlagBits2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DISABLE_OPTIMIZATION => Some("DISABLE_OPTIMIZATION"),
                Self::ALLOW_DERIVATIVES => Some("ALLOW_DERIVATIVES"),
                Self::DERIVATIVE => Some("DERIVATIVE"),
                Self::VIEW_INDEX_FROM_DEVICE_INDEX => Some("VIEW_INDEX_FROM_DEVICE_INDEX"),
                Self::DISPATCH_BASE => Some("DISPATCH_BASE"),
                Self::FAIL_ON_PIPELINE_COMPILE_REQUIRED => {
                    Some("FAIL_ON_PIPELINE_COMPILE_REQUIRED")
                }
                Self::EARLY_RETURN_ON_FAILURE => Some("EARLY_RETURN_ON_FAILURE"),
                Self::NO_PROTECTED_ACCESS => Some("NO_PROTECTED_ACCESS"),
                Self::PROTECTED_ACCESS_ONLY => Some("PROTECTED_ACCESS_ONLY"),
                Self::EXECUTION_GRAPH_AMDX => Some("EXECUTION_GRAPH_AMDX"),
                Self::DISALLOW_OPACITY_MICROMAP_ARM => Some("DISALLOW_OPACITY_MICROMAP_ARM"),
                Self::DESCRIPTOR_HEAP_EXT => Some("DESCRIPTOR_HEAP_EXT"),
                Self::INDIRECT_BINDABLE_EXT => Some("INDIRECT_BINDABLE_EXT"),
                Self::ENABLE_LEGACY_DITHERING_EXT => Some("ENABLE_LEGACY_DITHERING_EXT"),
                Self::_64_INDEXING_EXT => Some("_64_INDEXING_EXT"),
                Self::DEFER_COMPILE_NV => Some("DEFER_COMPILE_NV"),
                Self::CAPTURE_STATISTICS_KHR => Some("CAPTURE_STATISTICS_KHR"),
                Self::CAPTURE_INTERNAL_REPRESENTATIONS_KHR => {
                    Some("CAPTURE_INTERNAL_REPRESENTATIONS_KHR")
                }
                Self::LINK_TIME_OPTIMIZATION_EXT => Some("LINK_TIME_OPTIMIZATION_EXT"),
                Self::LIBRARY_KHR => Some("LIBRARY_KHR"),
                Self::RAY_TRACING_SKIP_TRIANGLES_KHR => Some("RAY_TRACING_SKIP_TRIANGLES_KHR"),
                Self::RAY_TRACING_SKIP_AABBS_KHR => Some("RAY_TRACING_SKIP_AABBS_KHR"),
                Self::RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR => {
                    Some("RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR")
                }
                Self::RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR => {
                    Some("RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR")
                }
                Self::RAY_TRACING_NO_NULL_MISS_SHADERS_KHR => {
                    Some("RAY_TRACING_NO_NULL_MISS_SHADERS_KHR")
                }
                Self::RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR => {
                    Some("RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR")
                }
                Self::INDIRECT_BINDABLE_NV => Some("INDIRECT_BINDABLE_NV"),
                Self::RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR => {
                    Some("RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR")
                }
                Self::RAY_TRACING_ALLOW_MOTION_NV => Some("RAY_TRACING_ALLOW_MOTION_NV"),
                Self::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR => {
                    Some("RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR")
                }
                Self::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT => {
                    Some("RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT")
                }
                Self::RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT => {
                    Some("RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT")
                }
                Self::RAY_TRACING_OPACITY_MICROMAP_EXT => Some("RAY_TRACING_OPACITY_MICROMAP_EXT"),
                Self::COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT => {
                    Some("COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT")
                }
                Self::DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT => {
                    Some("DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT")
                }
                Self::RAY_TRACING_DISPLACEMENT_MICROMAP_NV => {
                    Some("RAY_TRACING_DISPLACEMENT_MICROMAP_NV")
                }
                Self::DESCRIPTOR_BUFFER_EXT => Some("DESCRIPTOR_BUFFER_EXT"),
                Self::CAPTURE_DATA_KHR => Some("CAPTURE_DATA_KHR"),
                Self::RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV => {
                    Some("RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV")
                }
                Self::PER_LAYER_FRAGMENT_DENSITY_VALVE => Some("PER_LAYER_FRAGMENT_DENSITY_VALVE"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferUsageFlags2.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct BufferUsageFlags2(Flags64);
    vk_bitflags_wrapped!(BufferUsageFlags2, Flags64);

    impl BufferUsageFlags2 {
        pub const TRANSFER_SRC: Self = Self(BufferUsageFlagBits2::TRANSFER_SRC.0);
        pub const TRANSFER_DST: Self = Self(BufferUsageFlagBits2::TRANSFER_DST.0);
        pub const UNIFORM_TEXEL_BUFFER: Self = Self(BufferUsageFlagBits2::UNIFORM_TEXEL_BUFFER.0);
        pub const STORAGE_TEXEL_BUFFER: Self = Self(BufferUsageFlagBits2::STORAGE_TEXEL_BUFFER.0);
        pub const UNIFORM_BUFFER: Self = Self(BufferUsageFlagBits2::UNIFORM_BUFFER.0);
        pub const STORAGE_BUFFER: Self = Self(BufferUsageFlagBits2::STORAGE_BUFFER.0);
        pub const INDEX_BUFFER: Self = Self(BufferUsageFlagBits2::INDEX_BUFFER.0);
        pub const VERTEX_BUFFER: Self = Self(BufferUsageFlagBits2::VERTEX_BUFFER.0);
        pub const INDIRECT_BUFFER: Self = Self(BufferUsageFlagBits2::INDIRECT_BUFFER.0);
        // VK_AMDX_dense_geometry_format
        pub const COMPRESSED_DATA_DGF1_AMDX: Self =
            Self(BufferUsageFlagBits2::COMPRESSED_DATA_DGF1_AMDX.0);

        // VK_AMDX_shader_enqueue
        pub const EXECUTION_GRAPH_SCRATCH_AMDX: Self =
            Self(BufferUsageFlagBits2::EXECUTION_GRAPH_SCRATCH_AMDX.0);

        // VK_ARM_data_graph
        pub const DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM: Self =
            Self(BufferUsageFlagBits2::DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM.0);

        // VK_EXT_descriptor_heap
        pub const DESCRIPTOR_HEAP_EXT: Self = Self(BufferUsageFlagBits2::DESCRIPTOR_HEAP_EXT.0);

        // VK_EXT_device_generated_commands
        pub const PREPROCESS_BUFFER_EXT: Self = Self(BufferUsageFlagBits2::PREPROCESS_BUFFER_EXT.0);

        // VK_EXT_memory_decompression
        pub const MEMORY_DECOMPRESSION_EXT: Self =
            Self(BufferUsageFlagBits2::MEMORY_DECOMPRESSION_EXT.0);

        // VK_KHR_maintenance5
        pub const CONDITIONAL_RENDERING_EXT: Self =
            Self(BufferUsageFlagBits2::CONDITIONAL_RENDERING_EXT.0);
        pub const SHADER_BINDING_TABLE_KHR: Self =
            Self(BufferUsageFlagBits2::SHADER_BINDING_TABLE_KHR.0);
        pub const TRANSFORM_FEEDBACK_BUFFER_EXT: Self =
            Self(BufferUsageFlagBits2::TRANSFORM_FEEDBACK_BUFFER_EXT.0);
        pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT: Self =
            Self(BufferUsageFlagBits2::TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT.0);
        pub const VIDEO_DECODE_SRC_KHR: Self = Self(BufferUsageFlagBits2::VIDEO_DECODE_SRC_KHR.0);
        pub const VIDEO_DECODE_DST_KHR: Self = Self(BufferUsageFlagBits2::VIDEO_DECODE_DST_KHR.0);
        pub const VIDEO_ENCODE_DST_KHR: Self = Self(BufferUsageFlagBits2::VIDEO_ENCODE_DST_KHR.0);
        pub const VIDEO_ENCODE_SRC_KHR: Self = Self(BufferUsageFlagBits2::VIDEO_ENCODE_SRC_KHR.0);
        pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR: Self =
            Self(BufferUsageFlagBits2::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR.0);
        pub const ACCELERATION_STRUCTURE_STORAGE_KHR: Self =
            Self(BufferUsageFlagBits2::ACCELERATION_STRUCTURE_STORAGE_KHR.0);
        pub const SAMPLER_DESCRIPTOR_BUFFER_EXT: Self =
            Self(BufferUsageFlagBits2::SAMPLER_DESCRIPTOR_BUFFER_EXT.0);
        pub const RESOURCE_DESCRIPTOR_BUFFER_EXT: Self =
            Self(BufferUsageFlagBits2::RESOURCE_DESCRIPTOR_BUFFER_EXT.0);
        pub const MICROMAP_BUILD_INPUT_READ_ONLY_EXT: Self =
            Self(BufferUsageFlagBits2::MICROMAP_BUILD_INPUT_READ_ONLY_EXT.0);
        pub const MICROMAP_STORAGE_EXT: Self = Self(BufferUsageFlagBits2::MICROMAP_STORAGE_EXT.0);
        pub const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT: Self =
            Self(BufferUsageFlagBits2::PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT.0);
        pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
        pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
        pub const UNIFORM_TEXEL_BUFFER_KHR: Self = Self::UNIFORM_TEXEL_BUFFER;
        pub const STORAGE_TEXEL_BUFFER_KHR: Self = Self::STORAGE_TEXEL_BUFFER;
        pub const UNIFORM_BUFFER_KHR: Self = Self::UNIFORM_BUFFER;
        pub const STORAGE_BUFFER_KHR: Self = Self::STORAGE_BUFFER;
        pub const INDEX_BUFFER_KHR: Self = Self::INDEX_BUFFER;
        pub const VERTEX_BUFFER_KHR: Self = Self::VERTEX_BUFFER;
        pub const INDIRECT_BUFFER_KHR: Self = Self::INDIRECT_BUFFER;
        pub const RAY_TRACING_NV: Self = Self::SHADER_BINDING_TABLE_KHR;
        pub const SHADER_DEVICE_ADDRESS_KHR: Self = Self::SHADER_DEVICE_ADDRESS;

        // VK_QCOM_tile_memory_heap
        pub const TILE_MEMORY_QCOM: Self = Self(BufferUsageFlagBits2::TILE_MEMORY_QCOM.0);

        // VK_VERSION_1_4
        pub const SHADER_DEVICE_ADDRESS: Self = Self(BufferUsageFlagBits2::SHADER_DEVICE_ADDRESS.0);
    }

    impl fmt::Debug for BufferUsageFlags2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags64, &str)] = &[
                (BufferUsageFlags2::TRANSFER_SRC.0, "TRANSFER_SRC"),
                (BufferUsageFlags2::TRANSFER_DST.0, "TRANSFER_DST"),
                (
                    BufferUsageFlags2::UNIFORM_TEXEL_BUFFER.0,
                    "UNIFORM_TEXEL_BUFFER",
                ),
                (
                    BufferUsageFlags2::STORAGE_TEXEL_BUFFER.0,
                    "STORAGE_TEXEL_BUFFER",
                ),
                (BufferUsageFlags2::UNIFORM_BUFFER.0, "UNIFORM_BUFFER"),
                (BufferUsageFlags2::STORAGE_BUFFER.0, "STORAGE_BUFFER"),
                (BufferUsageFlags2::INDEX_BUFFER.0, "INDEX_BUFFER"),
                (BufferUsageFlags2::VERTEX_BUFFER.0, "VERTEX_BUFFER"),
                (BufferUsageFlags2::INDIRECT_BUFFER.0, "INDIRECT_BUFFER"),
                (
                    BufferUsageFlags2::COMPRESSED_DATA_DGF1_AMDX.0,
                    "COMPRESSED_DATA_DGF1_AMDX",
                ),
                (
                    BufferUsageFlags2::EXECUTION_GRAPH_SCRATCH_AMDX.0,
                    "EXECUTION_GRAPH_SCRATCH_AMDX",
                ),
                (
                    BufferUsageFlags2::DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM.0,
                    "DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM",
                ),
                (
                    BufferUsageFlags2::DESCRIPTOR_HEAP_EXT.0,
                    "DESCRIPTOR_HEAP_EXT",
                ),
                (
                    BufferUsageFlags2::PREPROCESS_BUFFER_EXT.0,
                    "PREPROCESS_BUFFER_EXT",
                ),
                (
                    BufferUsageFlags2::MEMORY_DECOMPRESSION_EXT.0,
                    "MEMORY_DECOMPRESSION_EXT",
                ),
                (
                    BufferUsageFlags2::CONDITIONAL_RENDERING_EXT.0,
                    "CONDITIONAL_RENDERING_EXT",
                ),
                (
                    BufferUsageFlags2::SHADER_BINDING_TABLE_KHR.0,
                    "SHADER_BINDING_TABLE_KHR",
                ),
                (
                    BufferUsageFlags2::TRANSFORM_FEEDBACK_BUFFER_EXT.0,
                    "TRANSFORM_FEEDBACK_BUFFER_EXT",
                ),
                (
                    BufferUsageFlags2::TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT.0,
                    "TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT",
                ),
                (
                    BufferUsageFlags2::VIDEO_DECODE_SRC_KHR.0,
                    "VIDEO_DECODE_SRC_KHR",
                ),
                (
                    BufferUsageFlags2::VIDEO_DECODE_DST_KHR.0,
                    "VIDEO_DECODE_DST_KHR",
                ),
                (
                    BufferUsageFlags2::VIDEO_ENCODE_DST_KHR.0,
                    "VIDEO_ENCODE_DST_KHR",
                ),
                (
                    BufferUsageFlags2::VIDEO_ENCODE_SRC_KHR.0,
                    "VIDEO_ENCODE_SRC_KHR",
                ),
                (
                    BufferUsageFlags2::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR.0,
                    "ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR",
                ),
                (
                    BufferUsageFlags2::ACCELERATION_STRUCTURE_STORAGE_KHR.0,
                    "ACCELERATION_STRUCTURE_STORAGE_KHR",
                ),
                (
                    BufferUsageFlags2::SAMPLER_DESCRIPTOR_BUFFER_EXT.0,
                    "SAMPLER_DESCRIPTOR_BUFFER_EXT",
                ),
                (
                    BufferUsageFlags2::RESOURCE_DESCRIPTOR_BUFFER_EXT.0,
                    "RESOURCE_DESCRIPTOR_BUFFER_EXT",
                ),
                (
                    BufferUsageFlags2::MICROMAP_BUILD_INPUT_READ_ONLY_EXT.0,
                    "MICROMAP_BUILD_INPUT_READ_ONLY_EXT",
                ),
                (
                    BufferUsageFlags2::MICROMAP_STORAGE_EXT.0,
                    "MICROMAP_STORAGE_EXT",
                ),
                (
                    BufferUsageFlags2::PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT.0,
                    "PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT",
                ),
                (BufferUsageFlags2::TILE_MEMORY_QCOM.0, "TILE_MEMORY_QCOM"),
                (
                    BufferUsageFlags2::SHADER_DEVICE_ADDRESS.0,
                    "SHADER_DEVICE_ADDRESS",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferUsageFlagBits2.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
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
        // VK_AMDX_dense_geometry_format
        pub const COMPRESSED_DATA_DGF1_AMDX: Self = Self(1 << 33);

        // VK_AMDX_shader_enqueue
        pub const EXECUTION_GRAPH_SCRATCH_AMDX: Self = Self(1 << 25);

        // VK_ARM_data_graph
        pub const DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM: Self = Self(1 << 29);

        // VK_EXT_descriptor_heap
        pub const DESCRIPTOR_HEAP_EXT: Self = Self(1 << 28);

        // VK_EXT_device_generated_commands
        pub const PREPROCESS_BUFFER_EXT: Self = Self(1 << 31);

        // VK_EXT_memory_decompression
        pub const MEMORY_DECOMPRESSION_EXT: Self = Self(1 << 32);

        // VK_KHR_maintenance5
        pub const CONDITIONAL_RENDERING_EXT: Self = Self(1 << 9);
        pub const SHADER_BINDING_TABLE_KHR: Self = Self(1 << 10);
        pub const TRANSFORM_FEEDBACK_BUFFER_EXT: Self = Self(1 << 11);
        pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT: Self = Self(1 << 12);
        pub const VIDEO_DECODE_SRC_KHR: Self = Self(1 << 13);
        pub const VIDEO_DECODE_DST_KHR: Self = Self(1 << 14);
        pub const VIDEO_ENCODE_DST_KHR: Self = Self(1 << 15);
        pub const VIDEO_ENCODE_SRC_KHR: Self = Self(1 << 16);
        pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR: Self = Self(1 << 19);
        pub const ACCELERATION_STRUCTURE_STORAGE_KHR: Self = Self(1 << 20);
        pub const SAMPLER_DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 21);
        pub const RESOURCE_DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 22);
        pub const MICROMAP_BUILD_INPUT_READ_ONLY_EXT: Self = Self(1 << 23);
        pub const MICROMAP_STORAGE_EXT: Self = Self(1 << 24);
        pub const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 26);
        pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
        pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
        pub const UNIFORM_TEXEL_BUFFER_KHR: Self = Self::UNIFORM_TEXEL_BUFFER;
        pub const STORAGE_TEXEL_BUFFER_KHR: Self = Self::STORAGE_TEXEL_BUFFER;
        pub const UNIFORM_BUFFER_KHR: Self = Self::UNIFORM_BUFFER;
        pub const STORAGE_BUFFER_KHR: Self = Self::STORAGE_BUFFER;
        pub const INDEX_BUFFER_KHR: Self = Self::INDEX_BUFFER;
        pub const VERTEX_BUFFER_KHR: Self = Self::VERTEX_BUFFER;
        pub const INDIRECT_BUFFER_KHR: Self = Self::INDIRECT_BUFFER;
        pub const RAY_TRACING_NV: Self = Self::SHADER_BINDING_TABLE_KHR;
        pub const SHADER_DEVICE_ADDRESS_KHR: Self = Self::SHADER_DEVICE_ADDRESS;

        // VK_QCOM_tile_memory_heap
        pub const TILE_MEMORY_QCOM: Self = Self(1 << 27);

        // VK_VERSION_1_4
        pub const SHADER_DEVICE_ADDRESS: Self = Self(1 << 17);
    }

    impl fmt::Debug for BufferUsageFlagBits2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::TRANSFER_SRC => Some("TRANSFER_SRC"),
                Self::TRANSFER_DST => Some("TRANSFER_DST"),
                Self::UNIFORM_TEXEL_BUFFER => Some("UNIFORM_TEXEL_BUFFER"),
                Self::STORAGE_TEXEL_BUFFER => Some("STORAGE_TEXEL_BUFFER"),
                Self::UNIFORM_BUFFER => Some("UNIFORM_BUFFER"),
                Self::STORAGE_BUFFER => Some("STORAGE_BUFFER"),
                Self::INDEX_BUFFER => Some("INDEX_BUFFER"),
                Self::VERTEX_BUFFER => Some("VERTEX_BUFFER"),
                Self::INDIRECT_BUFFER => Some("INDIRECT_BUFFER"),
                Self::COMPRESSED_DATA_DGF1_AMDX => Some("COMPRESSED_DATA_DGF1_AMDX"),
                Self::EXECUTION_GRAPH_SCRATCH_AMDX => Some("EXECUTION_GRAPH_SCRATCH_AMDX"),
                Self::DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM => {
                    Some("DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM")
                }
                Self::DESCRIPTOR_HEAP_EXT => Some("DESCRIPTOR_HEAP_EXT"),
                Self::PREPROCESS_BUFFER_EXT => Some("PREPROCESS_BUFFER_EXT"),
                Self::MEMORY_DECOMPRESSION_EXT => Some("MEMORY_DECOMPRESSION_EXT"),
                Self::CONDITIONAL_RENDERING_EXT => Some("CONDITIONAL_RENDERING_EXT"),
                Self::SHADER_BINDING_TABLE_KHR => Some("SHADER_BINDING_TABLE_KHR"),
                Self::TRANSFORM_FEEDBACK_BUFFER_EXT => Some("TRANSFORM_FEEDBACK_BUFFER_EXT"),
                Self::TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT => {
                    Some("TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT")
                }
                Self::VIDEO_DECODE_SRC_KHR => Some("VIDEO_DECODE_SRC_KHR"),
                Self::VIDEO_DECODE_DST_KHR => Some("VIDEO_DECODE_DST_KHR"),
                Self::VIDEO_ENCODE_DST_KHR => Some("VIDEO_ENCODE_DST_KHR"),
                Self::VIDEO_ENCODE_SRC_KHR => Some("VIDEO_ENCODE_SRC_KHR"),
                Self::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR => {
                    Some("ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR")
                }
                Self::ACCELERATION_STRUCTURE_STORAGE_KHR => {
                    Some("ACCELERATION_STRUCTURE_STORAGE_KHR")
                }
                Self::SAMPLER_DESCRIPTOR_BUFFER_EXT => Some("SAMPLER_DESCRIPTOR_BUFFER_EXT"),
                Self::RESOURCE_DESCRIPTOR_BUFFER_EXT => Some("RESOURCE_DESCRIPTOR_BUFFER_EXT"),
                Self::MICROMAP_BUILD_INPUT_READ_ONLY_EXT => {
                    Some("MICROMAP_BUILD_INPUT_READ_ONLY_EXT")
                }
                Self::MICROMAP_STORAGE_EXT => Some("MICROMAP_STORAGE_EXT"),
                Self::PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT => {
                    Some("PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT")
                }
                Self::TILE_MEMORY_QCOM => Some("TILE_MEMORY_QCOM"),
                Self::SHADER_DEVICE_ADDRESS => Some("SHADER_DEVICE_ADDRESS"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkHostImageCopyFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct HostImageCopyFlags(Flags);
    vk_bitflags_wrapped!(HostImageCopyFlags, Flags);

    impl HostImageCopyFlags {
        pub const MEMCPY: Self = Self(HostImageCopyFlagBits::MEMCPY.0);
        // VK_EXT_host_image_copy
        pub const MEMCPY_EXT: Self = Self::MEMCPY;
    }

    impl fmt::Debug for HostImageCopyFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[(HostImageCopyFlags::MEMCPY.0, "MEMCPY")];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkHostImageCopyFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct HostImageCopyFlagBits(u32);

    impl HostImageCopyFlagBits {
        pub const MEMCPY: Self = Self(1 << 0);
        // VK_EXT_host_image_copy
        pub const MEMCPY_EXT: Self = Self::MEMCPY;
    }

    impl fmt::Debug for HostImageCopyFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::MEMCPY => Some("MEMCPY"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetRenderingAreaGranularity.html>
    pub type PFN_vkGetRenderingAreaGranularity = unsafe extern "system" fn(
        device: Device,
        p_rendering_area_info: *const RenderingAreaInfo<'_>,
        p_granularity: *mut Extent2D,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushDescriptorSet.html>
    pub type PFN_vkCmdPushDescriptorSet = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushDescriptorSetWithTemplate.html>
    pub type PFN_vkCmdPushDescriptorSetWithTemplate = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        p_data: *const c_void,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetLineStipple.html>
    pub type PFN_vkCmdSetLineStipple = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindIndexBuffer2.html>
    pub type PFN_vkCmdBindIndexBuffer2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyMemoryToImage.html>
    pub type PFN_vkCopyMemoryToImage = unsafe extern "system" fn(
        device: Device,
        p_copy_memory_to_image_info: *const CopyMemoryToImageInfo<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyImageToMemory.html>
    pub type PFN_vkCopyImageToMemory = unsafe extern "system" fn(
        device: Device,
        p_copy_image_to_memory_info: *const CopyImageToMemoryInfo<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyImageToImage.html>
    pub type PFN_vkCopyImageToImage = unsafe extern "system" fn(
        device: Device,
        p_copy_image_to_image_info: *const CopyImageToImageInfo<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkTransitionImageLayout.html>
    pub type PFN_vkTransitionImageLayout = unsafe extern "system" fn(
        device: Device,
        transition_count: u32,
        p_transitions: *const HostImageLayoutTransitionInfo<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageSubresourceLayout2.html>
    pub type PFN_vkGetImageSubresourceLayout2 = unsafe extern "system" fn(
        device: Device,
        image: Image,
        p_subresource: *const ImageSubresource2<'_>,
        p_layout: *mut SubresourceLayout2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceImageSubresourceLayout.html>
    pub type PFN_vkGetDeviceImageSubresourceLayout = unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceImageSubresourceInfo<'_>,
        p_layout: *mut SubresourceLayout2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkMapMemory2.html>
    pub type PFN_vkMapMemory2 = unsafe extern "system" fn(
        device: Device,
        p_memory_map_info: *const MemoryMapInfo<'_>,
        pp_data: *mut *mut c_void,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUnmapMemory2.html>
    pub type PFN_vkUnmapMemory2 = unsafe extern "system" fn(
        device: Device,
        p_memory_unmap_info: *const MemoryUnmapInfo<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindDescriptorSets2.html>
    pub type PFN_vkCmdBindDescriptorSets2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_bind_descriptor_sets_info: *const BindDescriptorSetsInfo<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushConstants2.html>
    pub type PFN_vkCmdPushConstants2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_push_constants_info: *const PushConstantsInfo<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushDescriptorSet2.html>
    pub type PFN_vkCmdPushDescriptorSet2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_push_descriptor_set_info: *const PushDescriptorSetInfo<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushDescriptorSetWithTemplate2.html>
    pub type PFN_vkCmdPushDescriptorSetWithTemplate2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRenderingAttachmentLocations.html>
    pub type PFN_vkCmdSetRenderingAttachmentLocations = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_location_info: *const RenderingAttachmentLocationInfo<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRenderingInputAttachmentIndices.html>
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
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                map_memory2: transmute(load(c"vkMapMemory2").ok_or(MissingEntryPointError)?),
                unmap_memory2: transmute(load(c"vkUnmapMemory2").ok_or(MissingEntryPointError)?),
                get_device_image_subresource_layout: transmute(
                    load(c"vkGetDeviceImageSubresourceLayout").ok_or(MissingEntryPointError)?,
                ),
                get_image_subresource_layout2: transmute(
                    load(c"vkGetImageSubresourceLayout2").ok_or(MissingEntryPointError)?,
                ),
                copy_memory_to_image: transmute(
                    load(c"vkCopyMemoryToImage").ok_or(MissingEntryPointError)?,
                ),
                copy_image_to_memory: transmute(
                    load(c"vkCopyImageToMemory").ok_or(MissingEntryPointError)?,
                ),
                copy_image_to_image: transmute(
                    load(c"vkCopyImageToImage").ok_or(MissingEntryPointError)?,
                ),
                transition_image_layout: transmute(
                    load(c"vkTransitionImageLayout").ok_or(MissingEntryPointError)?,
                ),
                cmd_push_descriptor_set: transmute(
                    load(c"vkCmdPushDescriptorSet").ok_or(MissingEntryPointError)?,
                ),
                cmd_push_descriptor_set_with_template: transmute(
                    load(c"vkCmdPushDescriptorSetWithTemplate").ok_or(MissingEntryPointError)?,
                ),
                cmd_bind_descriptor_sets2: transmute(
                    load(c"vkCmdBindDescriptorSets2").ok_or(MissingEntryPointError)?,
                ),
                cmd_push_constants2: transmute(
                    load(c"vkCmdPushConstants2").ok_or(MissingEntryPointError)?,
                ),
                cmd_push_descriptor_set2: transmute(
                    load(c"vkCmdPushDescriptorSet2").ok_or(MissingEntryPointError)?,
                ),
                cmd_push_descriptor_set_with_template2: transmute(
                    load(c"vkCmdPushDescriptorSetWithTemplate2").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_line_stipple: transmute(
                    load(c"vkCmdSetLineStipple").ok_or(MissingEntryPointError)?,
                ),
                cmd_bind_index_buffer2: transmute(
                    load(c"vkCmdBindIndexBuffer2").ok_or(MissingEntryPointError)?,
                ),
                get_rendering_area_granularity: transmute(
                    load(c"vkGetRenderingAreaGranularity").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_rendering_attachment_locations: transmute(
                    load(c"vkCmdSetRenderingAttachmentLocations").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_rendering_input_attachment_indices: transmute(
                    load(c"vkCmdSetRenderingInputAttachmentIndices")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkMapMemory2.html>
    pub unsafe fn map_memory2(
        &self,
        device: Device,
        memory_map_info: &MemoryMapInfo<'_>,
    ) -> crate::Result<*mut c_void> {
        unsafe {
            let mut data = core::mem::MaybeUninit::uninit();
            let result = (self.map_memory2)(device, memory_map_info, data.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(data.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUnmapMemory2.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceImageSubresourceLayout.html>
    pub unsafe fn get_device_image_subresource_layout(
        &self,
        device: Device,
        info: &DeviceImageSubresourceInfo<'_>,
        layout: &mut SubresourceLayout2<'_>,
    ) {
        unsafe { (self.get_device_image_subresource_layout)(device, info, layout) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageSubresourceLayout2.html>
    pub unsafe fn get_image_subresource_layout2(
        &self,
        device: Device,
        image: Image,
        subresource: &ImageSubresource2<'_>,
        layout: &mut SubresourceLayout2<'_>,
    ) {
        unsafe { (self.get_image_subresource_layout2)(device, image, subresource, layout) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyMemoryToImage.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyImageToMemory.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCopyImageToImage.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkTransitionImageLayout.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushDescriptorSet.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushDescriptorSetWithTemplate.html>
    pub unsafe fn cmd_push_descriptor_set_with_template(
        &self,
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        data: *const c_void,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindDescriptorSets2.html>
    pub unsafe fn cmd_bind_descriptor_sets2(
        &self,
        command_buffer: CommandBuffer,
        bind_descriptor_sets_info: &BindDescriptorSetsInfo<'_>,
    ) {
        unsafe { (self.cmd_bind_descriptor_sets2)(command_buffer, bind_descriptor_sets_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushConstants2.html>
    pub unsafe fn cmd_push_constants2(
        &self,
        command_buffer: CommandBuffer,
        push_constants_info: &PushConstantsInfo<'_>,
    ) {
        unsafe { (self.cmd_push_constants2)(command_buffer, push_constants_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushDescriptorSet2.html>
    pub unsafe fn cmd_push_descriptor_set2(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_info: &PushDescriptorSetInfo<'_>,
    ) {
        unsafe { (self.cmd_push_descriptor_set2)(command_buffer, push_descriptor_set_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushDescriptorSetWithTemplate2.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetLineStipple.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindIndexBuffer2.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetRenderingAreaGranularity.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRenderingAttachmentLocations.html>
    pub unsafe fn cmd_set_rendering_attachment_locations(
        &self,
        command_buffer: CommandBuffer,
        location_info: &RenderingAttachmentLocationInfo<'_>,
    ) {
        unsafe { (self.cmd_set_rendering_attachment_locations)(command_buffer, location_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRenderingInputAttachmentIndices.html>
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

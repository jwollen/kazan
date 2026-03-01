#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    pub const MAX_DRIVER_NAME_SIZE: u32 = 256;
    pub const MAX_DRIVER_INFO_SIZE: u32 = 256;
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct ConformanceVersion {
        pub major: u8,
        pub minor: u8,
        pub subminor: u8,
        pub patch: u8,
    }
    impl ConformanceVersion {
        pub fn major(mut self, major: u8) -> Self {
            self.major = major;
            self
        }
        pub fn minor(mut self, minor: u8) -> Self {
            self.minor = minor;
            self
        }
        pub fn subminor(mut self, subminor: u8) -> Self {
            self.subminor = subminor;
            self
        }
        pub fn patch(mut self, patch: u8) -> Self {
            self.patch = patch;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDriverProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub driver_id: DriverId,
        pub driver_name: [c_char; MAX_DRIVER_NAME_SIZE as usize],
        pub driver_info: [c_char; MAX_DRIVER_INFO_SIZE as usize],
        pub conformance_version: ConformanceVersion,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDriverProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_DRIVER_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>> for PhysicalDeviceDriverProperties<'a> {}
    impl Default for PhysicalDeviceDriverProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                driver_id: Default::default(),
                driver_name: [Default::default(); _],
                driver_info: [Default::default(); _],
                conformance_version: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDriverProperties<'a> {
        pub fn driver_id(mut self, driver_id: DriverId) -> Self {
            self.driver_id = driver_id;
            self
        }
        pub fn conformance_version(mut self, conformance_version: ConformanceVersion) -> Self {
            self.conformance_version = conformance_version;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_subgroup_extended_types: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderSubgroupExtendedTypesFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderSubgroupExtendedTypesFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceShaderSubgroupExtendedTypesFeatures<'a>
    {
    }
    impl Default for PhysicalDeviceShaderSubgroupExtendedTypesFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_subgroup_extended_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderSubgroupExtendedTypesFeatures<'a> {
        pub fn shader_subgroup_extended_types(
            mut self,
            shader_subgroup_extended_types: Bool32,
        ) -> Self {
            self.shader_subgroup_extended_types = shader_subgroup_extended_types;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceSamplerFilterMinmaxProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub filter_minmax_single_component_formats: Bool32,
        pub filter_minmax_image_component_mapping: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSamplerFilterMinmaxProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceSamplerFilterMinmaxProperties<'a>
    {
    }
    impl Default for PhysicalDeviceSamplerFilterMinmaxProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                filter_minmax_single_component_formats: Default::default(),
                filter_minmax_image_component_mapping: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceSamplerFilterMinmaxProperties<'a> {
        pub fn filter_minmax_single_component_formats(
            mut self,
            filter_minmax_single_component_formats: Bool32,
        ) -> Self {
            self.filter_minmax_single_component_formats = filter_minmax_single_component_formats;
            self
        }
        pub fn filter_minmax_image_component_mapping(
            mut self,
            filter_minmax_image_component_mapping: Bool32,
        ) -> Self {
            self.filter_minmax_image_component_mapping = filter_minmax_image_component_mapping;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SamplerReductionModeCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub reduction_mode: SamplerReductionMode,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SamplerReductionModeCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SAMPLER_REDUCTION_MODE_CREATE_INFO;
    }
    unsafe impl<'a> Extends<SamplerCreateInfo<'a>> for SamplerReductionModeCreateInfo<'a> {}
    impl Default for SamplerReductionModeCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                reduction_mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SamplerReductionModeCreateInfo<'a> {
        pub fn reduction_mode(mut self, reduction_mode: SamplerReductionMode) -> Self {
            self.reduction_mode = reduction_mode;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageFormatListCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub view_format_count: u32,
        pub p_view_formats: *const Format,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageFormatListCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_FORMAT_LIST_CREATE_INFO;
    }
    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for ImageFormatListCreateInfo<'a> {}
    unsafe impl<'a> Extends<SwapchainCreateInfoKHR<'a>> for ImageFormatListCreateInfo<'a> {}
    unsafe impl<'a> Extends<PhysicalDeviceImageFormatInfo2<'a>> for ImageFormatListCreateInfo<'a> {}
    impl Default for ImageFormatListCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                view_format_count: Default::default(),
                p_view_formats: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageFormatListCreateInfo<'a> {
        pub fn view_formats(mut self, view_formats: &'a [Format]) -> Self {
            self.view_format_count = view_formats.len().try_into().unwrap();
            self.p_view_formats = view_formats.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderFloat16Int8Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_float16: Bool32,
        pub shader_int8: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderFloat16Int8Features<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderFloat16Int8Features<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShaderFloat16Int8Features<'a> {}
    impl Default for PhysicalDeviceShaderFloat16Int8Features<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_float16: Default::default(),
                shader_int8: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderFloat16Int8Features<'a> {
        pub fn shader_float16(mut self, shader_float16: Bool32) -> Self {
            self.shader_float16 = shader_float16;
            self
        }
        pub fn shader_int8(mut self, shader_int8: Bool32) -> Self {
            self.shader_int8 = shader_int8;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceFloatControlsProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub denorm_behavior_independence: ShaderFloatControlsIndependence,
        pub rounding_mode_independence: ShaderFloatControlsIndependence,
        pub shader_signed_zero_inf_nan_preserve_float16: Bool32,
        pub shader_signed_zero_inf_nan_preserve_float32: Bool32,
        pub shader_signed_zero_inf_nan_preserve_float64: Bool32,
        pub shader_denorm_preserve_float16: Bool32,
        pub shader_denorm_preserve_float32: Bool32,
        pub shader_denorm_preserve_float64: Bool32,
        pub shader_denorm_flush_to_zero_float16: Bool32,
        pub shader_denorm_flush_to_zero_float32: Bool32,
        pub shader_denorm_flush_to_zero_float64: Bool32,
        pub shader_rounding_mode_rte_float16: Bool32,
        pub shader_rounding_mode_rte_float32: Bool32,
        pub shader_rounding_mode_rte_float64: Bool32,
        pub shader_rounding_mode_rtz_float16: Bool32,
        pub shader_rounding_mode_rtz_float32: Bool32,
        pub shader_rounding_mode_rtz_float64: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFloatControlsProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceFloatControlsProperties<'a>
    {
    }
    impl Default for PhysicalDeviceFloatControlsProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                denorm_behavior_independence: Default::default(),
                rounding_mode_independence: Default::default(),
                shader_signed_zero_inf_nan_preserve_float16: Default::default(),
                shader_signed_zero_inf_nan_preserve_float32: Default::default(),
                shader_signed_zero_inf_nan_preserve_float64: Default::default(),
                shader_denorm_preserve_float16: Default::default(),
                shader_denorm_preserve_float32: Default::default(),
                shader_denorm_preserve_float64: Default::default(),
                shader_denorm_flush_to_zero_float16: Default::default(),
                shader_denorm_flush_to_zero_float32: Default::default(),
                shader_denorm_flush_to_zero_float64: Default::default(),
                shader_rounding_mode_rte_float16: Default::default(),
                shader_rounding_mode_rte_float32: Default::default(),
                shader_rounding_mode_rte_float64: Default::default(),
                shader_rounding_mode_rtz_float16: Default::default(),
                shader_rounding_mode_rtz_float32: Default::default(),
                shader_rounding_mode_rtz_float64: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceFloatControlsProperties<'a> {
        pub fn denorm_behavior_independence(
            mut self,
            denorm_behavior_independence: ShaderFloatControlsIndependence,
        ) -> Self {
            self.denorm_behavior_independence = denorm_behavior_independence;
            self
        }
        pub fn rounding_mode_independence(
            mut self,
            rounding_mode_independence: ShaderFloatControlsIndependence,
        ) -> Self {
            self.rounding_mode_independence = rounding_mode_independence;
            self
        }
        pub fn shader_signed_zero_inf_nan_preserve_float16(
            mut self,
            shader_signed_zero_inf_nan_preserve_float16: Bool32,
        ) -> Self {
            self.shader_signed_zero_inf_nan_preserve_float16 =
                shader_signed_zero_inf_nan_preserve_float16;
            self
        }
        pub fn shader_signed_zero_inf_nan_preserve_float32(
            mut self,
            shader_signed_zero_inf_nan_preserve_float32: Bool32,
        ) -> Self {
            self.shader_signed_zero_inf_nan_preserve_float32 =
                shader_signed_zero_inf_nan_preserve_float32;
            self
        }
        pub fn shader_signed_zero_inf_nan_preserve_float64(
            mut self,
            shader_signed_zero_inf_nan_preserve_float64: Bool32,
        ) -> Self {
            self.shader_signed_zero_inf_nan_preserve_float64 =
                shader_signed_zero_inf_nan_preserve_float64;
            self
        }
        pub fn shader_denorm_preserve_float16(
            mut self,
            shader_denorm_preserve_float16: Bool32,
        ) -> Self {
            self.shader_denorm_preserve_float16 = shader_denorm_preserve_float16;
            self
        }
        pub fn shader_denorm_preserve_float32(
            mut self,
            shader_denorm_preserve_float32: Bool32,
        ) -> Self {
            self.shader_denorm_preserve_float32 = shader_denorm_preserve_float32;
            self
        }
        pub fn shader_denorm_preserve_float64(
            mut self,
            shader_denorm_preserve_float64: Bool32,
        ) -> Self {
            self.shader_denorm_preserve_float64 = shader_denorm_preserve_float64;
            self
        }
        pub fn shader_denorm_flush_to_zero_float16(
            mut self,
            shader_denorm_flush_to_zero_float16: Bool32,
        ) -> Self {
            self.shader_denorm_flush_to_zero_float16 = shader_denorm_flush_to_zero_float16;
            self
        }
        pub fn shader_denorm_flush_to_zero_float32(
            mut self,
            shader_denorm_flush_to_zero_float32: Bool32,
        ) -> Self {
            self.shader_denorm_flush_to_zero_float32 = shader_denorm_flush_to_zero_float32;
            self
        }
        pub fn shader_denorm_flush_to_zero_float64(
            mut self,
            shader_denorm_flush_to_zero_float64: Bool32,
        ) -> Self {
            self.shader_denorm_flush_to_zero_float64 = shader_denorm_flush_to_zero_float64;
            self
        }
        pub fn shader_rounding_mode_rte_float16(
            mut self,
            shader_rounding_mode_rte_float16: Bool32,
        ) -> Self {
            self.shader_rounding_mode_rte_float16 = shader_rounding_mode_rte_float16;
            self
        }
        pub fn shader_rounding_mode_rte_float32(
            mut self,
            shader_rounding_mode_rte_float32: Bool32,
        ) -> Self {
            self.shader_rounding_mode_rte_float32 = shader_rounding_mode_rte_float32;
            self
        }
        pub fn shader_rounding_mode_rte_float64(
            mut self,
            shader_rounding_mode_rte_float64: Bool32,
        ) -> Self {
            self.shader_rounding_mode_rte_float64 = shader_rounding_mode_rte_float64;
            self
        }
        pub fn shader_rounding_mode_rtz_float16(
            mut self,
            shader_rounding_mode_rtz_float16: Bool32,
        ) -> Self {
            self.shader_rounding_mode_rtz_float16 = shader_rounding_mode_rtz_float16;
            self
        }
        pub fn shader_rounding_mode_rtz_float32(
            mut self,
            shader_rounding_mode_rtz_float32: Bool32,
        ) -> Self {
            self.shader_rounding_mode_rtz_float32 = shader_rounding_mode_rtz_float32;
            self
        }
        pub fn shader_rounding_mode_rtz_float64(
            mut self,
            shader_rounding_mode_rtz_float64: Bool32,
        ) -> Self {
            self.shader_rounding_mode_rtz_float64 = shader_rounding_mode_rtz_float64;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceHostQueryResetFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub host_query_reset: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceHostQueryResetFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceHostQueryResetFeatures<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceHostQueryResetFeatures<'a> {}
    impl Default for PhysicalDeviceHostQueryResetFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                host_query_reset: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceHostQueryResetFeatures<'a> {
        pub fn host_query_reset(mut self, host_query_reset: Bool32) -> Self {
            self.host_query_reset = host_query_reset;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDescriptorIndexingFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_input_attachment_array_dynamic_indexing: Bool32,
        pub shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
        pub shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
        pub shader_uniform_buffer_array_non_uniform_indexing: Bool32,
        pub shader_sampled_image_array_non_uniform_indexing: Bool32,
        pub shader_storage_buffer_array_non_uniform_indexing: Bool32,
        pub shader_storage_image_array_non_uniform_indexing: Bool32,
        pub shader_input_attachment_array_non_uniform_indexing: Bool32,
        pub shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
        pub shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
        pub descriptor_binding_uniform_buffer_update_after_bind: Bool32,
        pub descriptor_binding_sampled_image_update_after_bind: Bool32,
        pub descriptor_binding_storage_image_update_after_bind: Bool32,
        pub descriptor_binding_storage_buffer_update_after_bind: Bool32,
        pub descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
        pub descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
        pub descriptor_binding_update_unused_while_pending: Bool32,
        pub descriptor_binding_partially_bound: Bool32,
        pub descriptor_binding_variable_descriptor_count: Bool32,
        pub runtime_descriptor_array: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDescriptorIndexingFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDescriptorIndexingFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceDescriptorIndexingFeatures<'a> {}
    impl Default for PhysicalDeviceDescriptorIndexingFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_input_attachment_array_dynamic_indexing: Default::default(),
                shader_uniform_texel_buffer_array_dynamic_indexing: Default::default(),
                shader_storage_texel_buffer_array_dynamic_indexing: Default::default(),
                shader_uniform_buffer_array_non_uniform_indexing: Default::default(),
                shader_sampled_image_array_non_uniform_indexing: Default::default(),
                shader_storage_buffer_array_non_uniform_indexing: Default::default(),
                shader_storage_image_array_non_uniform_indexing: Default::default(),
                shader_input_attachment_array_non_uniform_indexing: Default::default(),
                shader_uniform_texel_buffer_array_non_uniform_indexing: Default::default(),
                shader_storage_texel_buffer_array_non_uniform_indexing: Default::default(),
                descriptor_binding_uniform_buffer_update_after_bind: Default::default(),
                descriptor_binding_sampled_image_update_after_bind: Default::default(),
                descriptor_binding_storage_image_update_after_bind: Default::default(),
                descriptor_binding_storage_buffer_update_after_bind: Default::default(),
                descriptor_binding_uniform_texel_buffer_update_after_bind: Default::default(),
                descriptor_binding_storage_texel_buffer_update_after_bind: Default::default(),
                descriptor_binding_update_unused_while_pending: Default::default(),
                descriptor_binding_partially_bound: Default::default(),
                descriptor_binding_variable_descriptor_count: Default::default(),
                runtime_descriptor_array: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDescriptorIndexingFeatures<'a> {
        pub fn shader_input_attachment_array_dynamic_indexing(
            mut self,
            shader_input_attachment_array_dynamic_indexing: Bool32,
        ) -> Self {
            self.shader_input_attachment_array_dynamic_indexing =
                shader_input_attachment_array_dynamic_indexing;
            self
        }
        pub fn shader_uniform_texel_buffer_array_dynamic_indexing(
            mut self,
            shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
        ) -> Self {
            self.shader_uniform_texel_buffer_array_dynamic_indexing =
                shader_uniform_texel_buffer_array_dynamic_indexing;
            self
        }
        pub fn shader_storage_texel_buffer_array_dynamic_indexing(
            mut self,
            shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
        ) -> Self {
            self.shader_storage_texel_buffer_array_dynamic_indexing =
                shader_storage_texel_buffer_array_dynamic_indexing;
            self
        }
        pub fn shader_uniform_buffer_array_non_uniform_indexing(
            mut self,
            shader_uniform_buffer_array_non_uniform_indexing: Bool32,
        ) -> Self {
            self.shader_uniform_buffer_array_non_uniform_indexing =
                shader_uniform_buffer_array_non_uniform_indexing;
            self
        }
        pub fn shader_sampled_image_array_non_uniform_indexing(
            mut self,
            shader_sampled_image_array_non_uniform_indexing: Bool32,
        ) -> Self {
            self.shader_sampled_image_array_non_uniform_indexing =
                shader_sampled_image_array_non_uniform_indexing;
            self
        }
        pub fn shader_storage_buffer_array_non_uniform_indexing(
            mut self,
            shader_storage_buffer_array_non_uniform_indexing: Bool32,
        ) -> Self {
            self.shader_storage_buffer_array_non_uniform_indexing =
                shader_storage_buffer_array_non_uniform_indexing;
            self
        }
        pub fn shader_storage_image_array_non_uniform_indexing(
            mut self,
            shader_storage_image_array_non_uniform_indexing: Bool32,
        ) -> Self {
            self.shader_storage_image_array_non_uniform_indexing =
                shader_storage_image_array_non_uniform_indexing;
            self
        }
        pub fn shader_input_attachment_array_non_uniform_indexing(
            mut self,
            shader_input_attachment_array_non_uniform_indexing: Bool32,
        ) -> Self {
            self.shader_input_attachment_array_non_uniform_indexing =
                shader_input_attachment_array_non_uniform_indexing;
            self
        }
        pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(
            mut self,
            shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
        ) -> Self {
            self.shader_uniform_texel_buffer_array_non_uniform_indexing =
                shader_uniform_texel_buffer_array_non_uniform_indexing;
            self
        }
        pub fn shader_storage_texel_buffer_array_non_uniform_indexing(
            mut self,
            shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
        ) -> Self {
            self.shader_storage_texel_buffer_array_non_uniform_indexing =
                shader_storage_texel_buffer_array_non_uniform_indexing;
            self
        }
        pub fn descriptor_binding_uniform_buffer_update_after_bind(
            mut self,
            descriptor_binding_uniform_buffer_update_after_bind: Bool32,
        ) -> Self {
            self.descriptor_binding_uniform_buffer_update_after_bind =
                descriptor_binding_uniform_buffer_update_after_bind;
            self
        }
        pub fn descriptor_binding_sampled_image_update_after_bind(
            mut self,
            descriptor_binding_sampled_image_update_after_bind: Bool32,
        ) -> Self {
            self.descriptor_binding_sampled_image_update_after_bind =
                descriptor_binding_sampled_image_update_after_bind;
            self
        }
        pub fn descriptor_binding_storage_image_update_after_bind(
            mut self,
            descriptor_binding_storage_image_update_after_bind: Bool32,
        ) -> Self {
            self.descriptor_binding_storage_image_update_after_bind =
                descriptor_binding_storage_image_update_after_bind;
            self
        }
        pub fn descriptor_binding_storage_buffer_update_after_bind(
            mut self,
            descriptor_binding_storage_buffer_update_after_bind: Bool32,
        ) -> Self {
            self.descriptor_binding_storage_buffer_update_after_bind =
                descriptor_binding_storage_buffer_update_after_bind;
            self
        }
        pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(
            mut self,
            descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
        ) -> Self {
            self.descriptor_binding_uniform_texel_buffer_update_after_bind =
                descriptor_binding_uniform_texel_buffer_update_after_bind;
            self
        }
        pub fn descriptor_binding_storage_texel_buffer_update_after_bind(
            mut self,
            descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
        ) -> Self {
            self.descriptor_binding_storage_texel_buffer_update_after_bind =
                descriptor_binding_storage_texel_buffer_update_after_bind;
            self
        }
        pub fn descriptor_binding_update_unused_while_pending(
            mut self,
            descriptor_binding_update_unused_while_pending: Bool32,
        ) -> Self {
            self.descriptor_binding_update_unused_while_pending =
                descriptor_binding_update_unused_while_pending;
            self
        }
        pub fn descriptor_binding_partially_bound(
            mut self,
            descriptor_binding_partially_bound: Bool32,
        ) -> Self {
            self.descriptor_binding_partially_bound = descriptor_binding_partially_bound;
            self
        }
        pub fn descriptor_binding_variable_descriptor_count(
            mut self,
            descriptor_binding_variable_descriptor_count: Bool32,
        ) -> Self {
            self.descriptor_binding_variable_descriptor_count =
                descriptor_binding_variable_descriptor_count;
            self
        }
        pub fn runtime_descriptor_array(mut self, runtime_descriptor_array: Bool32) -> Self {
            self.runtime_descriptor_array = runtime_descriptor_array;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDescriptorIndexingProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_update_after_bind_descriptors_in_all_pools: u32,
        pub shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
        pub shader_sampled_image_array_non_uniform_indexing_native: Bool32,
        pub shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
        pub shader_storage_image_array_non_uniform_indexing_native: Bool32,
        pub shader_input_attachment_array_non_uniform_indexing_native: Bool32,
        pub robust_buffer_access_update_after_bind: Bool32,
        pub quad_divergent_implicit_lod: Bool32,
        pub max_per_stage_descriptor_update_after_bind_samplers: u32,
        pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
        pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
        pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
        pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
        pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
        pub max_per_stage_update_after_bind_resources: u32,
        pub max_descriptor_set_update_after_bind_samplers: u32,
        pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
        pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
        pub max_descriptor_set_update_after_bind_storage_buffers: u32,
        pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
        pub max_descriptor_set_update_after_bind_sampled_images: u32,
        pub max_descriptor_set_update_after_bind_storage_images: u32,
        pub max_descriptor_set_update_after_bind_input_attachments: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDescriptorIndexingProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceDescriptorIndexingProperties<'a>
    {
    }
    impl Default for PhysicalDeviceDescriptorIndexingProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_update_after_bind_descriptors_in_all_pools: Default::default(),
                shader_uniform_buffer_array_non_uniform_indexing_native: Default::default(),
                shader_sampled_image_array_non_uniform_indexing_native: Default::default(),
                shader_storage_buffer_array_non_uniform_indexing_native: Default::default(),
                shader_storage_image_array_non_uniform_indexing_native: Default::default(),
                shader_input_attachment_array_non_uniform_indexing_native: Default::default(),
                robust_buffer_access_update_after_bind: Default::default(),
                quad_divergent_implicit_lod: Default::default(),
                max_per_stage_descriptor_update_after_bind_samplers: Default::default(),
                max_per_stage_descriptor_update_after_bind_uniform_buffers: Default::default(),
                max_per_stage_descriptor_update_after_bind_storage_buffers: Default::default(),
                max_per_stage_descriptor_update_after_bind_sampled_images: Default::default(),
                max_per_stage_descriptor_update_after_bind_storage_images: Default::default(),
                max_per_stage_descriptor_update_after_bind_input_attachments: Default::default(),
                max_per_stage_update_after_bind_resources: Default::default(),
                max_descriptor_set_update_after_bind_samplers: Default::default(),
                max_descriptor_set_update_after_bind_uniform_buffers: Default::default(),
                max_descriptor_set_update_after_bind_uniform_buffers_dynamic: Default::default(),
                max_descriptor_set_update_after_bind_storage_buffers: Default::default(),
                max_descriptor_set_update_after_bind_storage_buffers_dynamic: Default::default(),
                max_descriptor_set_update_after_bind_sampled_images: Default::default(),
                max_descriptor_set_update_after_bind_storage_images: Default::default(),
                max_descriptor_set_update_after_bind_input_attachments: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDescriptorIndexingProperties<'a> {
        pub fn max_update_after_bind_descriptors_in_all_pools(
            mut self,
            max_update_after_bind_descriptors_in_all_pools: u32,
        ) -> Self {
            self.max_update_after_bind_descriptors_in_all_pools =
                max_update_after_bind_descriptors_in_all_pools;
            self
        }
        pub fn shader_uniform_buffer_array_non_uniform_indexing_native(
            mut self,
            shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
        ) -> Self {
            self.shader_uniform_buffer_array_non_uniform_indexing_native =
                shader_uniform_buffer_array_non_uniform_indexing_native;
            self
        }
        pub fn shader_sampled_image_array_non_uniform_indexing_native(
            mut self,
            shader_sampled_image_array_non_uniform_indexing_native: Bool32,
        ) -> Self {
            self.shader_sampled_image_array_non_uniform_indexing_native =
                shader_sampled_image_array_non_uniform_indexing_native;
            self
        }
        pub fn shader_storage_buffer_array_non_uniform_indexing_native(
            mut self,
            shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
        ) -> Self {
            self.shader_storage_buffer_array_non_uniform_indexing_native =
                shader_storage_buffer_array_non_uniform_indexing_native;
            self
        }
        pub fn shader_storage_image_array_non_uniform_indexing_native(
            mut self,
            shader_storage_image_array_non_uniform_indexing_native: Bool32,
        ) -> Self {
            self.shader_storage_image_array_non_uniform_indexing_native =
                shader_storage_image_array_non_uniform_indexing_native;
            self
        }
        pub fn shader_input_attachment_array_non_uniform_indexing_native(
            mut self,
            shader_input_attachment_array_non_uniform_indexing_native: Bool32,
        ) -> Self {
            self.shader_input_attachment_array_non_uniform_indexing_native =
                shader_input_attachment_array_non_uniform_indexing_native;
            self
        }
        pub fn robust_buffer_access_update_after_bind(
            mut self,
            robust_buffer_access_update_after_bind: Bool32,
        ) -> Self {
            self.robust_buffer_access_update_after_bind = robust_buffer_access_update_after_bind;
            self
        }
        pub fn quad_divergent_implicit_lod(mut self, quad_divergent_implicit_lod: Bool32) -> Self {
            self.quad_divergent_implicit_lod = quad_divergent_implicit_lod;
            self
        }
        pub fn max_per_stage_descriptor_update_after_bind_samplers(
            mut self,
            max_per_stage_descriptor_update_after_bind_samplers: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_samplers =
                max_per_stage_descriptor_update_after_bind_samplers;
            self
        }
        pub fn max_per_stage_descriptor_update_after_bind_uniform_buffers(
            mut self,
            max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_uniform_buffers =
                max_per_stage_descriptor_update_after_bind_uniform_buffers;
            self
        }
        pub fn max_per_stage_descriptor_update_after_bind_storage_buffers(
            mut self,
            max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_storage_buffers =
                max_per_stage_descriptor_update_after_bind_storage_buffers;
            self
        }
        pub fn max_per_stage_descriptor_update_after_bind_sampled_images(
            mut self,
            max_per_stage_descriptor_update_after_bind_sampled_images: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_sampled_images =
                max_per_stage_descriptor_update_after_bind_sampled_images;
            self
        }
        pub fn max_per_stage_descriptor_update_after_bind_storage_images(
            mut self,
            max_per_stage_descriptor_update_after_bind_storage_images: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_storage_images =
                max_per_stage_descriptor_update_after_bind_storage_images;
            self
        }
        pub fn max_per_stage_descriptor_update_after_bind_input_attachments(
            mut self,
            max_per_stage_descriptor_update_after_bind_input_attachments: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_input_attachments =
                max_per_stage_descriptor_update_after_bind_input_attachments;
            self
        }
        pub fn max_per_stage_update_after_bind_resources(
            mut self,
            max_per_stage_update_after_bind_resources: u32,
        ) -> Self {
            self.max_per_stage_update_after_bind_resources =
                max_per_stage_update_after_bind_resources;
            self
        }
        pub fn max_descriptor_set_update_after_bind_samplers(
            mut self,
            max_descriptor_set_update_after_bind_samplers: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_samplers =
                max_descriptor_set_update_after_bind_samplers;
            self
        }
        pub fn max_descriptor_set_update_after_bind_uniform_buffers(
            mut self,
            max_descriptor_set_update_after_bind_uniform_buffers: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_uniform_buffers =
                max_descriptor_set_update_after_bind_uniform_buffers;
            self
        }
        pub fn max_descriptor_set_update_after_bind_uniform_buffers_dynamic(
            mut self,
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_uniform_buffers_dynamic =
                max_descriptor_set_update_after_bind_uniform_buffers_dynamic;
            self
        }
        pub fn max_descriptor_set_update_after_bind_storage_buffers(
            mut self,
            max_descriptor_set_update_after_bind_storage_buffers: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_storage_buffers =
                max_descriptor_set_update_after_bind_storage_buffers;
            self
        }
        pub fn max_descriptor_set_update_after_bind_storage_buffers_dynamic(
            mut self,
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_storage_buffers_dynamic =
                max_descriptor_set_update_after_bind_storage_buffers_dynamic;
            self
        }
        pub fn max_descriptor_set_update_after_bind_sampled_images(
            mut self,
            max_descriptor_set_update_after_bind_sampled_images: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_sampled_images =
                max_descriptor_set_update_after_bind_sampled_images;
            self
        }
        pub fn max_descriptor_set_update_after_bind_storage_images(
            mut self,
            max_descriptor_set_update_after_bind_storage_images: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_storage_images =
                max_descriptor_set_update_after_bind_storage_images;
            self
        }
        pub fn max_descriptor_set_update_after_bind_input_attachments(
            mut self,
            max_descriptor_set_update_after_bind_input_attachments: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_input_attachments =
                max_descriptor_set_update_after_bind_input_attachments;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DescriptorSetLayoutBindingFlagsCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub binding_count: u32,
        pub p_binding_flags: *const DescriptorBindingFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DescriptorSetLayoutBindingFlagsCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO;
    }
    unsafe impl<'a> Extends<DescriptorSetLayoutCreateInfo<'a>>
        for DescriptorSetLayoutBindingFlagsCreateInfo<'a>
    {
    }
    impl Default for DescriptorSetLayoutBindingFlagsCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                binding_count: Default::default(),
                p_binding_flags: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DescriptorSetLayoutBindingFlagsCreateInfo<'a> {
        pub fn binding_flags(mut self, binding_flags: &'a [DescriptorBindingFlags]) -> Self {
            self.binding_count = binding_flags.len().try_into().unwrap();
            self.p_binding_flags = binding_flags.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DescriptorSetVariableDescriptorCountAllocateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub descriptor_set_count: u32,
        pub p_descriptor_counts: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DescriptorSetVariableDescriptorCountAllocateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO;
    }
    unsafe impl<'a> Extends<DescriptorSetAllocateInfo<'a>>
        for DescriptorSetVariableDescriptorCountAllocateInfo<'a>
    {
    }
    impl Default for DescriptorSetVariableDescriptorCountAllocateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                descriptor_set_count: Default::default(),
                p_descriptor_counts: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DescriptorSetVariableDescriptorCountAllocateInfo<'a> {
        pub fn descriptor_counts(mut self, descriptor_counts: &'a [u32]) -> Self {
            self.descriptor_set_count = descriptor_counts.len().try_into().unwrap();
            self.p_descriptor_counts = descriptor_counts.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DescriptorSetVariableDescriptorCountLayoutSupport<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_variable_descriptor_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DescriptorSetVariableDescriptorCountLayoutSupport<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT;
    }
    unsafe impl<'a> Extends<DescriptorSetLayoutSupport<'a>>
        for DescriptorSetVariableDescriptorCountLayoutSupport<'a>
    {
    }
    impl Default for DescriptorSetVariableDescriptorCountLayoutSupport<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_variable_descriptor_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DescriptorSetVariableDescriptorCountLayoutSupport<'a> {
        pub fn max_variable_descriptor_count(mut self, max_variable_descriptor_count: u32) -> Self {
            self.max_variable_descriptor_count = max_variable_descriptor_count;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AttachmentDescription2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: AttachmentDescriptionFlags,
        pub format: Format,
        pub samples: SampleCountFlagBits,
        pub load_op: AttachmentLoadOp,
        pub store_op: AttachmentStoreOp,
        pub stencil_load_op: AttachmentLoadOp,
        pub stencil_store_op: AttachmentStoreOp,
        pub initial_layout: ImageLayout,
        pub final_layout: ImageLayout,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for AttachmentDescription2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ATTACHMENT_DESCRIPTION_2;
    }
    impl Default for AttachmentDescription2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                format: Default::default(),
                samples: Default::default(),
                load_op: Default::default(),
                store_op: Default::default(),
                stencil_load_op: Default::default(),
                stencil_store_op: Default::default(),
                initial_layout: Default::default(),
                final_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> AttachmentDescription2<'a> {
        pub fn flags(mut self, flags: AttachmentDescriptionFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
        pub fn samples(mut self, samples: SampleCountFlagBits) -> Self {
            self.samples = samples;
            self
        }
        pub fn load_op(mut self, load_op: AttachmentLoadOp) -> Self {
            self.load_op = load_op;
            self
        }
        pub fn store_op(mut self, store_op: AttachmentStoreOp) -> Self {
            self.store_op = store_op;
            self
        }
        pub fn stencil_load_op(mut self, stencil_load_op: AttachmentLoadOp) -> Self {
            self.stencil_load_op = stencil_load_op;
            self
        }
        pub fn stencil_store_op(mut self, stencil_store_op: AttachmentStoreOp) -> Self {
            self.stencil_store_op = stencil_store_op;
            self
        }
        pub fn initial_layout(mut self, initial_layout: ImageLayout) -> Self {
            self.initial_layout = initial_layout;
            self
        }
        pub fn final_layout(mut self, final_layout: ImageLayout) -> Self {
            self.final_layout = final_layout;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AttachmentReference2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub attachment: u32,
        pub layout: ImageLayout,
        pub aspect_mask: ImageAspectFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for AttachmentReference2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ATTACHMENT_REFERENCE_2;
    }
    impl Default for AttachmentReference2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                attachment: Default::default(),
                layout: Default::default(),
                aspect_mask: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> AttachmentReference2<'a> {
        pub fn attachment(mut self, attachment: u32) -> Self {
            self.attachment = attachment;
            self
        }
        pub fn layout(mut self, layout: ImageLayout) -> Self {
            self.layout = layout;
            self
        }
        pub fn aspect_mask(mut self, aspect_mask: ImageAspectFlags) -> Self {
            self.aspect_mask = aspect_mask;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SubpassDescription2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: SubpassDescriptionFlags,
        pub pipeline_bind_point: PipelineBindPoint,
        pub view_mask: u32,
        pub input_attachment_count: u32,
        pub p_input_attachments: *const AttachmentReference2<'a>,
        pub color_attachment_count: u32,
        pub p_color_attachments: *const AttachmentReference2<'a>,
        pub p_resolve_attachments: *const AttachmentReference2<'a>,
        pub p_depth_stencil_attachment: *const AttachmentReference2<'a>,
        pub preserve_attachment_count: u32,
        pub p_preserve_attachments: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SubpassDescription2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SUBPASS_DESCRIPTION_2;
    }
    impl Default for SubpassDescription2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                pipeline_bind_point: Default::default(),
                view_mask: Default::default(),
                input_attachment_count: Default::default(),
                p_input_attachments: core::ptr::null(),
                color_attachment_count: Default::default(),
                p_color_attachments: core::ptr::null(),
                p_resolve_attachments: core::ptr::null(),
                p_depth_stencil_attachment: core::ptr::null(),
                preserve_attachment_count: Default::default(),
                p_preserve_attachments: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SubpassDescription2<'a> {
        pub fn flags(mut self, flags: SubpassDescriptionFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn pipeline_bind_point(mut self, pipeline_bind_point: PipelineBindPoint) -> Self {
            self.pipeline_bind_point = pipeline_bind_point;
            self
        }
        pub fn view_mask(mut self, view_mask: u32) -> Self {
            self.view_mask = view_mask;
            self
        }
        pub fn input_attachments(
            mut self,
            input_attachments: &'a [AttachmentReference2<'a>],
        ) -> Self {
            self.input_attachment_count = input_attachments.len().try_into().unwrap();
            self.p_input_attachments = input_attachments.as_ptr();
            self
        }
        pub fn color_attachments(
            mut self,
            color_attachments: &'a [AttachmentReference2<'a>],
        ) -> Self {
            self.color_attachment_count = color_attachments.len().try_into().unwrap();
            self.p_color_attachments = color_attachments.as_ptr();
            self
        }
        pub fn resolve_attachments(
            mut self,
            resolve_attachments: &'a [AttachmentReference2<'a>],
        ) -> Self {
            self.color_attachment_count = resolve_attachments.len().try_into().unwrap();
            self.p_resolve_attachments = resolve_attachments.as_ptr();
            self
        }
        pub fn depth_stencil_attachment(
            mut self,
            depth_stencil_attachment: &'a AttachmentReference2<'a>,
        ) -> Self {
            self.p_depth_stencil_attachment = depth_stencil_attachment;
            self
        }
        pub fn preserve_attachments(mut self, preserve_attachments: &'a [u32]) -> Self {
            self.preserve_attachment_count = preserve_attachments.len().try_into().unwrap();
            self.p_preserve_attachments = preserve_attachments.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SubpassDependency2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_subpass: u32,
        pub dst_subpass: u32,
        pub src_stage_mask: PipelineStageFlags,
        pub dst_stage_mask: PipelineStageFlags,
        pub src_access_mask: AccessFlags,
        pub dst_access_mask: AccessFlags,
        pub dependency_flags: DependencyFlags,
        pub view_offset: i32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SubpassDependency2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SUBPASS_DEPENDENCY_2;
    }
    impl Default for SubpassDependency2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_subpass: Default::default(),
                dst_subpass: Default::default(),
                src_stage_mask: Default::default(),
                dst_stage_mask: Default::default(),
                src_access_mask: Default::default(),
                dst_access_mask: Default::default(),
                dependency_flags: Default::default(),
                view_offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SubpassDependency2<'a> {
        pub fn src_subpass(mut self, src_subpass: u32) -> Self {
            self.src_subpass = src_subpass;
            self
        }
        pub fn dst_subpass(mut self, dst_subpass: u32) -> Self {
            self.dst_subpass = dst_subpass;
            self
        }
        pub fn src_stage_mask(mut self, src_stage_mask: PipelineStageFlags) -> Self {
            self.src_stage_mask = src_stage_mask;
            self
        }
        pub fn dst_stage_mask(mut self, dst_stage_mask: PipelineStageFlags) -> Self {
            self.dst_stage_mask = dst_stage_mask;
            self
        }
        pub fn src_access_mask(mut self, src_access_mask: AccessFlags) -> Self {
            self.src_access_mask = src_access_mask;
            self
        }
        pub fn dst_access_mask(mut self, dst_access_mask: AccessFlags) -> Self {
            self.dst_access_mask = dst_access_mask;
            self
        }
        pub fn dependency_flags(mut self, dependency_flags: DependencyFlags) -> Self {
            self.dependency_flags = dependency_flags;
            self
        }
        pub fn view_offset(mut self, view_offset: i32) -> Self {
            self.view_offset = view_offset;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderPassCreateInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: RenderPassCreateFlags,
        pub attachment_count: u32,
        pub p_attachments: *const AttachmentDescription2<'a>,
        pub subpass_count: u32,
        pub p_subpasses: *const SubpassDescription2<'a>,
        pub dependency_count: u32,
        pub p_dependencies: *const SubpassDependency2<'a>,
        pub correlated_view_mask_count: u32,
        pub p_correlated_view_masks: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for RenderPassCreateInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDER_PASS_CREATE_INFO_2;
    }
    impl Default for RenderPassCreateInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                attachment_count: Default::default(),
                p_attachments: core::ptr::null(),
                subpass_count: Default::default(),
                p_subpasses: core::ptr::null(),
                dependency_count: Default::default(),
                p_dependencies: core::ptr::null(),
                correlated_view_mask_count: Default::default(),
                p_correlated_view_masks: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderPassCreateInfo2<'a> {
        pub fn flags(mut self, flags: RenderPassCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn attachments(mut self, attachments: &'a [AttachmentDescription2<'a>]) -> Self {
            self.attachment_count = attachments.len().try_into().unwrap();
            self.p_attachments = attachments.as_ptr();
            self
        }
        pub fn subpasses(mut self, subpasses: &'a [SubpassDescription2<'a>]) -> Self {
            self.subpass_count = subpasses.len().try_into().unwrap();
            self.p_subpasses = subpasses.as_ptr();
            self
        }
        pub fn dependencies(mut self, dependencies: &'a [SubpassDependency2<'a>]) -> Self {
            self.dependency_count = dependencies.len().try_into().unwrap();
            self.p_dependencies = dependencies.as_ptr();
            self
        }
        pub fn correlated_view_masks(mut self, correlated_view_masks: &'a [u32]) -> Self {
            self.correlated_view_mask_count = correlated_view_masks.len().try_into().unwrap();
            self.p_correlated_view_masks = correlated_view_masks.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SubpassBeginInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub contents: SubpassContents,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SubpassBeginInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SUBPASS_BEGIN_INFO;
    }
    impl Default for SubpassBeginInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                contents: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SubpassBeginInfo<'a> {
        pub fn contents(mut self, contents: SubpassContents) -> Self {
            self.contents = contents;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SubpassEndInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SubpassEndInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SUBPASS_END_INFO;
    }
    impl Default for SubpassEndInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SubpassEndInfo<'a> {}
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceTimelineSemaphoreFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub timeline_semaphore: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTimelineSemaphoreFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceTimelineSemaphoreFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceTimelineSemaphoreFeatures<'a> {}
    impl Default for PhysicalDeviceTimelineSemaphoreFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                timeline_semaphore: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceTimelineSemaphoreFeatures<'a> {
        pub fn timeline_semaphore(mut self, timeline_semaphore: Bool32) -> Self {
            self.timeline_semaphore = timeline_semaphore;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceTimelineSemaphoreProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_timeline_semaphore_value_difference: u64,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTimelineSemaphoreProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceTimelineSemaphoreProperties<'a>
    {
    }
    impl Default for PhysicalDeviceTimelineSemaphoreProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_timeline_semaphore_value_difference: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceTimelineSemaphoreProperties<'a> {
        pub fn max_timeline_semaphore_value_difference(
            mut self,
            max_timeline_semaphore_value_difference: u64,
        ) -> Self {
            self.max_timeline_semaphore_value_difference = max_timeline_semaphore_value_difference;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SemaphoreTypeCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub semaphore_type: SemaphoreType,
        pub initial_value: u64,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SemaphoreTypeCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SEMAPHORE_TYPE_CREATE_INFO;
    }
    unsafe impl<'a> Extends<SemaphoreCreateInfo<'a>> for SemaphoreTypeCreateInfo<'a> {}
    unsafe impl<'a> Extends<PhysicalDeviceExternalSemaphoreInfo<'a>> for SemaphoreTypeCreateInfo<'a> {}
    impl Default for SemaphoreTypeCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                semaphore_type: Default::default(),
                initial_value: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SemaphoreTypeCreateInfo<'a> {
        pub fn semaphore_type(mut self, semaphore_type: SemaphoreType) -> Self {
            self.semaphore_type = semaphore_type;
            self
        }
        pub fn initial_value(mut self, initial_value: u64) -> Self {
            self.initial_value = initial_value;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct TimelineSemaphoreSubmitInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub wait_semaphore_value_count: u32,
        pub p_wait_semaphore_values: *const u64,
        pub signal_semaphore_value_count: u32,
        pub p_signal_semaphore_values: *const u64,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for TimelineSemaphoreSubmitInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TIMELINE_SEMAPHORE_SUBMIT_INFO;
    }
    unsafe impl<'a> Extends<SubmitInfo<'a>> for TimelineSemaphoreSubmitInfo<'a> {}
    unsafe impl<'a> Extends<BindSparseInfo<'a>> for TimelineSemaphoreSubmitInfo<'a> {}
    impl Default for TimelineSemaphoreSubmitInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                wait_semaphore_value_count: Default::default(),
                p_wait_semaphore_values: core::ptr::null(),
                signal_semaphore_value_count: Default::default(),
                p_signal_semaphore_values: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> TimelineSemaphoreSubmitInfo<'a> {
        pub fn wait_semaphore_values(mut self, wait_semaphore_values: &'a [u64]) -> Self {
            self.wait_semaphore_value_count = wait_semaphore_values.len().try_into().unwrap();
            self.p_wait_semaphore_values = wait_semaphore_values.as_ptr();
            self
        }
        pub fn signal_semaphore_values(mut self, signal_semaphore_values: &'a [u64]) -> Self {
            self.signal_semaphore_value_count = signal_semaphore_values.len().try_into().unwrap();
            self.p_signal_semaphore_values = signal_semaphore_values.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SemaphoreWaitInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: SemaphoreWaitFlags,
        pub semaphore_count: u32,
        pub p_semaphores: *const Semaphore,
        pub p_values: *const u64,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SemaphoreWaitInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SEMAPHORE_WAIT_INFO;
    }
    impl Default for SemaphoreWaitInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                semaphore_count: Default::default(),
                p_semaphores: core::ptr::null(),
                p_values: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SemaphoreWaitInfo<'a> {
        pub fn flags(mut self, flags: SemaphoreWaitFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn semaphores(mut self, semaphores: &'a [Semaphore]) -> Self {
            self.semaphore_count = semaphores.len().try_into().unwrap();
            self.p_semaphores = semaphores.as_ptr();
            self
        }
        pub fn values(mut self, values: &'a [u64]) -> Self {
            self.semaphore_count = values.len().try_into().unwrap();
            self.p_values = values.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SemaphoreSignalInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub semaphore: Semaphore,
        pub value: u64,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SemaphoreSignalInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SEMAPHORE_SIGNAL_INFO;
    }
    impl Default for SemaphoreSignalInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                semaphore: Default::default(),
                value: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SemaphoreSignalInfo<'a> {
        pub fn semaphore(mut self, semaphore: Semaphore) -> Self {
            self.semaphore = semaphore;
            self
        }
        pub fn value(mut self, value: u64) -> Self {
            self.value = value;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevice8BitStorageFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub storage_buffer8_bit_access: Bool32,
        pub uniform_and_storage_buffer8_bit_access: Bool32,
        pub storage_push_constant8: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevice8BitStorageFeatures<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDevice8BitStorageFeatures<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDevice8BitStorageFeatures<'a> {}
    impl Default for PhysicalDevice8BitStorageFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                storage_buffer8_bit_access: Default::default(),
                uniform_and_storage_buffer8_bit_access: Default::default(),
                storage_push_constant8: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevice8BitStorageFeatures<'a> {
        pub fn storage_buffer8_bit_access(mut self, storage_buffer8_bit_access: Bool32) -> Self {
            self.storage_buffer8_bit_access = storage_buffer8_bit_access;
            self
        }
        pub fn uniform_and_storage_buffer8_bit_access(
            mut self,
            uniform_and_storage_buffer8_bit_access: Bool32,
        ) -> Self {
            self.uniform_and_storage_buffer8_bit_access = uniform_and_storage_buffer8_bit_access;
            self
        }
        pub fn storage_push_constant8(mut self, storage_push_constant8: Bool32) -> Self {
            self.storage_push_constant8 = storage_push_constant8;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVulkanMemoryModelFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub vulkan_memory_model: Bool32,
        pub vulkan_memory_model_device_scope: Bool32,
        pub vulkan_memory_model_availability_visibility_chains: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVulkanMemoryModelFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceVulkanMemoryModelFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceVulkanMemoryModelFeatures<'a> {}
    impl Default for PhysicalDeviceVulkanMemoryModelFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                vulkan_memory_model: Default::default(),
                vulkan_memory_model_device_scope: Default::default(),
                vulkan_memory_model_availability_visibility_chains: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceVulkanMemoryModelFeatures<'a> {
        pub fn vulkan_memory_model(mut self, vulkan_memory_model: Bool32) -> Self {
            self.vulkan_memory_model = vulkan_memory_model;
            self
        }
        pub fn vulkan_memory_model_device_scope(
            mut self,
            vulkan_memory_model_device_scope: Bool32,
        ) -> Self {
            self.vulkan_memory_model_device_scope = vulkan_memory_model_device_scope;
            self
        }
        pub fn vulkan_memory_model_availability_visibility_chains(
            mut self,
            vulkan_memory_model_availability_visibility_chains: Bool32,
        ) -> Self {
            self.vulkan_memory_model_availability_visibility_chains =
                vulkan_memory_model_availability_visibility_chains;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderAtomicInt64Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_buffer_int64_atomics: Bool32,
        pub shader_shared_int64_atomics: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderAtomicInt64Features<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderAtomicInt64Features<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShaderAtomicInt64Features<'a> {}
    impl Default for PhysicalDeviceShaderAtomicInt64Features<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_buffer_int64_atomics: Default::default(),
                shader_shared_int64_atomics: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderAtomicInt64Features<'a> {
        pub fn shader_buffer_int64_atomics(mut self, shader_buffer_int64_atomics: Bool32) -> Self {
            self.shader_buffer_int64_atomics = shader_buffer_int64_atomics;
            self
        }
        pub fn shader_shared_int64_atomics(mut self, shader_shared_int64_atomics: Bool32) -> Self {
            self.shader_shared_int64_atomics = shader_shared_int64_atomics;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDepthStencilResolveProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub supported_depth_resolve_modes: ResolveModeFlags,
        pub supported_stencil_resolve_modes: ResolveModeFlags,
        pub independent_resolve_none: Bool32,
        pub independent_resolve: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDepthStencilResolveProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceDepthStencilResolveProperties<'a>
    {
    }
    impl Default for PhysicalDeviceDepthStencilResolveProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                supported_depth_resolve_modes: Default::default(),
                supported_stencil_resolve_modes: Default::default(),
                independent_resolve_none: Default::default(),
                independent_resolve: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDepthStencilResolveProperties<'a> {
        pub fn supported_depth_resolve_modes(
            mut self,
            supported_depth_resolve_modes: ResolveModeFlags,
        ) -> Self {
            self.supported_depth_resolve_modes = supported_depth_resolve_modes;
            self
        }
        pub fn supported_stencil_resolve_modes(
            mut self,
            supported_stencil_resolve_modes: ResolveModeFlags,
        ) -> Self {
            self.supported_stencil_resolve_modes = supported_stencil_resolve_modes;
            self
        }
        pub fn independent_resolve_none(mut self, independent_resolve_none: Bool32) -> Self {
            self.independent_resolve_none = independent_resolve_none;
            self
        }
        pub fn independent_resolve(mut self, independent_resolve: Bool32) -> Self {
            self.independent_resolve = independent_resolve;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SubpassDescriptionDepthStencilResolve<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub depth_resolve_mode: ResolveModeFlagBits,
        pub stencil_resolve_mode: ResolveModeFlagBits,
        pub p_depth_stencil_resolve_attachment: *const AttachmentReference2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SubpassDescriptionDepthStencilResolve<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE;
    }
    unsafe impl<'a> Extends<SubpassDescription2<'a>> for SubpassDescriptionDepthStencilResolve<'a> {}
    impl Default for SubpassDescriptionDepthStencilResolve<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                depth_resolve_mode: Default::default(),
                stencil_resolve_mode: Default::default(),
                p_depth_stencil_resolve_attachment: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SubpassDescriptionDepthStencilResolve<'a> {
        pub fn depth_resolve_mode(mut self, depth_resolve_mode: ResolveModeFlagBits) -> Self {
            self.depth_resolve_mode = depth_resolve_mode;
            self
        }
        pub fn stencil_resolve_mode(mut self, stencil_resolve_mode: ResolveModeFlagBits) -> Self {
            self.stencil_resolve_mode = stencil_resolve_mode;
            self
        }
        pub fn depth_stencil_resolve_attachment(
            mut self,
            depth_stencil_resolve_attachment: &'a AttachmentReference2<'a>,
        ) -> Self {
            self.p_depth_stencil_resolve_attachment = depth_stencil_resolve_attachment;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageStencilUsageCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub stencil_usage: ImageUsageFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageStencilUsageCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_STENCIL_USAGE_CREATE_INFO;
    }
    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for ImageStencilUsageCreateInfo<'a> {}
    unsafe impl<'a> Extends<PhysicalDeviceImageFormatInfo2<'a>> for ImageStencilUsageCreateInfo<'a> {}
    impl Default for ImageStencilUsageCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                stencil_usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageStencilUsageCreateInfo<'a> {
        pub fn stencil_usage(mut self, stencil_usage: ImageUsageFlags) -> Self {
            self.stencil_usage = stencil_usage;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceScalarBlockLayoutFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub scalar_block_layout: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceScalarBlockLayoutFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceScalarBlockLayoutFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceScalarBlockLayoutFeatures<'a> {}
    impl Default for PhysicalDeviceScalarBlockLayoutFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                scalar_block_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceScalarBlockLayoutFeatures<'a> {
        pub fn scalar_block_layout(mut self, scalar_block_layout: Bool32) -> Self {
            self.scalar_block_layout = scalar_block_layout;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceUniformBufferStandardLayoutFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub uniform_buffer_standard_layout: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceUniformBufferStandardLayoutFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceUniformBufferStandardLayoutFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceUniformBufferStandardLayoutFeatures<'a>
    {
    }
    impl Default for PhysicalDeviceUniformBufferStandardLayoutFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                uniform_buffer_standard_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceUniformBufferStandardLayoutFeatures<'a> {
        pub fn uniform_buffer_standard_layout(
            mut self,
            uniform_buffer_standard_layout: Bool32,
        ) -> Self {
            self.uniform_buffer_standard_layout = uniform_buffer_standard_layout;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceBufferDeviceAddressFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub buffer_device_address: Bool32,
        pub buffer_device_address_capture_replay: Bool32,
        pub buffer_device_address_multi_device: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceBufferDeviceAddressFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceBufferDeviceAddressFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceBufferDeviceAddressFeatures<'a> {}
    impl Default for PhysicalDeviceBufferDeviceAddressFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                buffer_device_address: Default::default(),
                buffer_device_address_capture_replay: Default::default(),
                buffer_device_address_multi_device: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceBufferDeviceAddressFeatures<'a> {
        pub fn buffer_device_address(mut self, buffer_device_address: Bool32) -> Self {
            self.buffer_device_address = buffer_device_address;
            self
        }
        pub fn buffer_device_address_capture_replay(
            mut self,
            buffer_device_address_capture_replay: Bool32,
        ) -> Self {
            self.buffer_device_address_capture_replay = buffer_device_address_capture_replay;
            self
        }
        pub fn buffer_device_address_multi_device(
            mut self,
            buffer_device_address_multi_device: Bool32,
        ) -> Self {
            self.buffer_device_address_multi_device = buffer_device_address_multi_device;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BufferDeviceAddressInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub buffer: Buffer,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BufferDeviceAddressInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_DEVICE_ADDRESS_INFO;
    }
    impl Default for BufferDeviceAddressInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                buffer: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BufferDeviceAddressInfo<'a> {
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BufferOpaqueCaptureAddressCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub opaque_capture_address: u64,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BufferOpaqueCaptureAddressCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO;
    }
    unsafe impl<'a> Extends<BufferCreateInfo<'a>> for BufferOpaqueCaptureAddressCreateInfo<'a> {}
    impl Default for BufferOpaqueCaptureAddressCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                opaque_capture_address: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BufferOpaqueCaptureAddressCreateInfo<'a> {
        pub fn opaque_capture_address(mut self, opaque_capture_address: u64) -> Self {
            self.opaque_capture_address = opaque_capture_address;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceImagelessFramebufferFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub imageless_framebuffer: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImagelessFramebufferFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceImagelessFramebufferFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceImagelessFramebufferFeatures<'a> {}
    impl Default for PhysicalDeviceImagelessFramebufferFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                imageless_framebuffer: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceImagelessFramebufferFeatures<'a> {
        pub fn imageless_framebuffer(mut self, imageless_framebuffer: Bool32) -> Self {
            self.imageless_framebuffer = imageless_framebuffer;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct FramebufferAttachmentsCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub attachment_image_info_count: u32,
        pub p_attachment_image_infos: *const FramebufferAttachmentImageInfo<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for FramebufferAttachmentsCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO;
    }
    unsafe impl<'a> Extends<FramebufferCreateInfo<'a>> for FramebufferAttachmentsCreateInfo<'a> {}
    impl Default for FramebufferAttachmentsCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                attachment_image_info_count: Default::default(),
                p_attachment_image_infos: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> FramebufferAttachmentsCreateInfo<'a> {
        pub fn attachment_image_infos(
            mut self,
            attachment_image_infos: &'a [FramebufferAttachmentImageInfo<'a>],
        ) -> Self {
            self.attachment_image_info_count = attachment_image_infos.len().try_into().unwrap();
            self.p_attachment_image_infos = attachment_image_infos.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct FramebufferAttachmentImageInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: ImageCreateFlags,
        pub usage: ImageUsageFlags,
        pub width: u32,
        pub height: u32,
        pub layer_count: u32,
        pub view_format_count: u32,
        pub p_view_formats: *const Format,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for FramebufferAttachmentImageInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO;
    }
    impl Default for FramebufferAttachmentImageInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                usage: Default::default(),
                width: Default::default(),
                height: Default::default(),
                layer_count: Default::default(),
                view_format_count: Default::default(),
                p_view_formats: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> FramebufferAttachmentImageInfo<'a> {
        pub fn flags(mut self, flags: ImageCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn usage(mut self, usage: ImageUsageFlags) -> Self {
            self.usage = usage;
            self
        }
        pub fn width(mut self, width: u32) -> Self {
            self.width = width;
            self
        }
        pub fn height(mut self, height: u32) -> Self {
            self.height = height;
            self
        }
        pub fn layer_count(mut self, layer_count: u32) -> Self {
            self.layer_count = layer_count;
            self
        }
        pub fn view_formats(mut self, view_formats: &'a [Format]) -> Self {
            self.view_format_count = view_formats.len().try_into().unwrap();
            self.p_view_formats = view_formats.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderPassAttachmentBeginInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub attachment_count: u32,
        pub p_attachments: *const ImageView,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for RenderPassAttachmentBeginInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDER_PASS_ATTACHMENT_BEGIN_INFO;
    }
    unsafe impl<'a> Extends<RenderPassBeginInfo<'a>> for RenderPassAttachmentBeginInfo<'a> {}
    impl Default for RenderPassAttachmentBeginInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                attachment_count: Default::default(),
                p_attachments: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderPassAttachmentBeginInfo<'a> {
        pub fn attachments(mut self, attachments: &'a [ImageView]) -> Self {
            self.attachment_count = attachments.len().try_into().unwrap();
            self.p_attachments = attachments.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub separate_depth_stencil_layouts: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSeparateDepthStencilLayoutsFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceSeparateDepthStencilLayoutsFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceSeparateDepthStencilLayoutsFeatures<'a>
    {
    }
    impl Default for PhysicalDeviceSeparateDepthStencilLayoutsFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                separate_depth_stencil_layouts: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceSeparateDepthStencilLayoutsFeatures<'a> {
        pub fn separate_depth_stencil_layouts(
            mut self,
            separate_depth_stencil_layouts: Bool32,
        ) -> Self {
            self.separate_depth_stencil_layouts = separate_depth_stencil_layouts;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AttachmentReferenceStencilLayout<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub stencil_layout: ImageLayout,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for AttachmentReferenceStencilLayout<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ATTACHMENT_REFERENCE_STENCIL_LAYOUT;
    }
    unsafe impl<'a> Extends<AttachmentReference2<'a>> for AttachmentReferenceStencilLayout<'a> {}
    impl Default for AttachmentReferenceStencilLayout<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                stencil_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> AttachmentReferenceStencilLayout<'a> {
        pub fn stencil_layout(mut self, stencil_layout: ImageLayout) -> Self {
            self.stencil_layout = stencil_layout;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AttachmentDescriptionStencilLayout<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub stencil_initial_layout: ImageLayout,
        pub stencil_final_layout: ImageLayout,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for AttachmentDescriptionStencilLayout<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT;
    }
    unsafe impl<'a> Extends<AttachmentDescription2<'a>> for AttachmentDescriptionStencilLayout<'a> {}
    impl Default for AttachmentDescriptionStencilLayout<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                stencil_initial_layout: Default::default(),
                stencil_final_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> AttachmentDescriptionStencilLayout<'a> {
        pub fn stencil_initial_layout(mut self, stencil_initial_layout: ImageLayout) -> Self {
            self.stencil_initial_layout = stencil_initial_layout;
            self
        }
        pub fn stencil_final_layout(mut self, stencil_final_layout: ImageLayout) -> Self {
            self.stencil_final_layout = stencil_final_layout;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryOpaqueCaptureAddressAllocateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub opaque_capture_address: u64,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryOpaqueCaptureAddressAllocateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO;
    }
    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for MemoryOpaqueCaptureAddressAllocateInfo<'a> {}
    impl Default for MemoryOpaqueCaptureAddressAllocateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                opaque_capture_address: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryOpaqueCaptureAddressAllocateInfo<'a> {
        pub fn opaque_capture_address(mut self, opaque_capture_address: u64) -> Self {
            self.opaque_capture_address = opaque_capture_address;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceMemoryOpaqueCaptureAddressInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DeviceMemoryOpaqueCaptureAddressInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO;
    }
    impl Default for DeviceMemoryOpaqueCaptureAddressInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceMemoryOpaqueCaptureAddressInfo<'a> {
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVulkan11Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub storage_buffer16_bit_access: Bool32,
        pub uniform_and_storage_buffer16_bit_access: Bool32,
        pub storage_push_constant16: Bool32,
        pub storage_input_output16: Bool32,
        pub multiview: Bool32,
        pub multiview_geometry_shader: Bool32,
        pub multiview_tessellation_shader: Bool32,
        pub variable_pointers_storage_buffer: Bool32,
        pub variable_pointers: Bool32,
        pub protected_memory: Bool32,
        pub sampler_ycbcr_conversion: Bool32,
        pub shader_draw_parameters: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVulkan11Features<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceVulkan11Features<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceVulkan11Features<'a> {}
    impl Default for PhysicalDeviceVulkan11Features<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                storage_buffer16_bit_access: Default::default(),
                uniform_and_storage_buffer16_bit_access: Default::default(),
                storage_push_constant16: Default::default(),
                storage_input_output16: Default::default(),
                multiview: Default::default(),
                multiview_geometry_shader: Default::default(),
                multiview_tessellation_shader: Default::default(),
                variable_pointers_storage_buffer: Default::default(),
                variable_pointers: Default::default(),
                protected_memory: Default::default(),
                sampler_ycbcr_conversion: Default::default(),
                shader_draw_parameters: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceVulkan11Features<'a> {
        pub fn storage_buffer16_bit_access(mut self, storage_buffer16_bit_access: Bool32) -> Self {
            self.storage_buffer16_bit_access = storage_buffer16_bit_access;
            self
        }
        pub fn uniform_and_storage_buffer16_bit_access(
            mut self,
            uniform_and_storage_buffer16_bit_access: Bool32,
        ) -> Self {
            self.uniform_and_storage_buffer16_bit_access = uniform_and_storage_buffer16_bit_access;
            self
        }
        pub fn storage_push_constant16(mut self, storage_push_constant16: Bool32) -> Self {
            self.storage_push_constant16 = storage_push_constant16;
            self
        }
        pub fn storage_input_output16(mut self, storage_input_output16: Bool32) -> Self {
            self.storage_input_output16 = storage_input_output16;
            self
        }
        pub fn multiview(mut self, multiview: Bool32) -> Self {
            self.multiview = multiview;
            self
        }
        pub fn multiview_geometry_shader(mut self, multiview_geometry_shader: Bool32) -> Self {
            self.multiview_geometry_shader = multiview_geometry_shader;
            self
        }
        pub fn multiview_tessellation_shader(
            mut self,
            multiview_tessellation_shader: Bool32,
        ) -> Self {
            self.multiview_tessellation_shader = multiview_tessellation_shader;
            self
        }
        pub fn variable_pointers_storage_buffer(
            mut self,
            variable_pointers_storage_buffer: Bool32,
        ) -> Self {
            self.variable_pointers_storage_buffer = variable_pointers_storage_buffer;
            self
        }
        pub fn variable_pointers(mut self, variable_pointers: Bool32) -> Self {
            self.variable_pointers = variable_pointers;
            self
        }
        pub fn protected_memory(mut self, protected_memory: Bool32) -> Self {
            self.protected_memory = protected_memory;
            self
        }
        pub fn sampler_ycbcr_conversion(mut self, sampler_ycbcr_conversion: Bool32) -> Self {
            self.sampler_ycbcr_conversion = sampler_ycbcr_conversion;
            self
        }
        pub fn shader_draw_parameters(mut self, shader_draw_parameters: Bool32) -> Self {
            self.shader_draw_parameters = shader_draw_parameters;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVulkan11Properties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub device_uuid: [u8; UUID_SIZE as usize],
        pub driver_uuid: [u8; UUID_SIZE as usize],
        pub device_luid: [u8; LUID_SIZE as usize],
        pub device_node_mask: u32,
        pub device_luid_valid: Bool32,
        pub subgroup_size: u32,
        pub subgroup_supported_stages: ShaderStageFlags,
        pub subgroup_supported_operations: SubgroupFeatureFlags,
        pub subgroup_quad_operations_in_all_stages: Bool32,
        pub point_clipping_behavior: PointClippingBehavior,
        pub max_multiview_view_count: u32,
        pub max_multiview_instance_index: u32,
        pub protected_no_fault: Bool32,
        pub max_per_set_descriptors: u32,
        pub max_memory_allocation_size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVulkan11Properties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>> for PhysicalDeviceVulkan11Properties<'a> {}
    impl Default for PhysicalDeviceVulkan11Properties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                device_uuid: [Default::default(); _],
                driver_uuid: [Default::default(); _],
                device_luid: [Default::default(); _],
                device_node_mask: Default::default(),
                device_luid_valid: Default::default(),
                subgroup_size: Default::default(),
                subgroup_supported_stages: Default::default(),
                subgroup_supported_operations: Default::default(),
                subgroup_quad_operations_in_all_stages: Default::default(),
                point_clipping_behavior: Default::default(),
                max_multiview_view_count: Default::default(),
                max_multiview_instance_index: Default::default(),
                protected_no_fault: Default::default(),
                max_per_set_descriptors: Default::default(),
                max_memory_allocation_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceVulkan11Properties<'a> {
        pub fn device_uuid(mut self, device_uuid: [u8; UUID_SIZE as usize]) -> Self {
            self.device_uuid = device_uuid;
            self
        }
        pub fn driver_uuid(mut self, driver_uuid: [u8; UUID_SIZE as usize]) -> Self {
            self.driver_uuid = driver_uuid;
            self
        }
        pub fn device_luid(mut self, device_luid: [u8; LUID_SIZE as usize]) -> Self {
            self.device_luid = device_luid;
            self
        }
        pub fn device_node_mask(mut self, device_node_mask: u32) -> Self {
            self.device_node_mask = device_node_mask;
            self
        }
        pub fn device_luid_valid(mut self, device_luid_valid: Bool32) -> Self {
            self.device_luid_valid = device_luid_valid;
            self
        }
        pub fn subgroup_size(mut self, subgroup_size: u32) -> Self {
            self.subgroup_size = subgroup_size;
            self
        }
        pub fn subgroup_supported_stages(
            mut self,
            subgroup_supported_stages: ShaderStageFlags,
        ) -> Self {
            self.subgroup_supported_stages = subgroup_supported_stages;
            self
        }
        pub fn subgroup_supported_operations(
            mut self,
            subgroup_supported_operations: SubgroupFeatureFlags,
        ) -> Self {
            self.subgroup_supported_operations = subgroup_supported_operations;
            self
        }
        pub fn subgroup_quad_operations_in_all_stages(
            mut self,
            subgroup_quad_operations_in_all_stages: Bool32,
        ) -> Self {
            self.subgroup_quad_operations_in_all_stages = subgroup_quad_operations_in_all_stages;
            self
        }
        pub fn point_clipping_behavior(
            mut self,
            point_clipping_behavior: PointClippingBehavior,
        ) -> Self {
            self.point_clipping_behavior = point_clipping_behavior;
            self
        }
        pub fn max_multiview_view_count(mut self, max_multiview_view_count: u32) -> Self {
            self.max_multiview_view_count = max_multiview_view_count;
            self
        }
        pub fn max_multiview_instance_index(mut self, max_multiview_instance_index: u32) -> Self {
            self.max_multiview_instance_index = max_multiview_instance_index;
            self
        }
        pub fn protected_no_fault(mut self, protected_no_fault: Bool32) -> Self {
            self.protected_no_fault = protected_no_fault;
            self
        }
        pub fn max_per_set_descriptors(mut self, max_per_set_descriptors: u32) -> Self {
            self.max_per_set_descriptors = max_per_set_descriptors;
            self
        }
        pub fn max_memory_allocation_size(
            mut self,
            max_memory_allocation_size: DeviceSize,
        ) -> Self {
            self.max_memory_allocation_size = max_memory_allocation_size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVulkan12Features<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub sampler_mirror_clamp_to_edge: Bool32,
        pub draw_indirect_count: Bool32,
        pub storage_buffer8_bit_access: Bool32,
        pub uniform_and_storage_buffer8_bit_access: Bool32,
        pub storage_push_constant8: Bool32,
        pub shader_buffer_int64_atomics: Bool32,
        pub shader_shared_int64_atomics: Bool32,
        pub shader_float16: Bool32,
        pub shader_int8: Bool32,
        pub descriptor_indexing: Bool32,
        pub shader_input_attachment_array_dynamic_indexing: Bool32,
        pub shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
        pub shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
        pub shader_uniform_buffer_array_non_uniform_indexing: Bool32,
        pub shader_sampled_image_array_non_uniform_indexing: Bool32,
        pub shader_storage_buffer_array_non_uniform_indexing: Bool32,
        pub shader_storage_image_array_non_uniform_indexing: Bool32,
        pub shader_input_attachment_array_non_uniform_indexing: Bool32,
        pub shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
        pub shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
        pub descriptor_binding_uniform_buffer_update_after_bind: Bool32,
        pub descriptor_binding_sampled_image_update_after_bind: Bool32,
        pub descriptor_binding_storage_image_update_after_bind: Bool32,
        pub descriptor_binding_storage_buffer_update_after_bind: Bool32,
        pub descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
        pub descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
        pub descriptor_binding_update_unused_while_pending: Bool32,
        pub descriptor_binding_partially_bound: Bool32,
        pub descriptor_binding_variable_descriptor_count: Bool32,
        pub runtime_descriptor_array: Bool32,
        pub sampler_filter_minmax: Bool32,
        pub scalar_block_layout: Bool32,
        pub imageless_framebuffer: Bool32,
        pub uniform_buffer_standard_layout: Bool32,
        pub shader_subgroup_extended_types: Bool32,
        pub separate_depth_stencil_layouts: Bool32,
        pub host_query_reset: Bool32,
        pub timeline_semaphore: Bool32,
        pub buffer_device_address: Bool32,
        pub buffer_device_address_capture_replay: Bool32,
        pub buffer_device_address_multi_device: Bool32,
        pub vulkan_memory_model: Bool32,
        pub vulkan_memory_model_device_scope: Bool32,
        pub vulkan_memory_model_availability_visibility_chains: Bool32,
        pub shader_output_viewport_index: Bool32,
        pub shader_output_layer: Bool32,
        pub subgroup_broadcast_dynamic_id: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVulkan12Features<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceVulkan12Features<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceVulkan12Features<'a> {}
    impl Default for PhysicalDeviceVulkan12Features<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                sampler_mirror_clamp_to_edge: Default::default(),
                draw_indirect_count: Default::default(),
                storage_buffer8_bit_access: Default::default(),
                uniform_and_storage_buffer8_bit_access: Default::default(),
                storage_push_constant8: Default::default(),
                shader_buffer_int64_atomics: Default::default(),
                shader_shared_int64_atomics: Default::default(),
                shader_float16: Default::default(),
                shader_int8: Default::default(),
                descriptor_indexing: Default::default(),
                shader_input_attachment_array_dynamic_indexing: Default::default(),
                shader_uniform_texel_buffer_array_dynamic_indexing: Default::default(),
                shader_storage_texel_buffer_array_dynamic_indexing: Default::default(),
                shader_uniform_buffer_array_non_uniform_indexing: Default::default(),
                shader_sampled_image_array_non_uniform_indexing: Default::default(),
                shader_storage_buffer_array_non_uniform_indexing: Default::default(),
                shader_storage_image_array_non_uniform_indexing: Default::default(),
                shader_input_attachment_array_non_uniform_indexing: Default::default(),
                shader_uniform_texel_buffer_array_non_uniform_indexing: Default::default(),
                shader_storage_texel_buffer_array_non_uniform_indexing: Default::default(),
                descriptor_binding_uniform_buffer_update_after_bind: Default::default(),
                descriptor_binding_sampled_image_update_after_bind: Default::default(),
                descriptor_binding_storage_image_update_after_bind: Default::default(),
                descriptor_binding_storage_buffer_update_after_bind: Default::default(),
                descriptor_binding_uniform_texel_buffer_update_after_bind: Default::default(),
                descriptor_binding_storage_texel_buffer_update_after_bind: Default::default(),
                descriptor_binding_update_unused_while_pending: Default::default(),
                descriptor_binding_partially_bound: Default::default(),
                descriptor_binding_variable_descriptor_count: Default::default(),
                runtime_descriptor_array: Default::default(),
                sampler_filter_minmax: Default::default(),
                scalar_block_layout: Default::default(),
                imageless_framebuffer: Default::default(),
                uniform_buffer_standard_layout: Default::default(),
                shader_subgroup_extended_types: Default::default(),
                separate_depth_stencil_layouts: Default::default(),
                host_query_reset: Default::default(),
                timeline_semaphore: Default::default(),
                buffer_device_address: Default::default(),
                buffer_device_address_capture_replay: Default::default(),
                buffer_device_address_multi_device: Default::default(),
                vulkan_memory_model: Default::default(),
                vulkan_memory_model_device_scope: Default::default(),
                vulkan_memory_model_availability_visibility_chains: Default::default(),
                shader_output_viewport_index: Default::default(),
                shader_output_layer: Default::default(),
                subgroup_broadcast_dynamic_id: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceVulkan12Features<'a> {
        pub fn sampler_mirror_clamp_to_edge(
            mut self,
            sampler_mirror_clamp_to_edge: Bool32,
        ) -> Self {
            self.sampler_mirror_clamp_to_edge = sampler_mirror_clamp_to_edge;
            self
        }
        pub fn draw_indirect_count(mut self, draw_indirect_count: Bool32) -> Self {
            self.draw_indirect_count = draw_indirect_count;
            self
        }
        pub fn storage_buffer8_bit_access(mut self, storage_buffer8_bit_access: Bool32) -> Self {
            self.storage_buffer8_bit_access = storage_buffer8_bit_access;
            self
        }
        pub fn uniform_and_storage_buffer8_bit_access(
            mut self,
            uniform_and_storage_buffer8_bit_access: Bool32,
        ) -> Self {
            self.uniform_and_storage_buffer8_bit_access = uniform_and_storage_buffer8_bit_access;
            self
        }
        pub fn storage_push_constant8(mut self, storage_push_constant8: Bool32) -> Self {
            self.storage_push_constant8 = storage_push_constant8;
            self
        }
        pub fn shader_buffer_int64_atomics(mut self, shader_buffer_int64_atomics: Bool32) -> Self {
            self.shader_buffer_int64_atomics = shader_buffer_int64_atomics;
            self
        }
        pub fn shader_shared_int64_atomics(mut self, shader_shared_int64_atomics: Bool32) -> Self {
            self.shader_shared_int64_atomics = shader_shared_int64_atomics;
            self
        }
        pub fn shader_float16(mut self, shader_float16: Bool32) -> Self {
            self.shader_float16 = shader_float16;
            self
        }
        pub fn shader_int8(mut self, shader_int8: Bool32) -> Self {
            self.shader_int8 = shader_int8;
            self
        }
        pub fn descriptor_indexing(mut self, descriptor_indexing: Bool32) -> Self {
            self.descriptor_indexing = descriptor_indexing;
            self
        }
        pub fn shader_input_attachment_array_dynamic_indexing(
            mut self,
            shader_input_attachment_array_dynamic_indexing: Bool32,
        ) -> Self {
            self.shader_input_attachment_array_dynamic_indexing =
                shader_input_attachment_array_dynamic_indexing;
            self
        }
        pub fn shader_uniform_texel_buffer_array_dynamic_indexing(
            mut self,
            shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
        ) -> Self {
            self.shader_uniform_texel_buffer_array_dynamic_indexing =
                shader_uniform_texel_buffer_array_dynamic_indexing;
            self
        }
        pub fn shader_storage_texel_buffer_array_dynamic_indexing(
            mut self,
            shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
        ) -> Self {
            self.shader_storage_texel_buffer_array_dynamic_indexing =
                shader_storage_texel_buffer_array_dynamic_indexing;
            self
        }
        pub fn shader_uniform_buffer_array_non_uniform_indexing(
            mut self,
            shader_uniform_buffer_array_non_uniform_indexing: Bool32,
        ) -> Self {
            self.shader_uniform_buffer_array_non_uniform_indexing =
                shader_uniform_buffer_array_non_uniform_indexing;
            self
        }
        pub fn shader_sampled_image_array_non_uniform_indexing(
            mut self,
            shader_sampled_image_array_non_uniform_indexing: Bool32,
        ) -> Self {
            self.shader_sampled_image_array_non_uniform_indexing =
                shader_sampled_image_array_non_uniform_indexing;
            self
        }
        pub fn shader_storage_buffer_array_non_uniform_indexing(
            mut self,
            shader_storage_buffer_array_non_uniform_indexing: Bool32,
        ) -> Self {
            self.shader_storage_buffer_array_non_uniform_indexing =
                shader_storage_buffer_array_non_uniform_indexing;
            self
        }
        pub fn shader_storage_image_array_non_uniform_indexing(
            mut self,
            shader_storage_image_array_non_uniform_indexing: Bool32,
        ) -> Self {
            self.shader_storage_image_array_non_uniform_indexing =
                shader_storage_image_array_non_uniform_indexing;
            self
        }
        pub fn shader_input_attachment_array_non_uniform_indexing(
            mut self,
            shader_input_attachment_array_non_uniform_indexing: Bool32,
        ) -> Self {
            self.shader_input_attachment_array_non_uniform_indexing =
                shader_input_attachment_array_non_uniform_indexing;
            self
        }
        pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(
            mut self,
            shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
        ) -> Self {
            self.shader_uniform_texel_buffer_array_non_uniform_indexing =
                shader_uniform_texel_buffer_array_non_uniform_indexing;
            self
        }
        pub fn shader_storage_texel_buffer_array_non_uniform_indexing(
            mut self,
            shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
        ) -> Self {
            self.shader_storage_texel_buffer_array_non_uniform_indexing =
                shader_storage_texel_buffer_array_non_uniform_indexing;
            self
        }
        pub fn descriptor_binding_uniform_buffer_update_after_bind(
            mut self,
            descriptor_binding_uniform_buffer_update_after_bind: Bool32,
        ) -> Self {
            self.descriptor_binding_uniform_buffer_update_after_bind =
                descriptor_binding_uniform_buffer_update_after_bind;
            self
        }
        pub fn descriptor_binding_sampled_image_update_after_bind(
            mut self,
            descriptor_binding_sampled_image_update_after_bind: Bool32,
        ) -> Self {
            self.descriptor_binding_sampled_image_update_after_bind =
                descriptor_binding_sampled_image_update_after_bind;
            self
        }
        pub fn descriptor_binding_storage_image_update_after_bind(
            mut self,
            descriptor_binding_storage_image_update_after_bind: Bool32,
        ) -> Self {
            self.descriptor_binding_storage_image_update_after_bind =
                descriptor_binding_storage_image_update_after_bind;
            self
        }
        pub fn descriptor_binding_storage_buffer_update_after_bind(
            mut self,
            descriptor_binding_storage_buffer_update_after_bind: Bool32,
        ) -> Self {
            self.descriptor_binding_storage_buffer_update_after_bind =
                descriptor_binding_storage_buffer_update_after_bind;
            self
        }
        pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(
            mut self,
            descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
        ) -> Self {
            self.descriptor_binding_uniform_texel_buffer_update_after_bind =
                descriptor_binding_uniform_texel_buffer_update_after_bind;
            self
        }
        pub fn descriptor_binding_storage_texel_buffer_update_after_bind(
            mut self,
            descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
        ) -> Self {
            self.descriptor_binding_storage_texel_buffer_update_after_bind =
                descriptor_binding_storage_texel_buffer_update_after_bind;
            self
        }
        pub fn descriptor_binding_update_unused_while_pending(
            mut self,
            descriptor_binding_update_unused_while_pending: Bool32,
        ) -> Self {
            self.descriptor_binding_update_unused_while_pending =
                descriptor_binding_update_unused_while_pending;
            self
        }
        pub fn descriptor_binding_partially_bound(
            mut self,
            descriptor_binding_partially_bound: Bool32,
        ) -> Self {
            self.descriptor_binding_partially_bound = descriptor_binding_partially_bound;
            self
        }
        pub fn descriptor_binding_variable_descriptor_count(
            mut self,
            descriptor_binding_variable_descriptor_count: Bool32,
        ) -> Self {
            self.descriptor_binding_variable_descriptor_count =
                descriptor_binding_variable_descriptor_count;
            self
        }
        pub fn runtime_descriptor_array(mut self, runtime_descriptor_array: Bool32) -> Self {
            self.runtime_descriptor_array = runtime_descriptor_array;
            self
        }
        pub fn sampler_filter_minmax(mut self, sampler_filter_minmax: Bool32) -> Self {
            self.sampler_filter_minmax = sampler_filter_minmax;
            self
        }
        pub fn scalar_block_layout(mut self, scalar_block_layout: Bool32) -> Self {
            self.scalar_block_layout = scalar_block_layout;
            self
        }
        pub fn imageless_framebuffer(mut self, imageless_framebuffer: Bool32) -> Self {
            self.imageless_framebuffer = imageless_framebuffer;
            self
        }
        pub fn uniform_buffer_standard_layout(
            mut self,
            uniform_buffer_standard_layout: Bool32,
        ) -> Self {
            self.uniform_buffer_standard_layout = uniform_buffer_standard_layout;
            self
        }
        pub fn shader_subgroup_extended_types(
            mut self,
            shader_subgroup_extended_types: Bool32,
        ) -> Self {
            self.shader_subgroup_extended_types = shader_subgroup_extended_types;
            self
        }
        pub fn separate_depth_stencil_layouts(
            mut self,
            separate_depth_stencil_layouts: Bool32,
        ) -> Self {
            self.separate_depth_stencil_layouts = separate_depth_stencil_layouts;
            self
        }
        pub fn host_query_reset(mut self, host_query_reset: Bool32) -> Self {
            self.host_query_reset = host_query_reset;
            self
        }
        pub fn timeline_semaphore(mut self, timeline_semaphore: Bool32) -> Self {
            self.timeline_semaphore = timeline_semaphore;
            self
        }
        pub fn buffer_device_address(mut self, buffer_device_address: Bool32) -> Self {
            self.buffer_device_address = buffer_device_address;
            self
        }
        pub fn buffer_device_address_capture_replay(
            mut self,
            buffer_device_address_capture_replay: Bool32,
        ) -> Self {
            self.buffer_device_address_capture_replay = buffer_device_address_capture_replay;
            self
        }
        pub fn buffer_device_address_multi_device(
            mut self,
            buffer_device_address_multi_device: Bool32,
        ) -> Self {
            self.buffer_device_address_multi_device = buffer_device_address_multi_device;
            self
        }
        pub fn vulkan_memory_model(mut self, vulkan_memory_model: Bool32) -> Self {
            self.vulkan_memory_model = vulkan_memory_model;
            self
        }
        pub fn vulkan_memory_model_device_scope(
            mut self,
            vulkan_memory_model_device_scope: Bool32,
        ) -> Self {
            self.vulkan_memory_model_device_scope = vulkan_memory_model_device_scope;
            self
        }
        pub fn vulkan_memory_model_availability_visibility_chains(
            mut self,
            vulkan_memory_model_availability_visibility_chains: Bool32,
        ) -> Self {
            self.vulkan_memory_model_availability_visibility_chains =
                vulkan_memory_model_availability_visibility_chains;
            self
        }
        pub fn shader_output_viewport_index(
            mut self,
            shader_output_viewport_index: Bool32,
        ) -> Self {
            self.shader_output_viewport_index = shader_output_viewport_index;
            self
        }
        pub fn shader_output_layer(mut self, shader_output_layer: Bool32) -> Self {
            self.shader_output_layer = shader_output_layer;
            self
        }
        pub fn subgroup_broadcast_dynamic_id(
            mut self,
            subgroup_broadcast_dynamic_id: Bool32,
        ) -> Self {
            self.subgroup_broadcast_dynamic_id = subgroup_broadcast_dynamic_id;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVulkan12Properties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub driver_id: DriverId,
        pub driver_name: [c_char; MAX_DRIVER_NAME_SIZE as usize],
        pub driver_info: [c_char; MAX_DRIVER_INFO_SIZE as usize],
        pub conformance_version: ConformanceVersion,
        pub denorm_behavior_independence: ShaderFloatControlsIndependence,
        pub rounding_mode_independence: ShaderFloatControlsIndependence,
        pub shader_signed_zero_inf_nan_preserve_float16: Bool32,
        pub shader_signed_zero_inf_nan_preserve_float32: Bool32,
        pub shader_signed_zero_inf_nan_preserve_float64: Bool32,
        pub shader_denorm_preserve_float16: Bool32,
        pub shader_denorm_preserve_float32: Bool32,
        pub shader_denorm_preserve_float64: Bool32,
        pub shader_denorm_flush_to_zero_float16: Bool32,
        pub shader_denorm_flush_to_zero_float32: Bool32,
        pub shader_denorm_flush_to_zero_float64: Bool32,
        pub shader_rounding_mode_rte_float16: Bool32,
        pub shader_rounding_mode_rte_float32: Bool32,
        pub shader_rounding_mode_rte_float64: Bool32,
        pub shader_rounding_mode_rtz_float16: Bool32,
        pub shader_rounding_mode_rtz_float32: Bool32,
        pub shader_rounding_mode_rtz_float64: Bool32,
        pub max_update_after_bind_descriptors_in_all_pools: u32,
        pub shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
        pub shader_sampled_image_array_non_uniform_indexing_native: Bool32,
        pub shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
        pub shader_storage_image_array_non_uniform_indexing_native: Bool32,
        pub shader_input_attachment_array_non_uniform_indexing_native: Bool32,
        pub robust_buffer_access_update_after_bind: Bool32,
        pub quad_divergent_implicit_lod: Bool32,
        pub max_per_stage_descriptor_update_after_bind_samplers: u32,
        pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
        pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
        pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
        pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
        pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
        pub max_per_stage_update_after_bind_resources: u32,
        pub max_descriptor_set_update_after_bind_samplers: u32,
        pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
        pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
        pub max_descriptor_set_update_after_bind_storage_buffers: u32,
        pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
        pub max_descriptor_set_update_after_bind_sampled_images: u32,
        pub max_descriptor_set_update_after_bind_storage_images: u32,
        pub max_descriptor_set_update_after_bind_input_attachments: u32,
        pub supported_depth_resolve_modes: ResolveModeFlags,
        pub supported_stencil_resolve_modes: ResolveModeFlags,
        pub independent_resolve_none: Bool32,
        pub independent_resolve: Bool32,
        pub filter_minmax_single_component_formats: Bool32,
        pub filter_minmax_image_component_mapping: Bool32,
        pub max_timeline_semaphore_value_difference: u64,
        pub framebuffer_integer_color_sample_counts: SampleCountFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVulkan12Properties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>> for PhysicalDeviceVulkan12Properties<'a> {}
    impl Default for PhysicalDeviceVulkan12Properties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                driver_id: Default::default(),
                driver_name: [Default::default(); _],
                driver_info: [Default::default(); _],
                conformance_version: Default::default(),
                denorm_behavior_independence: Default::default(),
                rounding_mode_independence: Default::default(),
                shader_signed_zero_inf_nan_preserve_float16: Default::default(),
                shader_signed_zero_inf_nan_preserve_float32: Default::default(),
                shader_signed_zero_inf_nan_preserve_float64: Default::default(),
                shader_denorm_preserve_float16: Default::default(),
                shader_denorm_preserve_float32: Default::default(),
                shader_denorm_preserve_float64: Default::default(),
                shader_denorm_flush_to_zero_float16: Default::default(),
                shader_denorm_flush_to_zero_float32: Default::default(),
                shader_denorm_flush_to_zero_float64: Default::default(),
                shader_rounding_mode_rte_float16: Default::default(),
                shader_rounding_mode_rte_float32: Default::default(),
                shader_rounding_mode_rte_float64: Default::default(),
                shader_rounding_mode_rtz_float16: Default::default(),
                shader_rounding_mode_rtz_float32: Default::default(),
                shader_rounding_mode_rtz_float64: Default::default(),
                max_update_after_bind_descriptors_in_all_pools: Default::default(),
                shader_uniform_buffer_array_non_uniform_indexing_native: Default::default(),
                shader_sampled_image_array_non_uniform_indexing_native: Default::default(),
                shader_storage_buffer_array_non_uniform_indexing_native: Default::default(),
                shader_storage_image_array_non_uniform_indexing_native: Default::default(),
                shader_input_attachment_array_non_uniform_indexing_native: Default::default(),
                robust_buffer_access_update_after_bind: Default::default(),
                quad_divergent_implicit_lod: Default::default(),
                max_per_stage_descriptor_update_after_bind_samplers: Default::default(),
                max_per_stage_descriptor_update_after_bind_uniform_buffers: Default::default(),
                max_per_stage_descriptor_update_after_bind_storage_buffers: Default::default(),
                max_per_stage_descriptor_update_after_bind_sampled_images: Default::default(),
                max_per_stage_descriptor_update_after_bind_storage_images: Default::default(),
                max_per_stage_descriptor_update_after_bind_input_attachments: Default::default(),
                max_per_stage_update_after_bind_resources: Default::default(),
                max_descriptor_set_update_after_bind_samplers: Default::default(),
                max_descriptor_set_update_after_bind_uniform_buffers: Default::default(),
                max_descriptor_set_update_after_bind_uniform_buffers_dynamic: Default::default(),
                max_descriptor_set_update_after_bind_storage_buffers: Default::default(),
                max_descriptor_set_update_after_bind_storage_buffers_dynamic: Default::default(),
                max_descriptor_set_update_after_bind_sampled_images: Default::default(),
                max_descriptor_set_update_after_bind_storage_images: Default::default(),
                max_descriptor_set_update_after_bind_input_attachments: Default::default(),
                supported_depth_resolve_modes: Default::default(),
                supported_stencil_resolve_modes: Default::default(),
                independent_resolve_none: Default::default(),
                independent_resolve: Default::default(),
                filter_minmax_single_component_formats: Default::default(),
                filter_minmax_image_component_mapping: Default::default(),
                max_timeline_semaphore_value_difference: Default::default(),
                framebuffer_integer_color_sample_counts: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceVulkan12Properties<'a> {
        pub fn driver_id(mut self, driver_id: DriverId) -> Self {
            self.driver_id = driver_id;
            self
        }
        pub fn conformance_version(mut self, conformance_version: ConformanceVersion) -> Self {
            self.conformance_version = conformance_version;
            self
        }
        pub fn denorm_behavior_independence(
            mut self,
            denorm_behavior_independence: ShaderFloatControlsIndependence,
        ) -> Self {
            self.denorm_behavior_independence = denorm_behavior_independence;
            self
        }
        pub fn rounding_mode_independence(
            mut self,
            rounding_mode_independence: ShaderFloatControlsIndependence,
        ) -> Self {
            self.rounding_mode_independence = rounding_mode_independence;
            self
        }
        pub fn shader_signed_zero_inf_nan_preserve_float16(
            mut self,
            shader_signed_zero_inf_nan_preserve_float16: Bool32,
        ) -> Self {
            self.shader_signed_zero_inf_nan_preserve_float16 =
                shader_signed_zero_inf_nan_preserve_float16;
            self
        }
        pub fn shader_signed_zero_inf_nan_preserve_float32(
            mut self,
            shader_signed_zero_inf_nan_preserve_float32: Bool32,
        ) -> Self {
            self.shader_signed_zero_inf_nan_preserve_float32 =
                shader_signed_zero_inf_nan_preserve_float32;
            self
        }
        pub fn shader_signed_zero_inf_nan_preserve_float64(
            mut self,
            shader_signed_zero_inf_nan_preserve_float64: Bool32,
        ) -> Self {
            self.shader_signed_zero_inf_nan_preserve_float64 =
                shader_signed_zero_inf_nan_preserve_float64;
            self
        }
        pub fn shader_denorm_preserve_float16(
            mut self,
            shader_denorm_preserve_float16: Bool32,
        ) -> Self {
            self.shader_denorm_preserve_float16 = shader_denorm_preserve_float16;
            self
        }
        pub fn shader_denorm_preserve_float32(
            mut self,
            shader_denorm_preserve_float32: Bool32,
        ) -> Self {
            self.shader_denorm_preserve_float32 = shader_denorm_preserve_float32;
            self
        }
        pub fn shader_denorm_preserve_float64(
            mut self,
            shader_denorm_preserve_float64: Bool32,
        ) -> Self {
            self.shader_denorm_preserve_float64 = shader_denorm_preserve_float64;
            self
        }
        pub fn shader_denorm_flush_to_zero_float16(
            mut self,
            shader_denorm_flush_to_zero_float16: Bool32,
        ) -> Self {
            self.shader_denorm_flush_to_zero_float16 = shader_denorm_flush_to_zero_float16;
            self
        }
        pub fn shader_denorm_flush_to_zero_float32(
            mut self,
            shader_denorm_flush_to_zero_float32: Bool32,
        ) -> Self {
            self.shader_denorm_flush_to_zero_float32 = shader_denorm_flush_to_zero_float32;
            self
        }
        pub fn shader_denorm_flush_to_zero_float64(
            mut self,
            shader_denorm_flush_to_zero_float64: Bool32,
        ) -> Self {
            self.shader_denorm_flush_to_zero_float64 = shader_denorm_flush_to_zero_float64;
            self
        }
        pub fn shader_rounding_mode_rte_float16(
            mut self,
            shader_rounding_mode_rte_float16: Bool32,
        ) -> Self {
            self.shader_rounding_mode_rte_float16 = shader_rounding_mode_rte_float16;
            self
        }
        pub fn shader_rounding_mode_rte_float32(
            mut self,
            shader_rounding_mode_rte_float32: Bool32,
        ) -> Self {
            self.shader_rounding_mode_rte_float32 = shader_rounding_mode_rte_float32;
            self
        }
        pub fn shader_rounding_mode_rte_float64(
            mut self,
            shader_rounding_mode_rte_float64: Bool32,
        ) -> Self {
            self.shader_rounding_mode_rte_float64 = shader_rounding_mode_rte_float64;
            self
        }
        pub fn shader_rounding_mode_rtz_float16(
            mut self,
            shader_rounding_mode_rtz_float16: Bool32,
        ) -> Self {
            self.shader_rounding_mode_rtz_float16 = shader_rounding_mode_rtz_float16;
            self
        }
        pub fn shader_rounding_mode_rtz_float32(
            mut self,
            shader_rounding_mode_rtz_float32: Bool32,
        ) -> Self {
            self.shader_rounding_mode_rtz_float32 = shader_rounding_mode_rtz_float32;
            self
        }
        pub fn shader_rounding_mode_rtz_float64(
            mut self,
            shader_rounding_mode_rtz_float64: Bool32,
        ) -> Self {
            self.shader_rounding_mode_rtz_float64 = shader_rounding_mode_rtz_float64;
            self
        }
        pub fn max_update_after_bind_descriptors_in_all_pools(
            mut self,
            max_update_after_bind_descriptors_in_all_pools: u32,
        ) -> Self {
            self.max_update_after_bind_descriptors_in_all_pools =
                max_update_after_bind_descriptors_in_all_pools;
            self
        }
        pub fn shader_uniform_buffer_array_non_uniform_indexing_native(
            mut self,
            shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
        ) -> Self {
            self.shader_uniform_buffer_array_non_uniform_indexing_native =
                shader_uniform_buffer_array_non_uniform_indexing_native;
            self
        }
        pub fn shader_sampled_image_array_non_uniform_indexing_native(
            mut self,
            shader_sampled_image_array_non_uniform_indexing_native: Bool32,
        ) -> Self {
            self.shader_sampled_image_array_non_uniform_indexing_native =
                shader_sampled_image_array_non_uniform_indexing_native;
            self
        }
        pub fn shader_storage_buffer_array_non_uniform_indexing_native(
            mut self,
            shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
        ) -> Self {
            self.shader_storage_buffer_array_non_uniform_indexing_native =
                shader_storage_buffer_array_non_uniform_indexing_native;
            self
        }
        pub fn shader_storage_image_array_non_uniform_indexing_native(
            mut self,
            shader_storage_image_array_non_uniform_indexing_native: Bool32,
        ) -> Self {
            self.shader_storage_image_array_non_uniform_indexing_native =
                shader_storage_image_array_non_uniform_indexing_native;
            self
        }
        pub fn shader_input_attachment_array_non_uniform_indexing_native(
            mut self,
            shader_input_attachment_array_non_uniform_indexing_native: Bool32,
        ) -> Self {
            self.shader_input_attachment_array_non_uniform_indexing_native =
                shader_input_attachment_array_non_uniform_indexing_native;
            self
        }
        pub fn robust_buffer_access_update_after_bind(
            mut self,
            robust_buffer_access_update_after_bind: Bool32,
        ) -> Self {
            self.robust_buffer_access_update_after_bind = robust_buffer_access_update_after_bind;
            self
        }
        pub fn quad_divergent_implicit_lod(mut self, quad_divergent_implicit_lod: Bool32) -> Self {
            self.quad_divergent_implicit_lod = quad_divergent_implicit_lod;
            self
        }
        pub fn max_per_stage_descriptor_update_after_bind_samplers(
            mut self,
            max_per_stage_descriptor_update_after_bind_samplers: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_samplers =
                max_per_stage_descriptor_update_after_bind_samplers;
            self
        }
        pub fn max_per_stage_descriptor_update_after_bind_uniform_buffers(
            mut self,
            max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_uniform_buffers =
                max_per_stage_descriptor_update_after_bind_uniform_buffers;
            self
        }
        pub fn max_per_stage_descriptor_update_after_bind_storage_buffers(
            mut self,
            max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_storage_buffers =
                max_per_stage_descriptor_update_after_bind_storage_buffers;
            self
        }
        pub fn max_per_stage_descriptor_update_after_bind_sampled_images(
            mut self,
            max_per_stage_descriptor_update_after_bind_sampled_images: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_sampled_images =
                max_per_stage_descriptor_update_after_bind_sampled_images;
            self
        }
        pub fn max_per_stage_descriptor_update_after_bind_storage_images(
            mut self,
            max_per_stage_descriptor_update_after_bind_storage_images: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_storage_images =
                max_per_stage_descriptor_update_after_bind_storage_images;
            self
        }
        pub fn max_per_stage_descriptor_update_after_bind_input_attachments(
            mut self,
            max_per_stage_descriptor_update_after_bind_input_attachments: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_input_attachments =
                max_per_stage_descriptor_update_after_bind_input_attachments;
            self
        }
        pub fn max_per_stage_update_after_bind_resources(
            mut self,
            max_per_stage_update_after_bind_resources: u32,
        ) -> Self {
            self.max_per_stage_update_after_bind_resources =
                max_per_stage_update_after_bind_resources;
            self
        }
        pub fn max_descriptor_set_update_after_bind_samplers(
            mut self,
            max_descriptor_set_update_after_bind_samplers: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_samplers =
                max_descriptor_set_update_after_bind_samplers;
            self
        }
        pub fn max_descriptor_set_update_after_bind_uniform_buffers(
            mut self,
            max_descriptor_set_update_after_bind_uniform_buffers: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_uniform_buffers =
                max_descriptor_set_update_after_bind_uniform_buffers;
            self
        }
        pub fn max_descriptor_set_update_after_bind_uniform_buffers_dynamic(
            mut self,
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_uniform_buffers_dynamic =
                max_descriptor_set_update_after_bind_uniform_buffers_dynamic;
            self
        }
        pub fn max_descriptor_set_update_after_bind_storage_buffers(
            mut self,
            max_descriptor_set_update_after_bind_storage_buffers: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_storage_buffers =
                max_descriptor_set_update_after_bind_storage_buffers;
            self
        }
        pub fn max_descriptor_set_update_after_bind_storage_buffers_dynamic(
            mut self,
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_storage_buffers_dynamic =
                max_descriptor_set_update_after_bind_storage_buffers_dynamic;
            self
        }
        pub fn max_descriptor_set_update_after_bind_sampled_images(
            mut self,
            max_descriptor_set_update_after_bind_sampled_images: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_sampled_images =
                max_descriptor_set_update_after_bind_sampled_images;
            self
        }
        pub fn max_descriptor_set_update_after_bind_storage_images(
            mut self,
            max_descriptor_set_update_after_bind_storage_images: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_storage_images =
                max_descriptor_set_update_after_bind_storage_images;
            self
        }
        pub fn max_descriptor_set_update_after_bind_input_attachments(
            mut self,
            max_descriptor_set_update_after_bind_input_attachments: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_input_attachments =
                max_descriptor_set_update_after_bind_input_attachments;
            self
        }
        pub fn supported_depth_resolve_modes(
            mut self,
            supported_depth_resolve_modes: ResolveModeFlags,
        ) -> Self {
            self.supported_depth_resolve_modes = supported_depth_resolve_modes;
            self
        }
        pub fn supported_stencil_resolve_modes(
            mut self,
            supported_stencil_resolve_modes: ResolveModeFlags,
        ) -> Self {
            self.supported_stencil_resolve_modes = supported_stencil_resolve_modes;
            self
        }
        pub fn independent_resolve_none(mut self, independent_resolve_none: Bool32) -> Self {
            self.independent_resolve_none = independent_resolve_none;
            self
        }
        pub fn independent_resolve(mut self, independent_resolve: Bool32) -> Self {
            self.independent_resolve = independent_resolve;
            self
        }
        pub fn filter_minmax_single_component_formats(
            mut self,
            filter_minmax_single_component_formats: Bool32,
        ) -> Self {
            self.filter_minmax_single_component_formats = filter_minmax_single_component_formats;
            self
        }
        pub fn filter_minmax_image_component_mapping(
            mut self,
            filter_minmax_image_component_mapping: Bool32,
        ) -> Self {
            self.filter_minmax_image_component_mapping = filter_minmax_image_component_mapping;
            self
        }
        pub fn max_timeline_semaphore_value_difference(
            mut self,
            max_timeline_semaphore_value_difference: u64,
        ) -> Self {
            self.max_timeline_semaphore_value_difference = max_timeline_semaphore_value_difference;
            self
        }
        pub fn framebuffer_integer_color_sample_counts(
            mut self,
            framebuffer_integer_color_sample_counts: SampleCountFlags,
        ) -> Self {
            self.framebuffer_integer_color_sample_counts = framebuffer_integer_color_sample_counts;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SemaphoreType(i32);
    impl SemaphoreType {
        pub const BINARY: Self = Self(0);
        pub const TIMELINE: Self = Self(1);
        pub const BINARY_KHR: Self = Self::BINARY;
        pub const TIMELINE_KHR: Self = Self::TIMELINE;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SamplerReductionMode(i32);
    impl SamplerReductionMode {
        pub const WEIGHTED_AVERAGE: Self = Self(0);
        pub const MIN: Self = Self(1);
        pub const MAX: Self = Self(2);
        pub const WEIGHTED_AVERAGE_RANGECLAMP_QCOM: Self = Self(1000521000);
        pub const MAX_EXT: Self = Self::MAX;
        pub const MIN_EXT: Self = Self::MIN;
        pub const WEIGHTED_AVERAGE_EXT: Self = Self::WEIGHTED_AVERAGE;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DriverId(i32);
    impl DriverId {
        pub const AMD_PROPRIETARY: Self = Self(1);
        pub const AMD_OPEN_SOURCE: Self = Self(2);
        pub const MESA_RADV: Self = Self(3);
        pub const NVIDIA_PROPRIETARY: Self = Self(4);
        pub const INTEL_PROPRIETARY_WINDOWS: Self = Self(5);
        pub const INTEL_OPEN_SOURCE_MESA: Self = Self(6);
        pub const IMAGINATION_PROPRIETARY: Self = Self(7);
        pub const QUALCOMM_PROPRIETARY: Self = Self(8);
        pub const ARM_PROPRIETARY: Self = Self(9);
        pub const GOOGLE_SWIFTSHADER: Self = Self(10);
        pub const GGP_PROPRIETARY: Self = Self(11);
        pub const BROADCOM_PROPRIETARY: Self = Self(12);
        pub const MESA_LLVMPIPE: Self = Self(13);
        pub const MOLTENVK: Self = Self(14);
        pub const COREAVI_PROPRIETARY: Self = Self(15);
        pub const JUICE_PROPRIETARY: Self = Self(16);
        pub const VERISILICON_PROPRIETARY: Self = Self(17);
        pub const MESA_TURNIP: Self = Self(18);
        pub const MESA_V3DV: Self = Self(19);
        pub const MESA_PANVK: Self = Self(20);
        pub const SAMSUNG_PROPRIETARY: Self = Self(21);
        pub const MESA_VENUS: Self = Self(22);
        pub const MESA_DOZEN: Self = Self(23);
        pub const MESA_NVK: Self = Self(24);
        pub const IMAGINATION_OPEN_SOURCE_MESA: Self = Self(25);
        pub const MESA_HONEYKRISP: Self = Self(26);
        pub const VULKAN_SC_EMULATION_ON_VULKAN: Self = Self(27);
        pub const MESA_KOSMICKRISP: Self = Self(28);
        pub const AMD_OPEN_SOURCE_KHR: Self = Self::AMD_OPEN_SOURCE;
        pub const AMD_PROPRIETARY_KHR: Self = Self::AMD_PROPRIETARY;
        pub const ARM_PROPRIETARY_KHR: Self = Self::ARM_PROPRIETARY;
        pub const BROADCOM_PROPRIETARY_KHR: Self = Self::BROADCOM_PROPRIETARY;
        pub const GGP_PROPRIETARY_KHR: Self = Self::GGP_PROPRIETARY;
        pub const GOOGLE_SWIFTSHADER_KHR: Self = Self::GOOGLE_SWIFTSHADER;
        pub const IMAGINATION_PROPRIETARY_KHR: Self = Self::IMAGINATION_PROPRIETARY;
        pub const INTEL_OPEN_SOURCE_MESA_KHR: Self = Self::INTEL_OPEN_SOURCE_MESA;
        pub const INTEL_PROPRIETARY_WINDOWS_KHR: Self = Self::INTEL_PROPRIETARY_WINDOWS;
        pub const MESA_RADV_KHR: Self = Self::MESA_RADV;
        pub const NVIDIA_PROPRIETARY_KHR: Self = Self::NVIDIA_PROPRIETARY;
        pub const QUALCOMM_PROPRIETARY_KHR: Self = Self::QUALCOMM_PROPRIETARY;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ShaderFloatControlsIndependence(i32);
    impl ShaderFloatControlsIndependence {
        pub const _32_BIT_ONLY: Self = Self(0);
        pub const ALL: Self = Self(1);
        pub const NONE: Self = Self(2);
        pub const _32_BIT_ONLY_KHR: Self = Self::_32_BIT_ONLY;
        pub const ALL_KHR: Self = Self::ALL;
        pub const NONE_KHR: Self = Self::NONE;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SemaphoreWaitFlags(Flags);
    vk_bitflags_wrapped!(SemaphoreWaitFlags, Flags);
    impl SemaphoreWaitFlags {
        pub const ANY: Self = Self(SemaphoreWaitFlagBits::ANY.0);
        pub const ANY_KHR: Self = Self::ANY;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct SemaphoreWaitFlagBits(u32);
    impl SemaphoreWaitFlagBits {
        pub const ANY: Self = Self(1 << 0);
        pub const ANY_KHR: Self = Self::ANY;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DescriptorBindingFlags(Flags);
    vk_bitflags_wrapped!(DescriptorBindingFlags, Flags);
    impl DescriptorBindingFlags {
        pub const UPDATE_AFTER_BIND: Self = Self(DescriptorBindingFlagBits::UPDATE_AFTER_BIND.0);
        pub const UPDATE_UNUSED_WHILE_PENDING: Self =
            Self(DescriptorBindingFlagBits::UPDATE_UNUSED_WHILE_PENDING.0);
        pub const PARTIALLY_BOUND: Self = Self(DescriptorBindingFlagBits::PARTIALLY_BOUND.0);
        pub const VARIABLE_DESCRIPTOR_COUNT: Self =
            Self(DescriptorBindingFlagBits::VARIABLE_DESCRIPTOR_COUNT.0);
        pub const PARTIALLY_BOUND_EXT: Self = Self::PARTIALLY_BOUND;
        pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
        pub const UPDATE_UNUSED_WHILE_PENDING_EXT: Self = Self::UPDATE_UNUSED_WHILE_PENDING;
        pub const VARIABLE_DESCRIPTOR_COUNT_EXT: Self = Self::VARIABLE_DESCRIPTOR_COUNT;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DescriptorBindingFlagBits(u32);
    impl DescriptorBindingFlagBits {
        pub const UPDATE_AFTER_BIND: Self = Self(1 << 0);
        pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self(1 << 1);
        pub const PARTIALLY_BOUND: Self = Self(1 << 2);
        pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self(1 << 3);
        pub const PARTIALLY_BOUND_EXT: Self = Self::PARTIALLY_BOUND;
        pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
        pub const UPDATE_UNUSED_WHILE_PENDING_EXT: Self = Self::UPDATE_UNUSED_WHILE_PENDING;
        pub const VARIABLE_DESCRIPTOR_COUNT_EXT: Self = Self::VARIABLE_DESCRIPTOR_COUNT;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ResolveModeFlags(Flags);
    vk_bitflags_wrapped!(ResolveModeFlags, Flags);
    impl ResolveModeFlags {
        pub const SAMPLE_ZERO: Self = Self(ResolveModeFlagBits::SAMPLE_ZERO.0);
        pub const AVERAGE: Self = Self(ResolveModeFlagBits::AVERAGE.0);
        pub const MIN: Self = Self(ResolveModeFlagBits::MIN.0);
        pub const MAX: Self = Self(ResolveModeFlagBits::MAX.0);
        pub const EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID: Self =
            Self(ResolveModeFlagBits::EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID.0);
        pub const CUSTOM_EXT: Self = Self(ResolveModeFlagBits::CUSTOM_EXT.0);
        pub const AVERAGE_KHR: Self = Self::AVERAGE;
        pub const MAX_KHR: Self = Self::MAX;
        pub const MIN_KHR: Self = Self::MIN;
        pub const NONE_KHR: Self = Self::NONE;
        pub const SAMPLE_ZERO_KHR: Self = Self::SAMPLE_ZERO;
        pub const NONE: Self = Self(0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ResolveModeFlagBits(u32);
    impl ResolveModeFlagBits {
        pub const SAMPLE_ZERO: Self = Self(1 << 0);
        pub const AVERAGE: Self = Self(1 << 1);
        pub const MIN: Self = Self(1 << 2);
        pub const MAX: Self = Self(1 << 3);
        pub const EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID: Self = Self(1 << 4);
        pub const CUSTOM_EXT: Self = Self(1 << 5);
        pub const AVERAGE_KHR: Self = Self::AVERAGE;
        pub const MAX_KHR: Self = Self::MAX;
        pub const MIN_KHR: Self = Self::MIN;
        pub const SAMPLE_ZERO_KHR: Self = Self::SAMPLE_ZERO;
    }
    pub type PFN_vkResetQueryPool = unsafe extern "system" fn(
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    );
    pub type PFN_vkCreateRenderPass2 = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const RenderPassCreateInfo2<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_render_pass: *mut RenderPass,
    ) -> vk::Result;
    pub type PFN_vkCmdBeginRenderPass2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_render_pass_begin: *const RenderPassBeginInfo<'_>,
        p_subpass_begin_info: *const SubpassBeginInfo<'_>,
    );
    pub type PFN_vkCmdNextSubpass2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_subpass_begin_info: *const SubpassBeginInfo<'_>,
        p_subpass_end_info: *const SubpassEndInfo<'_>,
    );
    pub type PFN_vkCmdEndRenderPass2 = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_subpass_end_info: *const SubpassEndInfo<'_>,
    );
    pub type PFN_vkGetSemaphoreCounterValue = unsafe extern "system" fn(
        device: Device,
        semaphore: Semaphore,
        p_value: *mut u64,
    ) -> vk::Result;
    pub type PFN_vkWaitSemaphores = unsafe extern "system" fn(
        device: Device,
        p_wait_info: *const SemaphoreWaitInfo<'_>,
        timeout: u64,
    ) -> vk::Result;
    pub type PFN_vkSignalSemaphore = unsafe extern "system" fn(
        device: Device,
        p_signal_info: *const SemaphoreSignalInfo<'_>,
    ) -> vk::Result;
    pub type PFN_vkCmdDrawIndirectCount = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    );
    pub type PFN_vkCmdDrawIndexedIndirectCount = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    );
    pub type PFN_vkGetBufferOpaqueCaptureAddress = unsafe extern "system" fn(
        device: Device,
        p_info: *const BufferDeviceAddressInfo<'_>,
    ) -> u64;
    pub type PFN_vkGetBufferDeviceAddress = unsafe extern "system" fn(
        device: Device,
        p_info: *const BufferDeviceAddressInfo<'_>,
    ) -> DeviceAddress;
    pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceMemoryOpaqueCaptureAddressInfo<'_>,
    ) -> u64;
}
pub struct DeviceFn {
    reset_query_pool: PFN_vkResetQueryPool,
    get_semaphore_counter_value: PFN_vkGetSemaphoreCounterValue,
    wait_semaphores: PFN_vkWaitSemaphores,
    signal_semaphore: PFN_vkSignalSemaphore,
    get_buffer_device_address: PFN_vkGetBufferDeviceAddress,
    get_buffer_opaque_capture_address: PFN_vkGetBufferOpaqueCaptureAddress,
    get_device_memory_opaque_capture_address: PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
    cmd_draw_indirect_count: PFN_vkCmdDrawIndirectCount,
    cmd_draw_indexed_indirect_count: PFN_vkCmdDrawIndexedIndirectCount,
    create_render_pass2: PFN_vkCreateRenderPass2,
    cmd_begin_render_pass2: PFN_vkCmdBeginRenderPass2,
    cmd_next_subpass2: PFN_vkCmdNextSubpass2,
    cmd_end_render_pass2: PFN_vkCmdEndRenderPass2,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                reset_query_pool: transmute(load(c"vkResetQueryPool").ok_or(LoadingError)?),
                get_semaphore_counter_value: transmute(
                    load(c"vkGetSemaphoreCounterValue").ok_or(LoadingError)?,
                ),
                wait_semaphores: transmute(load(c"vkWaitSemaphores").ok_or(LoadingError)?),
                signal_semaphore: transmute(load(c"vkSignalSemaphore").ok_or(LoadingError)?),
                get_buffer_device_address: transmute(
                    load(c"vkGetBufferDeviceAddress").ok_or(LoadingError)?,
                ),
                get_buffer_opaque_capture_address: transmute(
                    load(c"vkGetBufferOpaqueCaptureAddress").ok_or(LoadingError)?,
                ),
                get_device_memory_opaque_capture_address: transmute(
                    load(c"vkGetDeviceMemoryOpaqueCaptureAddress").ok_or(LoadingError)?,
                ),
                cmd_draw_indirect_count: transmute(
                    load(c"vkCmdDrawIndirectCount").ok_or(LoadingError)?,
                ),
                cmd_draw_indexed_indirect_count: transmute(
                    load(c"vkCmdDrawIndexedIndirectCount").ok_or(LoadingError)?,
                ),
                create_render_pass2: transmute(load(c"vkCreateRenderPass2").ok_or(LoadingError)?),
                cmd_begin_render_pass2: transmute(
                    load(c"vkCmdBeginRenderPass2").ok_or(LoadingError)?,
                ),
                cmd_next_subpass2: transmute(load(c"vkCmdNextSubpass2").ok_or(LoadingError)?),
                cmd_end_render_pass2: transmute(load(c"vkCmdEndRenderPass2").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn reset_query_pool(
        &self,
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        unsafe { (self.reset_query_pool)(device, query_pool, first_query, query_count) }
    }
    pub unsafe fn get_semaphore_counter_value(
        &self,
        device: Device,
        semaphore: Semaphore,
    ) -> crate::Result<u64> {
        unsafe {
            let mut value = core::mem::MaybeUninit::uninit();
            let result = (self.get_semaphore_counter_value)(device, semaphore, value.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(value.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn wait_semaphores(
        &self,
        device: Device,
        wait_info: &SemaphoreWaitInfo<'_>,
        timeout: u64,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.wait_semaphores)(device, wait_info, timeout);

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::TIMEOUT => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn signal_semaphore(
        &self,
        device: Device,
        signal_info: &SemaphoreSignalInfo<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.signal_semaphore)(device, signal_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_buffer_device_address(
        &self,
        device: Device,
        info: &BufferDeviceAddressInfo<'_>,
    ) -> DeviceAddress {
        unsafe { (self.get_buffer_device_address)(device, info) }
    }
    pub unsafe fn get_buffer_opaque_capture_address(
        &self,
        device: Device,
        info: &BufferDeviceAddressInfo<'_>,
    ) -> u64 {
        unsafe { (self.get_buffer_opaque_capture_address)(device, info) }
    }
    pub unsafe fn get_device_memory_opaque_capture_address(
        &self,
        device: Device,
        info: &DeviceMemoryOpaqueCaptureAddressInfo<'_>,
    ) -> u64 {
        unsafe { (self.get_device_memory_opaque_capture_address)(device, info) }
    }
    pub unsafe fn cmd_draw_indirect_count(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_indirect_count)(
                command_buffer,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        }
    }
    pub unsafe fn cmd_draw_indexed_indirect_count(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_indexed_indirect_count)(
                command_buffer,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        }
    }
    pub unsafe fn create_render_pass2(
        &self,
        device: Device,
        create_info: &RenderPassCreateInfo2<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<RenderPass> {
        unsafe {
            let mut render_pass = core::mem::MaybeUninit::uninit();
            let result = (self.create_render_pass2)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                render_pass.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(render_pass.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn cmd_begin_render_pass2(
        &self,
        command_buffer: CommandBuffer,
        render_pass_begin: &RenderPassBeginInfo<'_>,
        subpass_begin_info: &SubpassBeginInfo<'_>,
    ) {
        unsafe {
            (self.cmd_begin_render_pass2)(command_buffer, render_pass_begin, subpass_begin_info)
        }
    }
    pub unsafe fn cmd_next_subpass2(
        &self,
        command_buffer: CommandBuffer,
        subpass_begin_info: &SubpassBeginInfo<'_>,
        subpass_end_info: &SubpassEndInfo<'_>,
    ) {
        unsafe { (self.cmd_next_subpass2)(command_buffer, subpass_begin_info, subpass_end_info) }
    }
    pub unsafe fn cmd_end_render_pass2(
        &self,
        command_buffer: CommandBuffer,
        subpass_end_info: &SubpassEndInfo<'_>,
    ) {
        unsafe { (self.cmd_end_render_pass2)(command_buffer, subpass_end_info) }
    }
}

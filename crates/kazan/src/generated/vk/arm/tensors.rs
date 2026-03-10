//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_ARM_tensors.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_ARM_tensors";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    handle_nondispatchable!(
        TensorARM,
        TENSOR_ARM,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorARM.html>"
    );
    handle_nondispatchable!(
        TensorViewARM,
        TENSOR_VIEW_ARM,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorViewARM.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorDescriptionARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct TensorDescriptionARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub tiling: TensorTilingARM,
        pub format: Format,
        pub dimension_count: u32,
        pub p_dimensions: *const i64,
        pub p_strides: *const i64,
        pub usage: TensorUsageFlagsARM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for TensorDescriptionARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TensorDescriptionARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("tiling", &self.tiling)
                .field("format", &self.format)
                .field("dimension_count", &self.dimension_count)
                .field("p_dimensions", &self.p_dimensions)
                .field("p_strides", &self.p_strides)
                .field("usage", &self.usage)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TensorDescriptionARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TENSOR_DESCRIPTION_ARM;
    }

    unsafe impl Extends<DataGraphPipelineResourceInfoARM<'_>> for TensorDescriptionARM<'_> {}
    unsafe impl Extends<DataGraphPipelineConstantARM<'_>> for TensorDescriptionARM<'_> {}

    impl Default for TensorDescriptionARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                tiling: Default::default(),
                format: Default::default(),
                dimension_count: Default::default(),
                p_dimensions: ptr::null(),
                p_strides: ptr::null(),
                usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TensorDescriptionARM<'a> {
        #[inline]
        pub fn tiling(mut self, tiling: TensorTilingARM) -> Self {
            self.tiling = tiling;
            self
        }

        #[inline]
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }

        #[inline]
        pub fn dimensions(mut self, dimensions: &'a [i64], strides: Option<&'a [i64]>) -> Self {
            self.dimension_count = dimensions.len().try_into().unwrap();
            if let Some(s) = &strides {
                assert_eq!(s.len(), self.dimension_count as usize);
            }
            self.p_dimensions = dimensions.as_ptr() as _;
            self.p_strides = strides.map_or(ptr::null(), |s| s.as_ptr() as _);
            self
        }

        #[inline]
        pub fn usage(mut self, usage: TensorUsageFlagsARM) -> Self {
            self.usage = usage;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorCreateInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct TensorCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: TensorCreateFlagsARM,
        pub p_description: *const TensorDescriptionARM<'a>,
        pub sharing_mode: SharingMode,
        pub queue_family_index_count: u32,
        pub p_queue_family_indices: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for TensorCreateInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TensorCreateInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("p_description", &self.p_description)
                .field("sharing_mode", &self.sharing_mode)
                .field("queue_family_index_count", &self.queue_family_index_count)
                .field("p_queue_family_indices", &self.p_queue_family_indices)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TensorCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TENSOR_CREATE_INFO_ARM;
    }

    impl Default for TensorCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                p_description: ptr::null(),
                sharing_mode: Default::default(),
                queue_family_index_count: Default::default(),
                p_queue_family_indices: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TensorCreateInfoARM<'a> {
        #[inline]
        pub fn flags(mut self, flags: TensorCreateFlagsARM) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn description(mut self, description: &'a TensorDescriptionARM<'a>) -> Self {
            self.p_description = description;
            self
        }

        #[inline]
        pub fn sharing_mode(mut self, sharing_mode: SharingMode) -> Self {
            self.sharing_mode = sharing_mode;
            self
        }

        #[inline]
        pub fn queue_family_indices(mut self, queue_family_indices: &'a [u32]) -> Self {
            self.queue_family_index_count = queue_family_indices.len().try_into().unwrap();
            self.p_queue_family_indices = queue_family_indices.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorViewCreateInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct TensorViewCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: TensorViewCreateFlagsARM,
        pub tensor: TensorARM,
        pub format: Format,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for TensorViewCreateInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TensorViewCreateInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("tensor", &self.tensor)
                .field("format", &self.format)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TensorViewCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TENSOR_VIEW_CREATE_INFO_ARM;
    }

    impl Default for TensorViewCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                tensor: Default::default(),
                format: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TensorViewCreateInfoARM<'a> {
        #[inline]
        pub fn flags(mut self, flags: TensorViewCreateFlagsARM) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn tensor(mut self, tensor: TensorARM) -> Self {
            self.tensor = tensor;
            self
        }

        #[inline]
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorMemoryRequirementsInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct TensorMemoryRequirementsInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub tensor: TensorARM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for TensorMemoryRequirementsInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TensorMemoryRequirementsInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("tensor", &self.tensor)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TensorMemoryRequirementsInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TENSOR_MEMORY_REQUIREMENTS_INFO_ARM;
    }

    impl Default for TensorMemoryRequirementsInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                tensor: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TensorMemoryRequirementsInfoARM<'a> {
        #[inline]
        pub fn tensor(mut self, tensor: TensorARM) -> Self {
            self.tensor = tensor;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindTensorMemoryInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BindTensorMemoryInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub tensor: TensorARM,
        pub memory: DeviceMemory,
        pub memory_offset: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BindTensorMemoryInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BindTensorMemoryInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("tensor", &self.tensor)
                .field("memory", &self.memory)
                .field("memory_offset", &self.memory_offset)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BindTensorMemoryInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_TENSOR_MEMORY_INFO_ARM;
    }

    impl Default for BindTensorMemoryInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                tensor: Default::default(),
                memory: Default::default(),
                memory_offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BindTensorMemoryInfoARM<'a> {
        #[inline]
        pub fn tensor(mut self, tensor: TensorARM) -> Self {
            self.tensor = tensor;
            self
        }

        #[inline]
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }

        #[inline]
        pub fn memory_offset(mut self, memory_offset: DeviceSize) -> Self {
            self.memory_offset = memory_offset;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkWriteDescriptorSetTensorARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct WriteDescriptorSetTensorARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub tensor_view_count: u32,
        pub p_tensor_views: *const TensorViewARM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for WriteDescriptorSetTensorARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("WriteDescriptorSetTensorARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("tensor_view_count", &self.tensor_view_count)
                .field("p_tensor_views", &self.p_tensor_views)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for WriteDescriptorSetTensorARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::WRITE_DESCRIPTOR_SET_TENSOR_ARM;
    }

    unsafe impl Extends<WriteDescriptorSet<'_>> for WriteDescriptorSetTensorARM<'_> {}

    impl Default for WriteDescriptorSetTensorARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                tensor_view_count: Default::default(),
                p_tensor_views: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> WriteDescriptorSetTensorARM<'a> {
        #[inline]
        pub fn tensor_views(mut self, tensor_views: &'a [TensorViewARM]) -> Self {
            self.tensor_view_count = tensor_views.len().try_into().unwrap();
            self.p_tensor_views = tensor_views.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorFormatPropertiesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct TensorFormatPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub optimal_tiling_tensor_features: FormatFeatureFlags2,
        pub linear_tiling_tensor_features: FormatFeatureFlags2,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for TensorFormatPropertiesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TensorFormatPropertiesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "optimal_tiling_tensor_features",
                    &self.optimal_tiling_tensor_features,
                )
                .field(
                    "linear_tiling_tensor_features",
                    &self.linear_tiling_tensor_features,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TensorFormatPropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TENSOR_FORMAT_PROPERTIES_ARM;
    }

    unsafe impl Extends<FormatProperties2<'_>> for TensorFormatPropertiesARM<'_> {}

    impl Default for TensorFormatPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                optimal_tiling_tensor_features: Default::default(),
                linear_tiling_tensor_features: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TensorFormatPropertiesARM<'a> {
        #[inline]
        pub fn optimal_tiling_tensor_features(
            mut self,
            optimal_tiling_tensor_features: FormatFeatureFlags2,
        ) -> Self {
            self.optimal_tiling_tensor_features = optimal_tiling_tensor_features;
            self
        }

        #[inline]
        pub fn linear_tiling_tensor_features(
            mut self,
            linear_tiling_tensor_features: FormatFeatureFlags2,
        ) -> Self {
            self.linear_tiling_tensor_features = linear_tiling_tensor_features;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceTensorPropertiesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceTensorPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_tensor_dimension_count: u32,
        pub max_tensor_elements: u64,
        pub max_per_dimension_tensor_elements: u64,
        pub max_tensor_stride: i64,
        pub max_tensor_size: u64,
        pub max_tensor_shader_access_array_length: u32,
        pub max_tensor_shader_access_size: u32,
        pub max_descriptor_set_storage_tensors: u32,
        pub max_per_stage_descriptor_set_storage_tensors: u32,
        pub max_descriptor_set_update_after_bind_storage_tensors: u32,
        pub max_per_stage_descriptor_update_after_bind_storage_tensors: u32,
        pub shader_storage_tensor_array_non_uniform_indexing_native: Bool32,
        pub shader_tensor_supported_stages: ShaderStageFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceTensorPropertiesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceTensorPropertiesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_tensor_dimension_count",
                    &self.max_tensor_dimension_count,
                )
                .field("max_tensor_elements", &self.max_tensor_elements)
                .field(
                    "max_per_dimension_tensor_elements",
                    &self.max_per_dimension_tensor_elements,
                )
                .field("max_tensor_stride", &self.max_tensor_stride)
                .field("max_tensor_size", &self.max_tensor_size)
                .field(
                    "max_tensor_shader_access_array_length",
                    &self.max_tensor_shader_access_array_length,
                )
                .field(
                    "max_tensor_shader_access_size",
                    &self.max_tensor_shader_access_size,
                )
                .field(
                    "max_descriptor_set_storage_tensors",
                    &self.max_descriptor_set_storage_tensors,
                )
                .field(
                    "max_per_stage_descriptor_set_storage_tensors",
                    &self.max_per_stage_descriptor_set_storage_tensors,
                )
                .field(
                    "max_descriptor_set_update_after_bind_storage_tensors",
                    &self.max_descriptor_set_update_after_bind_storage_tensors,
                )
                .field(
                    "max_per_stage_descriptor_update_after_bind_storage_tensors",
                    &self.max_per_stage_descriptor_update_after_bind_storage_tensors,
                )
                .field(
                    "shader_storage_tensor_array_non_uniform_indexing_native",
                    &self.shader_storage_tensor_array_non_uniform_indexing_native,
                )
                .field(
                    "shader_tensor_supported_stages",
                    &self.shader_tensor_supported_stages,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTensorPropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>> for PhysicalDeviceTensorPropertiesARM<'_> {}

    impl Default for PhysicalDeviceTensorPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_tensor_dimension_count: Default::default(),
                max_tensor_elements: Default::default(),
                max_per_dimension_tensor_elements: Default::default(),
                max_tensor_stride: Default::default(),
                max_tensor_size: Default::default(),
                max_tensor_shader_access_array_length: Default::default(),
                max_tensor_shader_access_size: Default::default(),
                max_descriptor_set_storage_tensors: Default::default(),
                max_per_stage_descriptor_set_storage_tensors: Default::default(),
                max_descriptor_set_update_after_bind_storage_tensors: Default::default(),
                max_per_stage_descriptor_update_after_bind_storage_tensors: Default::default(),
                shader_storage_tensor_array_non_uniform_indexing_native: Default::default(),
                shader_tensor_supported_stages: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceTensorPropertiesARM<'a> {
        #[inline]
        pub fn max_tensor_dimension_count(mut self, max_tensor_dimension_count: u32) -> Self {
            self.max_tensor_dimension_count = max_tensor_dimension_count;
            self
        }

        #[inline]
        pub fn max_tensor_elements(mut self, max_tensor_elements: u64) -> Self {
            self.max_tensor_elements = max_tensor_elements;
            self
        }

        #[inline]
        pub fn max_per_dimension_tensor_elements(
            mut self,
            max_per_dimension_tensor_elements: u64,
        ) -> Self {
            self.max_per_dimension_tensor_elements = max_per_dimension_tensor_elements;
            self
        }

        #[inline]
        pub fn max_tensor_stride(mut self, max_tensor_stride: i64) -> Self {
            self.max_tensor_stride = max_tensor_stride;
            self
        }

        #[inline]
        pub fn max_tensor_size(mut self, max_tensor_size: u64) -> Self {
            self.max_tensor_size = max_tensor_size;
            self
        }

        #[inline]
        pub fn max_tensor_shader_access_array_length(
            mut self,
            max_tensor_shader_access_array_length: u32,
        ) -> Self {
            self.max_tensor_shader_access_array_length = max_tensor_shader_access_array_length;
            self
        }

        #[inline]
        pub fn max_tensor_shader_access_size(mut self, max_tensor_shader_access_size: u32) -> Self {
            self.max_tensor_shader_access_size = max_tensor_shader_access_size;
            self
        }

        #[inline]
        pub fn max_descriptor_set_storage_tensors(
            mut self,
            max_descriptor_set_storage_tensors: u32,
        ) -> Self {
            self.max_descriptor_set_storage_tensors = max_descriptor_set_storage_tensors;
            self
        }

        #[inline]
        pub fn max_per_stage_descriptor_set_storage_tensors(
            mut self,
            max_per_stage_descriptor_set_storage_tensors: u32,
        ) -> Self {
            self.max_per_stage_descriptor_set_storage_tensors =
                max_per_stage_descriptor_set_storage_tensors;
            self
        }

        #[inline]
        pub fn max_descriptor_set_update_after_bind_storage_tensors(
            mut self,
            max_descriptor_set_update_after_bind_storage_tensors: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_storage_tensors =
                max_descriptor_set_update_after_bind_storage_tensors;
            self
        }

        #[inline]
        pub fn max_per_stage_descriptor_update_after_bind_storage_tensors(
            mut self,
            max_per_stage_descriptor_update_after_bind_storage_tensors: u32,
        ) -> Self {
            self.max_per_stage_descriptor_update_after_bind_storage_tensors =
                max_per_stage_descriptor_update_after_bind_storage_tensors;
            self
        }

        #[inline]
        pub fn shader_storage_tensor_array_non_uniform_indexing_native(
            mut self,
            shader_storage_tensor_array_non_uniform_indexing_native: bool,
        ) -> Self {
            self.shader_storage_tensor_array_non_uniform_indexing_native =
                shader_storage_tensor_array_non_uniform_indexing_native.into();
            self
        }

        #[inline]
        pub fn shader_tensor_supported_stages(
            mut self,
            shader_tensor_supported_stages: ShaderStageFlags,
        ) -> Self {
            self.shader_tensor_supported_stages = shader_tensor_supported_stages;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorMemoryBarrierARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct TensorMemoryBarrierARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_stage_mask: PipelineStageFlags2,
        pub src_access_mask: AccessFlags2,
        pub dst_stage_mask: PipelineStageFlags2,
        pub dst_access_mask: AccessFlags2,
        pub src_queue_family_index: u32,
        pub dst_queue_family_index: u32,
        pub tensor: TensorARM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for TensorMemoryBarrierARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TensorMemoryBarrierARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_stage_mask", &self.src_stage_mask)
                .field("src_access_mask", &self.src_access_mask)
                .field("dst_stage_mask", &self.dst_stage_mask)
                .field("dst_access_mask", &self.dst_access_mask)
                .field("src_queue_family_index", &self.src_queue_family_index)
                .field("dst_queue_family_index", &self.dst_queue_family_index)
                .field("tensor", &self.tensor)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TensorMemoryBarrierARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TENSOR_MEMORY_BARRIER_ARM;
    }

    unsafe impl Extends<DependencyInfo<'_>> for TensorMemoryBarrierARM<'_> {}

    impl Default for TensorMemoryBarrierARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_stage_mask: Default::default(),
                src_access_mask: Default::default(),
                dst_stage_mask: Default::default(),
                dst_access_mask: Default::default(),
                src_queue_family_index: Default::default(),
                dst_queue_family_index: Default::default(),
                tensor: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TensorMemoryBarrierARM<'a> {
        #[inline]
        pub fn src_stage_mask(mut self, src_stage_mask: PipelineStageFlags2) -> Self {
            self.src_stage_mask = src_stage_mask;
            self
        }

        #[inline]
        pub fn src_access_mask(mut self, src_access_mask: AccessFlags2) -> Self {
            self.src_access_mask = src_access_mask;
            self
        }

        #[inline]
        pub fn dst_stage_mask(mut self, dst_stage_mask: PipelineStageFlags2) -> Self {
            self.dst_stage_mask = dst_stage_mask;
            self
        }

        #[inline]
        pub fn dst_access_mask(mut self, dst_access_mask: AccessFlags2) -> Self {
            self.dst_access_mask = dst_access_mask;
            self
        }

        #[inline]
        pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
            self.src_queue_family_index = src_queue_family_index;
            self
        }

        #[inline]
        pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
            self.dst_queue_family_index = dst_queue_family_index;
            self
        }

        #[inline]
        pub fn tensor(mut self, tensor: TensorARM) -> Self {
            self.tensor = tensor;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorDependencyInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct TensorDependencyInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub tensor_memory_barrier_count: u32,
        pub p_tensor_memory_barriers: *const TensorMemoryBarrierARM<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for TensorDependencyInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TensorDependencyInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "tensor_memory_barrier_count",
                    &self.tensor_memory_barrier_count,
                )
                .field("p_tensor_memory_barriers", &self.p_tensor_memory_barriers)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TensorDependencyInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TENSOR_DEPENDENCY_INFO_ARM;
    }

    unsafe impl Extends<DependencyInfo<'_>> for TensorDependencyInfoARM<'_> {}

    impl Default for TensorDependencyInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                tensor_memory_barrier_count: Default::default(),
                p_tensor_memory_barriers: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TensorDependencyInfoARM<'a> {
        #[inline]
        pub fn tensor_memory_barrier_count(mut self, tensor_memory_barrier_count: u32) -> Self {
            self.tensor_memory_barrier_count = tensor_memory_barrier_count;
            self
        }

        #[inline]
        pub fn tensor_memory_barriers(
            mut self,
            tensor_memory_barriers: &'a TensorMemoryBarrierARM<'a>,
        ) -> Self {
            self.p_tensor_memory_barriers = tensor_memory_barriers;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceTensorFeaturesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceTensorFeaturesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub tensor_non_packed: Bool32,
        pub shader_tensor_access: Bool32,
        pub shader_storage_tensor_array_dynamic_indexing: Bool32,
        pub shader_storage_tensor_array_non_uniform_indexing: Bool32,
        pub descriptor_binding_storage_tensor_update_after_bind: Bool32,
        pub tensors: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceTensorFeaturesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceTensorFeaturesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("tensor_non_packed", &self.tensor_non_packed)
                .field("shader_tensor_access", &self.shader_tensor_access)
                .field(
                    "shader_storage_tensor_array_dynamic_indexing",
                    &self.shader_storage_tensor_array_dynamic_indexing,
                )
                .field(
                    "shader_storage_tensor_array_non_uniform_indexing",
                    &self.shader_storage_tensor_array_non_uniform_indexing,
                )
                .field(
                    "descriptor_binding_storage_tensor_update_after_bind",
                    &self.descriptor_binding_storage_tensor_update_after_bind,
                )
                .field("tensors", &self.tensors)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceTensorFeaturesARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_TENSOR_FEATURES_ARM;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceTensorFeaturesARM<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceTensorFeaturesARM<'_> {}

    impl Default for PhysicalDeviceTensorFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                tensor_non_packed: Default::default(),
                shader_tensor_access: Default::default(),
                shader_storage_tensor_array_dynamic_indexing: Default::default(),
                shader_storage_tensor_array_non_uniform_indexing: Default::default(),
                descriptor_binding_storage_tensor_update_after_bind: Default::default(),
                tensors: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceTensorFeaturesARM<'a> {
        #[inline]
        pub fn tensor_non_packed(mut self, tensor_non_packed: bool) -> Self {
            self.tensor_non_packed = tensor_non_packed.into();
            self
        }

        #[inline]
        pub fn shader_tensor_access(mut self, shader_tensor_access: bool) -> Self {
            self.shader_tensor_access = shader_tensor_access.into();
            self
        }

        #[inline]
        pub fn shader_storage_tensor_array_dynamic_indexing(
            mut self,
            shader_storage_tensor_array_dynamic_indexing: bool,
        ) -> Self {
            self.shader_storage_tensor_array_dynamic_indexing =
                shader_storage_tensor_array_dynamic_indexing.into();
            self
        }

        #[inline]
        pub fn shader_storage_tensor_array_non_uniform_indexing(
            mut self,
            shader_storage_tensor_array_non_uniform_indexing: bool,
        ) -> Self {
            self.shader_storage_tensor_array_non_uniform_indexing =
                shader_storage_tensor_array_non_uniform_indexing.into();
            self
        }

        #[inline]
        pub fn descriptor_binding_storage_tensor_update_after_bind(
            mut self,
            descriptor_binding_storage_tensor_update_after_bind: bool,
        ) -> Self {
            self.descriptor_binding_storage_tensor_update_after_bind =
                descriptor_binding_storage_tensor_update_after_bind.into();
            self
        }

        #[inline]
        pub fn tensors(mut self, tensors: bool) -> Self {
            self.tensors = tensors.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceTensorMemoryRequirementsARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceTensorMemoryRequirementsARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_create_info: *const TensorCreateInfoARM<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceTensorMemoryRequirementsARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceTensorMemoryRequirementsARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_create_info", &self.p_create_info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceTensorMemoryRequirementsARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM;
    }

    impl Default for DeviceTensorMemoryRequirementsARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_create_info: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceTensorMemoryRequirementsARM<'a> {
        #[inline]
        pub fn create_info(mut self, create_info: &'a TensorCreateInfoARM<'a>) -> Self {
            self.p_create_info = create_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyTensorInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CopyTensorInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_tensor: TensorARM,
        pub dst_tensor: TensorARM,
        pub region_count: u32,
        pub p_regions: *const TensorCopyARM<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyTensorInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyTensorInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_tensor", &self.src_tensor)
                .field("dst_tensor", &self.dst_tensor)
                .field("region_count", &self.region_count)
                .field("p_regions", &self.p_regions)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyTensorInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_TENSOR_INFO_ARM;
    }

    impl Default for CopyTensorInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                src_tensor: Default::default(),
                dst_tensor: Default::default(),
                region_count: Default::default(),
                p_regions: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CopyTensorInfoARM<'a> {
        #[inline]
        pub fn src_tensor(mut self, src_tensor: TensorARM) -> Self {
            self.src_tensor = src_tensor;
            self
        }

        #[inline]
        pub fn dst_tensor(mut self, dst_tensor: TensorARM) -> Self {
            self.dst_tensor = dst_tensor;
            self
        }

        #[inline]
        pub fn regions(mut self, regions: &'a [TensorCopyARM<'_>]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorCopyARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct TensorCopyARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub dimension_count: u32,
        pub p_src_offset: *const u64,
        pub p_dst_offset: *const u64,
        pub p_extent: *const u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for TensorCopyARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TensorCopyARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("dimension_count", &self.dimension_count)
                .field("p_src_offset", &self.p_src_offset)
                .field("p_dst_offset", &self.p_dst_offset)
                .field("p_extent", &self.p_extent)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TensorCopyARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TENSOR_COPY_ARM;
    }

    impl Default for TensorCopyARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                dimension_count: Default::default(),
                p_src_offset: ptr::null(),
                p_dst_offset: ptr::null(),
                p_extent: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TensorCopyARM<'a> {
        #[inline]
        pub fn dimensions(
            mut self,
            src_offset: Option<&'a [u64]>,
            dst_offset: Option<&'a [u64]>,
            extent: Option<&'a [u64]>,
        ) -> Self {
            self.dimension_count = None
                .or_else(|| src_offset.as_deref().map(|s| s.len()))
                .or_else(|| dst_offset.as_deref().map(|s| s.len()))
                .or_else(|| extent.as_deref().map(|s| s.len()))
                .unwrap_or(0)
                .try_into()
                .unwrap();
            if let Some(s) = &dst_offset {
                assert_eq!(s.len(), self.dimension_count as usize);
            }
            if let Some(s) = &extent {
                assert_eq!(s.len(), self.dimension_count as usize);
            }
            self.p_src_offset = src_offset.map_or(ptr::null(), |s| s.as_ptr() as _);
            self.p_dst_offset = dst_offset.map_or(ptr::null(), |s| s.as_ptr() as _);
            self.p_extent = extent.map_or(ptr::null(), |s| s.as_ptr() as _);
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryDedicatedAllocateInfoTensorARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryDedicatedAllocateInfoTensorARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub tensor: TensorARM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryDedicatedAllocateInfoTensorARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryDedicatedAllocateInfoTensorARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("tensor", &self.tensor)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryDedicatedAllocateInfoTensorARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM;
    }

    unsafe impl Extends<MemoryAllocateInfo<'_>> for MemoryDedicatedAllocateInfoTensorARM<'_> {}

    impl Default for MemoryDedicatedAllocateInfoTensorARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                tensor: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryDedicatedAllocateInfoTensorARM<'a> {
        #[inline]
        pub fn tensor(mut self, tensor: TensorARM) -> Self {
            self.tensor = tensor;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDescriptorBufferTensorPropertiesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDescriptorBufferTensorPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub tensor_capture_replay_descriptor_data_size: usize,
        pub tensor_view_capture_replay_descriptor_data_size: usize,
        pub tensor_descriptor_size: usize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDescriptorBufferTensorPropertiesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDescriptorBufferTensorPropertiesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "tensor_capture_replay_descriptor_data_size",
                    &self.tensor_capture_replay_descriptor_data_size,
                )
                .field(
                    "tensor_view_capture_replay_descriptor_data_size",
                    &self.tensor_view_capture_replay_descriptor_data_size,
                )
                .field("tensor_descriptor_size", &self.tensor_descriptor_size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDescriptorBufferTensorPropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceDescriptorBufferTensorPropertiesARM<'_>
    {
    }

    impl Default for PhysicalDeviceDescriptorBufferTensorPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                tensor_capture_replay_descriptor_data_size: Default::default(),
                tensor_view_capture_replay_descriptor_data_size: Default::default(),
                tensor_descriptor_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDescriptorBufferTensorPropertiesARM<'a> {
        #[inline]
        pub fn tensor_capture_replay_descriptor_data_size(
            mut self,
            tensor_capture_replay_descriptor_data_size: usize,
        ) -> Self {
            self.tensor_capture_replay_descriptor_data_size =
                tensor_capture_replay_descriptor_data_size;
            self
        }

        #[inline]
        pub fn tensor_view_capture_replay_descriptor_data_size(
            mut self,
            tensor_view_capture_replay_descriptor_data_size: usize,
        ) -> Self {
            self.tensor_view_capture_replay_descriptor_data_size =
                tensor_view_capture_replay_descriptor_data_size;
            self
        }

        #[inline]
        pub fn tensor_descriptor_size(mut self, tensor_descriptor_size: usize) -> Self {
            self.tensor_descriptor_size = tensor_descriptor_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDescriptorBufferTensorFeaturesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDescriptorBufferTensorFeaturesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub descriptor_buffer_tensor_descriptors: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDescriptorBufferTensorFeaturesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDescriptorBufferTensorFeaturesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "descriptor_buffer_tensor_descriptors",
                    &self.descriptor_buffer_tensor_descriptors,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDescriptorBufferTensorFeaturesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceDescriptorBufferTensorFeaturesARM<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceDescriptorBufferTensorFeaturesARM<'_> {}

    impl Default for PhysicalDeviceDescriptorBufferTensorFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                descriptor_buffer_tensor_descriptors: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDescriptorBufferTensorFeaturesARM<'a> {
        #[inline]
        pub fn descriptor_buffer_tensor_descriptors(
            mut self,
            descriptor_buffer_tensor_descriptors: bool,
        ) -> Self {
            self.descriptor_buffer_tensor_descriptors = descriptor_buffer_tensor_descriptors.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorCaptureDescriptorDataInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct TensorCaptureDescriptorDataInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub tensor: TensorARM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for TensorCaptureDescriptorDataInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TensorCaptureDescriptorDataInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("tensor", &self.tensor)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TensorCaptureDescriptorDataInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM;
    }

    impl Default for TensorCaptureDescriptorDataInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                tensor: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TensorCaptureDescriptorDataInfoARM<'a> {
        #[inline]
        pub fn tensor(mut self, tensor: TensorARM) -> Self {
            self.tensor = tensor;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorViewCaptureDescriptorDataInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct TensorViewCaptureDescriptorDataInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub tensor_view: TensorViewARM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for TensorViewCaptureDescriptorDataInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TensorViewCaptureDescriptorDataInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("tensor_view", &self.tensor_view)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TensorViewCaptureDescriptorDataInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM;
    }

    impl Default for TensorViewCaptureDescriptorDataInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                tensor_view: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TensorViewCaptureDescriptorDataInfoARM<'a> {
        #[inline]
        pub fn tensor_view(mut self, tensor_view: TensorViewARM) -> Self {
            self.tensor_view = tensor_view;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorGetTensorInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DescriptorGetTensorInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub tensor_view: TensorViewARM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorGetTensorInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorGetTensorInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("tensor_view", &self.tensor_view)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DescriptorGetTensorInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DESCRIPTOR_GET_TENSOR_INFO_ARM;
    }

    unsafe impl Extends<DescriptorGetInfoEXT<'_>> for DescriptorGetTensorInfoARM<'_> {}

    impl Default for DescriptorGetTensorInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                tensor_view: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DescriptorGetTensorInfoARM<'a> {
        #[inline]
        pub fn tensor_view(mut self, tensor_view: TensorViewARM) -> Self {
            self.tensor_view = tensor_view;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFrameBoundaryTensorsARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct FrameBoundaryTensorsARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub tensor_count: u32,
        pub p_tensors: *const TensorARM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for FrameBoundaryTensorsARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("FrameBoundaryTensorsARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("tensor_count", &self.tensor_count)
                .field("p_tensors", &self.p_tensors)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for FrameBoundaryTensorsARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::FRAME_BOUNDARY_TENSORS_ARM;
    }

    unsafe impl Extends<SubmitInfo<'_>> for FrameBoundaryTensorsARM<'_> {}
    unsafe impl Extends<SubmitInfo2<'_>> for FrameBoundaryTensorsARM<'_> {}
    unsafe impl Extends<PresentInfoKHR<'_>> for FrameBoundaryTensorsARM<'_> {}
    unsafe impl Extends<BindSparseInfo<'_>> for FrameBoundaryTensorsARM<'_> {}

    impl Default for FrameBoundaryTensorsARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                tensor_count: Default::default(),
                p_tensors: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> FrameBoundaryTensorsARM<'a> {
        #[inline]
        pub fn tensors(mut self, tensors: &'a [TensorARM]) -> Self {
            self.tensor_count = tensors.len().try_into().unwrap();
            self.p_tensors = tensors.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExternalTensorInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceExternalTensorInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: TensorCreateFlagsARM,
        pub p_description: *const TensorDescriptionARM<'a>,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceExternalTensorInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceExternalTensorInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("p_description", &self.p_description)
                .field("handle_type", &self.handle_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExternalTensorInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM;
    }

    impl Default for PhysicalDeviceExternalTensorInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                p_description: ptr::null(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceExternalTensorInfoARM<'a> {
        #[inline]
        pub fn flags(mut self, flags: TensorCreateFlagsARM) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn description(mut self, description: &'a TensorDescriptionARM<'a>) -> Self {
            self.p_description = description;
            self
        }

        #[inline]
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalTensorPropertiesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExternalTensorPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub external_memory_properties: ExternalMemoryProperties,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExternalTensorPropertiesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExternalTensorPropertiesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "external_memory_properties",
                    &self.external_memory_properties,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExternalTensorPropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_TENSOR_PROPERTIES_ARM;
    }

    impl Default for ExternalTensorPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                external_memory_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExternalTensorPropertiesARM<'a> {
        #[inline]
        pub fn external_memory_properties(
            mut self,
            external_memory_properties: ExternalMemoryProperties,
        ) -> Self {
            self.external_memory_properties = external_memory_properties;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryTensorCreateInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExternalMemoryTensorCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_types: ExternalMemoryHandleTypeFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExternalMemoryTensorCreateInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExternalMemoryTensorCreateInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("handle_types", &self.handle_types)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExternalMemoryTensorCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM;
    }

    unsafe impl Extends<TensorCreateInfoARM<'_>> for ExternalMemoryTensorCreateInfoARM<'_> {}

    impl Default for ExternalMemoryTensorCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                handle_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExternalMemoryTensorCreateInfoARM<'a> {
        #[inline]
        pub fn handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlags) -> Self {
            self.handle_types = handle_types;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorTilingARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TensorTilingARM(i32);

    impl TensorTilingARM {
        pub const OPTIMAL_ARM: Self = Self(0);
        pub const LINEAR_ARM: Self = Self(1);
    }

    impl fmt::Debug for TensorTilingARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::OPTIMAL_ARM => Some("OPTIMAL_ARM"),
                Self::LINEAR_ARM => Some("LINEAR_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorCreateFlagsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct TensorCreateFlagsARM(Flags64);
    vk_bitflags_wrapped!(TensorCreateFlagsARM, Flags64, TensorCreateFlagBitsARM);

    impl fmt::Debug for TensorCreateFlagsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags64, &str)] = &[
                (
                    TensorCreateFlagBitsARM::MUTABLE_FORMAT_ARM.0,
                    "MUTABLE_FORMAT_ARM",
                ),
                (TensorCreateFlagBitsARM::PROTECTED_ARM.0, "PROTECTED_ARM"),
                (
                    TensorCreateFlagBitsARM::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM.0,
                    "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM",
                ),
                (
                    TensorCreateFlagBitsARM::DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM.0,
                    "DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorCreateFlagBitsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct TensorCreateFlagBitsARM(u64);

    impl TensorCreateFlagBitsARM {
        pub const MUTABLE_FORMAT_ARM: Self = Self(1 << 0);
        pub const PROTECTED_ARM: Self = Self(1 << 1);
        // VK_ARM_tensors
        pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM: Self = Self(1 << 2);

        // VK_EXT_descriptor_heap
        pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM: Self = Self(1 << 3);
    }

    impl fmt::Debug for TensorCreateFlagBitsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::MUTABLE_FORMAT_ARM => Some("MUTABLE_FORMAT_ARM"),
                Self::PROTECTED_ARM => Some("PROTECTED_ARM"),
                Self::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM => {
                    Some("DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM")
                }
                Self::DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM => {
                    Some("DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM")
                }
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorUsageFlagsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct TensorUsageFlagsARM(Flags64);
    vk_bitflags_wrapped!(TensorUsageFlagsARM, Flags64, TensorUsageFlagBitsARM);

    impl fmt::Debug for TensorUsageFlagsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags64, &str)] = &[
                (TensorUsageFlagBitsARM::SHADER_ARM.0, "SHADER_ARM"),
                (
                    TensorUsageFlagBitsARM::TRANSFER_SRC_ARM.0,
                    "TRANSFER_SRC_ARM",
                ),
                (
                    TensorUsageFlagBitsARM::TRANSFER_DST_ARM.0,
                    "TRANSFER_DST_ARM",
                ),
                (
                    TensorUsageFlagBitsARM::IMAGE_ALIASING_ARM.0,
                    "IMAGE_ALIASING_ARM",
                ),
                (TensorUsageFlagBitsARM::DATA_GRAPH_ARM.0, "DATA_GRAPH_ARM"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorUsageFlagBitsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct TensorUsageFlagBitsARM(u64);

    impl TensorUsageFlagBitsARM {
        /// Tensor written/read through shader descriptor
        pub const SHADER_ARM: Self = Self(1 << 1);
        /// Tensor can be src of a transfer operation
        pub const TRANSFER_SRC_ARM: Self = Self(1 << 2);
        /// Tensor can be dst of a transfer operation
        pub const TRANSFER_DST_ARM: Self = Self(1 << 3);
        /// Tensor can be aliased with an image
        pub const IMAGE_ALIASING_ARM: Self = Self(1 << 4);
        // VK_ARM_data_graph
        pub const DATA_GRAPH_ARM: Self = Self(1 << 5);
    }

    impl fmt::Debug for TensorUsageFlagBitsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SHADER_ARM => Some("SHADER_ARM"),
                Self::TRANSFER_SRC_ARM => Some("TRANSFER_SRC_ARM"),
                Self::TRANSFER_DST_ARM => Some("TRANSFER_DST_ARM"),
                Self::IMAGE_ALIASING_ARM => Some("IMAGE_ALIASING_ARM"),
                Self::DATA_GRAPH_ARM => Some("DATA_GRAPH_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorViewCreateFlagsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct TensorViewCreateFlagsARM(Flags64);
    vk_bitflags_wrapped!(
        TensorViewCreateFlagsARM,
        Flags64,
        TensorViewCreateFlagBitsARM
    );

    impl fmt::Debug for TensorViewCreateFlagsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags64, &str)] = &[(
                TensorViewCreateFlagBitsARM::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM.0,
                "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM",
            )];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorViewCreateFlagBitsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct TensorViewCreateFlagBitsARM(u64);

    impl TensorViewCreateFlagBitsARM {
        // VK_ARM_tensors
        pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM: Self = Self(1 << 0);
    }

    impl fmt::Debug for TensorViewCreateFlagBitsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM => {
                    Some("DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM")
                }
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateTensorARM.html>
    pub type PFN_vkCreateTensorARM = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const TensorCreateInfoARM<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_tensor: *mut TensorARM,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyTensorARM.html>
    pub type PFN_vkDestroyTensorARM = unsafe extern "system" fn(
        device: Device,
        tensor: TensorARM,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateTensorViewARM.html>
    pub type PFN_vkCreateTensorViewARM = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const TensorViewCreateInfoARM<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_view: *mut TensorViewARM,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyTensorViewARM.html>
    pub type PFN_vkDestroyTensorViewARM = unsafe extern "system" fn(
        device: Device,
        tensor_view: TensorViewARM,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetTensorMemoryRequirementsARM.html>
    pub type PFN_vkGetTensorMemoryRequirementsARM = unsafe extern "system" fn(
        device: Device,
        p_info: *const TensorMemoryRequirementsInfoARM<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBindTensorMemoryARM.html>
    pub type PFN_vkBindTensorMemoryARM = unsafe extern "system" fn(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindTensorMemoryInfoARM<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceTensorMemoryRequirementsARM.html>
    pub type PFN_vkGetDeviceTensorMemoryRequirementsARM = unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceTensorMemoryRequirementsARM<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyTensorARM.html>
    pub type PFN_vkCmdCopyTensorARM = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_tensor_info: *const CopyTensorInfoARM<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetTensorOpaqueCaptureDescriptorDataARM.html>
    pub type PFN_vkGetTensorOpaqueCaptureDescriptorDataARM =
        unsafe extern "system" fn(
            device: Device,
            p_info: *const TensorCaptureDescriptorDataInfoARM<'_>,
            p_data: *mut c_void,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetTensorViewOpaqueCaptureDescriptorDataARM.html>
    pub type PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM =
        unsafe extern "system" fn(
            device: Device,
            p_info: *const TensorViewCaptureDescriptorDataInfoARM<'_>,
            p_data: *mut c_void,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceExternalTensorPropertiesARM.html>
    pub type PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_external_tensor_info: *const PhysicalDeviceExternalTensorInfoARM<'_>,
        p_external_tensor_properties: *mut ExternalTensorPropertiesARM<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkTensorARM = TensorARM;
    pub type VkTensorViewARM = TensorViewARM;
    pub type VkTensorDescriptionARM = TensorDescriptionARM<'static>;
    pub type VkTensorCreateInfoARM = TensorCreateInfoARM<'static>;
    pub type VkTensorViewCreateInfoARM = TensorViewCreateInfoARM<'static>;
    pub type VkTensorMemoryRequirementsInfoARM = TensorMemoryRequirementsInfoARM<'static>;
    pub type VkBindTensorMemoryInfoARM = BindTensorMemoryInfoARM<'static>;
    pub type VkWriteDescriptorSetTensorARM = WriteDescriptorSetTensorARM<'static>;
    pub type VkTensorFormatPropertiesARM = TensorFormatPropertiesARM<'static>;
    pub type VkPhysicalDeviceTensorPropertiesARM = PhysicalDeviceTensorPropertiesARM<'static>;
    pub type VkTensorMemoryBarrierARM = TensorMemoryBarrierARM<'static>;
    pub type VkTensorDependencyInfoARM = TensorDependencyInfoARM<'static>;
    pub type VkPhysicalDeviceTensorFeaturesARM = PhysicalDeviceTensorFeaturesARM<'static>;
    pub type VkDeviceTensorMemoryRequirementsARM = DeviceTensorMemoryRequirementsARM<'static>;
    pub type VkCopyTensorInfoARM = CopyTensorInfoARM<'static>;
    pub type VkTensorCopyARM = TensorCopyARM<'static>;
    pub type VkMemoryDedicatedAllocateInfoTensorARM = MemoryDedicatedAllocateInfoTensorARM<'static>;
    pub type VkPhysicalDeviceDescriptorBufferTensorPropertiesARM =
        PhysicalDeviceDescriptorBufferTensorPropertiesARM<'static>;
    pub type VkPhysicalDeviceDescriptorBufferTensorFeaturesARM =
        PhysicalDeviceDescriptorBufferTensorFeaturesARM<'static>;
    pub type VkTensorCaptureDescriptorDataInfoARM = TensorCaptureDescriptorDataInfoARM<'static>;
    pub type VkTensorViewCaptureDescriptorDataInfoARM =
        TensorViewCaptureDescriptorDataInfoARM<'static>;
    pub type VkDescriptorGetTensorInfoARM = DescriptorGetTensorInfoARM<'static>;
    pub type VkFrameBoundaryTensorsARM = FrameBoundaryTensorsARM<'static>;
    pub type VkPhysicalDeviceExternalTensorInfoARM = PhysicalDeviceExternalTensorInfoARM<'static>;
    pub type VkExternalTensorPropertiesARM = ExternalTensorPropertiesARM<'static>;
    pub type VkExternalMemoryTensorCreateInfoARM = ExternalMemoryTensorCreateInfoARM<'static>;
    pub type VkTensorTilingARM = TensorTilingARM;
    pub type VkTensorCreateFlagsARM = TensorCreateFlagsARM;
    pub type VkTensorCreateFlagBitsARM = TensorCreateFlagBitsARM;
    pub type VkTensorUsageFlagsARM = TensorUsageFlagsARM;
    pub type VkTensorUsageFlagBitsARM = TensorUsageFlagBitsARM;
    pub type VkTensorViewCreateFlagsARM = TensorViewCreateFlagsARM;
    pub type VkTensorViewCreateFlagBitsARM = TensorViewCreateFlagBitsARM;
    impl TensorDescriptionARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkTensorDescriptionARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl TensorCreateInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkTensorCreateInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl TensorViewCreateInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkTensorViewCreateInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl TensorMemoryRequirementsInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkTensorMemoryRequirementsInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BindTensorMemoryInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBindTensorMemoryInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl WriteDescriptorSetTensorARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkWriteDescriptorSetTensorARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl TensorFormatPropertiesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkTensorFormatPropertiesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceTensorPropertiesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceTensorPropertiesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl TensorMemoryBarrierARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkTensorMemoryBarrierARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl TensorDependencyInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkTensorDependencyInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceTensorFeaturesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceTensorFeaturesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceTensorMemoryRequirementsARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceTensorMemoryRequirementsARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CopyTensorInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCopyTensorInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl TensorCopyARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkTensorCopyARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MemoryDedicatedAllocateInfoTensorARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMemoryDedicatedAllocateInfoTensorARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceDescriptorBufferTensorPropertiesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceDescriptorBufferTensorPropertiesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceDescriptorBufferTensorFeaturesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceDescriptorBufferTensorFeaturesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl TensorCaptureDescriptorDataInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkTensorCaptureDescriptorDataInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl TensorViewCaptureDescriptorDataInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkTensorViewCaptureDescriptorDataInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DescriptorGetTensorInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDescriptorGetTensorInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl FrameBoundaryTensorsARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkFrameBoundaryTensorsARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceExternalTensorInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceExternalTensorInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExternalTensorPropertiesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExternalTensorPropertiesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExternalMemoryTensorCreateInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExternalMemoryTensorCreateInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    get_physical_device_external_tensor_properties_arm:
        PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_external_tensor_properties_arm: transmute(
                    load(c"vkGetPhysicalDeviceExternalTensorPropertiesARM")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceExternalTensorPropertiesARM.html>
    #[inline]
    pub unsafe fn get_physical_device_external_tensor_properties_arm(
        &self,
        physical_device: PhysicalDevice,
        external_tensor_info: &PhysicalDeviceExternalTensorInfoARM<'_>,
        external_tensor_properties: &mut ExternalTensorPropertiesARM<'_>,
    ) {
        unsafe {
            (self.get_physical_device_external_tensor_properties_arm)(
                physical_device,
                external_tensor_info,
                external_tensor_properties,
            )
        }
    }
}

pub struct DeviceFn {
    create_tensor_arm: PFN_vkCreateTensorARM,
    destroy_tensor_arm: PFN_vkDestroyTensorARM,
    create_tensor_view_arm: PFN_vkCreateTensorViewARM,
    destroy_tensor_view_arm: PFN_vkDestroyTensorViewARM,
    get_tensor_memory_requirements_arm: PFN_vkGetTensorMemoryRequirementsARM,
    bind_tensor_memory_arm: PFN_vkBindTensorMemoryARM,
    get_device_tensor_memory_requirements_arm: PFN_vkGetDeviceTensorMemoryRequirementsARM,
    cmd_copy_tensor_arm: PFN_vkCmdCopyTensorARM,
    get_tensor_opaque_capture_descriptor_data_arm:
        Option<PFN_vkGetTensorOpaqueCaptureDescriptorDataARM>,
    get_tensor_view_opaque_capture_descriptor_data_arm:
        Option<PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM>,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_tensor_arm: transmute(
                    load(c"vkCreateTensorARM").ok_or(MissingEntryPointError)?,
                ),
                destroy_tensor_arm: transmute(
                    load(c"vkDestroyTensorARM").ok_or(MissingEntryPointError)?,
                ),
                create_tensor_view_arm: transmute(
                    load(c"vkCreateTensorViewARM").ok_or(MissingEntryPointError)?,
                ),
                destroy_tensor_view_arm: transmute(
                    load(c"vkDestroyTensorViewARM").ok_or(MissingEntryPointError)?,
                ),
                get_tensor_memory_requirements_arm: transmute(
                    load(c"vkGetTensorMemoryRequirementsARM").ok_or(MissingEntryPointError)?,
                ),
                bind_tensor_memory_arm: transmute(
                    load(c"vkBindTensorMemoryARM").ok_or(MissingEntryPointError)?,
                ),
                get_device_tensor_memory_requirements_arm: transmute(
                    load(c"vkGetDeviceTensorMemoryRequirementsARM")
                        .ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_tensor_arm: transmute(
                    load(c"vkCmdCopyTensorARM").ok_or(MissingEntryPointError)?,
                ),
                get_tensor_opaque_capture_descriptor_data_arm: transmute(load(
                    c"vkGetTensorOpaqueCaptureDescriptorDataARM",
                )),
                get_tensor_view_opaque_capture_descriptor_data_arm: transmute(load(
                    c"vkGetTensorViewOpaqueCaptureDescriptorDataARM",
                )),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateTensorARM.html>
    #[inline]
    pub unsafe fn create_tensor_arm(
        &self,
        device: Device,
        create_info: &TensorCreateInfoARM<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<TensorARM> {
        unsafe {
            let mut tensor = core::mem::MaybeUninit::uninit();
            let result = (self.create_tensor_arm)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                tensor.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(tensor.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyTensorARM.html>
    #[inline]
    pub unsafe fn destroy_tensor_arm(
        &self,
        device: Device,
        tensor: TensorARM,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_tensor_arm)(device, tensor, allocator.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateTensorViewARM.html>
    #[inline]
    pub unsafe fn create_tensor_view_arm(
        &self,
        device: Device,
        create_info: &TensorViewCreateInfoARM<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<TensorViewARM> {
        unsafe {
            let mut view = core::mem::MaybeUninit::uninit();
            let result = (self.create_tensor_view_arm)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                view.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(view.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyTensorViewARM.html>
    #[inline]
    pub unsafe fn destroy_tensor_view_arm(
        &self,
        device: Device,
        tensor_view: TensorViewARM,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_tensor_view_arm)(device, tensor_view, allocator.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetTensorMemoryRequirementsARM.html>
    #[inline]
    pub unsafe fn get_tensor_memory_requirements_arm(
        &self,
        device: Device,
        info: &TensorMemoryRequirementsInfoARM<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe { (self.get_tensor_memory_requirements_arm)(device, info, memory_requirements) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBindTensorMemoryARM.html>
    #[inline]
    pub unsafe fn bind_tensor_memory_arm(
        &self,
        device: Device,
        bind_infos: &[BindTensorMemoryInfoARM<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.bind_tensor_memory_arm)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceTensorMemoryRequirementsARM.html>
    #[inline]
    pub unsafe fn get_device_tensor_memory_requirements_arm(
        &self,
        device: Device,
        info: &DeviceTensorMemoryRequirementsARM<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe {
            (self.get_device_tensor_memory_requirements_arm)(device, info, memory_requirements)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyTensorARM.html>
    #[inline]
    pub unsafe fn cmd_copy_tensor_arm(
        &self,
        command_buffer: CommandBuffer,
        copy_tensor_info: &CopyTensorInfoARM<'_>,
    ) {
        unsafe { (self.cmd_copy_tensor_arm)(command_buffer, copy_tensor_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetTensorOpaqueCaptureDescriptorDataARM.html>
    #[inline]
    pub unsafe fn get_tensor_opaque_capture_descriptor_data_arm(
        &self,
        device: Device,
        info: &TensorCaptureDescriptorDataInfoARM<'_>,
        data: *mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            let result =
                (self.get_tensor_opaque_capture_descriptor_data_arm.unwrap())(device, info, data);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetTensorViewOpaqueCaptureDescriptorDataARM.html>
    #[inline]
    pub unsafe fn get_tensor_view_opaque_capture_descriptor_data_arm(
        &self,
        device: Device,
        info: &TensorViewCaptureDescriptorDataInfoARM<'_>,
        data: *mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self
                .get_tensor_view_opaque_capture_descriptor_data_arm
                .unwrap())(device, info, data);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}

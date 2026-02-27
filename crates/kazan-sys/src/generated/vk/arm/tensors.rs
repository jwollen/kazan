#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct TensorViewARM(u64);
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for TensorDescriptionARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::TENSOR_DESCRIPTION_ARM,
            p_next: core::ptr::null(),
            tiling: Default::default(),
            format: Default::default(),
            dimension_count: Default::default(),
            p_dimensions: core::ptr::null(),
            p_strides: core::ptr::null(),
            usage: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> TensorDescriptionARM<'a> {
    pub fn tiling(mut self, tiling: TensorTilingARM) -> Self {
        self.tiling = tiling;
        self
    }
    pub fn format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }
    pub fn dimensions(mut self, dimensions: &'a [i64]) -> Self {
        self.dimension_count = dimensions.len().try_into().unwrap();
        self.p_dimensions = dimensions.as_ptr();
        self
    }
    pub fn strides(mut self, strides: &'a [i64]) -> Self {
        self.dimension_count = strides.len().try_into().unwrap();
        self.p_strides = strides.as_ptr();
        self
    }
    pub fn usage(mut self, usage: TensorUsageFlagsARM) -> Self {
        self.usage = usage;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for TensorCreateInfoARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::TENSOR_CREATE_INFO_ARM,
            p_next: core::ptr::null(),
            flags: Default::default(),
            p_description: core::ptr::null(),
            sharing_mode: Default::default(),
            queue_family_index_count: Default::default(),
            p_queue_family_indices: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> TensorCreateInfoARM<'a> {
    pub fn flags(mut self, flags: TensorCreateFlagsARM) -> Self {
        self.flags = flags;
        self
    }
    pub fn description(mut self, description: &'a TensorDescriptionARM<'a>) -> Self {
        self.p_description = description;
        self
    }
    pub fn sharing_mode(mut self, sharing_mode: SharingMode) -> Self {
        self.sharing_mode = sharing_mode;
        self
    }
    pub fn queue_family_indices(mut self, queue_family_indices: &'a [u32]) -> Self {
        self.queue_family_index_count = queue_family_indices.len().try_into().unwrap();
        self.p_queue_family_indices = queue_family_indices.as_ptr();
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorMemoryRequirementsInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor: TensorARM,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for TensorMemoryRequirementsInfoARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::TENSOR_MEMORY_REQUIREMENTS_INFO_ARM,
            p_next: core::ptr::null(),
            tensor: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> TensorMemoryRequirementsInfoARM<'a> {
    pub fn tensor(mut self, tensor: TensorARM) -> Self {
        self.tensor = tensor;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindTensorMemoryInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor: TensorARM,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BindTensorMemoryInfoARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BIND_TENSOR_MEMORY_INFO_ARM,
            p_next: core::ptr::null(),
            tensor: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> BindTensorMemoryInfoARM<'a> {
    pub fn tensor(mut self, tensor: TensorARM) -> Self {
        self.tensor = tensor;
        self
    }
    pub fn memory(mut self, memory: DeviceMemory) -> Self {
        self.memory = memory;
        self
    }
    pub fn memory_offset(mut self, memory_offset: DeviceSize) -> Self {
        self.memory_offset = memory_offset;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteDescriptorSetTensorARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_view_count: u32,
    pub p_tensor_views: *const TensorViewARM,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for WriteDescriptorSetTensorARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::WRITE_DESCRIPTOR_SET_TENSOR_ARM,
            p_next: core::ptr::null(),
            tensor_view_count: Default::default(),
            p_tensor_views: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> WriteDescriptorSetTensorARM<'a> {
    pub fn tensor_views(mut self, tensor_views: &'a [TensorViewARM]) -> Self {
        self.tensor_view_count = tensor_views.len().try_into().unwrap();
        self.p_tensor_views = tensor_views.as_ptr();
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorFormatPropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optimal_tiling_tensor_features: FormatFeatureFlags2,
    pub linear_tiling_tensor_features: FormatFeatureFlags2,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for TensorFormatPropertiesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::TENSOR_FORMAT_PROPERTIES_ARM,
            p_next: core::ptr::null_mut(),
            optimal_tiling_tensor_features: Default::default(),
            linear_tiling_tensor_features: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> TensorFormatPropertiesARM<'a> {
    pub fn optimal_tiling_tensor_features(
        mut self,
        optimal_tiling_tensor_features: FormatFeatureFlags2,
    ) -> Self {
        self.optimal_tiling_tensor_features = optimal_tiling_tensor_features;
        self
    }
    pub fn linear_tiling_tensor_features(
        mut self,
        linear_tiling_tensor_features: FormatFeatureFlags2,
    ) -> Self {
        self.linear_tiling_tensor_features = linear_tiling_tensor_features;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for PhysicalDeviceTensorPropertiesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM,
            p_next: core::ptr::null_mut(),
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
    pub fn max_tensor_dimension_count(mut self, max_tensor_dimension_count: u32) -> Self {
        self.max_tensor_dimension_count = max_tensor_dimension_count;
        self
    }
    pub fn max_tensor_elements(mut self, max_tensor_elements: u64) -> Self {
        self.max_tensor_elements = max_tensor_elements;
        self
    }
    pub fn max_per_dimension_tensor_elements(
        mut self,
        max_per_dimension_tensor_elements: u64,
    ) -> Self {
        self.max_per_dimension_tensor_elements = max_per_dimension_tensor_elements;
        self
    }
    pub fn max_tensor_stride(mut self, max_tensor_stride: i64) -> Self {
        self.max_tensor_stride = max_tensor_stride;
        self
    }
    pub fn max_tensor_size(mut self, max_tensor_size: u64) -> Self {
        self.max_tensor_size = max_tensor_size;
        self
    }
    pub fn max_tensor_shader_access_array_length(
        mut self,
        max_tensor_shader_access_array_length: u32,
    ) -> Self {
        self.max_tensor_shader_access_array_length = max_tensor_shader_access_array_length;
        self
    }
    pub fn max_tensor_shader_access_size(mut self, max_tensor_shader_access_size: u32) -> Self {
        self.max_tensor_shader_access_size = max_tensor_shader_access_size;
        self
    }
    pub fn max_descriptor_set_storage_tensors(
        mut self,
        max_descriptor_set_storage_tensors: u32,
    ) -> Self {
        self.max_descriptor_set_storage_tensors = max_descriptor_set_storage_tensors;
        self
    }
    pub fn max_per_stage_descriptor_set_storage_tensors(
        mut self,
        max_per_stage_descriptor_set_storage_tensors: u32,
    ) -> Self {
        self.max_per_stage_descriptor_set_storage_tensors =
            max_per_stage_descriptor_set_storage_tensors;
        self
    }
    pub fn max_descriptor_set_update_after_bind_storage_tensors(
        mut self,
        max_descriptor_set_update_after_bind_storage_tensors: u32,
    ) -> Self {
        self.max_descriptor_set_update_after_bind_storage_tensors =
            max_descriptor_set_update_after_bind_storage_tensors;
        self
    }
    pub fn max_per_stage_descriptor_update_after_bind_storage_tensors(
        mut self,
        max_per_stage_descriptor_update_after_bind_storage_tensors: u32,
    ) -> Self {
        self.max_per_stage_descriptor_update_after_bind_storage_tensors =
            max_per_stage_descriptor_update_after_bind_storage_tensors;
        self
    }
    pub fn shader_storage_tensor_array_non_uniform_indexing_native(
        mut self,
        shader_storage_tensor_array_non_uniform_indexing_native: Bool32,
    ) -> Self {
        self.shader_storage_tensor_array_non_uniform_indexing_native =
            shader_storage_tensor_array_non_uniform_indexing_native;
        self
    }
    pub fn shader_tensor_supported_stages(
        mut self,
        shader_tensor_supported_stages: ShaderStageFlags,
    ) -> Self {
        self.shader_tensor_supported_stages = shader_tensor_supported_stages;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for TensorMemoryBarrierARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::TENSOR_MEMORY_BARRIER_ARM,
            p_next: core::ptr::null(),
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
    pub fn src_stage_mask(mut self, src_stage_mask: PipelineStageFlags2) -> Self {
        self.src_stage_mask = src_stage_mask;
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: AccessFlags2) -> Self {
        self.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_stage_mask(mut self, dst_stage_mask: PipelineStageFlags2) -> Self {
        self.dst_stage_mask = dst_stage_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: AccessFlags2) -> Self {
        self.dst_access_mask = dst_access_mask;
        self
    }
    pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.src_queue_family_index = src_queue_family_index;
        self
    }
    pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.dst_queue_family_index = dst_queue_family_index;
        self
    }
    pub fn tensor(mut self, tensor: TensorARM) -> Self {
        self.tensor = tensor;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorDependencyInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_memory_barrier_count: u32,
    pub p_tensor_memory_barriers: *const TensorMemoryBarrierARM<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for TensorDependencyInfoARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::TENSOR_DEPENDENCY_INFO_ARM,
            p_next: core::ptr::null(),
            tensor_memory_barrier_count: Default::default(),
            p_tensor_memory_barriers: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> TensorDependencyInfoARM<'a> {
    pub fn tensor_memory_barrier_count(mut self, tensor_memory_barrier_count: u32) -> Self {
        self.tensor_memory_barrier_count = tensor_memory_barrier_count;
        self
    }
    pub fn tensor_memory_barriers(
        mut self,
        tensor_memory_barriers: &'a TensorMemoryBarrierARM<'a>,
    ) -> Self {
        self.p_tensor_memory_barriers = tensor_memory_barriers;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for PhysicalDeviceTensorFeaturesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_TENSOR_FEATURES_ARM,
            p_next: core::ptr::null_mut(),
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
    pub fn tensor_non_packed(mut self, tensor_non_packed: Bool32) -> Self {
        self.tensor_non_packed = tensor_non_packed;
        self
    }
    pub fn shader_tensor_access(mut self, shader_tensor_access: Bool32) -> Self {
        self.shader_tensor_access = shader_tensor_access;
        self
    }
    pub fn shader_storage_tensor_array_dynamic_indexing(
        mut self,
        shader_storage_tensor_array_dynamic_indexing: Bool32,
    ) -> Self {
        self.shader_storage_tensor_array_dynamic_indexing =
            shader_storage_tensor_array_dynamic_indexing;
        self
    }
    pub fn shader_storage_tensor_array_non_uniform_indexing(
        mut self,
        shader_storage_tensor_array_non_uniform_indexing: Bool32,
    ) -> Self {
        self.shader_storage_tensor_array_non_uniform_indexing =
            shader_storage_tensor_array_non_uniform_indexing;
        self
    }
    pub fn descriptor_binding_storage_tensor_update_after_bind(
        mut self,
        descriptor_binding_storage_tensor_update_after_bind: Bool32,
    ) -> Self {
        self.descriptor_binding_storage_tensor_update_after_bind =
            descriptor_binding_storage_tensor_update_after_bind;
        self
    }
    pub fn tensors(mut self, tensors: Bool32) -> Self {
        self.tensors = tensors;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceTensorMemoryRequirementsARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_create_info: *const TensorCreateInfoARM<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceTensorMemoryRequirementsARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM,
            p_next: core::ptr::null(),
            p_create_info: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DeviceTensorMemoryRequirementsARM<'a> {
    pub fn create_info(mut self, create_info: &'a TensorCreateInfoARM<'a>) -> Self {
        self.p_create_info = create_info;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyTensorInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_tensor: TensorARM,
    pub dst_tensor: TensorARM,
    pub region_count: u32,
    pub p_regions: *const TensorCopyARM<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CopyTensorInfoARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COPY_TENSOR_INFO_ARM,
            p_next: core::ptr::null(),
            src_tensor: Default::default(),
            dst_tensor: Default::default(),
            region_count: Default::default(),
            p_regions: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> CopyTensorInfoARM<'a> {
    pub fn src_tensor(mut self, src_tensor: TensorARM) -> Self {
        self.src_tensor = src_tensor;
        self
    }
    pub fn dst_tensor(mut self, dst_tensor: TensorARM) -> Self {
        self.dst_tensor = dst_tensor;
        self
    }
    pub fn regions(mut self, regions: &'a [TensorCopyARM<'a>]) -> Self {
        self.region_count = regions.len().try_into().unwrap();
        self.p_regions = regions.as_ptr();
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorCopyARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dimension_count: u32,
    pub p_src_offset: *const u64,
    pub p_dst_offset: *const u64,
    pub p_extent: *const u64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for TensorCopyARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::TENSOR_COPY_ARM,
            p_next: core::ptr::null(),
            dimension_count: Default::default(),
            p_src_offset: core::ptr::null(),
            p_dst_offset: core::ptr::null(),
            p_extent: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> TensorCopyARM<'a> {
    pub fn src_offset(mut self, src_offset: &'a [u64]) -> Self {
        self.dimension_count = src_offset.len().try_into().unwrap();
        self.p_src_offset = src_offset.as_ptr();
        self
    }
    pub fn dst_offset(mut self, dst_offset: &'a [u64]) -> Self {
        self.dimension_count = dst_offset.len().try_into().unwrap();
        self.p_dst_offset = dst_offset.as_ptr();
        self
    }
    pub fn extent(mut self, extent: &'a [u64]) -> Self {
        self.dimension_count = extent.len().try_into().unwrap();
        self.p_extent = extent.as_ptr();
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryDedicatedAllocateInfoTensorARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor: TensorARM,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryDedicatedAllocateInfoTensorARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM,
            p_next: core::ptr::null(),
            tensor: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> MemoryDedicatedAllocateInfoTensorARM<'a> {
    pub fn tensor(mut self, tensor: TensorARM) -> Self {
        self.tensor = tensor;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorBufferTensorPropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tensor_capture_replay_descriptor_data_size: usize,
    pub tensor_view_capture_replay_descriptor_data_size: usize,
    pub tensor_descriptor_size: usize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDescriptorBufferTensorPropertiesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM,
            p_next: core::ptr::null_mut(),
            tensor_capture_replay_descriptor_data_size: Default::default(),
            tensor_view_capture_replay_descriptor_data_size: Default::default(),
            tensor_descriptor_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceDescriptorBufferTensorPropertiesARM<'a> {
    pub fn tensor_capture_replay_descriptor_data_size(
        mut self,
        tensor_capture_replay_descriptor_data_size: usize,
    ) -> Self {
        self.tensor_capture_replay_descriptor_data_size =
            tensor_capture_replay_descriptor_data_size;
        self
    }
    pub fn tensor_view_capture_replay_descriptor_data_size(
        mut self,
        tensor_view_capture_replay_descriptor_data_size: usize,
    ) -> Self {
        self.tensor_view_capture_replay_descriptor_data_size =
            tensor_view_capture_replay_descriptor_data_size;
        self
    }
    pub fn tensor_descriptor_size(mut self, tensor_descriptor_size: usize) -> Self {
        self.tensor_descriptor_size = tensor_descriptor_size;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorBufferTensorFeaturesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_buffer_tensor_descriptors: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDescriptorBufferTensorFeaturesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM,
            p_next: core::ptr::null_mut(),
            descriptor_buffer_tensor_descriptors: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceDescriptorBufferTensorFeaturesARM<'a> {
    pub fn descriptor_buffer_tensor_descriptors(
        mut self,
        descriptor_buffer_tensor_descriptors: Bool32,
    ) -> Self {
        self.descriptor_buffer_tensor_descriptors = descriptor_buffer_tensor_descriptors;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorCaptureDescriptorDataInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor: TensorARM,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for TensorCaptureDescriptorDataInfoARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM,
            p_next: core::ptr::null(),
            tensor: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> TensorCaptureDescriptorDataInfoARM<'a> {
    pub fn tensor(mut self, tensor: TensorARM) -> Self {
        self.tensor = tensor;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorViewCaptureDescriptorDataInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_view: TensorViewARM,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for TensorViewCaptureDescriptorDataInfoARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM,
            p_next: core::ptr::null(),
            tensor_view: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> TensorViewCaptureDescriptorDataInfoARM<'a> {
    pub fn tensor_view(mut self, tensor_view: TensorViewARM) -> Self {
        self.tensor_view = tensor_view;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorGetTensorInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_view: TensorViewARM,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorGetTensorInfoARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DESCRIPTOR_GET_TENSOR_INFO_ARM,
            p_next: core::ptr::null(),
            tensor_view: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DescriptorGetTensorInfoARM<'a> {
    pub fn tensor_view(mut self, tensor_view: TensorViewARM) -> Self {
        self.tensor_view = tensor_view;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FrameBoundaryTensorsARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_count: u32,
    pub p_tensors: *const TensorARM,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for FrameBoundaryTensorsARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::FRAME_BOUNDARY_TENSORS_ARM,
            p_next: core::ptr::null(),
            tensor_count: Default::default(),
            p_tensors: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> FrameBoundaryTensorsARM<'a> {
    pub fn tensors(mut self, tensors: &'a [TensorARM]) -> Self {
        self.tensor_count = tensors.len().try_into().unwrap();
        self.p_tensors = tensors.as_ptr();
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalTensorInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: TensorCreateFlagsARM,
    pub p_description: *const TensorDescriptionARM<'a>,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceExternalTensorInfoARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM,
            p_next: core::ptr::null(),
            flags: Default::default(),
            p_description: core::ptr::null(),
            handle_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceExternalTensorInfoARM<'a> {
    pub fn flags(mut self, flags: TensorCreateFlagsARM) -> Self {
        self.flags = flags;
        self
    }
    pub fn description(mut self, description: &'a TensorDescriptionARM<'a>) -> Self {
        self.p_description = description;
        self
    }
    pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
        self.handle_type = handle_type;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalTensorPropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub external_memory_properties: ExternalMemoryProperties,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExternalTensorPropertiesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXTERNAL_TENSOR_PROPERTIES_ARM,
            p_next: core::ptr::null(),
            external_memory_properties: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ExternalTensorPropertiesARM<'a> {
    pub fn external_memory_properties(
        mut self,
        external_memory_properties: ExternalMemoryProperties,
    ) -> Self {
        self.external_memory_properties = external_memory_properties;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryTensorCreateInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExternalMemoryTensorCreateInfoARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM,
            p_next: core::ptr::null(),
            handle_types: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ExternalMemoryTensorCreateInfoARM<'a> {
    pub fn handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlags) -> Self {
        self.handle_types = handle_types;
        self
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorTilingARM(i32);
impl TensorTilingARM {
    pub const OPTIMAL_ARM: Self = Self(0);
    pub const LINEAR_ARM: Self = Self(1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TensorCreateFlagsARM: Flags64 {
        const MUTABLE_FORMAT_ARM = TensorCreateFlagBitsARM::MUTABLE_FORMAT_ARM.0;
        const PROTECTED_ARM = TensorCreateFlagBitsARM::PROTECTED_ARM.0;
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM = TensorCreateFlagBitsARM::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM.0;
        const DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM = TensorCreateFlagBitsARM::DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorCreateFlagBitsARM(u64);
impl TensorCreateFlagBitsARM {
    pub const MUTABLE_FORMAT_ARM: Self = Self(1 << 0);
    pub const PROTECTED_ARM: Self = Self(1 << 1);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM: Self = Self(1 << 2);
    pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TensorUsageFlagsARM: Flags64 {
        const SHADER_ARM = TensorUsageFlagBitsARM::SHADER_ARM.0;
        const TRANSFER_SRC_ARM = TensorUsageFlagBitsARM::TRANSFER_SRC_ARM.0;
        const TRANSFER_DST_ARM = TensorUsageFlagBitsARM::TRANSFER_DST_ARM.0;
        const IMAGE_ALIASING_ARM = TensorUsageFlagBitsARM::IMAGE_ALIASING_ARM.0;
        const DATA_GRAPH_ARM = TensorUsageFlagBitsARM::DATA_GRAPH_ARM.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorUsageFlagBitsARM(u64);
impl TensorUsageFlagBitsARM {
    pub const SHADER_ARM: Self = Self(1 << 1);
    pub const TRANSFER_SRC_ARM: Self = Self(1 << 2);
    pub const TRANSFER_DST_ARM: Self = Self(1 << 3);
    pub const IMAGE_ALIASING_ARM: Self = Self(1 << 4);
    pub const DATA_GRAPH_ARM: Self = Self(1 << 5);
}
pub type PFN_vkCreateTensorARM = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const TensorCreateInfoARM<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_tensor: *mut TensorARM,
) -> Result;
pub type PFN_vkDestroyTensorARM = unsafe extern "system" fn(
    device: Device,
    tensor: TensorARM,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCreateTensorViewARM = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const TensorViewCreateInfoARM<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_view: *mut TensorViewARM,
) -> Result;
pub type PFN_vkDestroyTensorViewARM = unsafe extern "system" fn(
    device: Device,
    tensor_view: TensorViewARM,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkGetTensorMemoryRequirementsARM = unsafe extern "system" fn(
    device: Device,
    p_info: *const TensorMemoryRequirementsInfoARM<'_>,
    p_memory_requirements: *mut MemoryRequirements2<'_>,
);
pub type PFN_vkBindTensorMemoryARM = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindTensorMemoryInfoARM<'_>,
) -> Result;
pub type PFN_vkGetDeviceTensorMemoryRequirementsARM = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceTensorMemoryRequirementsARM<'_>,
    p_memory_requirements: *mut MemoryRequirements2<'_>,
);
pub type PFN_vkCmdCopyTensorARM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_tensor_info: *const CopyTensorInfoARM<'_>,
);
pub type PFN_vkGetTensorOpaqueCaptureDescriptorDataARM = unsafe extern "system" fn(
    device: Device,
    p_info: *const TensorCaptureDescriptorDataInfoARM<'_>,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM = unsafe extern "system" fn(
    device: Device,
    p_info: *const TensorViewCaptureDescriptorDataInfoARM<'_>,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_tensor_info: *const PhysicalDeviceExternalTensorInfoARM<'_>,
    p_external_tensor_properties: *mut ExternalTensorPropertiesARM<'_>,
);

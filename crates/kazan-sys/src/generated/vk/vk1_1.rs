#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const MAX_DEVICE_GROUP_SIZE: u32 = 32;
pub const LUID_SIZE: u32 = 8;
pub const QUEUE_FAMILY_EXTERNAL: u32 = !1;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DescriptorUpdateTemplate(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SamplerYcbcrConversion(u64);
pub type PhysicalDeviceVariablePointerFeatures<'a> = PhysicalDeviceVariablePointersFeatures<'a>;
pub type PhysicalDeviceShaderDrawParameterFeatures<'a> =
    PhysicalDeviceShaderDrawParametersFeatures<'a>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFeatures2<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub features: PhysicalDeviceFeatures,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFeatures2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FEATURES_2,
            p_next: core::ptr::null_mut(),
            features: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProperties2<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub properties: PhysicalDeviceProperties,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceProperties2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PROPERTIES_2,
            p_next: core::ptr::null_mut(),
            properties: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormatProperties2<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format_properties: FormatProperties,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for FormatProperties2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::FORMAT_PROPERTIES_2,
            p_next: core::ptr::null_mut(),
            format_properties: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageFormatProperties2<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_format_properties: ImageFormatProperties,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageFormatProperties2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_FORMAT_PROPERTIES_2,
            p_next: core::ptr::null_mut(),
            image_format_properties: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageFormatInfo2<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub ty: ImageType,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub flags: ImageCreateFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceImageFormatInfo2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2,
            p_next: core::ptr::null(),
            format: Default::default(),
            ty: Default::default(),
            tiling: Default::default(),
            usage: Default::default(),
            flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyProperties2<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub queue_family_properties: QueueFamilyProperties,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for QueueFamilyProperties2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::QUEUE_FAMILY_PROPERTIES_2,
            p_next: core::ptr::null_mut(),
            queue_family_properties: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMemoryProperties2<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_properties: PhysicalDeviceMemoryProperties,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMemoryProperties2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2,
            p_next: core::ptr::null_mut(),
            memory_properties: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageFormatProperties2<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub properties: SparseImageFormatProperties,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SparseImageFormatProperties2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SPARSE_IMAGE_FORMAT_PROPERTIES_2,
            p_next: core::ptr::null_mut(),
            properties: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSparseImageFormatInfo2<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub ty: ImageType,
    pub samples: SampleCountFlagBits,
    pub usage: ImageUsageFlags,
    pub tiling: ImageTiling,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceSparseImageFormatInfo2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2,
            p_next: core::ptr::null(),
            format: Default::default(),
            ty: Default::default(),
            samples: Default::default(),
            usage: Default::default(),
            tiling: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVariablePointersFeatures<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub variable_pointers_storage_buffer: Bool32,
    pub variable_pointers: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceVariablePointersFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES,
            p_next: core::ptr::null_mut(),
            variable_pointers_storage_buffer: Default::default(),
            variable_pointers: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ExternalMemoryProperties {
    pub external_memory_features: ExternalMemoryFeatureFlags,
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
    pub compatible_handle_types: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalImageFormatInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceExternalImageFormatInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO,
            p_next: core::ptr::null(),
            handle_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalImageFormatProperties<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_memory_properties: ExternalMemoryProperties,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExternalImageFormatProperties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXTERNAL_IMAGE_FORMAT_PROPERTIES,
            p_next: core::ptr::null_mut(),
            external_memory_properties: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalBufferInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferCreateFlags,
    pub usage: BufferUsageFlags,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceExternalBufferInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO,
            p_next: core::ptr::null(),
            flags: Default::default(),
            usage: Default::default(),
            handle_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalBufferProperties<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_memory_properties: ExternalMemoryProperties,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExternalBufferProperties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXTERNAL_BUFFER_PROPERTIES,
            p_next: core::ptr::null_mut(),
            external_memory_properties: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceIDProperties<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_uuid: [u8; UUID_SIZE as usize],
    pub driver_uuid: [u8; UUID_SIZE as usize],
    pub device_luid: [u8; LUID_SIZE as usize],
    pub device_node_mask: u32,
    pub device_luid_valid: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceIDProperties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_ID_PROPERTIES,
            p_next: core::ptr::null_mut(),
            device_uuid: [Default::default(); _],
            driver_uuid: [Default::default(); _],
            device_luid: [Default::default(); _],
            device_node_mask: Default::default(),
            device_luid_valid: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryImageCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExternalMemoryImageCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO,
            p_next: core::ptr::null(),
            handle_types: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryBufferCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExternalMemoryBufferCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXTERNAL_MEMORY_BUFFER_CREATE_INFO,
            p_next: core::ptr::null(),
            handle_types: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMemoryAllocateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExportMemoryAllocateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXPORT_MEMORY_ALLOCATE_INFO,
            p_next: core::ptr::null(),
            handle_types: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalSemaphoreInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceExternalSemaphoreInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO,
            p_next: core::ptr::null(),
            handle_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalSemaphoreProperties<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub external_semaphore_features: ExternalSemaphoreFeatureFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExternalSemaphoreProperties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXTERNAL_SEMAPHORE_PROPERTIES,
            p_next: core::ptr::null_mut(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
            external_semaphore_features: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportSemaphoreCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalSemaphoreHandleTypeFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExportSemaphoreCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXPORT_SEMAPHORE_CREATE_INFO,
            p_next: core::ptr::null(),
            handle_types: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalFenceInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalFenceHandleTypeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceExternalFenceInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO,
            p_next: core::ptr::null(),
            handle_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalFenceProperties<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
    pub compatible_handle_types: ExternalFenceHandleTypeFlags,
    pub external_fence_features: ExternalFenceFeatureFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExternalFenceProperties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXTERNAL_FENCE_PROPERTIES,
            p_next: core::ptr::null_mut(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
            external_fence_features: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportFenceCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalFenceHandleTypeFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExportFenceCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXPORT_FENCE_CREATE_INFO,
            p_next: core::ptr::null(),
            handle_types: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultiviewFeatures<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub multiview: Bool32,
    pub multiview_geometry_shader: Bool32,
    pub multiview_tessellation_shader: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMultiviewFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MULTIVIEW_FEATURES,
            p_next: core::ptr::null_mut(),
            multiview: Default::default(),
            multiview_geometry_shader: Default::default(),
            multiview_tessellation_shader: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultiviewProperties<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_multiview_view_count: u32,
    pub max_multiview_instance_index: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMultiviewProperties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES,
            p_next: core::ptr::null_mut(),
            max_multiview_view_count: Default::default(),
            max_multiview_instance_index: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassMultiviewCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub subpass_count: u32,
    pub p_view_masks: *const u32,
    pub dependency_count: u32,
    pub p_view_offsets: *const i32,
    pub correlation_mask_count: u32,
    pub p_correlation_masks: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RenderPassMultiviewCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDER_PASS_MULTIVIEW_CREATE_INFO,
            p_next: core::ptr::null(),
            subpass_count: Default::default(),
            p_view_masks: core::ptr::null(),
            dependency_count: Default::default(),
            p_view_offsets: core::ptr::null(),
            correlation_mask_count: Default::default(),
            p_correlation_masks: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceGroupProperties<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub physical_device_count: u32,
    pub physical_devices: [PhysicalDevice; MAX_DEVICE_GROUP_SIZE as usize],
    pub subset_allocation: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceGroupProperties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_GROUP_PROPERTIES,
            p_next: core::ptr::null_mut(),
            physical_device_count: Default::default(),
            physical_devices: [Default::default(); _],
            subset_allocation: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryAllocateFlagsInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MemoryAllocateFlags,
    pub device_mask: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryAllocateFlagsInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_ALLOCATE_FLAGS_INFO,
            p_next: core::ptr::null(),
            flags: Default::default(),
            device_mask: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindBufferMemoryInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BindBufferMemoryInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BIND_BUFFER_MEMORY_INFO,
            p_next: core::ptr::null(),
            buffer: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindBufferMemoryDeviceGroupInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BindBufferMemoryDeviceGroupInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO,
            p_next: core::ptr::null(),
            device_index_count: Default::default(),
            p_device_indices: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImageMemoryInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BindImageMemoryInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BIND_IMAGE_MEMORY_INFO,
            p_next: core::ptr::null(),
            image: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImageMemoryDeviceGroupInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
    pub split_instance_bind_region_count: u32,
    pub p_split_instance_bind_regions: *const Rect2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BindImageMemoryDeviceGroupInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO,
            p_next: core::ptr::null(),
            device_index_count: Default::default(),
            p_device_indices: core::ptr::null(),
            split_instance_bind_region_count: Default::default(),
            p_split_instance_bind_regions: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupRenderPassBeginInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_mask: u32,
    pub device_render_area_count: u32,
    pub p_device_render_areas: *const Rect2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceGroupRenderPassBeginInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO,
            p_next: core::ptr::null(),
            device_mask: Default::default(),
            device_render_area_count: Default::default(),
            p_device_render_areas: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupCommandBufferBeginInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_mask: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceGroupCommandBufferBeginInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
            p_next: core::ptr::null(),
            device_mask: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupSubmitInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphore_device_indices: *const u32,
    pub command_buffer_count: u32,
    pub p_command_buffer_device_masks: *const u32,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphore_device_indices: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceGroupSubmitInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_GROUP_SUBMIT_INFO,
            p_next: core::ptr::null(),
            wait_semaphore_count: Default::default(),
            p_wait_semaphore_device_indices: core::ptr::null(),
            command_buffer_count: Default::default(),
            p_command_buffer_device_masks: core::ptr::null(),
            signal_semaphore_count: Default::default(),
            p_signal_semaphore_device_indices: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupBindSparseInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub resource_device_index: u32,
    pub memory_device_index: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceGroupBindSparseInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_GROUP_BIND_SPARSE_INFO,
            p_next: core::ptr::null(),
            resource_device_index: Default::default(),
            memory_device_index: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupDeviceCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub physical_device_count: u32,
    pub p_physical_devices: *const PhysicalDevice,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceGroupDeviceCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_GROUP_DEVICE_CREATE_INFO,
            p_next: core::ptr::null(),
            physical_device_count: Default::default(),
            p_physical_devices: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DescriptorUpdateTemplateEntry {
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: DescriptorType,
    pub offset: usize,
    pub stride: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorUpdateTemplateCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorUpdateTemplateCreateFlags,
    pub descriptor_update_entry_count: u32,
    pub p_descriptor_update_entries: *const DescriptorUpdateTemplateEntry,
    pub template_type: DescriptorUpdateTemplateType,
    pub descriptor_set_layout: DescriptorSetLayout,
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline_layout: PipelineLayout,
    pub set: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorUpdateTemplateCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO,
            p_next: core::ptr::null(),
            flags: Default::default(),
            descriptor_update_entry_count: Default::default(),
            p_descriptor_update_entries: core::ptr::null(),
            template_type: Default::default(),
            descriptor_set_layout: Default::default(),
            pipeline_bind_point: Default::default(),
            pipeline_layout: Default::default(),
            set: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct InputAttachmentAspectReference {
    pub subpass: u32,
    pub input_attachment_index: u32,
    pub aspect_mask: ImageAspectFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassInputAttachmentAspectCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub aspect_reference_count: u32,
    pub p_aspect_references: *const InputAttachmentAspectReference,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RenderPassInputAttachmentAspectCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO,
            p_next: core::ptr::null(),
            aspect_reference_count: Default::default(),
            p_aspect_references: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevice16BitStorageFeatures<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub storage_buffer16_bit_access: Bool32,
    pub uniform_and_storage_buffer16_bit_access: Bool32,
    pub storage_push_constant16: Bool32,
    pub storage_input_output16: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevice16BitStorageFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES,
            p_next: core::ptr::null_mut(),
            storage_buffer16_bit_access: Default::default(),
            uniform_and_storage_buffer16_bit_access: Default::default(),
            storage_push_constant16: Default::default(),
            storage_input_output16: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSubgroupProperties<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subgroup_size: u32,
    pub supported_stages: ShaderStageFlags,
    pub supported_operations: SubgroupFeatureFlags,
    pub quad_operations_in_all_stages: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceSubgroupProperties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES,
            p_next: core::ptr::null_mut(),
            subgroup_size: Default::default(),
            supported_stages: Default::default(),
            supported_operations: Default::default(),
            quad_operations_in_all_stages: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferMemoryRequirementsInfo2<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BufferMemoryRequirementsInfo2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BUFFER_MEMORY_REQUIREMENTS_INFO_2,
            p_next: core::ptr::null(),
            buffer: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageMemoryRequirementsInfo2<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageMemoryRequirementsInfo2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_MEMORY_REQUIREMENTS_INFO_2,
            p_next: core::ptr::null(),
            image: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageSparseMemoryRequirementsInfo2<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageSparseMemoryRequirementsInfo2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2,
            p_next: core::ptr::null(),
            image: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryRequirements2<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_requirements: MemoryRequirements,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryRequirements2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_REQUIREMENTS_2,
            p_next: core::ptr::null_mut(),
            memory_requirements: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryRequirements2<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_requirements: SparseImageMemoryRequirements,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SparseImageMemoryRequirements2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2,
            p_next: core::ptr::null_mut(),
            memory_requirements: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePointClippingProperties<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub point_clipping_behavior: PointClippingBehavior,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePointClippingProperties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES,
            p_next: core::ptr::null_mut(),
            point_clipping_behavior: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryDedicatedRequirements<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub prefers_dedicated_allocation: Bool32,
    pub requires_dedicated_allocation: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryDedicatedRequirements<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_DEDICATED_REQUIREMENTS,
            p_next: core::ptr::null_mut(),
            prefers_dedicated_allocation: Default::default(),
            requires_dedicated_allocation: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryDedicatedAllocateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub buffer: Buffer,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryDedicatedAllocateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_DEDICATED_ALLOCATE_INFO,
            p_next: core::ptr::null(),
            image: Default::default(),
            buffer: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewUsageCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub usage: ImageUsageFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageViewUsageCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_VIEW_USAGE_CREATE_INFO,
            p_next: core::ptr::null(),
            usage: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineTessellationDomainOriginStateCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub domain_origin: TessellationDomainOrigin,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineTessellationDomainOriginStateCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO,
            p_next: core::ptr::null(),
            domain_origin: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerYcbcrConversionInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub conversion: SamplerYcbcrConversion,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SamplerYcbcrConversionInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SAMPLER_YCBCR_CONVERSION_INFO,
            p_next: core::ptr::null(),
            conversion: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerYcbcrConversionCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub ycbcr_model: SamplerYcbcrModelConversion,
    pub ycbcr_range: SamplerYcbcrRange,
    pub components: ComponentMapping,
    pub x_chroma_offset: ChromaLocation,
    pub y_chroma_offset: ChromaLocation,
    pub chroma_filter: Filter,
    pub force_explicit_reconstruction: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SamplerYcbcrConversionCreateInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SAMPLER_YCBCR_CONVERSION_CREATE_INFO,
            p_next: core::ptr::null(),
            format: Default::default(),
            ycbcr_model: Default::default(),
            ycbcr_range: Default::default(),
            components: Default::default(),
            x_chroma_offset: Default::default(),
            y_chroma_offset: Default::default(),
            chroma_filter: Default::default(),
            force_explicit_reconstruction: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImagePlaneMemoryInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane_aspect: ImageAspectFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BindImagePlaneMemoryInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BIND_IMAGE_PLANE_MEMORY_INFO,
            p_next: core::ptr::null(),
            plane_aspect: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImagePlaneMemoryRequirementsInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane_aspect: ImageAspectFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImagePlaneMemoryRequirementsInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO,
            p_next: core::ptr::null(),
            plane_aspect: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub sampler_ycbcr_conversion: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceSamplerYcbcrConversionFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES,
            p_next: core::ptr::null_mut(),
            sampler_ycbcr_conversion: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerYcbcrConversionImageFormatProperties<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub combined_image_sampler_descriptor_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SamplerYcbcrConversionImageFormatProperties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES,
            p_next: core::ptr::null_mut(),
            combined_image_sampler_descriptor_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProtectedSubmitInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub protected_submit: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ProtectedSubmitInfo<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PROTECTED_SUBMIT_INFO,
            p_next: core::ptr::null(),
            protected_submit: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProtectedMemoryFeatures<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub protected_memory: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceProtectedMemoryFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES,
            p_next: core::ptr::null_mut(),
            protected_memory: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProtectedMemoryProperties<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub protected_no_fault: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceProtectedMemoryProperties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES,
            p_next: core::ptr::null_mut(),
            protected_no_fault: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceQueueInfo2<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_index: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceQueueInfo2<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_QUEUE_INFO_2,
            p_next: core::ptr::null(),
            flags: Default::default(),
            queue_family_index: Default::default(),
            queue_index: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance3Properties<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_per_set_descriptors: u32,
    pub max_memory_allocation_size: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMaintenance3Properties<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES,
            p_next: core::ptr::null_mut(),
            max_per_set_descriptors: Default::default(),
            max_memory_allocation_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutSupport<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorSetLayoutSupport<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DESCRIPTOR_SET_LAYOUT_SUPPORT,
            p_next: core::ptr::null_mut(),
            supported: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderDrawParametersFeatures<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_draw_parameters: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderDrawParametersFeatures<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES,
            p_next: core::ptr::null_mut(),
            shader_draw_parameters: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorUpdateTemplateType(i32);
impl DescriptorUpdateTemplateType {
    pub const DESCRIPTOR_SET: Self = Self(0);
    pub const PUSH_DESCRIPTORS: Self = Self(1);
    pub const DESCRIPTOR_SET_KHR: Self = Self::DESCRIPTOR_SET;
    pub const PUSH_DESCRIPTORS_KHR: Self = Self::PUSH_DESCRIPTORS;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PointClippingBehavior(i32);
impl PointClippingBehavior {
    pub const ALL_CLIP_PLANES: Self = Self(0);
    pub const USER_CLIP_PLANES_ONLY: Self = Self(1);
    pub const ALL_CLIP_PLANES_KHR: Self = Self::ALL_CLIP_PLANES;
    pub const USER_CLIP_PLANES_ONLY_KHR: Self = Self::USER_CLIP_PLANES_ONLY;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TessellationDomainOrigin(i32);
impl TessellationDomainOrigin {
    pub const UPPER_LEFT: Self = Self(0);
    pub const LOWER_LEFT: Self = Self(1);
    pub const LOWER_LEFT_KHR: Self = Self::LOWER_LEFT;
    pub const UPPER_LEFT_KHR: Self = Self::UPPER_LEFT;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerYcbcrModelConversion(i32);
impl SamplerYcbcrModelConversion {
    pub const RGB_IDENTITY: Self = Self(0);
    pub const YCBCR_IDENTITY: Self = Self(1);
    pub const YCBCR_709: Self = Self(2);
    pub const YCBCR_601: Self = Self(3);
    pub const YCBCR_2020: Self = Self(4);
    pub const RGB_IDENTITY_KHR: Self = Self::RGB_IDENTITY;
    pub const YCBCR_2020_KHR: Self = Self::YCBCR_2020;
    pub const YCBCR_601_KHR: Self = Self::YCBCR_601;
    pub const YCBCR_709_KHR: Self = Self::YCBCR_709;
    pub const YCBCR_IDENTITY_KHR: Self = Self::YCBCR_IDENTITY;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerYcbcrRange(i32);
impl SamplerYcbcrRange {
    pub const ITU_FULL: Self = Self(0);
    pub const ITU_NARROW: Self = Self(1);
    pub const ITU_FULL_KHR: Self = Self::ITU_FULL;
    pub const ITU_NARROW_KHR: Self = Self::ITU_NARROW;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChromaLocation(i32);
impl ChromaLocation {
    pub const COSITED_EVEN: Self = Self(0);
    pub const MIDPOINT: Self = Self(1);
    pub const COSITED_EVEN_KHR: Self = Self::COSITED_EVEN;
    pub const MIDPOINT_KHR: Self = Self::MIDPOINT;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SubgroupFeatureFlags: Flags {
        const BASIC = SubgroupFeatureFlagBits::BASIC.0;
        const VOTE = SubgroupFeatureFlagBits::VOTE.0;
        const ARITHMETIC = SubgroupFeatureFlagBits::ARITHMETIC.0;
        const BALLOT = SubgroupFeatureFlagBits::BALLOT.0;
        const SHUFFLE = SubgroupFeatureFlagBits::SHUFFLE.0;
        const SHUFFLE_RELATIVE = SubgroupFeatureFlagBits::SHUFFLE_RELATIVE.0;
        const CLUSTERED = SubgroupFeatureFlagBits::CLUSTERED.0;
        const QUAD = SubgroupFeatureFlagBits::QUAD.0;
        const PARTITIONED_EXT = SubgroupFeatureFlagBits::PARTITIONED_EXT.0;
        const ROTATE = SubgroupFeatureFlagBits::ROTATE.0;
        const ROTATE_CLUSTERED = SubgroupFeatureFlagBits::ROTATE_CLUSTERED.0;
        const PARTITIONED_NV = Self::PARTITIONED_EXT.bits();
        const ROTATE_KHR = Self::ROTATE.bits();
        const ROTATE_CLUSTERED_KHR = Self::ROTATE_CLUSTERED.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubgroupFeatureFlagBits(u32);
impl SubgroupFeatureFlagBits {
    pub const BASIC: Self = Self(1 << 0);
    pub const VOTE: Self = Self(1 << 1);
    pub const ARITHMETIC: Self = Self(1 << 2);
    pub const BALLOT: Self = Self(1 << 3);
    pub const SHUFFLE: Self = Self(1 << 4);
    pub const SHUFFLE_RELATIVE: Self = Self(1 << 5);
    pub const CLUSTERED: Self = Self(1 << 6);
    pub const QUAD: Self = Self(1 << 7);
    pub const PARTITIONED_EXT: Self = Self(1 << 8);
    pub const ROTATE: Self = Self(1 << 9);
    pub const ROTATE_CLUSTERED: Self = Self(1 << 10);
    pub const PARTITIONED_NV: Self = Self::PARTITIONED_EXT;
    pub const ROTATE_KHR: Self = Self::ROTATE;
    pub const ROTATE_CLUSTERED_KHR: Self = Self::ROTATE_CLUSTERED;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DescriptorUpdateTemplateCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PeerMemoryFeatureFlags: Flags {
        const COPY_SRC = PeerMemoryFeatureFlagBits::COPY_SRC.0;
        const COPY_DST = PeerMemoryFeatureFlagBits::COPY_DST.0;
        const GENERIC_SRC = PeerMemoryFeatureFlagBits::GENERIC_SRC.0;
        const GENERIC_DST = PeerMemoryFeatureFlagBits::GENERIC_DST.0;
        const COPY_DST_KHR = Self::COPY_DST.bits();
        const COPY_SRC_KHR = Self::COPY_SRC.bits();
        const GENERIC_DST_KHR = Self::GENERIC_DST.bits();
        const GENERIC_SRC_KHR = Self::GENERIC_SRC.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeerMemoryFeatureFlagBits(u32);
impl PeerMemoryFeatureFlagBits {
    pub const COPY_SRC: Self = Self(1 << 0);
    pub const COPY_DST: Self = Self(1 << 1);
    pub const GENERIC_SRC: Self = Self(1 << 2);
    pub const GENERIC_DST: Self = Self(1 << 3);
    pub const COPY_DST_KHR: Self = Self::COPY_DST;
    pub const COPY_SRC_KHR: Self = Self::COPY_SRC;
    pub const GENERIC_DST_KHR: Self = Self::GENERIC_DST;
    pub const GENERIC_SRC_KHR: Self = Self::GENERIC_SRC;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MemoryAllocateFlags: Flags {
        const DEVICE_MASK = MemoryAllocateFlagBits::DEVICE_MASK.0;
        const DEVICE_ADDRESS = MemoryAllocateFlagBits::DEVICE_ADDRESS.0;
        const DEVICE_ADDRESS_CAPTURE_REPLAY = MemoryAllocateFlagBits::DEVICE_ADDRESS_CAPTURE_REPLAY.0;
        const ZERO_INITIALIZE_EXT = MemoryAllocateFlagBits::ZERO_INITIALIZE_EXT.0;
        const DEVICE_ADDRESS_KHR = Self::DEVICE_ADDRESS.bits();
        const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR = Self::DEVICE_ADDRESS_CAPTURE_REPLAY.bits();
        const DEVICE_MASK_KHR = Self::DEVICE_MASK.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryAllocateFlagBits(u32);
impl MemoryAllocateFlagBits {
    pub const DEVICE_MASK: Self = Self(1 << 0);
    pub const DEVICE_ADDRESS: Self = Self(1 << 1);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(1 << 2);
    pub const ZERO_INITIALIZE_EXT: Self = Self(1 << 3);
    pub const DEVICE_ADDRESS_KHR: Self = Self::DEVICE_ADDRESS;
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
    pub const DEVICE_MASK_KHR: Self = Self::DEVICE_MASK;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CommandPoolTrimFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ExternalMemoryHandleTypeFlags: Flags {
        const OPAQUE_FD = ExternalMemoryHandleTypeFlagBits::OPAQUE_FD.0;
        const OPAQUE_WIN32 = ExternalMemoryHandleTypeFlagBits::OPAQUE_WIN32.0;
        const OPAQUE_WIN32_KMT = ExternalMemoryHandleTypeFlagBits::OPAQUE_WIN32_KMT.0;
        const D3D11_TEXTURE = ExternalMemoryHandleTypeFlagBits::D3D11_TEXTURE.0;
        const D3D11_TEXTURE_KMT = ExternalMemoryHandleTypeFlagBits::D3D11_TEXTURE_KMT.0;
        const D3D12_HEAP = ExternalMemoryHandleTypeFlagBits::D3D12_HEAP.0;
        const D3D12_RESOURCE = ExternalMemoryHandleTypeFlagBits::D3D12_RESOURCE.0;
        const HOST_ALLOCATION_EXT = ExternalMemoryHandleTypeFlagBits::HOST_ALLOCATION_EXT.0;
        const HOST_MAPPED_FOREIGN_MEMORY_EXT = ExternalMemoryHandleTypeFlagBits::HOST_MAPPED_FOREIGN_MEMORY_EXT.0;
        const DMA_BUF_EXT = ExternalMemoryHandleTypeFlagBits::DMA_BUF_EXT.0;
        const ANDROID_HARDWARE_BUFFER_ANDROID = ExternalMemoryHandleTypeFlagBits::ANDROID_HARDWARE_BUFFER_ANDROID.0;
        const ZIRCON_VMO_FUCHSIA = ExternalMemoryHandleTypeFlagBits::ZIRCON_VMO_FUCHSIA.0;
        const RDMA_ADDRESS_NV = ExternalMemoryHandleTypeFlagBits::RDMA_ADDRESS_NV.0;
        const SCREEN_BUFFER_QNX = ExternalMemoryHandleTypeFlagBits::SCREEN_BUFFER_QNX.0;
        const OH_NATIVE_BUFFER_OHOS = ExternalMemoryHandleTypeFlagBits::OH_NATIVE_BUFFER_OHOS.0;
        const MTLBUFFER_EXT = ExternalMemoryHandleTypeFlagBits::MTLBUFFER_EXT.0;
        const MTLTEXTURE_EXT = ExternalMemoryHandleTypeFlagBits::MTLTEXTURE_EXT.0;
        const MTLHEAP_EXT = ExternalMemoryHandleTypeFlagBits::MTLHEAP_EXT.0;
        const D3D11_TEXTURE_KHR = Self::D3D11_TEXTURE.bits();
        const D3D11_TEXTURE_KMT_KHR = Self::D3D11_TEXTURE_KMT.bits();
        const D3D12_HEAP_KHR = Self::D3D12_HEAP.bits();
        const D3D12_RESOURCE_KHR = Self::D3D12_RESOURCE.bits();
        const OPAQUE_FD_KHR = Self::OPAQUE_FD.bits();
        const OPAQUE_WIN32_KHR = Self::OPAQUE_WIN32.bits();
        const OPAQUE_WIN32_KMT_KHR = Self::OPAQUE_WIN32_KMT.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalMemoryHandleTypeFlagBits(u32);
impl ExternalMemoryHandleTypeFlagBits {
    pub const OPAQUE_FD: Self = Self(1 << 0);
    pub const OPAQUE_WIN32: Self = Self(1 << 1);
    pub const OPAQUE_WIN32_KMT: Self = Self(1 << 2);
    pub const D3D11_TEXTURE: Self = Self(1 << 3);
    pub const D3D11_TEXTURE_KMT: Self = Self(1 << 4);
    pub const D3D12_HEAP: Self = Self(1 << 5);
    pub const D3D12_RESOURCE: Self = Self(1 << 6);
    pub const HOST_ALLOCATION_EXT: Self = Self(1 << 7);
    pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self = Self(1 << 8);
    pub const DMA_BUF_EXT: Self = Self(1 << 9);
    pub const ANDROID_HARDWARE_BUFFER_ANDROID: Self = Self(1 << 10);
    pub const ZIRCON_VMO_FUCHSIA: Self = Self(1 << 11);
    pub const RDMA_ADDRESS_NV: Self = Self(1 << 12);
    pub const SCREEN_BUFFER_QNX: Self = Self(1 << 14);
    pub const OH_NATIVE_BUFFER_OHOS: Self = Self(1 << 15);
    pub const MTLBUFFER_EXT: Self = Self(1 << 16);
    pub const MTLTEXTURE_EXT: Self = Self(1 << 17);
    pub const MTLHEAP_EXT: Self = Self(1 << 18);
    pub const D3D11_TEXTURE_KHR: Self = Self::D3D11_TEXTURE;
    pub const D3D11_TEXTURE_KMT_KHR: Self = Self::D3D11_TEXTURE_KMT;
    pub const D3D12_HEAP_KHR: Self = Self::D3D12_HEAP;
    pub const D3D12_RESOURCE_KHR: Self = Self::D3D12_RESOURCE;
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ExternalMemoryFeatureFlags: Flags {
        const DEDICATED_ONLY = ExternalMemoryFeatureFlagBits::DEDICATED_ONLY.0;
        const EXPORTABLE = ExternalMemoryFeatureFlagBits::EXPORTABLE.0;
        const IMPORTABLE = ExternalMemoryFeatureFlagBits::IMPORTABLE.0;
        const DEDICATED_ONLY_KHR = Self::DEDICATED_ONLY.bits();
        const EXPORTABLE_KHR = Self::EXPORTABLE.bits();
        const IMPORTABLE_KHR = Self::IMPORTABLE.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalMemoryFeatureFlagBits(u32);
impl ExternalMemoryFeatureFlagBits {
    pub const DEDICATED_ONLY: Self = Self(1 << 0);
    pub const EXPORTABLE: Self = Self(1 << 1);
    pub const IMPORTABLE: Self = Self(1 << 2);
    pub const DEDICATED_ONLY_KHR: Self = Self::DEDICATED_ONLY;
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ExternalSemaphoreHandleTypeFlags: Flags {
        const OPAQUE_FD = ExternalSemaphoreHandleTypeFlagBits::OPAQUE_FD.0;
        const OPAQUE_WIN32 = ExternalSemaphoreHandleTypeFlagBits::OPAQUE_WIN32.0;
        const OPAQUE_WIN32_KMT = ExternalSemaphoreHandleTypeFlagBits::OPAQUE_WIN32_KMT.0;
        const D3D12_FENCE = ExternalSemaphoreHandleTypeFlagBits::D3D12_FENCE.0;
        const SYNC_FD = ExternalSemaphoreHandleTypeFlagBits::SYNC_FD.0;
        const ZIRCON_EVENT_FUCHSIA = ExternalSemaphoreHandleTypeFlagBits::ZIRCON_EVENT_FUCHSIA.0;
        const D3D11_FENCE = Self::D3D12_FENCE.bits();
        const D3D12_FENCE_KHR = Self::D3D12_FENCE.bits();
        const OPAQUE_FD_KHR = Self::OPAQUE_FD.bits();
        const OPAQUE_WIN32_KHR = Self::OPAQUE_WIN32.bits();
        const OPAQUE_WIN32_KMT_KHR = Self::OPAQUE_WIN32_KMT.bits();
        const SYNC_FD_KHR = Self::SYNC_FD.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalSemaphoreHandleTypeFlagBits(u32);
impl ExternalSemaphoreHandleTypeFlagBits {
    pub const OPAQUE_FD: Self = Self(1 << 0);
    pub const OPAQUE_WIN32: Self = Self(1 << 1);
    pub const OPAQUE_WIN32_KMT: Self = Self(1 << 2);
    pub const D3D12_FENCE: Self = Self(1 << 3);
    pub const SYNC_FD: Self = Self(1 << 4);
    pub const ZIRCON_EVENT_FUCHSIA: Self = Self(1 << 7);
    pub const D3D12_FENCE_KHR: Self = Self::D3D12_FENCE;
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ExternalSemaphoreFeatureFlags: Flags {
        const EXPORTABLE = ExternalSemaphoreFeatureFlagBits::EXPORTABLE.0;
        const IMPORTABLE = ExternalSemaphoreFeatureFlagBits::IMPORTABLE.0;
        const EXPORTABLE_KHR = Self::EXPORTABLE.bits();
        const IMPORTABLE_KHR = Self::IMPORTABLE.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalSemaphoreFeatureFlagBits(u32);
impl ExternalSemaphoreFeatureFlagBits {
    pub const EXPORTABLE: Self = Self(1 << 0);
    pub const IMPORTABLE: Self = Self(1 << 1);
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SemaphoreImportFlags: Flags {
        const TEMPORARY = SemaphoreImportFlagBits::TEMPORARY.0;
        const TEMPORARY_KHR = Self::TEMPORARY.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemaphoreImportFlagBits(u32);
impl SemaphoreImportFlagBits {
    pub const TEMPORARY: Self = Self(1 << 0);
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ExternalFenceHandleTypeFlags: Flags {
        const OPAQUE_FD = ExternalFenceHandleTypeFlagBits::OPAQUE_FD.0;
        const OPAQUE_WIN32 = ExternalFenceHandleTypeFlagBits::OPAQUE_WIN32.0;
        const OPAQUE_WIN32_KMT = ExternalFenceHandleTypeFlagBits::OPAQUE_WIN32_KMT.0;
        const SYNC_FD = ExternalFenceHandleTypeFlagBits::SYNC_FD.0;
        const OPAQUE_FD_KHR = Self::OPAQUE_FD.bits();
        const OPAQUE_WIN32_KHR = Self::OPAQUE_WIN32.bits();
        const OPAQUE_WIN32_KMT_KHR = Self::OPAQUE_WIN32_KMT.bits();
        const SYNC_FD_KHR = Self::SYNC_FD.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalFenceHandleTypeFlagBits(u32);
impl ExternalFenceHandleTypeFlagBits {
    pub const OPAQUE_FD: Self = Self(1 << 0);
    pub const OPAQUE_WIN32: Self = Self(1 << 1);
    pub const OPAQUE_WIN32_KMT: Self = Self(1 << 2);
    pub const SYNC_FD: Self = Self(1 << 3);
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ExternalFenceFeatureFlags: Flags {
        const EXPORTABLE = ExternalFenceFeatureFlagBits::EXPORTABLE.0;
        const IMPORTABLE = ExternalFenceFeatureFlagBits::IMPORTABLE.0;
        const EXPORTABLE_KHR = Self::EXPORTABLE.bits();
        const IMPORTABLE_KHR = Self::IMPORTABLE.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalFenceFeatureFlagBits(u32);
impl ExternalFenceFeatureFlagBits {
    pub const EXPORTABLE: Self = Self(1 << 0);
    pub const IMPORTABLE: Self = Self(1 << 1);
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FenceImportFlags: Flags {
        const TEMPORARY = FenceImportFlagBits::TEMPORARY.0;
        const TEMPORARY_KHR = Self::TEMPORARY.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FenceImportFlagBits(u32);
impl FenceImportFlagBits {
    pub const TEMPORARY: Self = Self(1 << 0);
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
}
pub type PFN_vkEnumerateInstanceVersion =
    unsafe extern "system" fn(p_api_version: *mut u32) -> Result;
pub type PFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_features: *mut PhysicalDeviceFeatures2<'_>,
);
pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_properties: *mut PhysicalDeviceProperties2<'_>,
);
pub type PFN_vkGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    p_format_properties: *mut FormatProperties2<'_>,
);
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_image_format_info: *const PhysicalDeviceImageFormatInfo2<'_>,
    p_image_format_properties: *mut ImageFormatProperties2<'_>,
) -> Result;
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut QueueFamilyProperties2<'_>,
);
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_memory_properties: *mut PhysicalDeviceMemoryProperties2<'_>,
);
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_format_info: *const PhysicalDeviceSparseImageFormatInfo2<'_>,
    p_property_count: *mut u32,
    p_properties: *mut SparseImageFormatProperties2<'_>,
);
pub type PFN_vkTrimCommandPool = unsafe extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    flags: CommandPoolTrimFlags,
);
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo<'_>,
    p_external_buffer_properties: *mut ExternalBufferProperties<'_>,
);
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo<'_>,
    p_external_semaphore_properties: *mut ExternalSemaphoreProperties<'_>,
);
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_fence_info: *const PhysicalDeviceExternalFenceInfo<'_>,
    p_external_fence_properties: *mut ExternalFenceProperties<'_>,
);
pub type PFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
    instance: Instance,
    p_physical_device_group_count: *mut u32,
    p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties<'_>,
) -> Result;
pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(
    device: Device,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    p_peer_memory_features: *mut PeerMemoryFeatureFlags,
);
pub type PFN_vkBindBufferMemory2 = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindBufferMemoryInfo<'_>,
) -> Result;
pub type PFN_vkBindImageMemory2 = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindImageMemoryInfo<'_>,
) -> Result;
pub type PFN_vkCmdSetDeviceMask =
    unsafe extern "system" fn(command_buffer: CommandBuffer, device_mask: u32);
pub type PFN_vkCmdDispatchBase = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    base_group_x: u32,
    base_group_y: u32,
    base_group_z: u32,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
pub type PFN_vkCreateDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorUpdateTemplateCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_descriptor_update_template: *mut DescriptorUpdateTemplate,
) -> Result;
pub type PFN_vkDestroyDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: Device,
    descriptor_update_template: DescriptorUpdateTemplate,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(
    device: Device,
    descriptor_set: DescriptorSet,
    descriptor_update_template: DescriptorUpdateTemplate,
    p_data: *const c_void,
);
pub type PFN_vkGetBufferMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const BufferMemoryRequirementsInfo2<'_>,
    p_memory_requirements: *mut MemoryRequirements2<'_>,
);
pub type PFN_vkGetImageMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageMemoryRequirementsInfo2<'_>,
    p_memory_requirements: *mut MemoryRequirements2<'_>,
);
pub type PFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageSparseMemoryRequirementsInfo2<'_>,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
);
pub type PFN_vkCreateSamplerYcbcrConversion = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SamplerYcbcrConversionCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_ycbcr_conversion: *mut SamplerYcbcrConversion,
) -> Result;
pub type PFN_vkDestroySamplerYcbcrConversion = unsafe extern "system" fn(
    device: Device,
    ycbcr_conversion: SamplerYcbcrConversion,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkGetDeviceQueue2 = unsafe extern "system" fn(
    device: Device,
    p_queue_info: *const DeviceQueueInfo2<'_>,
    p_queue: *mut Queue,
);
pub type PFN_vkGetDescriptorSetLayoutSupport = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
    p_support: *mut DescriptorSetLayoutSupport<'_>,
);

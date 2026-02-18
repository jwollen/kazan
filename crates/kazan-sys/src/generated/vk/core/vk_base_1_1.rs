#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFeatures2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub features: PhysicalDeviceFeatures,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub properties: PhysicalDeviceProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format_properties: FormatProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageFormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_format_properties: ImageFormatProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageFormatInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub ty: ImageType,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub flags: ImageCreateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub queue_family_properties: QueueFamilyProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMemoryProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_properties: PhysicalDeviceMemoryProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageFormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub properties: SparseImageFormatProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSparseImageFormatInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub ty: ImageType,
    pub samples: SampleCountFlags,
    pub usage: ImageUsageFlags,
    pub tiling: ImageTiling,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryProperties {
    pub external_memory_features: ExternalMemoryFeatureFlags,
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
    pub compatible_handle_types: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalImageFormatInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalImageFormatProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_memory_properties: ExternalMemoryProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalBufferInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferCreateFlags,
    pub usage: BufferUsageFlags,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalBufferProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_memory_properties: ExternalMemoryProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceIDProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_uuid: u8,
    pub driver_uuid: u8,
    pub device_luid: u8,
    pub device_node_mask: u32,
    pub device_luid_valid: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryImageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryBufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMemoryAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalSemaphoreInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalSemaphoreProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub external_semaphore_features: ExternalSemaphoreFeatureFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportSemaphoreCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalSemaphoreHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalFenceInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalFenceHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalFenceProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
    pub compatible_handle_types: ExternalFenceHandleTypeFlags,
    pub external_fence_features: ExternalFenceFeatureFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportFenceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalFenceHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceGroupProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub physical_device_count: u32,
    pub physical_devices: PhysicalDevice,
    pub subset_allocation: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryAllocateFlagsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MemoryAllocateFlags,
    pub device_mask: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindBufferMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindBufferMemoryDeviceGroupInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImageMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImageMemoryDeviceGroupInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
    pub split_instance_bind_region_count: u32,
    pub p_split_instance_bind_regions: *const Rect2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupCommandBufferBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_mask: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphore_device_indices: *const u32,
    pub command_buffer_count: u32,
    pub p_command_buffer_device_masks: *const u32,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphore_device_indices: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupBindSparseInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub resource_device_index: u32,
    pub memory_device_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupDeviceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub physical_device_count: u32,
    pub p_physical_devices: *const PhysicalDevice,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageSparseMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryRequirements2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_requirements: MemoryRequirements,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryRequirements2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_requirements: SparseImageMemoryRequirements,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryDedicatedRequirements {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub prefers_dedicated_allocation: Bool32,
    pub requires_dedicated_allocation: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryDedicatedAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub buffer: Buffer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewUsageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub usage: ImageUsageFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImagePlaneMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane_aspect: ImageAspectFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImagePlaneMemoryRequirementsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane_aspect: ImageAspectFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProtectedSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub protected_submit: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProtectedMemoryFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub protected_memory: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProtectedMemoryProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub protected_no_fault: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceQueueInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_index: u32,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PeerMemoryFeatureFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct MemoryAllocateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct CommandPoolTrimFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ExternalMemoryHandleTypeFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ExternalMemoryFeatureFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ExternalSemaphoreHandleTypeFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ExternalSemaphoreFeatureFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SemaphoreImportFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ExternalFenceHandleTypeFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ExternalFenceFeatureFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct FenceImportFlags: Flags {
    }
}
pub type PFN_vkEnumerateInstanceVersion =
    unsafe extern "system" fn(p_api_version: *mut u32) -> Result;
pub type PFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_features: *mut PhysicalDeviceFeatures2,
);
pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_properties: *mut PhysicalDeviceProperties2,
);
pub type PFN_vkGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    p_format_properties: *mut FormatProperties2,
);
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_image_format_info: *const PhysicalDeviceImageFormatInfo2,
    p_image_format_properties: *mut ImageFormatProperties2,
) -> Result;
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut QueueFamilyProperties2,
);
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_memory_properties: *mut PhysicalDeviceMemoryProperties2,
);
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_format_info: *const PhysicalDeviceSparseImageFormatInfo2,
    p_property_count: *mut u32,
    p_properties: *mut SparseImageFormatProperties2,
);
pub type PFN_vkTrimCommandPool = unsafe extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    flags: CommandPoolTrimFlags,
);
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
    p_external_buffer_properties: *mut ExternalBufferProperties,
);
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
    p_external_semaphore_properties: *mut ExternalSemaphoreProperties,
);
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_fence_info: *const PhysicalDeviceExternalFenceInfo,
    p_external_fence_properties: *mut ExternalFenceProperties,
);
pub type PFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
    instance: Instance,
    p_physical_device_group_count: *mut u32,
    p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
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
    p_bind_infos: *const BindBufferMemoryInfo,
) -> Result;
pub type PFN_vkBindImageMemory2 = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindImageMemoryInfo,
) -> Result;
pub type PFN_vkCmdSetDeviceMask =
    unsafe extern "system" fn(command_buffer: CommandBuffer, device_mask: u32);
pub type PFN_vkGetBufferMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const BufferMemoryRequirementsInfo2,
    p_memory_requirements: *mut MemoryRequirements2,
);
pub type PFN_vkGetImageMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageMemoryRequirementsInfo2,
    p_memory_requirements: *mut MemoryRequirements2,
);
pub type PFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageSparseMemoryRequirementsInfo2,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
);
pub type PFN_vkGetDeviceQueue2 = unsafe extern "system" fn(
    device: Device,
    p_queue_info: *const DeviceQueueInfo2,
    p_queue: *mut Queue,
);

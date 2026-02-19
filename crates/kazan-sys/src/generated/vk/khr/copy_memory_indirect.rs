#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StridedDeviceAddressRangeKHR {
    pub address: DeviceAddress,
    pub size: DeviceSize,
    pub stride: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyMemoryIndirectCommandKHR {
    pub src_address: DeviceAddress,
    pub dst_address: DeviceAddress,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyMemoryIndirectInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_copy_flags: AddressCopyFlagsKHR,
    pub dst_copy_flags: AddressCopyFlagsKHR,
    pub copy_count: u32,
    pub copy_address_range: StridedDeviceAddressRangeKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyMemoryToImageIndirectCommandKHR {
    pub src_address: DeviceAddress,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyMemoryToImageIndirectInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_copy_flags: AddressCopyFlagsKHR,
    pub copy_count: u32,
    pub copy_address_range: StridedDeviceAddressRangeKHR,
    pub dst_image: Image,
    pub dst_image_layout: ImageLayout,
    pub p_image_subresources: *const ImageSubresourceLayers,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCopyMemoryIndirectFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub indirect_memory_copy: Bool32,
    pub indirect_memory_to_image_copy: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCopyMemoryIndirectPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_queues: QueueFlags,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct AddressCopyFlagsKHR: Flags {
        const DEVICE_LOCAL_KHR = 1 << 0;
        const SPARSE_KHR = 1 << 1;
        const PROTECTED_KHR = 1 << 2;
    }
}
pub type PFN_vkCmdCopyMemoryIndirectKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_memory_indirect_info: *const CopyMemoryIndirectInfoKHR,
);
pub type PFN_vkCmdCopyMemoryToImageIndirectKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_memory_to_image_indirect_info: *const CopyMemoryToImageIndirectInfoKHR,
);

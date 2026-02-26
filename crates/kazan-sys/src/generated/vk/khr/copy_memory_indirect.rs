#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct StridedDeviceAddressRangeKHR {
    pub address: DeviceAddress,
    pub size: DeviceSize,
    pub stride: DeviceSize,
}
#[repr(C)]
pub struct CopyMemoryIndirectCommandKHR {
    pub src_address: DeviceAddress,
    pub dst_address: DeviceAddress,
    pub size: DeviceSize,
}
#[repr(C)]
pub struct CopyMemoryIndirectInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_copy_flags: AddressCopyFlagsKHR,
    pub dst_copy_flags: AddressCopyFlagsKHR,
    pub copy_count: u32,
    pub copy_address_range: StridedDeviceAddressRangeKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct CopyMemoryToImageIndirectCommandKHR {
    pub src_address: DeviceAddress,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}
#[repr(C)]
pub struct CopyMemoryToImageIndirectInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_copy_flags: AddressCopyFlagsKHR,
    pub copy_count: u32,
    pub copy_address_range: StridedDeviceAddressRangeKHR,
    pub dst_image: Image,
    pub dst_image_layout: ImageLayout,
    pub p_image_subresources: *const ImageSubresourceLayers,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceCopyMemoryIndirectFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub indirect_memory_copy: Bool32,
    pub indirect_memory_to_image_copy: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceCopyMemoryIndirectPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_queues: QueueFlags,
    pub _marker: PhantomData<&'a ()>,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AddressCopyFlagsKHR: Flags {
        const DEVICE_LOCAL_KHR = AddressCopyFlagBitsKHR::DEVICE_LOCAL_KHR.0;
        const SPARSE_KHR = AddressCopyFlagBitsKHR::SPARSE_KHR.0;
        const PROTECTED_KHR = AddressCopyFlagBitsKHR::PROTECTED_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddressCopyFlagBitsKHR(u32);
impl AddressCopyFlagBitsKHR {
    pub const DEVICE_LOCAL_KHR: Self = Self(1 << 0);
    pub const SPARSE_KHR: Self = Self(1 << 1);
    pub const PROTECTED_KHR: Self = Self(1 << 2);
}
pub type PFN_vkCmdCopyMemoryIndirectKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_memory_indirect_info: *const CopyMemoryIndirectInfoKHR<'_>,
);
pub type PFN_vkCmdCopyMemoryToImageIndirectKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_memory_to_image_indirect_info: *const CopyMemoryToImageIndirectInfoKHR<'_>,
);

#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StridedDeviceAddressRangeKHR {
    pub address: DeviceAddress,
    pub size: DeviceSize,
    pub stride: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct CopyMemoryIndirectCommandKHR {
    pub src_address: DeviceAddress,
    pub dst_address: DeviceAddress,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyMemoryIndirectInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_copy_flags: AddressCopyFlagsKHR,
    pub dst_copy_flags: AddressCopyFlagsKHR,
    pub copy_count: u32,
    pub copy_address_range: StridedDeviceAddressRangeKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CopyMemoryIndirectInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COPY_MEMORY_INDIRECT_INFO_KHR,
            p_next: core::ptr::null(),
            src_copy_flags: Default::default(),
            dst_copy_flags: Default::default(),
            copy_count: Default::default(),
            copy_address_range: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
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
impl Default for CopyMemoryToImageIndirectInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COPY_MEMORY_TO_IMAGE_INDIRECT_INFO_KHR,
            p_next: core::ptr::null(),
            src_copy_flags: Default::default(),
            copy_count: Default::default(),
            copy_address_range: Default::default(),
            dst_image: Default::default(),
            dst_image_layout: Default::default(),
            p_image_subresources: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCopyMemoryIndirectFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub indirect_memory_copy: Bool32,
    pub indirect_memory_to_image_copy: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCopyMemoryIndirectFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            indirect_memory_copy: Default::default(),
            indirect_memory_to_image_copy: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCopyMemoryIndirectPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_queues: QueueFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCopyMemoryIndirectPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            supported_queues: Default::default(),
            _marker: PhantomData,
        }
    }
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

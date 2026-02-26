#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type CopyMemoryIndirectCommandNV = CopyMemoryIndirectCommandKHR;
pub type CopyMemoryToImageIndirectCommandNV = CopyMemoryToImageIndirectCommandKHR;
pub type PhysicalDeviceCopyMemoryIndirectPropertiesNV<'a> =
    PhysicalDeviceCopyMemoryIndirectPropertiesKHR<'a>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCopyMemoryIndirectFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub indirect_copy: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCopyMemoryIndirectFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            indirect_copy: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkCmdCopyMemoryIndirectNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    copy_buffer_address: DeviceAddress,
    copy_count: u32,
    stride: u32,
);
pub type PFN_vkCmdCopyMemoryToImageIndirectNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    copy_buffer_address: DeviceAddress,
    copy_count: u32,
    stride: u32,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    p_image_subresources: *const ImageSubresourceLayers,
);

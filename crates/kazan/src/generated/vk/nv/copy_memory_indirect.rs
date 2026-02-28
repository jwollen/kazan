#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
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
    impl<'a> PhysicalDeviceCopyMemoryIndirectFeaturesNV<'a> {
        pub fn indirect_copy(mut self, indirect_copy: Bool32) -> Self {
            self.indirect_copy = indirect_copy;
            self
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
}
pub struct DeviceFn {
    cmd_copy_memory_indirect_nv: PFN_vkCmdCopyMemoryIndirectNV,
    cmd_copy_memory_to_image_indirect_nv: PFN_vkCmdCopyMemoryToImageIndirectNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_copy_memory_indirect_nv: transmute(
                    load(c"vkCmdCopyMemoryIndirectNV").ok_or(LoadingError)?,
                ),
                cmd_copy_memory_to_image_indirect_nv: transmute(
                    load(c"vkCmdCopyMemoryToImageIndirectNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_copy_memory_indirect_nv(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_address: DeviceAddress,
        copy_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_copy_memory_indirect_nv)(
                command_buffer,
                copy_buffer_address,
                copy_count,
                stride,
            )
        }
    }
    pub unsafe fn cmd_copy_memory_to_image_indirect_nv(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_address: DeviceAddress,
        stride: u32,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        image_subresources: &[ImageSubresourceLayers],
    ) {
        unsafe {
            (self.cmd_copy_memory_to_image_indirect_nv)(
                command_buffer,
                copy_buffer_address,
                image_subresources.len().try_into().unwrap(),
                stride,
                dst_image,
                dst_image_layout,
                image_subresources.as_ptr() as _,
            )
        }
    }
}

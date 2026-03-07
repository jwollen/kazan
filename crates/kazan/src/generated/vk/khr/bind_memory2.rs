//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_bind_memory2.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_bind_memory2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindBufferMemoryInfoKHR.html>
    pub type BindBufferMemoryInfoKHR<'a> = BindBufferMemoryInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindImageMemoryInfoKHR.html>
    pub type BindImageMemoryInfoKHR<'a> = BindImageMemoryInfo<'a>;
    pub type PFN_vkBindBufferMemory2KHR = PFN_vkBindBufferMemory2;
    pub type PFN_vkBindImageMemory2KHR = PFN_vkBindImageMemory2;
}

pub struct DeviceFn {
    bind_buffer_memory2_khr: PFN_vkBindBufferMemory2,
    bind_image_memory2_khr: PFN_vkBindImageMemory2,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                bind_buffer_memory2_khr: transmute(
                    load(c"vkBindBufferMemory2KHR").ok_or(MissingEntryPointError)?,
                ),
                bind_image_memory2_khr: transmute(
                    load(c"vkBindImageMemory2KHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBindBufferMemory2KHR.html>
    #[inline]
    pub unsafe fn bind_buffer_memory2_khr(
        &self,
        device: Device,
        bind_infos: &[BindBufferMemoryInfo<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.bind_buffer_memory2_khr)(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBindImageMemory2KHR.html>
    #[inline]
    pub unsafe fn bind_image_memory2_khr(
        &self,
        device: Device,
        bind_infos: &[BindImageMemoryInfo<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.bind_image_memory2_khr)(
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
}

#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_map_memory2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryMapInfoKHR.html>
    pub type MemoryMapInfoKHR<'a> = MemoryMapInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryUnmapInfoKHR.html>
    pub type MemoryUnmapInfoKHR<'a> = MemoryUnmapInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryUnmapFlagsKHR.html>
    pub type MemoryUnmapFlagsKHR = MemoryUnmapFlags;
    pub type PFN_vkMapMemory2KHR = PFN_vkMapMemory2;
    pub type PFN_vkUnmapMemory2KHR = PFN_vkUnmapMemory2;
}

pub struct DeviceFn {
    map_memory2_khr: PFN_vkMapMemory2,
    unmap_memory2_khr: PFN_vkUnmapMemory2,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                map_memory2_khr: transmute(load(c"vkMapMemory2KHR").ok_or(MissingEntryPointError)?),
                unmap_memory2_khr: transmute(
                    load(c"vkUnmapMemory2KHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkMapMemory2KHR.html>
    #[inline]
    pub unsafe fn map_memory2_khr(
        &self,
        device: Device,
        memory_map_info: &MemoryMapInfo<'_>,
    ) -> crate::Result<*mut c_void> {
        unsafe {
            let mut data = core::mem::MaybeUninit::uninit();
            let result = (self.map_memory2_khr)(device, memory_map_info, data.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(data.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUnmapMemory2KHR.html>
    #[inline]
    pub unsafe fn unmap_memory2_khr(
        &self,
        device: Device,
        memory_unmap_info: &MemoryUnmapInfo<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.unmap_memory2_khr)(device, memory_unmap_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}

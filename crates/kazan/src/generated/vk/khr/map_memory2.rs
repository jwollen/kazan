//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_map_memory2.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_map_memory2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryMapInfoKHR.html>
    pub type MemoryMapInfoKHR<'a> = MemoryMapInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryUnmapInfoKHR.html>
    pub type MemoryUnmapInfoKHR<'a> = MemoryUnmapInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryUnmapFlagsKHR.html>
    pub type MemoryUnmapFlagsKHR = MemoryUnmapFlags;
    pub type PFN_vkMapMemory2KHR = PFN_vkMapMemory2;
    pub type PFN_vkUnmapMemory2KHR = PFN_vkUnmapMemory2;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkMemoryMapInfoKHR = MemoryMapInfoKHR<'static>;
    pub type VkMemoryUnmapInfoKHR = MemoryUnmapInfoKHR<'static>;
    pub type VkMemoryUnmapFlagsKHR = MemoryUnmapFlagsKHR;
}

pub struct DeviceFn {
    map_memory2: PFN_vkMapMemory2,
    unmap_memory2: PFN_vkUnmapMemory2,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                map_memory2: transmute(load(c"vkMapMemory2KHR").ok_or(MissingEntryPointError)?),
                unmap_memory2: transmute(load(c"vkUnmapMemory2KHR").ok_or(MissingEntryPointError)?),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkMapMemory2KHR.html>
    #[inline]
    pub unsafe fn map_memory2(
        &self,
        device: Device,
        memory_map_info: &MemoryMapInfo<'_>,
    ) -> crate::Result<*mut c_void> {
        unsafe {
            let mut data = core::mem::MaybeUninit::uninit();
            let result = (self.map_memory2)(device, memory_map_info, data.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(data.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUnmapMemory2KHR.html>
    #[inline]
    pub unsafe fn unmap_memory2(
        &self,
        device: Device,
        memory_unmap_info: &MemoryUnmapInfo<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.unmap_memory2)(device, memory_unmap_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}

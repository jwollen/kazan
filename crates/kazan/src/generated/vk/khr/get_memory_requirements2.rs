//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_get_memory_requirements2.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_get_memory_requirements2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferMemoryRequirementsInfo2KHR.html>
    pub type BufferMemoryRequirementsInfo2KHR<'a> = BufferMemoryRequirementsInfo2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageMemoryRequirementsInfo2KHR.html>
    pub type ImageMemoryRequirementsInfo2KHR<'a> = ImageMemoryRequirementsInfo2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageSparseMemoryRequirementsInfo2KHR.html>
    pub type ImageSparseMemoryRequirementsInfo2KHR<'a> = ImageSparseMemoryRequirementsInfo2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryRequirements2KHR.html>
    pub type MemoryRequirements2KHR<'a> = MemoryRequirements2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSparseImageMemoryRequirements2KHR.html>
    pub type SparseImageMemoryRequirements2KHR<'a> = SparseImageMemoryRequirements2<'a>;
    pub type PFN_vkGetBufferMemoryRequirements2KHR = PFN_vkGetBufferMemoryRequirements2;
    pub type PFN_vkGetImageMemoryRequirements2KHR = PFN_vkGetImageMemoryRequirements2;
    pub type PFN_vkGetImageSparseMemoryRequirements2KHR = PFN_vkGetImageSparseMemoryRequirements2;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkBufferMemoryRequirementsInfo2KHR = BufferMemoryRequirementsInfo2KHR<'static>;
    pub type VkImageMemoryRequirementsInfo2KHR = ImageMemoryRequirementsInfo2KHR<'static>;
    pub type VkImageSparseMemoryRequirementsInfo2KHR =
        ImageSparseMemoryRequirementsInfo2KHR<'static>;
    pub type VkMemoryRequirements2KHR = MemoryRequirements2KHR<'static>;
    pub type VkSparseImageMemoryRequirements2KHR = SparseImageMemoryRequirements2KHR<'static>;
}

pub struct DeviceFn {
    get_image_memory_requirements2: PFN_vkGetImageMemoryRequirements2,
    get_buffer_memory_requirements2: PFN_vkGetBufferMemoryRequirements2,
    get_image_sparse_memory_requirements2: PFN_vkGetImageSparseMemoryRequirements2,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_image_memory_requirements2: transmute(
                    load(c"vkGetImageMemoryRequirements2KHR").ok_or(MissingEntryPointError)?,
                ),
                get_buffer_memory_requirements2: transmute(
                    load(c"vkGetBufferMemoryRequirements2KHR").ok_or(MissingEntryPointError)?,
                ),
                get_image_sparse_memory_requirements2: transmute(
                    load(c"vkGetImageSparseMemoryRequirements2KHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageMemoryRequirements2KHR.html>
    #[inline]
    pub unsafe fn get_image_memory_requirements2(
        &self,
        device: Device,
        info: &ImageMemoryRequirementsInfo2<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe { (self.get_image_memory_requirements2)(device, info, memory_requirements) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetBufferMemoryRequirements2KHR.html>
    #[inline]
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        device: Device,
        info: &BufferMemoryRequirementsInfo2<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe { (self.get_buffer_memory_requirements2)(device, info, memory_requirements) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageSparseMemoryRequirements2KHR.html>
    #[inline]
    pub unsafe fn get_image_sparse_memory_requirements2<'a>(
        &self,
        device: Device,
        info: &ImageSparseMemoryRequirementsInfo2<'a>,
        mut sparse_memory_requirements: impl EnumerateInto<SparseImageMemoryRequirements2<'a>>,
    ) {
        unsafe {
            let call = |sparse_memory_requirement_count, sparse_memory_requirements| {
                (self.get_image_sparse_memory_requirements2)(
                    device,
                    info,
                    sparse_memory_requirement_count,
                    sparse_memory_requirements as _,
                )
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut());
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let sparse_memory_requirements_buf = sparse_memory_requirements.reserve(capacity);
            len = sparse_memory_requirements_buf.len().try_into().unwrap();
            call(
                &mut len,
                sparse_memory_requirements_buf.as_mut_ptr() as *mut _,
            );
            sparse_memory_requirements.set_len(len.try_into().unwrap());
        }
    }
}

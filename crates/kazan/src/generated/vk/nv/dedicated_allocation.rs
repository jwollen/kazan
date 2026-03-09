//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_dedicated_allocation.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_dedicated_allocation";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDedicatedAllocationImageCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DedicatedAllocationImageCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub dedicated_allocation: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DedicatedAllocationImageCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DedicatedAllocationImageCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("dedicated_allocation", &self.dedicated_allocation)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DedicatedAllocationImageCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV;
    }

    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for DedicatedAllocationImageCreateInfoNV<'a> {}

    impl Default for DedicatedAllocationImageCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                dedicated_allocation: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DedicatedAllocationImageCreateInfoNV<'a> {
        #[inline]
        pub fn dedicated_allocation(mut self, dedicated_allocation: bool) -> Self {
            self.dedicated_allocation = dedicated_allocation.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDedicatedAllocationBufferCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DedicatedAllocationBufferCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub dedicated_allocation: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DedicatedAllocationBufferCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DedicatedAllocationBufferCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("dedicated_allocation", &self.dedicated_allocation)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DedicatedAllocationBufferCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV;
    }

    unsafe impl<'a> Extends<BufferCreateInfo<'a>> for DedicatedAllocationBufferCreateInfoNV<'a> {}

    impl Default for DedicatedAllocationBufferCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                dedicated_allocation: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DedicatedAllocationBufferCreateInfoNV<'a> {
        #[inline]
        pub fn dedicated_allocation(mut self, dedicated_allocation: bool) -> Self {
            self.dedicated_allocation = dedicated_allocation.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDedicatedAllocationMemoryAllocateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DedicatedAllocationMemoryAllocateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image: Image,
        pub buffer: Buffer,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DedicatedAllocationMemoryAllocateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DedicatedAllocationMemoryAllocateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image", &self.image)
                .field("buffer", &self.buffer)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DedicatedAllocationMemoryAllocateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV;
    }

    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for DedicatedAllocationMemoryAllocateInfoNV<'a> {}

    impl Default for DedicatedAllocationMemoryAllocateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                image: Default::default(),
                buffer: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DedicatedAllocationMemoryAllocateInfoNV<'a> {
        #[inline]
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }

        #[inline]
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkDedicatedAllocationImageCreateInfoNV = DedicatedAllocationImageCreateInfoNV<'static>;
    pub type VkDedicatedAllocationBufferCreateInfoNV =
        DedicatedAllocationBufferCreateInfoNV<'static>;
    pub type VkDedicatedAllocationMemoryAllocateInfoNV =
        DedicatedAllocationMemoryAllocateInfoNV<'static>;
    impl DedicatedAllocationImageCreateInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDedicatedAllocationImageCreateInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DedicatedAllocationBufferCreateInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDedicatedAllocationBufferCreateInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DedicatedAllocationMemoryAllocateInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDedicatedAllocationMemoryAllocateInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}

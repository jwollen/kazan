#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_dedicated_allocation";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDedicatedAllocationImageCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
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
                p_next: core::ptr::null(),
                dedicated_allocation: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DedicatedAllocationImageCreateInfoNV<'a> {
        pub fn dedicated_allocation(mut self, dedicated_allocation: bool) -> Self {
            self.dedicated_allocation = dedicated_allocation.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDedicatedAllocationBufferCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
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
                p_next: core::ptr::null(),
                dedicated_allocation: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DedicatedAllocationBufferCreateInfoNV<'a> {
        pub fn dedicated_allocation(mut self, dedicated_allocation: bool) -> Self {
            self.dedicated_allocation = dedicated_allocation.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDedicatedAllocationMemoryAllocateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
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
                p_next: core::ptr::null(),
                image: Default::default(),
                buffer: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DedicatedAllocationMemoryAllocateInfoNV<'a> {
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }

        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
    }
}

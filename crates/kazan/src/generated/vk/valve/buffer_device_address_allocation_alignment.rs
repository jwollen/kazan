//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_VALVE_buffer_device_address_allocation_alignment.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_VALVE_buffer_device_address_allocation_alignment";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceBufferDeviceAddressAllocationAlignmentFeaturesVALVE.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceBufferDeviceAddressAllocationAlignmentFeaturesVALVE<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub buffer_device_address_allocation_alignment: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceBufferDeviceAddressAllocationAlignmentFeaturesVALVE<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceBufferDeviceAddressAllocationAlignmentFeaturesVALVE")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "buffer_device_address_allocation_alignment",
                    &self.buffer_device_address_allocation_alignment,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDeviceBufferDeviceAddressAllocationAlignmentFeaturesVALVE<'a>
    {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_ALLOCATION_ALIGNMENT_FEATURES_VALVE;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceBufferDeviceAddressAllocationAlignmentFeaturesVALVE<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceBufferDeviceAddressAllocationAlignmentFeaturesVALVE<'_>
    {
    }

    impl Default for PhysicalDeviceBufferDeviceAddressAllocationAlignmentFeaturesVALVE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                buffer_device_address_allocation_alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceBufferDeviceAddressAllocationAlignmentFeaturesVALVE<'a> {
        #[inline]
        pub fn buffer_device_address_allocation_alignment(
            mut self,
            buffer_device_address_allocation_alignment: bool,
        ) -> Self {
            self.buffer_device_address_allocation_alignment =
                buffer_device_address_allocation_alignment.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceBufferDeviceAddressAllocationAlignmentPropertiesVALVE.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceBufferDeviceAddressAllocationAlignmentPropertiesVALVE<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_buffer_device_address_allocation_alignment: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceBufferDeviceAddressAllocationAlignmentPropertiesVALVE<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceBufferDeviceAddressAllocationAlignmentPropertiesVALVE")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_buffer_device_address_allocation_alignment",
                    &self.max_buffer_device_address_allocation_alignment,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDeviceBufferDeviceAddressAllocationAlignmentPropertiesVALVE<'a>
    {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_ALLOCATION_ALIGNMENT_PROPERTIES_VALVE;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceBufferDeviceAddressAllocationAlignmentPropertiesVALVE<'_>
    {
    }

    impl Default for PhysicalDeviceBufferDeviceAddressAllocationAlignmentPropertiesVALVE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_buffer_device_address_allocation_alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceBufferDeviceAddressAllocationAlignmentPropertiesVALVE<'a> {
        #[inline]
        pub fn max_buffer_device_address_allocation_alignment(
            mut self,
            max_buffer_device_address_allocation_alignment: u32,
        ) -> Self {
            self.max_buffer_device_address_allocation_alignment =
                max_buffer_device_address_allocation_alignment;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferDeviceAddressAlignmentAllocateInfoVALVE.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BufferDeviceAddressAlignmentAllocateInfoVALVE<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub alignment: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BufferDeviceAddressAlignmentAllocateInfoVALVE<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BufferDeviceAddressAlignmentAllocateInfoVALVE")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("alignment", &self.alignment)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BufferDeviceAddressAlignmentAllocateInfoVALVE<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::BUFFER_DEVICE_ADDRESS_ALIGNMENT_ALLOCATE_INFO_VALVE;
    }

    unsafe impl Extends<BufferCreateInfo<'_>> for BufferDeviceAddressAlignmentAllocateInfoVALVE<'_> {}
    unsafe impl Extends<MemoryAllocateInfo<'_>> for BufferDeviceAddressAlignmentAllocateInfoVALVE<'_> {}

    impl Default for BufferDeviceAddressAlignmentAllocateInfoVALVE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BufferDeviceAddressAlignmentAllocateInfoVALVE<'a> {
        #[inline]
        pub fn alignment(mut self, alignment: u32) -> Self {
            self.alignment = alignment;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceBufferDeviceAddressAllocationAlignmentFeaturesVALVE =
        PhysicalDeviceBufferDeviceAddressAllocationAlignmentFeaturesVALVE<'static>;
    pub type VkPhysicalDeviceBufferDeviceAddressAllocationAlignmentPropertiesVALVE =
        PhysicalDeviceBufferDeviceAddressAllocationAlignmentPropertiesVALVE<'static>;
    pub type VkBufferDeviceAddressAlignmentAllocateInfoVALVE =
        BufferDeviceAddressAlignmentAllocateInfoVALVE<'static>;
    impl PhysicalDeviceBufferDeviceAddressAllocationAlignmentFeaturesVALVE<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceBufferDeviceAddressAllocationAlignmentFeaturesVALVE {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceBufferDeviceAddressAllocationAlignmentPropertiesVALVE<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceBufferDeviceAddressAllocationAlignmentPropertiesVALVE {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BufferDeviceAddressAlignmentAllocateInfoVALVE<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkBufferDeviceAddressAlignmentAllocateInfoVALVE {
            unsafe { core::mem::transmute(self) }
        }
    }
}

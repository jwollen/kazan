//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_memory_priority.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_memory_priority";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMemoryPriorityFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMemoryPriorityFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_priority: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMemoryPriorityFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMemoryPriorityFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory_priority", &self.memory_priority)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMemoryPriorityFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceMemoryPriorityFeaturesEXT<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceMemoryPriorityFeaturesEXT<'_> {}

    impl Default for PhysicalDeviceMemoryPriorityFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                memory_priority: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMemoryPriorityFeaturesEXT<'a> {
        #[inline]
        pub fn memory_priority(mut self, memory_priority: bool) -> Self {
            self.memory_priority = memory_priority.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryPriorityAllocateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryPriorityAllocateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub priority: f32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryPriorityAllocateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryPriorityAllocateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("priority", &self.priority)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryPriorityAllocateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_PRIORITY_ALLOCATE_INFO_EXT;
    }

    unsafe impl Extends<MemoryAllocateInfo<'_>> for MemoryPriorityAllocateInfoEXT<'_> {}

    impl Default for MemoryPriorityAllocateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                priority: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryPriorityAllocateInfoEXT<'a> {
        #[inline]
        pub fn priority(mut self, priority: f32) -> Self {
            self.priority = priority;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceMemoryPriorityFeaturesEXT =
        PhysicalDeviceMemoryPriorityFeaturesEXT<'static>;
    pub type VkMemoryPriorityAllocateInfoEXT = MemoryPriorityAllocateInfoEXT<'static>;
    impl PhysicalDeviceMemoryPriorityFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceMemoryPriorityFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MemoryPriorityAllocateInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMemoryPriorityAllocateInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

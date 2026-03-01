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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceMemoryOverallocationCreateInfoAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub overallocation_behavior: MemoryOverallocationBehaviorAMD,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DeviceMemoryOverallocationCreateInfoAMD<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD;
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for DeviceMemoryOverallocationCreateInfoAMD<'a> {}
    impl Default for DeviceMemoryOverallocationCreateInfoAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                overallocation_behavior: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceMemoryOverallocationCreateInfoAMD<'a> {
        pub fn overallocation_behavior(
            mut self,
            overallocation_behavior: MemoryOverallocationBehaviorAMD,
        ) -> Self {
            self.overallocation_behavior = overallocation_behavior;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MemoryOverallocationBehaviorAMD(i32);
    impl MemoryOverallocationBehaviorAMD {
        pub const DEFAULT_AMD: Self = Self(0);
        pub const ALLOWED_AMD: Self = Self(1);
        pub const DISALLOWED_AMD: Self = Self(2);
    }
}

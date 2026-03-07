#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_AMD_memory_overallocation_behavior";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceMemoryOverallocationCreateInfoAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceMemoryOverallocationCreateInfoAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub overallocation_behavior: MemoryOverallocationBehaviorAMD,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for DeviceMemoryOverallocationCreateInfoAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceMemoryOverallocationCreateInfoAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("overallocation_behavior", &self.overallocation_behavior)
                .finish()
        }
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryOverallocationBehaviorAMD.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MemoryOverallocationBehaviorAMD(i32);

    impl MemoryOverallocationBehaviorAMD {
        pub const DEFAULT_AMD: Self = Self(0);
        pub const ALLOWED_AMD: Self = Self(1);
        pub const DISALLOWED_AMD: Self = Self(2);
    }

    impl fmt::Debug for MemoryOverallocationBehaviorAMD {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEFAULT_AMD => Some("DEFAULT_AMD"),
                Self::ALLOWED_AMD => Some("ALLOWED_AMD"),
                Self::DISALLOWED_AMD => Some("DISALLOWED_AMD"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
}

//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_pageable_device_local_memory.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_pageable_device_local_memory";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pageable_device_local_memory: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "pageable_device_local_memory",
                    &self.pageable_device_local_memory,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'_>
    {
    }

    impl Default for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                pageable_device_local_memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'a> {
        #[inline]
        pub fn pageable_device_local_memory(mut self, pageable_device_local_memory: bool) -> Self {
            self.pageable_device_local_memory = pageable_device_local_memory.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetDeviceMemoryPriorityEXT.html>
    pub type PFN_vkSetDeviceMemoryPriorityEXT =
        unsafe extern "system" fn(device: Device, memory: DeviceMemory, priority: f32);
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT =
        PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'static>;
    impl PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    set_device_memory_priority_ext: PFN_vkSetDeviceMemoryPriorityEXT,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                set_device_memory_priority_ext: transmute(
                    load(c"vkSetDeviceMemoryPriorityEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetDeviceMemoryPriorityEXT.html>
    #[inline]
    pub unsafe fn set_device_memory_priority_ext(
        &self,
        device: Device,
        memory: DeviceMemory,
        priority: f32,
    ) {
        unsafe { (self.set_device_memory_priority_ext)(device, memory, priority) }
    }
}

//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_shader_split_barrier.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_shader_split_barrier";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderSplitBarrierFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderSplitBarrierFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_split_barrier: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderSplitBarrierFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderSplitBarrierFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_split_barrier", &self.shader_split_barrier)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderSplitBarrierFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_SPLIT_BARRIER_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceShaderSplitBarrierFeaturesEXT<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceShaderSplitBarrierFeaturesEXT<'_> {}

    impl Default for PhysicalDeviceShaderSplitBarrierFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_split_barrier: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderSplitBarrierFeaturesEXT<'a> {
        #[inline]
        pub fn shader_split_barrier(mut self, shader_split_barrier: bool) -> Self {
            self.shader_split_barrier = shader_split_barrier.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderSplitBarrierPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderSplitBarrierPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub split_barrier_reserved_shared_memory: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderSplitBarrierPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderSplitBarrierPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "split_barrier_reserved_shared_memory",
                    &self.split_barrier_reserved_shared_memory,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderSplitBarrierPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_SPLIT_BARRIER_PROPERTIES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceShaderSplitBarrierPropertiesEXT<'_>
    {
    }

    impl Default for PhysicalDeviceShaderSplitBarrierPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                split_barrier_reserved_shared_memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderSplitBarrierPropertiesEXT<'a> {
        #[inline]
        pub fn split_barrier_reserved_shared_memory(
            mut self,
            split_barrier_reserved_shared_memory: u32,
        ) -> Self {
            self.split_barrier_reserved_shared_memory = split_barrier_reserved_shared_memory;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceShaderSplitBarrierFeaturesEXT =
        PhysicalDeviceShaderSplitBarrierFeaturesEXT<'static>;
    pub type VkPhysicalDeviceShaderSplitBarrierPropertiesEXT =
        PhysicalDeviceShaderSplitBarrierPropertiesEXT<'static>;
    impl PhysicalDeviceShaderSplitBarrierFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderSplitBarrierFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceShaderSplitBarrierPropertiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderSplitBarrierPropertiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

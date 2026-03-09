//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_fragment_shader_interlock.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_fragment_shader_interlock";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub fragment_shader_sample_interlock: Bool32,
        pub fragment_shader_pixel_interlock: Bool32,
        pub fragment_shader_shading_rate_interlock: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceFragmentShaderInterlockFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "fragment_shader_sample_interlock",
                    &self.fragment_shader_sample_interlock,
                )
                .field(
                    "fragment_shader_pixel_interlock",
                    &self.fragment_shader_pixel_interlock,
                )
                .field(
                    "fragment_shader_shading_rate_interlock",
                    &self.fragment_shader_shading_rate_interlock,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                fragment_shader_sample_interlock: Default::default(),
                fragment_shader_pixel_interlock: Default::default(),
                fragment_shader_shading_rate_interlock: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'a> {
        #[inline]
        pub fn fragment_shader_sample_interlock(
            mut self,
            fragment_shader_sample_interlock: bool,
        ) -> Self {
            self.fragment_shader_sample_interlock = fragment_shader_sample_interlock.into();
            self
        }

        #[inline]
        pub fn fragment_shader_pixel_interlock(
            mut self,
            fragment_shader_pixel_interlock: bool,
        ) -> Self {
            self.fragment_shader_pixel_interlock = fragment_shader_pixel_interlock.into();
            self
        }

        #[inline]
        pub fn fragment_shader_shading_rate_interlock(
            mut self,
            fragment_shader_shading_rate_interlock: bool,
        ) -> Self {
            self.fragment_shader_shading_rate_interlock =
                fragment_shader_shading_rate_interlock.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT =
        PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'static>;
    impl PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

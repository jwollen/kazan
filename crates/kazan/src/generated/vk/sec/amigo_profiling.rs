//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_SEC_amigo_profiling.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_SEC_amigo_profiling";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceAmigoProfilingFeaturesSEC.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceAmigoProfilingFeaturesSEC<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub amigo_profiling: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceAmigoProfilingFeaturesSEC<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceAmigoProfilingFeaturesSEC")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("amigo_profiling", &self.amigo_profiling)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceAmigoProfilingFeaturesSEC<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceAmigoProfilingFeaturesSEC<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceAmigoProfilingFeaturesSEC<'a> {}

    impl Default for PhysicalDeviceAmigoProfilingFeaturesSEC<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                amigo_profiling: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceAmigoProfilingFeaturesSEC<'a> {
        #[inline]
        pub fn amigo_profiling(mut self, amigo_profiling: bool) -> Self {
            self.amigo_profiling = amigo_profiling.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAmigoProfilingSubmitInfoSEC.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AmigoProfilingSubmitInfoSEC<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub first_draw_timestamp: u64,
        pub swap_buffer_timestamp: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AmigoProfilingSubmitInfoSEC<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AmigoProfilingSubmitInfoSEC")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("first_draw_timestamp", &self.first_draw_timestamp)
                .field("swap_buffer_timestamp", &self.swap_buffer_timestamp)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AmigoProfilingSubmitInfoSEC<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::AMIGO_PROFILING_SUBMIT_INFO_SEC;
    }

    unsafe impl<'a> Extends<SubmitInfo<'a>> for AmigoProfilingSubmitInfoSEC<'a> {}

    impl Default for AmigoProfilingSubmitInfoSEC<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                first_draw_timestamp: Default::default(),
                swap_buffer_timestamp: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AmigoProfilingSubmitInfoSEC<'a> {
        #[inline]
        pub fn first_draw_timestamp(mut self, first_draw_timestamp: u64) -> Self {
            self.first_draw_timestamp = first_draw_timestamp;
            self
        }

        #[inline]
        pub fn swap_buffer_timestamp(mut self, swap_buffer_timestamp: u64) -> Self {
            self.swap_buffer_timestamp = swap_buffer_timestamp;
            self
        }
    }
}

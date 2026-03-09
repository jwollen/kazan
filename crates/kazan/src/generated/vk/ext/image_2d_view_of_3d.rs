//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_image_2d_view_of_3d.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_image_2d_view_of_3d";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImage2DViewOf3DFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceImage2DViewOf3DFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image2_d_view_of3_d: Bool32,
        pub sampler2_d_view_of3_d: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceImage2DViewOf3DFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceImage2DViewOf3DFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image2_d_view_of3_d", &self.image2_d_view_of3_d)
                .field("sampler2_d_view_of3_d", &self.sampler2_d_view_of3_d)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImage2DViewOf3DFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceImage2DViewOf3DFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceImage2DViewOf3DFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceImage2DViewOf3DFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                image2_d_view_of3_d: Default::default(),
                sampler2_d_view_of3_d: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceImage2DViewOf3DFeaturesEXT<'a> {
        #[inline]
        pub fn image2_d_view_of3_d(mut self, image2_d_view_of3_d: bool) -> Self {
            self.image2_d_view_of3_d = image2_d_view_of3_d.into();
            self
        }

        #[inline]
        pub fn sampler2_d_view_of3_d(mut self, sampler2_d_view_of3_d: bool) -> Self {
            self.sampler2_d_view_of3_d = sampler2_d_view_of3_d.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceImage2DViewOf3DFeaturesEXT =
        PhysicalDeviceImage2DViewOf3DFeaturesEXT<'static>;
    impl PhysicalDeviceImage2DViewOf3DFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceImage2DViewOf3DFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

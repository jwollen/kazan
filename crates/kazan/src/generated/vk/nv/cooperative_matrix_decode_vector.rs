//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_cooperative_matrix_decode_vector.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_cooperative_matrix_decode_vector";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cooperative_matrix_decode_vector: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "cooperative_matrix_decode_vector",
                    &self.cooperative_matrix_decode_vector,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_DECODE_VECTOR_FEATURES_NV;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'_>
    {
    }

    impl Default for PhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                cooperative_matrix_decode_vector: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'a> {
        #[inline]
        pub fn cooperative_matrix_decode_vector(
            mut self,
            cooperative_matrix_decode_vector: bool,
        ) -> Self {
            self.cooperative_matrix_decode_vector = cooperative_matrix_decode_vector.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV =
        PhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'static>;
    impl PhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}

//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_ARM_tensor_controls.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_ARM_tensor_controls";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorExplicitTilingFormatPropertiesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct TensorExplicitTilingFormatPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub brick16_tiling_tensor_features: FormatFeatureFlags2,
        pub brick8_tiling_tensor_features: FormatFeatureFlags2,
        pub brick4_tiling_tensor_features: FormatFeatureFlags2,
        pub block_u_tiling_tensor_features: FormatFeatureFlags2,
        pub block_u64k_tiling_tensor_features: FormatFeatureFlags2,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for TensorExplicitTilingFormatPropertiesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TensorExplicitTilingFormatPropertiesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "brick16_tiling_tensor_features",
                    &self.brick16_tiling_tensor_features,
                )
                .field(
                    "brick8_tiling_tensor_features",
                    &self.brick8_tiling_tensor_features,
                )
                .field(
                    "brick4_tiling_tensor_features",
                    &self.brick4_tiling_tensor_features,
                )
                .field(
                    "block_u_tiling_tensor_features",
                    &self.block_u_tiling_tensor_features,
                )
                .field(
                    "block_u64k_tiling_tensor_features",
                    &self.block_u64k_tiling_tensor_features,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TensorExplicitTilingFormatPropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::TENSOR_EXPLICIT_TILING_FORMAT_PROPERTIES_ARM;
    }

    unsafe impl Extends<FormatProperties2<'_>> for TensorExplicitTilingFormatPropertiesARM<'_> {}

    impl Default for TensorExplicitTilingFormatPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                brick16_tiling_tensor_features: Default::default(),
                brick8_tiling_tensor_features: Default::default(),
                brick4_tiling_tensor_features: Default::default(),
                block_u_tiling_tensor_features: Default::default(),
                block_u64k_tiling_tensor_features: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TensorExplicitTilingFormatPropertiesARM<'a> {
        #[inline]
        pub fn brick16_tiling_tensor_features(
            mut self,
            brick16_tiling_tensor_features: FormatFeatureFlags2,
        ) -> Self {
            self.brick16_tiling_tensor_features = brick16_tiling_tensor_features;
            self
        }

        #[inline]
        pub fn brick8_tiling_tensor_features(
            mut self,
            brick8_tiling_tensor_features: FormatFeatureFlags2,
        ) -> Self {
            self.brick8_tiling_tensor_features = brick8_tiling_tensor_features;
            self
        }

        #[inline]
        pub fn brick4_tiling_tensor_features(
            mut self,
            brick4_tiling_tensor_features: FormatFeatureFlags2,
        ) -> Self {
            self.brick4_tiling_tensor_features = brick4_tiling_tensor_features;
            self
        }

        #[inline]
        pub fn block_u_tiling_tensor_features(
            mut self,
            block_u_tiling_tensor_features: FormatFeatureFlags2,
        ) -> Self {
            self.block_u_tiling_tensor_features = block_u_tiling_tensor_features;
            self
        }

        #[inline]
        pub fn block_u64k_tiling_tensor_features(
            mut self,
            block_u64k_tiling_tensor_features: FormatFeatureFlags2,
        ) -> Self {
            self.block_u64k_tiling_tensor_features = block_u64k_tiling_tensor_features;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTensorRollingBackingCreateInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct TensorRollingBackingCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub wraps: [u32; MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM as usize],
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for TensorRollingBackingCreateInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TensorRollingBackingCreateInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("wraps", &self.wraps)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TensorRollingBackingCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TENSOR_ROLLING_BACKING_CREATE_INFO_ARM;
    }

    unsafe impl Extends<TensorCreateInfoARM<'_>> for TensorRollingBackingCreateInfoARM<'_> {}

    impl Default for TensorRollingBackingCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                wraps: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TensorRollingBackingCreateInfoARM<'a> {
        #[inline]
        pub fn wraps(
            mut self,
            wraps: [u32; MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM as usize],
        ) -> Self {
            self.wraps = wraps;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkTensorExplicitTilingFormatPropertiesARM =
        TensorExplicitTilingFormatPropertiesARM<'static>;
    pub type VkTensorRollingBackingCreateInfoARM = TensorRollingBackingCreateInfoARM<'static>;
    impl TensorExplicitTilingFormatPropertiesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkTensorExplicitTilingFormatPropertiesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl TensorRollingBackingCreateInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkTensorRollingBackingCreateInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
}

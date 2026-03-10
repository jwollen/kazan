//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_validation_features.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_validation_features";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkValidationFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ValidationFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub enabled_validation_feature_count: u32,
        pub p_enabled_validation_features: *const ValidationFeatureEnableEXT,
        pub disabled_validation_feature_count: u32,
        pub p_disabled_validation_features: *const ValidationFeatureDisableEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ValidationFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ValidationFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "enabled_validation_feature_count",
                    &self.enabled_validation_feature_count,
                )
                .field(
                    "p_enabled_validation_features",
                    &self.p_enabled_validation_features,
                )
                .field(
                    "disabled_validation_feature_count",
                    &self.disabled_validation_feature_count,
                )
                .field(
                    "p_disabled_validation_features",
                    &self.p_disabled_validation_features,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ValidationFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VALIDATION_FEATURES_EXT;
    }

    unsafe impl Extends<InstanceCreateInfo<'_>> for ValidationFeaturesEXT<'_> {}
    unsafe impl Extends<ShaderModuleCreateInfo<'_>> for ValidationFeaturesEXT<'_> {}
    unsafe impl Extends<ShaderCreateInfoEXT<'_>> for ValidationFeaturesEXT<'_> {}

    impl Default for ValidationFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                enabled_validation_feature_count: Default::default(),
                p_enabled_validation_features: ptr::null(),
                disabled_validation_feature_count: Default::default(),
                p_disabled_validation_features: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ValidationFeaturesEXT<'a> {
        #[inline]
        pub fn enabled_validation_features(
            mut self,
            enabled_validation_features: &'a [ValidationFeatureEnableEXT],
        ) -> Self {
            self.enabled_validation_feature_count =
                enabled_validation_features.len().try_into().unwrap();
            self.p_enabled_validation_features = enabled_validation_features.as_ptr() as _;
            self
        }

        #[inline]
        pub fn disabled_validation_features(
            mut self,
            disabled_validation_features: &'a [ValidationFeatureDisableEXT],
        ) -> Self {
            self.disabled_validation_feature_count =
                disabled_validation_features.len().try_into().unwrap();
            self.p_disabled_validation_features = disabled_validation_features.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkValidationFeatureEnableEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ValidationFeatureEnableEXT(i32);

    impl ValidationFeatureEnableEXT {
        pub const GPU_ASSISTED_EXT: Self = Self(0);
        pub const GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT: Self = Self(1);
        pub const BEST_PRACTICES_EXT: Self = Self(2);
        pub const DEBUG_PRINTF_EXT: Self = Self(3);
        pub const SYNCHRONIZATION_VALIDATION_EXT: Self = Self(4);
    }

    impl fmt::Debug for ValidationFeatureEnableEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::GPU_ASSISTED_EXT => Some("GPU_ASSISTED_EXT"),
                Self::GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT => {
                    Some("GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT")
                }
                Self::BEST_PRACTICES_EXT => Some("BEST_PRACTICES_EXT"),
                Self::DEBUG_PRINTF_EXT => Some("DEBUG_PRINTF_EXT"),
                Self::SYNCHRONIZATION_VALIDATION_EXT => Some("SYNCHRONIZATION_VALIDATION_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkValidationFeatureDisableEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ValidationFeatureDisableEXT(i32);

    impl ValidationFeatureDisableEXT {
        pub const ALL_EXT: Self = Self(0);
        pub const SHADERS_EXT: Self = Self(1);
        pub const THREAD_SAFETY_EXT: Self = Self(2);
        pub const API_PARAMETERS_EXT: Self = Self(3);
        pub const OBJECT_LIFETIMES_EXT: Self = Self(4);
        pub const CORE_CHECKS_EXT: Self = Self(5);
        pub const UNIQUE_HANDLES_EXT: Self = Self(6);
        pub const SHADER_VALIDATION_CACHE_EXT: Self = Self(7);
    }

    impl fmt::Debug for ValidationFeatureDisableEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ALL_EXT => Some("ALL_EXT"),
                Self::SHADERS_EXT => Some("SHADERS_EXT"),
                Self::THREAD_SAFETY_EXT => Some("THREAD_SAFETY_EXT"),
                Self::API_PARAMETERS_EXT => Some("API_PARAMETERS_EXT"),
                Self::OBJECT_LIFETIMES_EXT => Some("OBJECT_LIFETIMES_EXT"),
                Self::CORE_CHECKS_EXT => Some("CORE_CHECKS_EXT"),
                Self::UNIQUE_HANDLES_EXT => Some("UNIQUE_HANDLES_EXT"),
                Self::SHADER_VALIDATION_CACHE_EXT => Some("SHADER_VALIDATION_CACHE_EXT"),
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

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkValidationFeaturesEXT = ValidationFeaturesEXT<'static>;
    pub type VkValidationFeatureEnableEXT = ValidationFeatureEnableEXT;
    pub type VkValidationFeatureDisableEXT = ValidationFeatureDisableEXT;
    impl ValidationFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkValidationFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

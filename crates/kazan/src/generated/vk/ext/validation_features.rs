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
    pub struct ValidationFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub enabled_validation_feature_count: u32,
        pub p_enabled_validation_features: *const ValidationFeatureEnableEXT,
        pub disabled_validation_feature_count: u32,
        pub p_disabled_validation_features: *const ValidationFeatureDisableEXT,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ValidationFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VALIDATION_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<InstanceCreateInfo<'a>> for ValidationFeaturesEXT<'a> {}
    unsafe impl<'a> Extends<ShaderModuleCreateInfo<'a>> for ValidationFeaturesEXT<'a> {}
    unsafe impl<'a> Extends<ShaderCreateInfoEXT<'a>> for ValidationFeaturesEXT<'a> {}
    impl Default for ValidationFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                enabled_validation_feature_count: Default::default(),
                p_enabled_validation_features: core::ptr::null(),
                disabled_validation_feature_count: Default::default(),
                p_disabled_validation_features: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ValidationFeaturesEXT<'a> {
        pub fn enabled_validation_features(
            mut self,
            enabled_validation_features: &'a [ValidationFeatureEnableEXT],
        ) -> Self {
            self.enabled_validation_feature_count =
                enabled_validation_features.len().try_into().unwrap();
            self.p_enabled_validation_features = enabled_validation_features.as_ptr();
            self
        }
        pub fn disabled_validation_features(
            mut self,
            disabled_validation_features: &'a [ValidationFeatureDisableEXT],
        ) -> Self {
            self.disabled_validation_feature_count =
                disabled_validation_features.len().try_into().unwrap();
            self.p_disabled_validation_features = disabled_validation_features.as_ptr();
            self
        }
    }
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
}

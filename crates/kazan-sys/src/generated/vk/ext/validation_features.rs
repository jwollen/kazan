#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct ValidationFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub enabled_validation_feature_count: u32,
    pub p_enabled_validation_features: *const ValidationFeatureEnableEXT,
    pub disabled_validation_feature_count: u32,
    pub p_disabled_validation_features: *const ValidationFeatureDisableEXT,
    pub _marker: PhantomData<&'a ()>,
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

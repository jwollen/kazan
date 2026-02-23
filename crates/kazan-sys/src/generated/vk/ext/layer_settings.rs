#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct LayerSettingsCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub setting_count: u32,
    pub p_settings: *const LayerSettingEXT,
}
#[repr(C)]
pub struct LayerSettingEXT {
    pub p_layer_name: *const c_char,
    pub p_setting_name: *const c_char,
    pub ty: LayerSettingTypeEXT,
    pub value_count: u32,
    pub p_values: *const c_void,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayerSettingTypeEXT(i32);
impl LayerSettingTypeEXT {
    pub const BOOL32_EXT: Self = Self(0);
    pub const INT32_EXT: Self = Self(1);
    pub const INT64_EXT: Self = Self(2);
    pub const UINT32_EXT: Self = Self(3);
    pub const UINT64_EXT: Self = Self(4);
    pub const FLOAT32_EXT: Self = Self(5);
    pub const FLOAT64_EXT: Self = Self(6);
    pub const STRING_EXT: Self = Self(7);
}

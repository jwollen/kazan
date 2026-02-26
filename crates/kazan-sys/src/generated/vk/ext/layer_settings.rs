#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LayerSettingsCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub setting_count: u32,
    pub p_settings: *const LayerSettingEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for LayerSettingsCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::LAYER_SETTINGS_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            setting_count: Default::default(),
            p_settings: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LayerSettingEXT<'a> {
    pub p_layer_name: *const c_char,
    pub p_setting_name: *const c_char,
    pub ty: LayerSettingTypeEXT,
    pub value_count: u32,
    pub p_values: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for LayerSettingEXT<'_> {
    fn default() -> Self {
        Self {
            p_layer_name: core::ptr::null(),
            p_setting_name: core::ptr::null(),
            ty: Default::default(),
            value_count: Default::default(),
            p_values: core::ptr::null(),
            _marker: PhantomData,
        }
    }
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

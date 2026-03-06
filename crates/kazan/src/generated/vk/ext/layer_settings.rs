#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkLayerSettingsCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct LayerSettingsCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub setting_count: u32,
        pub p_settings: *const LayerSettingEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for LayerSettingsCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::LAYER_SETTINGS_CREATE_INFO_EXT;
    }
    unsafe impl<'a> Extends<InstanceCreateInfo<'a>> for LayerSettingsCreateInfoEXT<'a> {}
    impl Default for LayerSettingsCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                setting_count: Default::default(),
                p_settings: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> LayerSettingsCreateInfoEXT<'a> {
        pub fn settings(mut self, settings: &'a [LayerSettingEXT<'a>]) -> Self {
            self.setting_count = settings.len().try_into().unwrap();
            self.p_settings = settings.as_ptr();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkLayerSettingEXT.html>
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
    impl<'a> LayerSettingEXT<'a> {
        pub fn layer_name(mut self, layer_name: &'a CStr) -> Self {
            self.p_layer_name = layer_name.as_ptr();
            self
        }
        pub fn setting_name(mut self, setting_name: &'a CStr) -> Self {
            self.p_setting_name = setting_name.as_ptr();
            self
        }
        pub fn ty(mut self, ty: LayerSettingTypeEXT) -> Self {
            self.ty = ty;
            self
        }
        pub fn values(mut self, values: &'a [u8]) -> Self {
            self.value_count = values.len().try_into().unwrap();
            self.p_values = values.as_ptr() as _;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkLayerSettingTypeEXT.html>
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
    impl fmt::Debug for LayerSettingTypeEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::BOOL32_EXT => Some("BOOL32_EXT"),
                Self::INT32_EXT => Some("INT32_EXT"),
                Self::INT64_EXT => Some("INT64_EXT"),
                Self::UINT32_EXT => Some("UINT32_EXT"),
                Self::UINT64_EXT => Some("UINT64_EXT"),
                Self::FLOAT32_EXT => Some("FLOAT32_EXT"),
                Self::FLOAT64_EXT => Some("FLOAT64_EXT"),
                Self::STRING_EXT => Some("STRING_EXT"),
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

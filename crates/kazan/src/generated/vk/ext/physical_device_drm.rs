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
    pub struct PhysicalDeviceDrmPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub has_primary: Bool32,
        pub has_render: Bool32,
        pub primary_major: i64,
        pub primary_minor: i64,
        pub render_major: i64,
        pub render_minor: i64,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDrmPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_DRM_PROPERTIES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>> for PhysicalDeviceDrmPropertiesEXT<'a> {}
    impl Default for PhysicalDeviceDrmPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                has_primary: Default::default(),
                has_render: Default::default(),
                primary_major: Default::default(),
                primary_minor: Default::default(),
                render_major: Default::default(),
                render_minor: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDrmPropertiesEXT<'a> {
        pub fn has_primary(mut self, has_primary: Bool32) -> Self {
            self.has_primary = has_primary;
            self
        }
        pub fn has_render(mut self, has_render: Bool32) -> Self {
            self.has_render = has_render;
            self
        }
        pub fn primary_major(mut self, primary_major: i64) -> Self {
            self.primary_major = primary_major;
            self
        }
        pub fn primary_minor(mut self, primary_minor: i64) -> Self {
            self.primary_minor = primary_minor;
            self
        }
        pub fn render_major(mut self, render_major: i64) -> Self {
            self.render_major = render_major;
            self
        }
        pub fn render_minor(mut self, render_minor: i64) -> Self {
            self.render_minor = render_minor;
            self
        }
    }
}

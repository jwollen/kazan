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
    pub struct PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub workgroup_memory_explicit_layout: Bool32,
        pub workgroup_memory_explicit_layout_scalar_block_layout: Bool32,
        pub workgroup_memory_explicit_layout8_bit_access: Bool32,
        pub workgroup_memory_explicit_layout16_bit_access: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type:
                    StructureType::PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR,
                p_next: core::ptr::null_mut(),
                workgroup_memory_explicit_layout: Default::default(),
                workgroup_memory_explicit_layout_scalar_block_layout: Default::default(),
                workgroup_memory_explicit_layout8_bit_access: Default::default(),
                workgroup_memory_explicit_layout16_bit_access: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'a> {
        pub fn workgroup_memory_explicit_layout(
            mut self,
            workgroup_memory_explicit_layout: Bool32,
        ) -> Self {
            self.workgroup_memory_explicit_layout = workgroup_memory_explicit_layout;
            self
        }
        pub fn workgroup_memory_explicit_layout_scalar_block_layout(
            mut self,
            workgroup_memory_explicit_layout_scalar_block_layout: Bool32,
        ) -> Self {
            self.workgroup_memory_explicit_layout_scalar_block_layout =
                workgroup_memory_explicit_layout_scalar_block_layout;
            self
        }
        pub fn workgroup_memory_explicit_layout8_bit_access(
            mut self,
            workgroup_memory_explicit_layout8_bit_access: Bool32,
        ) -> Self {
            self.workgroup_memory_explicit_layout8_bit_access =
                workgroup_memory_explicit_layout8_bit_access;
            self
        }
        pub fn workgroup_memory_explicit_layout16_bit_access(
            mut self,
            workgroup_memory_explicit_layout16_bit_access: Bool32,
        ) -> Self {
            self.workgroup_memory_explicit_layout16_bit_access =
                workgroup_memory_explicit_layout16_bit_access;
            self
        }
    }
}

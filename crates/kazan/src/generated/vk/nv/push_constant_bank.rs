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
    pub struct PushConstantBankInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub bank: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PushConstantBankInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PUSH_CONSTANT_BANK_INFO_NV,
                p_next: core::ptr::null(),
                bank: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PushConstantBankInfoNV<'a> {
        pub fn bank(mut self, bank: u32) -> Self {
            self.bank = bank;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePushConstantBankFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub push_constant_bank: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDevicePushConstantBankFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV,
                p_next: core::ptr::null_mut(),
                push_constant_bank: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePushConstantBankFeaturesNV<'a> {
        pub fn push_constant_bank(mut self, push_constant_bank: Bool32) -> Self {
            self.push_constant_bank = push_constant_bank;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePushConstantBankPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_graphics_push_constant_banks: u32,
        pub max_compute_push_constant_banks: u32,
        pub max_graphics_push_data_banks: u32,
        pub max_compute_push_data_banks: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDevicePushConstantBankPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV,
                p_next: core::ptr::null_mut(),
                max_graphics_push_constant_banks: Default::default(),
                max_compute_push_constant_banks: Default::default(),
                max_graphics_push_data_banks: Default::default(),
                max_compute_push_data_banks: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePushConstantBankPropertiesNV<'a> {
        pub fn max_graphics_push_constant_banks(
            mut self,
            max_graphics_push_constant_banks: u32,
        ) -> Self {
            self.max_graphics_push_constant_banks = max_graphics_push_constant_banks;
            self
        }
        pub fn max_compute_push_constant_banks(
            mut self,
            max_compute_push_constant_banks: u32,
        ) -> Self {
            self.max_compute_push_constant_banks = max_compute_push_constant_banks;
            self
        }
        pub fn max_graphics_push_data_banks(mut self, max_graphics_push_data_banks: u32) -> Self {
            self.max_graphics_push_data_banks = max_graphics_push_data_banks;
            self
        }
        pub fn max_compute_push_data_banks(mut self, max_compute_push_data_banks: u32) -> Self {
            self.max_compute_push_data_banks = max_compute_push_data_banks;
            self
        }
    }
}

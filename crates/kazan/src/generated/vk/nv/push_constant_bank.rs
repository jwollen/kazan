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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PushConstantBankInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub bank: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PushConstantBankInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PUSH_CONSTANT_BANK_INFO_NV;
    }
    unsafe impl<'a> Extends<DescriptorSetAndBindingMappingEXT<'a>> for PushConstantBankInfoNV<'a> {}
    unsafe impl<'a> Extends<PushDataInfoEXT<'a>> for PushConstantBankInfoNV<'a> {}
    unsafe impl<'a> Extends<PushConstantsInfo<'a>> for PushConstantBankInfoNV<'a> {}
    unsafe impl<'a> Extends<IndirectCommandsLayoutTokenEXT<'a>> for PushConstantBankInfoNV<'a> {}
    impl Default for PushConstantBankInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePushConstantBankFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePushConstantBankFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDevicePushConstantBankFeaturesNV<'a> {}
    impl Default for PhysicalDevicePushConstantBankFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePushConstantBankPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDevicePushConstantBankPropertiesNV<'a>
    {
    }
    impl Default for PhysicalDevicePushConstantBankPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

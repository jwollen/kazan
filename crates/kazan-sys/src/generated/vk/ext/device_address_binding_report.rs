#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceAddressBindingReportFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub report_address_binding: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceAddressBindingReportFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            report_address_binding: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceAddressBindingCallbackDataEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: DeviceAddressBindingFlagsEXT,
    pub base_address: DeviceAddress,
    pub size: DeviceSize,
    pub binding_type: DeviceAddressBindingTypeEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceAddressBindingCallbackDataEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT,
            p_next: core::ptr::null_mut(),
            flags: Default::default(),
            base_address: Default::default(),
            size: Default::default(),
            binding_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceAddressBindingTypeEXT(i32);
impl DeviceAddressBindingTypeEXT {
    pub const BIND_EXT: Self = Self(0);
    pub const UNBIND_EXT: Self = Self(1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DeviceAddressBindingFlagsEXT: Flags {
        const INTERNAL_OBJECT_EXT = DeviceAddressBindingFlagBitsEXT::INTERNAL_OBJECT_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceAddressBindingFlagBitsEXT(u32);
impl DeviceAddressBindingFlagBitsEXT {
    pub const INTERNAL_OBJECT_EXT: Self = Self(1 << 0);
}

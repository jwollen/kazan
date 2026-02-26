#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceAddressBindingReportFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub report_address_binding: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DeviceAddressBindingCallbackDataEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: DeviceAddressBindingFlagsEXT,
    pub base_address: DeviceAddress,
    pub size: DeviceSize,
    pub binding_type: DeviceAddressBindingTypeEXT,
    pub _marker: PhantomData<&'a ()>,
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

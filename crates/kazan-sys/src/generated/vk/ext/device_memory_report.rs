#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_memory_report: Bool32,
}
#[repr(C)]
pub struct DeviceDeviceMemoryReportCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceMemoryReportFlagsEXT,
    pub pfn_user_callback: Option<PFN_vkDeviceMemoryReportCallbackEXT>,
    pub p_user_data: *mut c_void,
}
#[repr(C)]
pub struct DeviceMemoryReportCallbackDataEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: DeviceMemoryReportFlagsEXT,
    pub ty: DeviceMemoryReportEventTypeEXT,
    pub memory_object_id: u64,
    pub size: DeviceSize,
    pub object_type: ObjectType,
    pub object_handle: u64,
    pub heap_index: u32,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceMemoryReportEventTypeEXT(i32);
impl DeviceMemoryReportEventTypeEXT {
    pub const ALLOCATE_EXT: Self = Self(0);
    pub const FREE_EXT: Self = Self(1);
    pub const IMPORT_EXT: Self = Self(2);
    pub const UNIMPORT_EXT: Self = Self(3);
    pub const ALLOCATION_FAILED_EXT: Self = Self(4);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DeviceMemoryReportFlagsEXT: Flags {
    }
}
pub type PFN_vkDeviceMemoryReportCallbackEXT = unsafe extern "system" fn(
    p_callback_data: *const DeviceMemoryReportCallbackDataEXT,
    p_user_data: *mut c_void,
);

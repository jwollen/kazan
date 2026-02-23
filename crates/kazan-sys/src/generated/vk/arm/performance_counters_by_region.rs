#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDevicePerformanceCountersByRegionFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub performance_counters_by_region: Bool32,
}
#[repr(C)]
pub struct PhysicalDevicePerformanceCountersByRegionPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_per_region_performance_counters: u32,
    pub performance_counter_region_size: Extent2D,
    pub row_stride_alignment: u32,
    pub region_alignment: u32,
    pub identity_transform_order: Bool32,
}
#[repr(C)]
pub struct PerformanceCounterARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub counter_id: u32,
}
#[repr(C)]
pub struct PerformanceCounterDescriptionARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: PerformanceCounterDescriptionFlagsARM,
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
}
#[repr(C)]
pub struct RenderPassPerformanceCountersByRegionBeginInfoARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub counter_address_count: u32,
    pub p_counter_addresses: *const DeviceAddress,
    pub serialize_regions: Bool32,
    pub counter_index_count: u32,
    pub p_counter_indices: *mut u32,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PerformanceCounterDescriptionFlagsARM: Flags {
    }
}
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut PerformanceCounterARM,
        p_counter_descriptions: *mut PerformanceCounterDescriptionARM,
    ) -> Result;

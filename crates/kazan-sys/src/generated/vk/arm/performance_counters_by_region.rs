#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDevicePerformanceCountersByRegionFeaturesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub performance_counters_by_region: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDevicePerformanceCountersByRegionPropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_per_region_performance_counters: u32,
    pub performance_counter_region_size: Extent2D,
    pub row_stride_alignment: u32,
    pub region_alignment: u32,
    pub identity_transform_order: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PerformanceCounterARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub counter_id: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PerformanceCounterDescriptionARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: PerformanceCounterDescriptionFlagsARM,
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct RenderPassPerformanceCountersByRegionBeginInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub counter_address_count: u32,
    pub p_counter_addresses: *const DeviceAddress,
    pub serialize_regions: Bool32,
    pub counter_index_count: u32,
    pub p_counter_indices: *mut u32,
    pub _marker: PhantomData<&'a ()>,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PerformanceCounterDescriptionFlagsARM: Flags {
    }
}
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut PerformanceCounterARM<'_>,
        p_counter_descriptions: *mut PerformanceCounterDescriptionARM<'_>,
    ) -> Result;

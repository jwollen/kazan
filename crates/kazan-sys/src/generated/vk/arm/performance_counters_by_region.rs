#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePerformanceCountersByRegionFeaturesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub performance_counters_by_region: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePerformanceCountersByRegionFeaturesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM,
            p_next: core::ptr::null_mut(),
            performance_counters_by_region: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for PhysicalDevicePerformanceCountersByRegionPropertiesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM,
            p_next: core::ptr::null_mut(),
            max_per_region_performance_counters: Default::default(),
            performance_counter_region_size: Default::default(),
            row_stride_alignment: Default::default(),
            region_alignment: Default::default(),
            identity_transform_order: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerformanceCounterARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub counter_id: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PerformanceCounterARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PERFORMANCE_COUNTER_ARM,
            p_next: core::ptr::null_mut(),
            counter_id: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerformanceCounterDescriptionARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: PerformanceCounterDescriptionFlagsARM,
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PerformanceCounterDescriptionARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PERFORMANCE_COUNTER_DESCRIPTION_ARM,
            p_next: core::ptr::null_mut(),
            flags: Default::default(),
            name: [Default::default(); _],
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for RenderPassPerformanceCountersByRegionBeginInfoARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM,
            p_next: core::ptr::null_mut(),
            counter_address_count: Default::default(),
            p_counter_addresses: core::ptr::null(),
            serialize_regions: Default::default(),
            counter_index_count: Default::default(),
            p_counter_indices: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
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

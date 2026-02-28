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
    impl<'a> PhysicalDevicePerformanceCountersByRegionFeaturesARM<'a> {
        pub fn performance_counters_by_region(
            mut self,
            performance_counters_by_region: Bool32,
        ) -> Self {
            self.performance_counters_by_region = performance_counters_by_region;
            self
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
                s_type:
                    StructureType::PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM,
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
    impl<'a> PhysicalDevicePerformanceCountersByRegionPropertiesARM<'a> {
        pub fn max_per_region_performance_counters(
            mut self,
            max_per_region_performance_counters: u32,
        ) -> Self {
            self.max_per_region_performance_counters = max_per_region_performance_counters;
            self
        }
        pub fn performance_counter_region_size(
            mut self,
            performance_counter_region_size: Extent2D,
        ) -> Self {
            self.performance_counter_region_size = performance_counter_region_size;
            self
        }
        pub fn row_stride_alignment(mut self, row_stride_alignment: u32) -> Self {
            self.row_stride_alignment = row_stride_alignment;
            self
        }
        pub fn region_alignment(mut self, region_alignment: u32) -> Self {
            self.region_alignment = region_alignment;
            self
        }
        pub fn identity_transform_order(mut self, identity_transform_order: Bool32) -> Self {
            self.identity_transform_order = identity_transform_order;
            self
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
    impl<'a> PerformanceCounterARM<'a> {
        pub fn counter_id(mut self, counter_id: u32) -> Self {
            self.counter_id = counter_id;
            self
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
    impl<'a> PerformanceCounterDescriptionARM<'a> {
        pub fn flags(mut self, flags: PerformanceCounterDescriptionFlagsARM) -> Self {
            self.flags = flags;
            self
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
    impl<'a> RenderPassPerformanceCountersByRegionBeginInfoARM<'a> {
        pub fn counter_address_count(mut self, counter_address_count: u32) -> Self {
            self.counter_address_count = counter_address_count;
            self
        }
        pub fn counter_addresses(mut self, counter_addresses: &'a DeviceAddress) -> Self {
            self.p_counter_addresses = counter_addresses;
            self
        }
        pub fn serialize_regions(mut self, serialize_regions: Bool32) -> Self {
            self.serialize_regions = serialize_regions;
            self
        }
        pub fn counter_index_count(mut self, counter_index_count: u32) -> Self {
            self.counter_index_count = counter_index_count;
            self
        }
        pub fn counter_indices(mut self, counter_indices: &'a mut u32) -> Self {
            self.p_counter_indices = counter_indices;
            self
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
        ) -> vk::Result;
}
pub struct InstanceFn {
    enumerate_physical_device_queue_family_performance_counters_by_region_arm:
        PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                enumerate_physical_device_queue_family_performance_counters_by_region_arm:
                    transmute(
                        load(c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM")
                            .ok_or(LoadingError)?,
                    ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn enumerate_physical_device_queue_family_performance_counters_by_region_arm<'a>(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        counters: impl ExtendUninit<PerformanceCounterARM<'a>>,
        counter_descriptions: impl ExtendUninit<PerformanceCounterDescriptionARM<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit2(
                counters,
                counter_descriptions,
                |counter_count, counters, counter_descriptions| {
                    let result = (self
                        .enumerate_physical_device_queue_family_performance_counters_by_region_arm)(
                        physical_device,
                        queue_family_index,
                        counter_count,
                        counters as _,
                        counter_descriptions as _,
                    );

                    match result {
                        VkResult::SUCCESS => Ok(()),
                        VkResult::INCOMPLETE => Ok(()),
                        err => Err(err),
                    }
                },
            )
        }
    }
}

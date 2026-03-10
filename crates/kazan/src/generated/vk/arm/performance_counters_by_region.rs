//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_ARM_performance_counters_by_region.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_ARM_performance_counters_by_region";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePerformanceCountersByRegionFeaturesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePerformanceCountersByRegionFeaturesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub performance_counters_by_region: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePerformanceCountersByRegionFeaturesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePerformanceCountersByRegionFeaturesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "performance_counters_by_region",
                    &self.performance_counters_by_region,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePerformanceCountersByRegionFeaturesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDevicePerformanceCountersByRegionFeaturesARM<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDevicePerformanceCountersByRegionFeaturesARM<'_>
    {
    }

    impl Default for PhysicalDevicePerformanceCountersByRegionFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                performance_counters_by_region: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePerformanceCountersByRegionFeaturesARM<'a> {
        #[inline]
        pub fn performance_counters_by_region(
            mut self,
            performance_counters_by_region: bool,
        ) -> Self {
            self.performance_counters_by_region = performance_counters_by_region.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePerformanceCountersByRegionPropertiesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePerformanceCountersByRegionPropertiesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePerformanceCountersByRegionPropertiesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_per_region_performance_counters",
                    &self.max_per_region_performance_counters,
                )
                .field(
                    "performance_counter_region_size",
                    &self.performance_counter_region_size,
                )
                .field("row_stride_alignment", &self.row_stride_alignment)
                .field("region_alignment", &self.region_alignment)
                .field("identity_transform_order", &self.identity_transform_order)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePerformanceCountersByRegionPropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDevicePerformanceCountersByRegionPropertiesARM<'_>
    {
    }

    impl Default for PhysicalDevicePerformanceCountersByRegionPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
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
        #[inline]
        pub fn max_per_region_performance_counters(
            mut self,
            max_per_region_performance_counters: u32,
        ) -> Self {
            self.max_per_region_performance_counters = max_per_region_performance_counters;
            self
        }

        #[inline]
        pub fn performance_counter_region_size(
            mut self,
            performance_counter_region_size: Extent2D,
        ) -> Self {
            self.performance_counter_region_size = performance_counter_region_size;
            self
        }

        #[inline]
        pub fn row_stride_alignment(mut self, row_stride_alignment: u32) -> Self {
            self.row_stride_alignment = row_stride_alignment;
            self
        }

        #[inline]
        pub fn region_alignment(mut self, region_alignment: u32) -> Self {
            self.region_alignment = region_alignment;
            self
        }

        #[inline]
        pub fn identity_transform_order(mut self, identity_transform_order: bool) -> Self {
            self.identity_transform_order = identity_transform_order.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceCounterARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PerformanceCounterARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub counter_id: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PerformanceCounterARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PerformanceCounterARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("counter_id", &self.counter_id)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PerformanceCounterARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PERFORMANCE_COUNTER_ARM;
    }

    impl Default for PerformanceCounterARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                counter_id: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PerformanceCounterARM<'a> {
        #[inline]
        pub fn counter_id(mut self, counter_id: u32) -> Self {
            self.counter_id = counter_id;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceCounterDescriptionARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PerformanceCounterDescriptionARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: PerformanceCounterDescriptionFlagsARM,
        pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PerformanceCounterDescriptionARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PerformanceCounterDescriptionARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("name", &wrap_c_str_slice_until_nul(&self.name))
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PerformanceCounterDescriptionARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PERFORMANCE_COUNTER_DESCRIPTION_ARM;
    }

    impl Default for PerformanceCounterDescriptionARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                flags: Default::default(),
                name: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PerformanceCounterDescriptionARM<'a> {
        #[inline]
        pub fn flags(mut self, flags: PerformanceCounterDescriptionFlagsARM) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn name(
            mut self,
            name: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            write_c_str_slice_with_nul(&mut self.name, name)?;
            Ok(self)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassPerformanceCountersByRegionBeginInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderPassPerformanceCountersByRegionBeginInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderPassPerformanceCountersByRegionBeginInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("counter_address_count", &self.counter_address_count)
                .field("p_counter_addresses", &self.p_counter_addresses)
                .field("serialize_regions", &self.serialize_regions)
                .field("counter_index_count", &self.counter_index_count)
                .field("p_counter_indices", &self.p_counter_indices)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderPassPerformanceCountersByRegionBeginInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM;
    }

    unsafe impl Extends<RenderPassBeginInfo<'_>>
        for RenderPassPerformanceCountersByRegionBeginInfoARM<'_>
    {
    }
    unsafe impl Extends<RenderingInfo<'_>> for RenderPassPerformanceCountersByRegionBeginInfoARM<'_> {}

    impl Default for RenderPassPerformanceCountersByRegionBeginInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                counter_address_count: Default::default(),
                p_counter_addresses: ptr::null(),
                serialize_regions: Default::default(),
                counter_index_count: Default::default(),
                p_counter_indices: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RenderPassPerformanceCountersByRegionBeginInfoARM<'a> {
        #[inline]
        pub fn counter_addresses(mut self, counter_addresses: &'a [DeviceAddress]) -> Self {
            self.counter_address_count = counter_addresses.len().try_into().unwrap();
            self.p_counter_addresses = counter_addresses.as_ptr() as _;
            self
        }

        #[inline]
        pub fn serialize_regions(mut self, serialize_regions: bool) -> Self {
            self.serialize_regions = serialize_regions.into();
            self
        }

        #[inline]
        pub fn counter_indices(mut self, counter_indices: &'a mut [u32]) -> Self {
            self.counter_index_count = counter_indices.len().try_into().unwrap();
            self.p_counter_indices = counter_indices.as_mut_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceCounterDescriptionFlagsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PerformanceCounterDescriptionFlagsARM(Flags);
    vk_bitflags_wrapped!(PerformanceCounterDescriptionFlagsARM, Flags);

    impl fmt::Debug for PerformanceCounterDescriptionFlagsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM.html>
    pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
            p_counter_count: *mut u32,
            p_counters: *mut PerformanceCounterARM<'_>,
            p_counter_descriptions: *mut PerformanceCounterDescriptionARM<'_>,
        ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDevicePerformanceCountersByRegionFeaturesARM =
        PhysicalDevicePerformanceCountersByRegionFeaturesARM<'static>;
    pub type VkPhysicalDevicePerformanceCountersByRegionPropertiesARM =
        PhysicalDevicePerformanceCountersByRegionPropertiesARM<'static>;
    pub type VkPerformanceCounterARM = PerformanceCounterARM<'static>;
    pub type VkPerformanceCounterDescriptionARM = PerformanceCounterDescriptionARM<'static>;
    pub type VkRenderPassPerformanceCountersByRegionBeginInfoARM =
        RenderPassPerformanceCountersByRegionBeginInfoARM<'static>;
    pub type VkPerformanceCounterDescriptionFlagsARM = PerformanceCounterDescriptionFlagsARM;
    impl PhysicalDevicePerformanceCountersByRegionFeaturesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDevicePerformanceCountersByRegionFeaturesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDevicePerformanceCountersByRegionPropertiesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDevicePerformanceCountersByRegionPropertiesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PerformanceCounterARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPerformanceCounterARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PerformanceCounterDescriptionARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPerformanceCounterDescriptionARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl RenderPassPerformanceCountersByRegionBeginInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkRenderPassPerformanceCountersByRegionBeginInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    enumerate_physical_device_queue_family_performance_counters_by_region_arm:
        PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                enumerate_physical_device_queue_family_performance_counters_by_region_arm:
                    transmute(
                        load(c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM")
                            .ok_or(MissingEntryPointError)?,
                    ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM.html>
    #[inline]
    pub unsafe fn enumerate_physical_device_queue_family_performance_counters_by_region_arm<'a>(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        mut counters: impl ExtendUninit<PerformanceCounterARM<'a>>,
        mut counter_descriptions: impl ExtendUninit<PerformanceCounterDescriptionARM<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |counter_count, counters, counter_descriptions| {
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
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut(), std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let counters_buf = counters.reserve(capacity);
            len = counters_buf.len().try_into().unwrap();
            let counter_descriptions_buf = counter_descriptions.reserve(capacity);
            assert_eq!(counter_descriptions_buf.len(), counters_buf.len());
            let result = call(
                &mut len,
                counters_buf.as_mut_ptr() as *mut _,
                counter_descriptions_buf.as_mut_ptr() as *mut _,
            )?;
            counters.set_len(len.try_into().unwrap());
            counter_descriptions.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}

#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_performance_query";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePerformanceQueryFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePerformanceQueryFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub performance_counter_query_pools: Bool32,
        pub performance_counter_multiple_query_pools: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePerformanceQueryFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePerformanceQueryFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "performance_counter_query_pools",
                    &self.performance_counter_query_pools,
                )
                .field(
                    "performance_counter_multiple_query_pools",
                    &self.performance_counter_multiple_query_pools,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePerformanceQueryFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePerformanceQueryFeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDevicePerformanceQueryFeaturesKHR<'a> {}

    impl Default for PhysicalDevicePerformanceQueryFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                performance_counter_query_pools: Default::default(),
                performance_counter_multiple_query_pools: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePerformanceQueryFeaturesKHR<'a> {
        pub fn performance_counter_query_pools(
            mut self,
            performance_counter_query_pools: bool,
        ) -> Self {
            self.performance_counter_query_pools = performance_counter_query_pools.into();
            self
        }

        pub fn performance_counter_multiple_query_pools(
            mut self,
            performance_counter_multiple_query_pools: bool,
        ) -> Self {
            self.performance_counter_multiple_query_pools =
                performance_counter_multiple_query_pools.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePerformanceQueryPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePerformanceQueryPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub allow_command_buffer_query_copies: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePerformanceQueryPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePerformanceQueryPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "allow_command_buffer_query_copies",
                    &self.allow_command_buffer_query_copies,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePerformanceQueryPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDevicePerformanceQueryPropertiesKHR<'a>
    {
    }

    impl Default for PhysicalDevicePerformanceQueryPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                allow_command_buffer_query_copies: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePerformanceQueryPropertiesKHR<'a> {
        pub fn allow_command_buffer_query_copies(
            mut self,
            allow_command_buffer_query_copies: bool,
        ) -> Self {
            self.allow_command_buffer_query_copies = allow_command_buffer_query_copies.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceCounterKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PerformanceCounterKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub unit: PerformanceCounterUnitKHR,
        pub scope: PerformanceCounterScopeKHR,
        pub storage: PerformanceCounterStorageKHR,
        pub uuid: [u8; UUID_SIZE as usize],
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PerformanceCounterKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PerformanceCounterKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("unit", &self.unit)
                .field("scope", &self.scope)
                .field("storage", &self.storage)
                .field("uuid", &self.uuid)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PerformanceCounterKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PERFORMANCE_COUNTER_KHR;
    }

    impl Default for PerformanceCounterKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                unit: Default::default(),
                scope: Default::default(),
                storage: Default::default(),
                uuid: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PerformanceCounterKHR<'a> {
        pub fn unit(mut self, unit: PerformanceCounterUnitKHR) -> Self {
            self.unit = unit;
            self
        }

        pub fn scope(mut self, scope: PerformanceCounterScopeKHR) -> Self {
            self.scope = scope;
            self
        }

        pub fn storage(mut self, storage: PerformanceCounterStorageKHR) -> Self {
            self.storage = storage;
            self
        }

        pub fn uuid(mut self, uuid: [u8; UUID_SIZE as usize]) -> Self {
            self.uuid = uuid;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceCounterDescriptionKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PerformanceCounterDescriptionKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: PerformanceCounterDescriptionFlagsKHR,
        pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
        pub category: [c_char; MAX_DESCRIPTION_SIZE as usize],
        pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PerformanceCounterDescriptionKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PerformanceCounterDescriptionKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("name", &wrap_c_str_slice_until_nul(&self.name))
                .field("category", &wrap_c_str_slice_until_nul(&self.category))
                .field(
                    "description",
                    &wrap_c_str_slice_until_nul(&self.description),
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PerformanceCounterDescriptionKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PERFORMANCE_COUNTER_DESCRIPTION_KHR;
    }

    impl Default for PerformanceCounterDescriptionKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                flags: Default::default(),
                name: [Default::default(); _],
                category: [Default::default(); _],
                description: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PerformanceCounterDescriptionKHR<'a> {
        pub fn flags(mut self, flags: PerformanceCounterDescriptionFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        pub fn name(
            mut self,
            name: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            write_c_str_slice_with_nul(&mut self.name, name)?;
            Ok(self)
        }

        pub fn category(
            mut self,
            category: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            write_c_str_slice_with_nul(&mut self.category, category)?;
            Ok(self)
        }

        pub fn description(
            mut self,
            description: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            write_c_str_slice_with_nul(&mut self.description, description)?;
            Ok(self)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueryPoolPerformanceCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct QueryPoolPerformanceCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub queue_family_index: u32,
        pub counter_index_count: u32,
        pub p_counter_indices: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for QueryPoolPerformanceCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("QueryPoolPerformanceCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("queue_family_index", &self.queue_family_index)
                .field("counter_index_count", &self.counter_index_count)
                .field("p_counter_indices", &self.p_counter_indices)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for QueryPoolPerformanceCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR;
    }

    unsafe impl<'a> Extends<QueryPoolCreateInfo<'a>> for QueryPoolPerformanceCreateInfoKHR<'a> {}

    impl Default for QueryPoolPerformanceCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                queue_family_index: Default::default(),
                counter_index_count: Default::default(),
                p_counter_indices: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> QueryPoolPerformanceCreateInfoKHR<'a> {
        pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
            self.queue_family_index = queue_family_index;
            self
        }

        pub fn counter_indices(mut self, counter_indices: &'a [u32]) -> Self {
            self.counter_index_count = counter_indices.len().try_into().unwrap();
            self.p_counter_indices = counter_indices.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAcquireProfilingLockInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AcquireProfilingLockInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: AcquireProfilingLockFlagsKHR,
        pub timeout: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AcquireProfilingLockInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AcquireProfilingLockInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("timeout", &self.timeout)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AcquireProfilingLockInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ACQUIRE_PROFILING_LOCK_INFO_KHR;
    }

    impl Default for AcquireProfilingLockInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                timeout: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AcquireProfilingLockInfoKHR<'a> {
        pub fn flags(mut self, flags: AcquireProfilingLockFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        pub fn timeout(mut self, timeout: u64) -> Self {
            self.timeout = timeout;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceQuerySubmitInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PerformanceQuerySubmitInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub counter_pass_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PerformanceQuerySubmitInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PerformanceQuerySubmitInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("counter_pass_index", &self.counter_pass_index)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PerformanceQuerySubmitInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PERFORMANCE_QUERY_SUBMIT_INFO_KHR;
    }

    unsafe impl<'a> Extends<SubmitInfo<'a>> for PerformanceQuerySubmitInfoKHR<'a> {}
    unsafe impl<'a> Extends<SubmitInfo2<'a>> for PerformanceQuerySubmitInfoKHR<'a> {}

    impl Default for PerformanceQuerySubmitInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                counter_pass_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PerformanceQuerySubmitInfoKHR<'a> {
        pub fn counter_pass_index(mut self, counter_pass_index: u32) -> Self {
            self.counter_pass_index = counter_pass_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceCounterResultKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union PerformanceCounterResultKHR {
        pub int32: i32,
        pub int64: i64,
        pub uint32: u32,
        pub uint64: u64,
        pub float32: f32,
        pub float64: f64,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PerformanceCounterResultKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PerformanceCounterResultKHR").finish()
        }
    }

    impl Default for PerformanceCounterResultKHR {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceCounterScopeKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PerformanceCounterScopeKHR(i32);

    impl PerformanceCounterScopeKHR {
        pub const COMMAND_BUFFER_KHR: Self = Self(0);
        pub const RENDER_PASS_KHR: Self = Self(1);
        pub const COMMAND_KHR: Self = Self(2);
    }

    impl fmt::Debug for PerformanceCounterScopeKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::COMMAND_BUFFER_KHR => Some("COMMAND_BUFFER_KHR"),
                Self::RENDER_PASS_KHR => Some("RENDER_PASS_KHR"),
                Self::COMMAND_KHR => Some("COMMAND_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceCounterUnitKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PerformanceCounterUnitKHR(i32);

    impl PerformanceCounterUnitKHR {
        pub const GENERIC_KHR: Self = Self(0);
        pub const PERCENTAGE_KHR: Self = Self(1);
        pub const NANOSECONDS_KHR: Self = Self(2);
        pub const BYTES_KHR: Self = Self(3);
        pub const BYTES_PER_SECOND_KHR: Self = Self(4);
        pub const KELVIN_KHR: Self = Self(5);
        pub const WATTS_KHR: Self = Self(6);
        pub const VOLTS_KHR: Self = Self(7);
        pub const AMPS_KHR: Self = Self(8);
        pub const HERTZ_KHR: Self = Self(9);
        pub const CYCLES_KHR: Self = Self(10);
    }

    impl fmt::Debug for PerformanceCounterUnitKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::GENERIC_KHR => Some("GENERIC_KHR"),
                Self::PERCENTAGE_KHR => Some("PERCENTAGE_KHR"),
                Self::NANOSECONDS_KHR => Some("NANOSECONDS_KHR"),
                Self::BYTES_KHR => Some("BYTES_KHR"),
                Self::BYTES_PER_SECOND_KHR => Some("BYTES_PER_SECOND_KHR"),
                Self::KELVIN_KHR => Some("KELVIN_KHR"),
                Self::WATTS_KHR => Some("WATTS_KHR"),
                Self::VOLTS_KHR => Some("VOLTS_KHR"),
                Self::AMPS_KHR => Some("AMPS_KHR"),
                Self::HERTZ_KHR => Some("HERTZ_KHR"),
                Self::CYCLES_KHR => Some("CYCLES_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceCounterStorageKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PerformanceCounterStorageKHR(i32);

    impl PerformanceCounterStorageKHR {
        pub const INT32_KHR: Self = Self(0);
        pub const INT64_KHR: Self = Self(1);
        pub const UINT32_KHR: Self = Self(2);
        pub const UINT64_KHR: Self = Self(3);
        pub const FLOAT32_KHR: Self = Self(4);
        pub const FLOAT64_KHR: Self = Self(5);
    }

    impl fmt::Debug for PerformanceCounterStorageKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::INT32_KHR => Some("INT32_KHR"),
                Self::INT64_KHR => Some("INT64_KHR"),
                Self::UINT32_KHR => Some("UINT32_KHR"),
                Self::UINT64_KHR => Some("UINT64_KHR"),
                Self::FLOAT32_KHR => Some("FLOAT32_KHR"),
                Self::FLOAT64_KHR => Some("FLOAT64_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceCounterDescriptionFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PerformanceCounterDescriptionFlagsKHR(Flags);
    vk_bitflags_wrapped!(PerformanceCounterDescriptionFlagsKHR, Flags);

    impl PerformanceCounterDescriptionFlagsKHR {
        pub const PERFORMANCE_IMPACTING_KHR: Self =
            Self(PerformanceCounterDescriptionFlagBitsKHR::PERFORMANCE_IMPACTING_KHR.0);
        pub const CONCURRENTLY_IMPACTED_KHR: Self =
            Self(PerformanceCounterDescriptionFlagBitsKHR::CONCURRENTLY_IMPACTED_KHR.0);
    }

    impl fmt::Debug for PerformanceCounterDescriptionFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    PerformanceCounterDescriptionFlagsKHR::PERFORMANCE_IMPACTING_KHR.0,
                    "PERFORMANCE_IMPACTING_KHR",
                ),
                (
                    PerformanceCounterDescriptionFlagsKHR::CONCURRENTLY_IMPACTED_KHR.0,
                    "CONCURRENTLY_IMPACTED_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceCounterDescriptionFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PerformanceCounterDescriptionFlagBitsKHR(u32);

    impl PerformanceCounterDescriptionFlagBitsKHR {
        pub const PERFORMANCE_IMPACTING_KHR: Self = Self(1 << 0);
        pub const CONCURRENTLY_IMPACTED_KHR: Self = Self(1 << 1);
    }

    impl fmt::Debug for PerformanceCounterDescriptionFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::PERFORMANCE_IMPACTING_KHR => Some("PERFORMANCE_IMPACTING_KHR"),
                Self::CONCURRENTLY_IMPACTED_KHR => Some("CONCURRENTLY_IMPACTED_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAcquireProfilingLockFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct AcquireProfilingLockFlagsKHR(Flags);
    vk_bitflags_wrapped!(AcquireProfilingLockFlagsKHR, Flags);

    impl AcquireProfilingLockFlagsKHR {}

    impl fmt::Debug for AcquireProfilingLockFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAcquireProfilingLockFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct AcquireProfilingLockFlagBitsKHR(u32);

    impl AcquireProfilingLockFlagBitsKHR {}

    impl fmt::Debug for AcquireProfilingLockFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html>
    pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
            p_counter_count: *mut u32,
            p_counters: *mut PerformanceCounterKHR<'_>,
            p_counter_descriptions: *mut PerformanceCounterDescriptionKHR<'_>,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html>
    pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR<'_>,
            p_num_passes: *mut u32,
        );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAcquireProfilingLockKHR.html>
    pub type PFN_vkAcquireProfilingLockKHR = unsafe extern "system" fn(
        device: Device,
        p_info: *const AcquireProfilingLockInfoKHR<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkReleaseProfilingLockKHR.html>
    pub type PFN_vkReleaseProfilingLockKHR = unsafe extern "system" fn(device: Device);
}

pub struct InstanceFn {
    enumerate_physical_device_queue_family_performance_query_counters_khr:
        PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR,
    get_physical_device_queue_family_performance_query_passes_khr:
        PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                enumerate_physical_device_queue_family_performance_query_counters_khr: transmute(
                    load(c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_queue_family_performance_query_passes_khr: transmute(
                    load(c"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html>
    pub unsafe fn enumerate_physical_device_queue_family_performance_query_counters_khr<'a>(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        mut counters: impl ExtendUninit<PerformanceCounterKHR<'a>>,
        mut counter_descriptions: impl ExtendUninit<PerformanceCounterDescriptionKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |counter_count, counters, counter_descriptions| {
                let result = (self
                    .enumerate_physical_device_queue_family_performance_query_counters_khr)(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html>
    pub unsafe fn get_physical_device_queue_family_performance_query_passes_khr(
        &self,
        physical_device: PhysicalDevice,
        performance_query_create_info: &QueryPoolPerformanceCreateInfoKHR<'_>,
    ) -> u32 {
        unsafe {
            let mut num_passes = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_queue_family_performance_query_passes_khr)(
                physical_device,
                performance_query_create_info,
                num_passes.as_mut_ptr(),
            );
            num_passes.assume_init()
        }
    }
}

pub struct DeviceFn {
    acquire_profiling_lock_khr: PFN_vkAcquireProfilingLockKHR,
    release_profiling_lock_khr: PFN_vkReleaseProfilingLockKHR,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                acquire_profiling_lock_khr: transmute(
                    load(c"vkAcquireProfilingLockKHR").ok_or(MissingEntryPointError)?,
                ),
                release_profiling_lock_khr: transmute(
                    load(c"vkReleaseProfilingLockKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAcquireProfilingLockKHR.html>
    pub unsafe fn acquire_profiling_lock_khr(
        &self,
        device: Device,
        info: &AcquireProfilingLockInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.acquire_profiling_lock_khr)(device, info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkReleaseProfilingLockKHR.html>
    pub unsafe fn release_profiling_lock_khr(&self, device: Device) {
        unsafe { (self.release_profiling_lock_khr)(device) }
    }
}

#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_INTEL_performance_query";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    handle_nondispatchable!(
        PerformanceConfigurationINTEL,
        PERFORMANCE_CONFIGURATION_INTEL,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceConfigurationINTEL.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueryPoolCreateInfoINTEL.html>
    pub type QueryPoolCreateInfoINTEL<'a> = QueryPoolPerformanceQueryCreateInfoINTEL<'a>;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceValueINTEL.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PerformanceValueINTEL<'a> {
        pub ty: PerformanceValueTypeINTEL,
        pub data: PerformanceValueDataINTEL<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PerformanceValueINTEL<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PerformanceValueINTEL")
                .field("ty", &self.ty)
                .field("data", &self.data)
                .finish()
        }
    }

    impl Default for PerformanceValueINTEL<'_> {
        fn default() -> Self {
            Self {
                ty: Default::default(),
                data: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PerformanceValueINTEL<'a> {
        pub fn ty(mut self, ty: PerformanceValueTypeINTEL) -> Self {
            self.ty = ty;
            self
        }

        pub fn data(mut self, data: PerformanceValueDataINTEL<'a>) -> Self {
            self.data = data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkInitializePerformanceApiInfoINTEL.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct InitializePerformanceApiInfoINTEL<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_user_data: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for InitializePerformanceApiInfoINTEL<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("InitializePerformanceApiInfoINTEL")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_user_data", &self.p_user_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for InitializePerformanceApiInfoINTEL<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::INITIALIZE_PERFORMANCE_API_INFO_INTEL;
    }

    impl Default for InitializePerformanceApiInfoINTEL<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_user_data: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> InitializePerformanceApiInfoINTEL<'a> {
        pub fn user_data(mut self, user_data: *mut c_void) -> Self {
            self.p_user_data = user_data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueryPoolPerformanceQueryCreateInfoINTEL.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct QueryPoolPerformanceQueryCreateInfoINTEL<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub performance_counters_sampling: QueryPoolSamplingModeINTEL,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for QueryPoolPerformanceQueryCreateInfoINTEL<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("QueryPoolPerformanceQueryCreateInfoINTEL")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "performance_counters_sampling",
                    &self.performance_counters_sampling,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for QueryPoolPerformanceQueryCreateInfoINTEL<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL;
    }

    unsafe impl<'a> Extends<QueryPoolCreateInfo<'a>> for QueryPoolPerformanceQueryCreateInfoINTEL<'a> {}

    impl Default for QueryPoolPerformanceQueryCreateInfoINTEL<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                performance_counters_sampling: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> QueryPoolPerformanceQueryCreateInfoINTEL<'a> {
        pub fn performance_counters_sampling(
            mut self,
            performance_counters_sampling: QueryPoolSamplingModeINTEL,
        ) -> Self {
            self.performance_counters_sampling = performance_counters_sampling;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceMarkerInfoINTEL.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PerformanceMarkerInfoINTEL<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub marker: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PerformanceMarkerInfoINTEL<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PerformanceMarkerInfoINTEL")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("marker", &self.marker)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PerformanceMarkerInfoINTEL<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PERFORMANCE_MARKER_INFO_INTEL;
    }

    impl Default for PerformanceMarkerInfoINTEL<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                marker: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PerformanceMarkerInfoINTEL<'a> {
        pub fn marker(mut self, marker: u64) -> Self {
            self.marker = marker;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceStreamMarkerInfoINTEL.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PerformanceStreamMarkerInfoINTEL<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub marker: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PerformanceStreamMarkerInfoINTEL<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PerformanceStreamMarkerInfoINTEL")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("marker", &self.marker)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PerformanceStreamMarkerInfoINTEL<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PERFORMANCE_STREAM_MARKER_INFO_INTEL;
    }

    impl Default for PerformanceStreamMarkerInfoINTEL<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                marker: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PerformanceStreamMarkerInfoINTEL<'a> {
        pub fn marker(mut self, marker: u32) -> Self {
            self.marker = marker;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceOverrideInfoINTEL.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PerformanceOverrideInfoINTEL<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub ty: PerformanceOverrideTypeINTEL,
        pub enable: Bool32,
        pub parameter: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PerformanceOverrideInfoINTEL<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PerformanceOverrideInfoINTEL")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("ty", &self.ty)
                .field("enable", &self.enable)
                .field("parameter", &self.parameter)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PerformanceOverrideInfoINTEL<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PERFORMANCE_OVERRIDE_INFO_INTEL;
    }

    impl Default for PerformanceOverrideInfoINTEL<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                ty: Default::default(),
                enable: Default::default(),
                parameter: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PerformanceOverrideInfoINTEL<'a> {
        pub fn ty(mut self, ty: PerformanceOverrideTypeINTEL) -> Self {
            self.ty = ty;
            self
        }

        pub fn enable(mut self, enable: bool) -> Self {
            self.enable = enable.into();
            self
        }

        pub fn parameter(mut self, parameter: u64) -> Self {
            self.parameter = parameter;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceConfigurationAcquireInfoINTEL.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PerformanceConfigurationAcquireInfoINTEL<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub ty: PerformanceConfigurationTypeINTEL,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PerformanceConfigurationAcquireInfoINTEL<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PerformanceConfigurationAcquireInfoINTEL")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("ty", &self.ty)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PerformanceConfigurationAcquireInfoINTEL<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL;
    }

    impl Default for PerformanceConfigurationAcquireInfoINTEL<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                ty: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PerformanceConfigurationAcquireInfoINTEL<'a> {
        pub fn ty(mut self, ty: PerformanceConfigurationTypeINTEL) -> Self {
            self.ty = ty;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceValueDataINTEL.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union PerformanceValueDataINTEL<'a> {
        pub value32: u32,
        pub value64: u64,
        pub value_float: f32,
        pub value_bool: Bool32,
        pub value_string: *const c_char,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PerformanceValueDataINTEL<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PerformanceValueDataINTEL").finish()
        }
    }

    impl Default for PerformanceValueDataINTEL<'_> {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceConfigurationTypeINTEL.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PerformanceConfigurationTypeINTEL(i32);

    impl PerformanceConfigurationTypeINTEL {
        pub const COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: Self = Self(0);
    }

    impl fmt::Debug for PerformanceConfigurationTypeINTEL {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL => {
                    Some("COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL")
                }
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueryPoolSamplingModeINTEL.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct QueryPoolSamplingModeINTEL(i32);

    impl QueryPoolSamplingModeINTEL {
        pub const MANUAL_INTEL: Self = Self(0);
    }

    impl fmt::Debug for QueryPoolSamplingModeINTEL {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::MANUAL_INTEL => Some("MANUAL_INTEL"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceOverrideTypeINTEL.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PerformanceOverrideTypeINTEL(i32);

    impl PerformanceOverrideTypeINTEL {
        pub const NULL_HARDWARE_INTEL: Self = Self(0);
        pub const FLUSH_GPU_CACHES_INTEL: Self = Self(1);
    }

    impl fmt::Debug for PerformanceOverrideTypeINTEL {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::NULL_HARDWARE_INTEL => Some("NULL_HARDWARE_INTEL"),
                Self::FLUSH_GPU_CACHES_INTEL => Some("FLUSH_GPU_CACHES_INTEL"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceParameterTypeINTEL.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PerformanceParameterTypeINTEL(i32);

    impl PerformanceParameterTypeINTEL {
        pub const HW_COUNTERS_SUPPORTED_INTEL: Self = Self(0);
        pub const STREAM_MARKER_VALID_BITS_INTEL: Self = Self(1);
    }

    impl fmt::Debug for PerformanceParameterTypeINTEL {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::HW_COUNTERS_SUPPORTED_INTEL => Some("HW_COUNTERS_SUPPORTED_INTEL"),
                Self::STREAM_MARKER_VALID_BITS_INTEL => Some("STREAM_MARKER_VALID_BITS_INTEL"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPerformanceValueTypeINTEL.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PerformanceValueTypeINTEL(i32);

    impl PerformanceValueTypeINTEL {
        pub const UINT32_INTEL: Self = Self(0);
        pub const UINT64_INTEL: Self = Self(1);
        pub const FLOAT_INTEL: Self = Self(2);
        pub const BOOL_INTEL: Self = Self(3);
        pub const STRING_INTEL: Self = Self(4);
    }

    impl fmt::Debug for PerformanceValueTypeINTEL {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::UINT32_INTEL => Some("UINT32_INTEL"),
                Self::UINT64_INTEL => Some("UINT64_INTEL"),
                Self::FLOAT_INTEL => Some("FLOAT_INTEL"),
                Self::BOOL_INTEL => Some("BOOL_INTEL"),
                Self::STRING_INTEL => Some("STRING_INTEL"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkInitializePerformanceApiINTEL.html>
    pub type PFN_vkInitializePerformanceApiINTEL = unsafe extern "system" fn(
        device: Device,
        p_initialize_info: *const InitializePerformanceApiInfoINTEL<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUninitializePerformanceApiINTEL.html>
    pub type PFN_vkUninitializePerformanceApiINTEL = unsafe extern "system" fn(device: Device);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPerformanceMarkerINTEL.html>
    pub type PFN_vkCmdSetPerformanceMarkerINTEL = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const PerformanceMarkerInfoINTEL<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html>
    pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const PerformanceStreamMarkerInfoINTEL<'_>,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPerformanceOverrideINTEL.html>
    pub type PFN_vkCmdSetPerformanceOverrideINTEL = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_override_info: *const PerformanceOverrideInfoINTEL<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAcquirePerformanceConfigurationINTEL.html>
    pub type PFN_vkAcquirePerformanceConfigurationINTEL = unsafe extern "system" fn(
        device: Device,
        p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL<'_>,
        p_configuration: *mut PerformanceConfigurationINTEL,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkReleasePerformanceConfigurationINTEL.html>
    pub type PFN_vkReleasePerformanceConfigurationINTEL = unsafe extern "system" fn(
        device: Device,
        configuration: PerformanceConfigurationINTEL,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueSetPerformanceConfigurationINTEL.html>
    pub type PFN_vkQueueSetPerformanceConfigurationINTEL = unsafe extern "system" fn(
        queue: Queue,
        configuration: PerformanceConfigurationINTEL,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPerformanceParameterINTEL.html>
    pub type PFN_vkGetPerformanceParameterINTEL = unsafe extern "system" fn(
        device: Device,
        parameter: PerformanceParameterTypeINTEL,
        p_value: *mut PerformanceValueINTEL<'_>,
    ) -> vk::Result;
}

pub struct DeviceFn {
    initialize_performance_api_intel: PFN_vkInitializePerformanceApiINTEL,
    uninitialize_performance_api_intel: PFN_vkUninitializePerformanceApiINTEL,
    cmd_set_performance_marker_intel: PFN_vkCmdSetPerformanceMarkerINTEL,
    cmd_set_performance_stream_marker_intel: PFN_vkCmdSetPerformanceStreamMarkerINTEL,
    cmd_set_performance_override_intel: PFN_vkCmdSetPerformanceOverrideINTEL,
    acquire_performance_configuration_intel: PFN_vkAcquirePerformanceConfigurationINTEL,
    release_performance_configuration_intel: PFN_vkReleasePerformanceConfigurationINTEL,
    queue_set_performance_configuration_intel: PFN_vkQueueSetPerformanceConfigurationINTEL,
    get_performance_parameter_intel: PFN_vkGetPerformanceParameterINTEL,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                initialize_performance_api_intel: transmute(
                    load(c"vkInitializePerformanceApiINTEL").ok_or(MissingEntryPointError)?,
                ),
                uninitialize_performance_api_intel: transmute(
                    load(c"vkUninitializePerformanceApiINTEL").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_performance_marker_intel: transmute(
                    load(c"vkCmdSetPerformanceMarkerINTEL").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_performance_stream_marker_intel: transmute(
                    load(c"vkCmdSetPerformanceStreamMarkerINTEL").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_performance_override_intel: transmute(
                    load(c"vkCmdSetPerformanceOverrideINTEL").ok_or(MissingEntryPointError)?,
                ),
                acquire_performance_configuration_intel: transmute(
                    load(c"vkAcquirePerformanceConfigurationINTEL")
                        .ok_or(MissingEntryPointError)?,
                ),
                release_performance_configuration_intel: transmute(
                    load(c"vkReleasePerformanceConfigurationINTEL")
                        .ok_or(MissingEntryPointError)?,
                ),
                queue_set_performance_configuration_intel: transmute(
                    load(c"vkQueueSetPerformanceConfigurationINTEL")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_performance_parameter_intel: transmute(
                    load(c"vkGetPerformanceParameterINTEL").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkInitializePerformanceApiINTEL.html>
    pub unsafe fn initialize_performance_api_intel(
        &self,
        device: Device,
        initialize_info: &InitializePerformanceApiInfoINTEL<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.initialize_performance_api_intel)(device, initialize_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUninitializePerformanceApiINTEL.html>
    pub unsafe fn uninitialize_performance_api_intel(&self, device: Device) {
        unsafe { (self.uninitialize_performance_api_intel)(device) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPerformanceMarkerINTEL.html>
    pub unsafe fn cmd_set_performance_marker_intel(
        &self,
        command_buffer: CommandBuffer,
        marker_info: &PerformanceMarkerInfoINTEL<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.cmd_set_performance_marker_intel)(command_buffer, marker_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html>
    pub unsafe fn cmd_set_performance_stream_marker_intel(
        &self,
        command_buffer: CommandBuffer,
        marker_info: &PerformanceStreamMarkerInfoINTEL<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result =
                (self.cmd_set_performance_stream_marker_intel)(command_buffer, marker_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPerformanceOverrideINTEL.html>
    pub unsafe fn cmd_set_performance_override_intel(
        &self,
        command_buffer: CommandBuffer,
        override_info: &PerformanceOverrideInfoINTEL<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.cmd_set_performance_override_intel)(command_buffer, override_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAcquirePerformanceConfigurationINTEL.html>
    pub unsafe fn acquire_performance_configuration_intel(
        &self,
        device: Device,
        acquire_info: &PerformanceConfigurationAcquireInfoINTEL<'_>,
    ) -> crate::Result<PerformanceConfigurationINTEL> {
        unsafe {
            let mut configuration = core::mem::MaybeUninit::uninit();
            let result = (self.acquire_performance_configuration_intel)(
                device,
                acquire_info,
                configuration.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(configuration.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkReleasePerformanceConfigurationINTEL.html>
    pub unsafe fn release_performance_configuration_intel(
        &self,
        device: Device,
        configuration: PerformanceConfigurationINTEL,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.release_performance_configuration_intel)(device, configuration);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueSetPerformanceConfigurationINTEL.html>
    pub unsafe fn queue_set_performance_configuration_intel(
        &self,
        queue: Queue,
        configuration: PerformanceConfigurationINTEL,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.queue_set_performance_configuration_intel)(queue, configuration);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPerformanceParameterINTEL.html>
    pub unsafe fn get_performance_parameter_intel(
        &self,
        device: Device,
        parameter: PerformanceParameterTypeINTEL,
    ) -> crate::Result<PerformanceValueINTEL<'_>> {
        unsafe {
            let mut value = core::mem::MaybeUninit::uninit();
            let result =
                (self.get_performance_parameter_intel)(device, parameter, value.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(value.assume_init()),
                err => Err(err),
            }
        }
    }
}

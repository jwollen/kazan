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
    #[derive(Copy, Clone, Default)]
    pub struct PerformanceConfigurationINTEL(u64);
    pub type QueryPoolCreateInfoINTEL<'a> = QueryPoolPerformanceQueryCreateInfoINTEL<'a>;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PerformanceValueINTEL<'a> {
        pub ty: PerformanceValueTypeINTEL,
        pub data: PerformanceValueDataINTEL<'a>,
        pub _marker: PhantomData<&'a ()>,
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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct InitializePerformanceApiInfoINTEL<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_user_data: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
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
        pub fn user_data(mut self, user_data: &'a mut c_void) -> Self {
            self.p_user_data = user_data;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct QueryPoolPerformanceQueryCreateInfoINTEL<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub performance_counters_sampling: QueryPoolSamplingModeINTEL,
        pub _marker: PhantomData<&'a ()>,
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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PerformanceMarkerInfoINTEL<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub marker: u64,
        pub _marker: PhantomData<&'a ()>,
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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PerformanceStreamMarkerInfoINTEL<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub marker: u32,
        pub _marker: PhantomData<&'a ()>,
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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PerformanceOverrideInfoINTEL<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub ty: PerformanceOverrideTypeINTEL,
        pub enable: Bool32,
        pub parameter: u64,
        pub _marker: PhantomData<&'a ()>,
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
        pub fn enable(mut self, enable: Bool32) -> Self {
            self.enable = enable;
            self
        }
        pub fn parameter(mut self, parameter: u64) -> Self {
            self.parameter = parameter;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PerformanceConfigurationAcquireInfoINTEL<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub ty: PerformanceConfigurationTypeINTEL,
        pub _marker: PhantomData<&'a ()>,
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
    impl Default for PerformanceValueDataINTEL<'_> {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PerformanceConfigurationTypeINTEL(i32);
    impl PerformanceConfigurationTypeINTEL {
        pub const COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: Self = Self(0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct QueryPoolSamplingModeINTEL(i32);
    impl QueryPoolSamplingModeINTEL {
        pub const MANUAL_INTEL: Self = Self(0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PerformanceOverrideTypeINTEL(i32);
    impl PerformanceOverrideTypeINTEL {
        pub const NULL_HARDWARE_INTEL: Self = Self(0);
        pub const FLUSH_GPU_CACHES_INTEL: Self = Self(1);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PerformanceParameterTypeINTEL(i32);
    impl PerformanceParameterTypeINTEL {
        pub const HW_COUNTERS_SUPPORTED_INTEL: Self = Self(0);
        pub const STREAM_MARKER_VALID_BITS_INTEL: Self = Self(1);
    }
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
    pub type PFN_vkInitializePerformanceApiINTEL = unsafe extern "system" fn(
        device: Device,
        p_initialize_info: *const InitializePerformanceApiInfoINTEL<'_>,
    ) -> vk::Result;
    pub type PFN_vkUninitializePerformanceApiINTEL = unsafe extern "system" fn(device: Device);
    pub type PFN_vkCmdSetPerformanceMarkerINTEL = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const PerformanceMarkerInfoINTEL<'_>,
    ) -> vk::Result;
    pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const PerformanceStreamMarkerInfoINTEL<'_>,
    )
        -> vk::Result;
    pub type PFN_vkCmdSetPerformanceOverrideINTEL = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_override_info: *const PerformanceOverrideInfoINTEL<'_>,
    ) -> vk::Result;
    pub type PFN_vkAcquirePerformanceConfigurationINTEL = unsafe extern "system" fn(
        device: Device,
        p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL<'_>,
        p_configuration: *mut PerformanceConfigurationINTEL,
    )
        -> vk::Result;
    pub type PFN_vkReleasePerformanceConfigurationINTEL = unsafe extern "system" fn(
        device: Device,
        configuration: PerformanceConfigurationINTEL,
    )
        -> vk::Result;
    pub type PFN_vkQueueSetPerformanceConfigurationINTEL = unsafe extern "system" fn(
        queue: Queue,
        configuration: PerformanceConfigurationINTEL,
    )
        -> vk::Result;
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
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                initialize_performance_api_intel: transmute(
                    load(c"vkInitializePerformanceApiINTEL").ok_or(LoadingError)?,
                ),
                uninitialize_performance_api_intel: transmute(
                    load(c"vkUninitializePerformanceApiINTEL").ok_or(LoadingError)?,
                ),
                cmd_set_performance_marker_intel: transmute(
                    load(c"vkCmdSetPerformanceMarkerINTEL").ok_or(LoadingError)?,
                ),
                cmd_set_performance_stream_marker_intel: transmute(
                    load(c"vkCmdSetPerformanceStreamMarkerINTEL").ok_or(LoadingError)?,
                ),
                cmd_set_performance_override_intel: transmute(
                    load(c"vkCmdSetPerformanceOverrideINTEL").ok_or(LoadingError)?,
                ),
                acquire_performance_configuration_intel: transmute(
                    load(c"vkAcquirePerformanceConfigurationINTEL").ok_or(LoadingError)?,
                ),
                release_performance_configuration_intel: transmute(
                    load(c"vkReleasePerformanceConfigurationINTEL").ok_or(LoadingError)?,
                ),
                queue_set_performance_configuration_intel: transmute(
                    load(c"vkQueueSetPerformanceConfigurationINTEL").ok_or(LoadingError)?,
                ),
                get_performance_parameter_intel: transmute(
                    load(c"vkGetPerformanceParameterINTEL").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
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
    pub unsafe fn uninitialize_performance_api_intel(&self, device: Device) {
        unsafe { (self.uninitialize_performance_api_intel)(device) }
    }
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

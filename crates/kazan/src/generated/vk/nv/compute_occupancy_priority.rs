#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    pub const COMPUTE_OCCUPANCY_PRIORITY_LOW_NV: f32 = 0.25;
    pub const COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV: f32 = 0.50;
    pub const COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV: f32 = 0.75;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ComputeOccupancyPriorityParametersNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub occupancy_priority: f32,
        pub occupancy_throttling: f32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ComputeOccupancyPriorityParametersNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV;
    }
    impl Default for ComputeOccupancyPriorityParametersNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                occupancy_priority: Default::default(),
                occupancy_throttling: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ComputeOccupancyPriorityParametersNV<'a> {
        pub fn occupancy_priority(mut self, occupancy_priority: f32) -> Self {
            self.occupancy_priority = occupancy_priority;
            self
        }
        pub fn occupancy_throttling(mut self, occupancy_throttling: f32) -> Self {
            self.occupancy_throttling = occupancy_throttling;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceComputeOccupancyPriorityFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub compute_occupancy_priority: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceComputeOccupancyPriorityFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceComputeOccupancyPriorityFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceComputeOccupancyPriorityFeaturesNV<'a>
    {
    }
    impl Default for PhysicalDeviceComputeOccupancyPriorityFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                compute_occupancy_priority: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceComputeOccupancyPriorityFeaturesNV<'a> {
        pub fn compute_occupancy_priority(mut self, compute_occupancy_priority: Bool32) -> Self {
            self.compute_occupancy_priority = compute_occupancy_priority;
            self
        }
    }
    pub type PFN_vkCmdSetComputeOccupancyPriorityNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_parameters: *const ComputeOccupancyPriorityParametersNV<'_>,
    );
}
pub struct DeviceFn {
    cmd_set_compute_occupancy_priority_nv: PFN_vkCmdSetComputeOccupancyPriorityNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_compute_occupancy_priority_nv: transmute(
                    load(c"vkCmdSetComputeOccupancyPriorityNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_compute_occupancy_priority_nv(
        &self,
        command_buffer: CommandBuffer,
        parameters: &ComputeOccupancyPriorityParametersNV<'_>,
    ) {
        unsafe { (self.cmd_set_compute_occupancy_priority_nv)(command_buffer, parameters) }
    }
}

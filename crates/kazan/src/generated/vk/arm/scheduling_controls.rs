#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceQueueShaderCoreControlCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_core_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DeviceQueueShaderCoreControlCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM;
    }
    unsafe impl<'a> Extends<DeviceQueueCreateInfo<'a>>
        for DeviceQueueShaderCoreControlCreateInfoARM<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for DeviceQueueShaderCoreControlCreateInfoARM<'a> {}
    impl Default for DeviceQueueShaderCoreControlCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_core_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceQueueShaderCoreControlCreateInfoARM<'a> {
        pub fn shader_core_count(mut self, shader_core_count: u32) -> Self {
            self.shader_core_count = shader_core_count;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceSchedulingControlsFeaturesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub scheduling_controls: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSchedulingControlsFeaturesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceSchedulingControlsFeaturesARM<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceSchedulingControlsFeaturesARM<'a> {}
    impl Default for PhysicalDeviceSchedulingControlsFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                scheduling_controls: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceSchedulingControlsFeaturesARM<'a> {
        pub fn scheduling_controls(mut self, scheduling_controls: Bool32) -> Self {
            self.scheduling_controls = scheduling_controls;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceSchedulingControlsPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub scheduling_controls_flags: PhysicalDeviceSchedulingControlsFlagsARM,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSchedulingControlsPropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceSchedulingControlsPropertiesARM<'a>
    {
    }
    impl Default for PhysicalDeviceSchedulingControlsPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                scheduling_controls_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceSchedulingControlsPropertiesARM<'a> {
        pub fn scheduling_controls_flags(
            mut self,
            scheduling_controls_flags: PhysicalDeviceSchedulingControlsFlagsARM,
        ) -> Self {
            self.scheduling_controls_flags = scheduling_controls_flags;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PhysicalDeviceSchedulingControlsFlagsARM(Flags64);
    vk_bitflags_wrapped!(PhysicalDeviceSchedulingControlsFlagsARM, Flags64);
    impl PhysicalDeviceSchedulingControlsFlagsARM {
        pub const SHADER_CORE_COUNT_ARM: Self =
            Self(PhysicalDeviceSchedulingControlsFlagBitsARM::SHADER_CORE_COUNT_ARM.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PhysicalDeviceSchedulingControlsFlagBitsARM(u64);
    impl PhysicalDeviceSchedulingControlsFlagBitsARM {
        pub const SHADER_CORE_COUNT_ARM: Self = Self(1 << 0);
    }
}

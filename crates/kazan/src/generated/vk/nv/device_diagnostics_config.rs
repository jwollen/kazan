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
    pub struct PhysicalDeviceDiagnosticsConfigFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub diagnostics_config: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDiagnosticsConfigFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDiagnosticsConfigFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceDiagnosticsConfigFeaturesNV<'a> {}
    impl Default for PhysicalDeviceDiagnosticsConfigFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                diagnostics_config: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDiagnosticsConfigFeaturesNV<'a> {
        pub fn diagnostics_config(mut self, diagnostics_config: Bool32) -> Self {
            self.diagnostics_config = diagnostics_config;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceDiagnosticsConfigCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DeviceDiagnosticsConfigFlagsNV,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DeviceDiagnosticsConfigCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV;
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for DeviceDiagnosticsConfigCreateInfoNV<'a> {}
    impl Default for DeviceDiagnosticsConfigCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceDiagnosticsConfigCreateInfoNV<'a> {
        pub fn flags(mut self, flags: DeviceDiagnosticsConfigFlagsNV) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DeviceDiagnosticsConfigFlagsNV(Flags);
    impl DeviceDiagnosticsConfigFlagsNV {
        pub const ENABLE_SHADER_DEBUG_INFO_NV: Self =
            Self(DeviceDiagnosticsConfigFlagBitsNV::ENABLE_SHADER_DEBUG_INFO_NV.0);
        pub const ENABLE_RESOURCE_TRACKING_NV: Self =
            Self(DeviceDiagnosticsConfigFlagBitsNV::ENABLE_RESOURCE_TRACKING_NV.0);
        pub const ENABLE_AUTOMATIC_CHECKPOINTS_NV: Self =
            Self(DeviceDiagnosticsConfigFlagBitsNV::ENABLE_AUTOMATIC_CHECKPOINTS_NV.0);
        pub const ENABLE_SHADER_ERROR_REPORTING_NV: Self =
            Self(DeviceDiagnosticsConfigFlagBitsNV::ENABLE_SHADER_ERROR_REPORTING_NV.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DeviceDiagnosticsConfigFlagBitsNV(u32);
    impl DeviceDiagnosticsConfigFlagBitsNV {
        pub const ENABLE_SHADER_DEBUG_INFO_NV: Self = Self(1 << 0);
        pub const ENABLE_RESOURCE_TRACKING_NV: Self = Self(1 << 1);
        pub const ENABLE_AUTOMATIC_CHECKPOINTS_NV: Self = Self(1 << 2);
        pub const ENABLE_SHADER_ERROR_REPORTING_NV: Self = Self(1 << 3);
    }
}

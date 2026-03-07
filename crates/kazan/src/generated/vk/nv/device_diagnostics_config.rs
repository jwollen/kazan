#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_device_diagnostics_config";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDiagnosticsConfigFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDiagnosticsConfigFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub diagnostics_config: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDiagnosticsConfigFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDiagnosticsConfigFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("diagnostics_config", &self.diagnostics_config)
                .finish()
        }
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
        pub fn diagnostics_config(mut self, diagnostics_config: bool) -> Self {
            self.diagnostics_config = diagnostics_config.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceDiagnosticsConfigCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceDiagnosticsConfigCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DeviceDiagnosticsConfigFlagsNV,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceDiagnosticsConfigCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceDiagnosticsConfigCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .finish()
        }
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceDiagnosticsConfigFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DeviceDiagnosticsConfigFlagsNV(Flags);
    vk_bitflags_wrapped!(DeviceDiagnosticsConfigFlagsNV, Flags);

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

    impl fmt::Debug for DeviceDiagnosticsConfigFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    DeviceDiagnosticsConfigFlagsNV::ENABLE_SHADER_DEBUG_INFO_NV.0,
                    "ENABLE_SHADER_DEBUG_INFO_NV",
                ),
                (
                    DeviceDiagnosticsConfigFlagsNV::ENABLE_RESOURCE_TRACKING_NV.0,
                    "ENABLE_RESOURCE_TRACKING_NV",
                ),
                (
                    DeviceDiagnosticsConfigFlagsNV::ENABLE_AUTOMATIC_CHECKPOINTS_NV.0,
                    "ENABLE_AUTOMATIC_CHECKPOINTS_NV",
                ),
                (
                    DeviceDiagnosticsConfigFlagsNV::ENABLE_SHADER_ERROR_REPORTING_NV.0,
                    "ENABLE_SHADER_ERROR_REPORTING_NV",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceDiagnosticsConfigFlagBitsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DeviceDiagnosticsConfigFlagBitsNV(u32);

    impl DeviceDiagnosticsConfigFlagBitsNV {
        pub const ENABLE_SHADER_DEBUG_INFO_NV: Self = Self(1 << 0);
        pub const ENABLE_RESOURCE_TRACKING_NV: Self = Self(1 << 1);
        pub const ENABLE_AUTOMATIC_CHECKPOINTS_NV: Self = Self(1 << 2);
        pub const ENABLE_SHADER_ERROR_REPORTING_NV: Self = Self(1 << 3);
    }

    impl fmt::Debug for DeviceDiagnosticsConfigFlagBitsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ENABLE_SHADER_DEBUG_INFO_NV => Some("ENABLE_SHADER_DEBUG_INFO_NV"),
                Self::ENABLE_RESOURCE_TRACKING_NV => Some("ENABLE_RESOURCE_TRACKING_NV"),
                Self::ENABLE_AUTOMATIC_CHECKPOINTS_NV => Some("ENABLE_AUTOMATIC_CHECKPOINTS_NV"),
                Self::ENABLE_SHADER_ERROR_REPORTING_NV => Some("ENABLE_SHADER_ERROR_REPORTING_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
}

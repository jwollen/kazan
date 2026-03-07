#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_AMD_anti_lag";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceAntiLagFeaturesAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceAntiLagFeaturesAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub anti_lag: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceAntiLagFeaturesAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceAntiLagFeaturesAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("anti_lag", &self.anti_lag)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceAntiLagFeaturesAMD<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceAntiLagFeaturesAMD<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceAntiLagFeaturesAMD<'a> {}

    impl Default for PhysicalDeviceAntiLagFeaturesAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                anti_lag: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceAntiLagFeaturesAMD<'a> {
        pub fn anti_lag(mut self, anti_lag: bool) -> Self {
            self.anti_lag = anti_lag.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAntiLagDataAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AntiLagDataAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub mode: AntiLagModeAMD,
        pub max_fps: u32,
        pub p_presentation_info: *const AntiLagPresentationInfoAMD<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AntiLagDataAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AntiLagDataAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("mode", &self.mode)
                .field("max_fps", &self.max_fps)
                .field("p_presentation_info", &self.p_presentation_info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AntiLagDataAMD<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ANTI_LAG_DATA_AMD;
    }

    impl Default for AntiLagDataAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                mode: Default::default(),
                max_fps: Default::default(),
                p_presentation_info: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AntiLagDataAMD<'a> {
        pub fn mode(mut self, mode: AntiLagModeAMD) -> Self {
            self.mode = mode;
            self
        }

        pub fn max_fps(mut self, max_fps: u32) -> Self {
            self.max_fps = max_fps;
            self
        }

        pub fn presentation_info(
            mut self,
            presentation_info: &'a AntiLagPresentationInfoAMD<'a>,
        ) -> Self {
            self.p_presentation_info = presentation_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAntiLagPresentationInfoAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AntiLagPresentationInfoAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub stage: AntiLagStageAMD,
        pub frame_index: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AntiLagPresentationInfoAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AntiLagPresentationInfoAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("stage", &self.stage)
                .field("frame_index", &self.frame_index)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AntiLagPresentationInfoAMD<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ANTI_LAG_PRESENTATION_INFO_AMD;
    }

    impl Default for AntiLagPresentationInfoAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                stage: Default::default(),
                frame_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AntiLagPresentationInfoAMD<'a> {
        pub fn stage(mut self, stage: AntiLagStageAMD) -> Self {
            self.stage = stage;
            self
        }

        pub fn frame_index(mut self, frame_index: u64) -> Self {
            self.frame_index = frame_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAntiLagModeAMD.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AntiLagModeAMD(i32);

    impl AntiLagModeAMD {
        pub const DRIVER_CONTROL_AMD: Self = Self(0);
        pub const ON_AMD: Self = Self(1);
        pub const OFF_AMD: Self = Self(2);
    }

    impl fmt::Debug for AntiLagModeAMD {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DRIVER_CONTROL_AMD => Some("DRIVER_CONTROL_AMD"),
                Self::ON_AMD => Some("ON_AMD"),
                Self::OFF_AMD => Some("OFF_AMD"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAntiLagStageAMD.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AntiLagStageAMD(i32);

    impl AntiLagStageAMD {
        pub const INPUT_AMD: Self = Self(0);
        pub const PRESENT_AMD: Self = Self(1);
    }

    impl fmt::Debug for AntiLagStageAMD {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::INPUT_AMD => Some("INPUT_AMD"),
                Self::PRESENT_AMD => Some("PRESENT_AMD"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAntiLagUpdateAMD.html>
    pub type PFN_vkAntiLagUpdateAMD =
        unsafe extern "system" fn(device: Device, p_data: *const AntiLagDataAMD<'_>);
}

pub struct DeviceFn {
    anti_lag_update_amd: PFN_vkAntiLagUpdateAMD,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                anti_lag_update_amd: transmute(
                    load(c"vkAntiLagUpdateAMD").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAntiLagUpdateAMD.html>
    pub unsafe fn anti_lag_update_amd(&self, device: Device, data: &AntiLagDataAMD<'_>) {
        unsafe { (self.anti_lag_update_amd)(device, data) }
    }
}

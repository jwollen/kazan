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
    pub struct PhysicalDeviceAntiLagFeaturesAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub anti_lag: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
        pub fn anti_lag(mut self, anti_lag: Bool32) -> Self {
            self.anti_lag = anti_lag;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AntiLagDataAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub mode: AntiLagModeAMD,
        pub max_fps: u32,
        pub p_presentation_info: *const AntiLagPresentationInfoAMD<'a>,
        pub _marker: PhantomData<&'a ()>,
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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AntiLagPresentationInfoAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub stage: AntiLagStageAMD,
        pub frame_index: u64,
        pub _marker: PhantomData<&'a ()>,
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AntiLagModeAMD(i32);
    impl AntiLagModeAMD {
        pub const DRIVER_CONTROL_AMD: Self = Self(0);
        pub const ON_AMD: Self = Self(1);
        pub const OFF_AMD: Self = Self(2);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AntiLagStageAMD(i32);
    impl AntiLagStageAMD {
        pub const INPUT_AMD: Self = Self(0);
        pub const PRESENT_AMD: Self = Self(1);
    }
    pub type PFN_vkAntiLagUpdateAMD =
        unsafe extern "system" fn(device: Device, p_data: *const AntiLagDataAMD<'_>);
}
pub struct DeviceFn {
    anti_lag_update_amd: PFN_vkAntiLagUpdateAMD,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                anti_lag_update_amd: transmute(load(c"vkAntiLagUpdateAMD").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn anti_lag_update_amd(&self, device: Device, data: &AntiLagDataAMD<'_>) {
        unsafe { (self.anti_lag_update_amd)(device, data) }
    }
}

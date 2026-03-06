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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub multiview_per_view_viewports: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'a>
    {
    }
    impl Default for PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                multiview_per_view_viewports: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'a> {
        pub fn multiview_per_view_viewports(mut self, multiview_per_view_viewports: bool) -> Self {
            self.multiview_per_view_viewports = multiview_per_view_viewports.into();
            self
        }
    }
}

#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_depth_clip_control";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDepthClipControlFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDepthClipControlFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub depth_clip_control: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDepthClipControlFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDepthClipControlFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("depth_clip_control", &self.depth_clip_control)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDepthClipControlFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDepthClipControlFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceDepthClipControlFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceDepthClipControlFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                depth_clip_control: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDepthClipControlFeaturesEXT<'a> {
        pub fn depth_clip_control(mut self, depth_clip_control: bool) -> Self {
            self.depth_clip_control = depth_clip_control.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineViewportDepthClipControlCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineViewportDepthClipControlCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub negative_one_to_one: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineViewportDepthClipControlCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineViewportDepthClipControlCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("negative_one_to_one", &self.negative_one_to_one)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineViewportDepthClipControlCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<PipelineViewportStateCreateInfo<'a>>
        for PipelineViewportDepthClipControlCreateInfoEXT<'a>
    {
    }

    impl Default for PipelineViewportDepthClipControlCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                negative_one_to_one: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineViewportDepthClipControlCreateInfoEXT<'a> {
        pub fn negative_one_to_one(mut self, negative_one_to_one: bool) -> Self {
            self.negative_one_to_one = negative_one_to_one.into();
            self
        }
    }
}

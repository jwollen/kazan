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
    pub struct PhysicalDeviceDepthClipEnableFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub depth_clip_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDepthClipEnableFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDepthClipEnableFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceDepthClipEnableFeaturesEXT<'a> {}
    impl Default for PhysicalDeviceDepthClipEnableFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                depth_clip_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDepthClipEnableFeaturesEXT<'a> {
        pub fn depth_clip_enable(mut self, depth_clip_enable: Bool32) -> Self {
            self.depth_clip_enable = depth_clip_enable;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineRasterizationDepthClipStateCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineRasterizationDepthClipStateCreateFlagsEXT,
        pub depth_clip_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineRasterizationDepthClipStateCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT;
    }
    unsafe impl<'a> Extends<PipelineRasterizationStateCreateInfo<'a>>
        for PipelineRasterizationDepthClipStateCreateInfoEXT<'a>
    {
    }
    impl Default for PipelineRasterizationDepthClipStateCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                depth_clip_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineRasterizationDepthClipStateCreateInfoEXT<'a> {
        pub fn flags(mut self, flags: PipelineRasterizationDepthClipStateCreateFlagsEXT) -> Self {
            self.flags = flags;
            self
        }
        pub fn depth_clip_enable(mut self, depth_clip_enable: Bool32) -> Self {
            self.depth_clip_enable = depth_clip_enable;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineRasterizationDepthClipStateCreateFlagsEXT(Flags);
    vk_bitflags_wrapped!(PipelineRasterizationDepthClipStateCreateFlagsEXT, Flags);
    impl PipelineRasterizationDepthClipStateCreateFlagsEXT {}
}

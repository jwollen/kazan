#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub depth_clip_enable: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDepthClipEnableFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            depth_clip_enable: Default::default(),
            _marker: PhantomData,
        }
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
impl Default for PipelineRasterizationDepthClipStateCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            flags: Default::default(),
            depth_clip_enable: Default::default(),
            _marker: PhantomData,
        }
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineRasterizationDepthClipStateCreateFlagsEXT: Flags {
    }
}

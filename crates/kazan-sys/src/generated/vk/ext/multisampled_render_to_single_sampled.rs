#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub multisampled_render_to_single_sampled: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassResolvePerformanceQueryEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optimal: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MultisampledRenderToSingleSampledInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub multisampled_render_to_single_sampled_enable: Bool32,
    pub rasterization_samples: SampleCountFlagBits,
}

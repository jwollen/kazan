#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub multisampled_render_to_single_sampled: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SubpassResolvePerformanceQueryEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optimal: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct MultisampledRenderToSingleSampledInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub multisampled_render_to_single_sampled_enable: Bool32,
    pub rasterization_samples: SampleCountFlagBits,
    pub _marker: PhantomData<&'a ()>,
}

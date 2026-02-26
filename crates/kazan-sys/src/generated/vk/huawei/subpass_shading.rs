#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct SubpassShadingPipelineCreateInfoHUAWEI<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceSubpassShadingPropertiesHUAWEI<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_subpass_shading_workgroup_size_aspect_ratio: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceSubpassShadingFeaturesHUAWEI<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subpass_shading: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI = unsafe extern "system" fn(
    device: Device,
    renderpass: RenderPass,
    p_max_workgroup_size: *mut Extent2D,
) -> Result;
pub type PFN_vkCmdSubpassShadingHUAWEI = unsafe extern "system" fn(command_buffer: CommandBuffer);

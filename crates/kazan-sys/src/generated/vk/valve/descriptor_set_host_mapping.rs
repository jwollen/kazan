#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_set_host_mapping: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetBindingReferenceVALVE {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_set_layout: DescriptorSetLayout,
    pub binding: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutHostMappingInfoVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_offset: usize,
    pub descriptor_size: u32,
}
pub type PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE = unsafe extern "system" fn(
    device: Device,
    p_binding_reference: *const DescriptorSetBindingReferenceVALVE,
    p_host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE,
);
pub type PFN_vkGetDescriptorSetHostMappingVALVE = unsafe extern "system" fn(
    device: Device,
    descriptor_set: DescriptorSet,
    pp_data: *mut *mut c_void,
);

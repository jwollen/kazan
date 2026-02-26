#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_set_host_mapping: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DescriptorSetBindingReferenceVALVE<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_set_layout: DescriptorSetLayout,
    pub binding: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DescriptorSetLayoutHostMappingInfoVALVE<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_offset: usize,
    pub descriptor_size: u32,
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE = unsafe extern "system" fn(
    device: Device,
    p_binding_reference: *const DescriptorSetBindingReferenceVALVE<'_>,
    p_host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE<'_>,
);
pub type PFN_vkGetDescriptorSetHostMappingVALVE = unsafe extern "system" fn(
    device: Device,
    descriptor_set: DescriptorSet,
    pp_data: *mut *mut c_void,
);

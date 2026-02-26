#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_set_host_mapping: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE,
            p_next: core::ptr::null_mut(),
            descriptor_set_host_mapping: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetBindingReferenceVALVE<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_set_layout: DescriptorSetLayout,
    pub binding: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorSetBindingReferenceVALVE<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DESCRIPTOR_SET_BINDING_REFERENCE_VALVE,
            p_next: core::ptr::null(),
            descriptor_set_layout: Default::default(),
            binding: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutHostMappingInfoVALVE<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_offset: usize,
    pub descriptor_size: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorSetLayoutHostMappingInfoVALVE<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE,
            p_next: core::ptr::null_mut(),
            descriptor_offset: Default::default(),
            descriptor_size: Default::default(),
            _marker: PhantomData,
        }
    }
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

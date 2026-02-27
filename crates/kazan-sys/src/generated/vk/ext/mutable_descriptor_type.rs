#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub mutable_descriptor_type: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            mutable_descriptor_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'a> {
    pub fn mutable_descriptor_type(mut self, mutable_descriptor_type: Bool32) -> Self {
        self.mutable_descriptor_type = mutable_descriptor_type;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MutableDescriptorTypeListEXT<'a> {
    pub descriptor_type_count: u32,
    pub p_descriptor_types: *const DescriptorType,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MutableDescriptorTypeListEXT<'_> {
    fn default() -> Self {
        Self {
            descriptor_type_count: Default::default(),
            p_descriptor_types: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> MutableDescriptorTypeListEXT<'a> {
    pub fn descriptor_types(mut self, descriptor_types: &'a [DescriptorType]) -> Self {
        self.descriptor_type_count = descriptor_types.len().try_into().unwrap();
        self.p_descriptor_types = descriptor_types.as_ptr();
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MutableDescriptorTypeCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mutable_descriptor_type_list_count: u32,
    pub p_mutable_descriptor_type_lists: *const MutableDescriptorTypeListEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MutableDescriptorTypeCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            mutable_descriptor_type_list_count: Default::default(),
            p_mutable_descriptor_type_lists: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> MutableDescriptorTypeCreateInfoEXT<'a> {
    pub fn mutable_descriptor_type_lists(
        mut self,
        mutable_descriptor_type_lists: &'a [MutableDescriptorTypeListEXT<'a>],
    ) -> Self {
        self.mutable_descriptor_type_list_count =
            mutable_descriptor_type_lists.len().try_into().unwrap();
        self.p_mutable_descriptor_type_lists = mutable_descriptor_type_lists.as_ptr();
        self
    }
}

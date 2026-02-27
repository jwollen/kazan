#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ValidationCacheEXT(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ValidationCacheCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ValidationCacheCreateFlagsEXT,
    pub initial_data_size: usize,
    pub p_initial_data: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ValidationCacheCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VALIDATION_CACHE_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            flags: Default::default(),
            initial_data_size: Default::default(),
            p_initial_data: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ValidationCacheCreateInfoEXT<'a> {
    pub fn flags(mut self, flags: ValidationCacheCreateFlagsEXT) -> Self {
        self.flags = flags;
        self
    }
    pub fn initial_data(mut self, initial_data: &'a [u8]) -> Self {
        self.initial_data_size = initial_data.len().try_into().unwrap();
        self.p_initial_data = initial_data.as_ptr() as _;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderModuleValidationCacheCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub validation_cache: ValidationCacheEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ShaderModuleValidationCacheCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            validation_cache: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ShaderModuleValidationCacheCreateInfoEXT<'a> {
    pub fn validation_cache(mut self, validation_cache: ValidationCacheEXT) -> Self {
        self.validation_cache = validation_cache;
        self
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidationCacheHeaderVersionEXT(i32);
impl ValidationCacheHeaderVersionEXT {
    pub const ONE_EXT: Self = Self(1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ValidationCacheCreateFlagsEXT: Flags {
    }
}
pub type PFN_vkCreateValidationCacheEXT = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ValidationCacheCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_validation_cache: *mut ValidationCacheEXT,
) -> Result;
pub type PFN_vkDestroyValidationCacheEXT = unsafe extern "system" fn(
    device: Device,
    validation_cache: ValidationCacheEXT,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkGetValidationCacheDataEXT = unsafe extern "system" fn(
    device: Device,
    validation_cache: ValidationCacheEXT,
    p_data_size: *mut usize,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkMergeValidationCachesEXT = unsafe extern "system" fn(
    device: Device,
    dst_cache: ValidationCacheEXT,
    src_cache_count: u32,
    p_src_caches: *const ValidationCacheEXT,
) -> Result;

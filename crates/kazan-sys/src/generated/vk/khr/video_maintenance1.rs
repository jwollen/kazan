#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceVideoMaintenance1FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_maintenance1: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoInlineQueryInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub query_pool: QueryPool,
    pub first_query: u32,
    pub query_count: u32,
    pub _marker: PhantomData<&'a ()>,
}

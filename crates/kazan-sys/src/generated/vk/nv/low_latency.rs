#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct QueryLowLatencySupportNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_queried_low_latency_data: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}

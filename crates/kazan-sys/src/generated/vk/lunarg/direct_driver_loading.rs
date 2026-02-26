#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct DirectDriverLoadingInfoLUNARG<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: DirectDriverLoadingFlagsLUNARG,
    pub pfn_get_instance_proc_addr: Option<PFN_vkGetInstanceProcAddrLUNARG>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DirectDriverLoadingListLUNARG<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mode: DirectDriverLoadingModeLUNARG,
    pub driver_count: u32,
    pub p_drivers: *const DirectDriverLoadingInfoLUNARG<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirectDriverLoadingModeLUNARG(i32);
impl DirectDriverLoadingModeLUNARG {
    pub const EXCLUSIVE_LUNARG: Self = Self(0);
    pub const INCLUSIVE_LUNARG: Self = Self(1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DirectDriverLoadingFlagsLUNARG: Flags {
    }
}
pub type PFN_vkGetInstanceProcAddrLUNARG =
    unsafe extern "system" fn(instance: Instance, p_name: *const c_char) -> PFN_vkVoidFunction;

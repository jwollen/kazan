#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct CuModuleNVX(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct CuFunctionNVX(u64);
#[repr(C)]
pub struct CuModuleCreateInfoNVX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub data_size: usize,
    pub p_data: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct CuModuleTexturingModeCreateInfoNVX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use64bit_texturing: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct CuFunctionCreateInfoNVX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub module: CuModuleNVX,
    pub p_name: *const c_char,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct CuLaunchInfoNVX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub function: CuFunctionNVX,
    pub grid_dim_x: u32,
    pub grid_dim_y: u32,
    pub grid_dim_z: u32,
    pub block_dim_x: u32,
    pub block_dim_y: u32,
    pub block_dim_z: u32,
    pub shared_mem_bytes: u32,
    pub param_count: usize,
    pub p_params: *const *const c_void,
    pub extra_count: usize,
    pub p_extras: *const *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkCreateCuModuleNVX = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CuModuleCreateInfoNVX<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_module: *mut CuModuleNVX,
) -> Result;
pub type PFN_vkCreateCuFunctionNVX = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CuFunctionCreateInfoNVX<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_function: *mut CuFunctionNVX,
) -> Result;
pub type PFN_vkDestroyCuModuleNVX = unsafe extern "system" fn(
    device: Device,
    module: CuModuleNVX,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkDestroyCuFunctionNVX = unsafe extern "system" fn(
    device: Device,
    function: CuFunctionNVX,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCmdCuLaunchKernelNVX = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_launch_info: *const CuLaunchInfoNVX<'_>,
);

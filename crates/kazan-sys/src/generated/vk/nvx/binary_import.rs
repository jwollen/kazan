#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CuModuleNVX(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CuFunctionNVX(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CuModuleCreateInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub data_size: usize,
    pub p_data: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CuModuleTexturingModeCreateInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use64bit_texturing: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CuFunctionCreateInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub module: CuModuleNVX,
    pub p_name: *const c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CuLaunchInfoNVX {
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
}
pub type PFN_vkCreateCuModuleNVX = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CuModuleCreateInfoNVX,
    p_allocator: *const AllocationCallbacks,
    p_module: *mut CuModuleNVX,
) -> Result;
pub type PFN_vkCreateCuFunctionNVX = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CuFunctionCreateInfoNVX,
    p_allocator: *const AllocationCallbacks,
    p_function: *mut CuFunctionNVX,
) -> Result;
pub type PFN_vkDestroyCuModuleNVX = unsafe extern "system" fn(
    device: Device,
    module: CuModuleNVX,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkDestroyCuFunctionNVX = unsafe extern "system" fn(
    device: Device,
    function: CuFunctionNVX,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCmdCuLaunchKernelNVX =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_launch_info: *const CuLaunchInfoNVX);

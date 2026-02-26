#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct CudaModuleNV(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct CudaFunctionNV(u64);
#[repr(C)]
pub struct CudaModuleCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub data_size: usize,
    pub p_data: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct CudaFunctionCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub module: CudaModuleNV,
    pub p_name: *const c_char,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct CudaLaunchInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub function: CudaFunctionNV,
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
#[repr(C)]
pub struct PhysicalDeviceCudaKernelLaunchFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cuda_kernel_launch_features: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceCudaKernelLaunchPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compute_capability_minor: u32,
    pub compute_capability_major: u32,
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkCreateCudaModuleNV = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CudaModuleCreateInfoNV<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_module: *mut CudaModuleNV,
) -> Result;
pub type PFN_vkGetCudaModuleCacheNV = unsafe extern "system" fn(
    device: Device,
    module: CudaModuleNV,
    p_cache_size: *mut usize,
    p_cache_data: *mut c_void,
) -> Result;
pub type PFN_vkCreateCudaFunctionNV = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CudaFunctionCreateInfoNV<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_function: *mut CudaFunctionNV,
) -> Result;
pub type PFN_vkDestroyCudaModuleNV = unsafe extern "system" fn(
    device: Device,
    module: CudaModuleNV,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkDestroyCudaFunctionNV = unsafe extern "system" fn(
    device: Device,
    function: CudaFunctionNV,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCmdCudaLaunchKernelNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_launch_info: *const CudaLaunchInfoNV<'_>,
);

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
#[derive(Copy, Clone)]
pub struct CudaModuleCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub data_size: usize,
    pub p_data: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CudaModuleCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CUDA_MODULE_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            data_size: Default::default(),
            p_data: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CudaFunctionCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub module: CudaModuleNV,
    pub p_name: *const c_char,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CudaFunctionCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CUDA_FUNCTION_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            module: Default::default(),
            p_name: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for CudaLaunchInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CUDA_LAUNCH_INFO_NV,
            p_next: core::ptr::null(),
            function: Default::default(),
            grid_dim_x: Default::default(),
            grid_dim_y: Default::default(),
            grid_dim_z: Default::default(),
            block_dim_x: Default::default(),
            block_dim_y: Default::default(),
            block_dim_z: Default::default(),
            shared_mem_bytes: Default::default(),
            param_count: Default::default(),
            p_params: core::ptr::null(),
            extra_count: Default::default(),
            p_extras: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCudaKernelLaunchFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cuda_kernel_launch_features: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCudaKernelLaunchFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            cuda_kernel_launch_features: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCudaKernelLaunchPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compute_capability_minor: u32,
    pub compute_capability_major: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCudaKernelLaunchPropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            compute_capability_minor: Default::default(),
            compute_capability_major: Default::default(),
            _marker: PhantomData,
        }
    }
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

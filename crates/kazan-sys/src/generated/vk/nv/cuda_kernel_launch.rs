#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
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
impl<'a> CudaModuleCreateInfoNV<'a> {
    pub fn data(mut self, data: &'a [u8]) -> Self {
        self.data_size = data.len().try_into().unwrap();
        self.p_data = data.as_ptr() as _;
        self
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
impl<'a> CudaFunctionCreateInfoNV<'a> {
    pub fn module(mut self, module: CudaModuleNV) -> Self {
        self.module = module;
        self
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
impl<'a> CudaLaunchInfoNV<'a> {
    pub fn function(mut self, function: CudaFunctionNV) -> Self {
        self.function = function;
        self
    }
    pub fn grid_dim_x(mut self, grid_dim_x: u32) -> Self {
        self.grid_dim_x = grid_dim_x;
        self
    }
    pub fn grid_dim_y(mut self, grid_dim_y: u32) -> Self {
        self.grid_dim_y = grid_dim_y;
        self
    }
    pub fn grid_dim_z(mut self, grid_dim_z: u32) -> Self {
        self.grid_dim_z = grid_dim_z;
        self
    }
    pub fn block_dim_x(mut self, block_dim_x: u32) -> Self {
        self.block_dim_x = block_dim_x;
        self
    }
    pub fn block_dim_y(mut self, block_dim_y: u32) -> Self {
        self.block_dim_y = block_dim_y;
        self
    }
    pub fn block_dim_z(mut self, block_dim_z: u32) -> Self {
        self.block_dim_z = block_dim_z;
        self
    }
    pub fn shared_mem_bytes(mut self, shared_mem_bytes: u32) -> Self {
        self.shared_mem_bytes = shared_mem_bytes;
        self
    }
    pub fn params(mut self, params: &'a [&'a c_void]) -> Self {
        self.param_count = params.len().try_into().unwrap();
        self.p_params = params.as_ptr() as _;
        self
    }
    pub fn extras(mut self, extras: &'a [&'a c_void]) -> Self {
        self.extra_count = extras.len().try_into().unwrap();
        self.p_extras = extras.as_ptr() as _;
        self
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
impl<'a> PhysicalDeviceCudaKernelLaunchFeaturesNV<'a> {
    pub fn cuda_kernel_launch_features(mut self, cuda_kernel_launch_features: Bool32) -> Self {
        self.cuda_kernel_launch_features = cuda_kernel_launch_features;
        self
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
impl<'a> PhysicalDeviceCudaKernelLaunchPropertiesNV<'a> {
    pub fn compute_capability_minor(mut self, compute_capability_minor: u32) -> Self {
        self.compute_capability_minor = compute_capability_minor;
        self
    }
    pub fn compute_capability_major(mut self, compute_capability_major: u32) -> Self {
        self.compute_capability_major = compute_capability_major;
        self
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

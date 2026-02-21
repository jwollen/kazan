#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    create_cuda_module_nv: PFN_vkCreateCudaModuleNV,
    get_cuda_module_cache_nv: PFN_vkGetCudaModuleCacheNV,
    create_cuda_function_nv: PFN_vkCreateCudaFunctionNV,
    destroy_cuda_module_nv: PFN_vkDestroyCudaModuleNV,
    destroy_cuda_function_nv: PFN_vkDestroyCudaFunctionNV,
    cmd_cuda_launch_kernel_nv: PFN_vkCmdCudaLaunchKernelNV,
}
impl DeviceFn {
    pub unsafe fn create_cuda_module_nv(
        &self,
        device: Device,
        create_info: &CudaModuleCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
        module: &mut CudaModuleNV,
    ) -> Result {
        unsafe { (self.create_cuda_module_nv)(device, create_info, allocator.to_raw_ptr(), module) }
    }
    pub unsafe fn get_cuda_module_cache_nv(
        &self,
        device: Device,
        module: CudaModuleNV,
        cache_data: impl ExtendUninit<u8>,
    ) -> Result {
        unsafe {
            try_extend_uninit(cache_data, |cache_size, cache_data| {
                (self.get_cuda_module_cache_nv)(device, module, cache_size, cache_data as _)
            })
        }
    }
    pub unsafe fn create_cuda_function_nv(
        &self,
        device: Device,
        create_info: &CudaFunctionCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
        function: &mut CudaFunctionNV,
    ) -> Result {
        unsafe {
            (self.create_cuda_function_nv)(device, create_info, allocator.to_raw_ptr(), function)
        }
    }
    pub unsafe fn destroy_cuda_module_nv(
        &self,
        device: Device,
        module: CudaModuleNV,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_cuda_module_nv)(device, module, allocator.to_raw_ptr()) }
    }
    pub unsafe fn destroy_cuda_function_nv(
        &self,
        device: Device,
        function: CudaFunctionNV,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_cuda_function_nv)(device, function, allocator.to_raw_ptr()) }
    }
    pub unsafe fn cmd_cuda_launch_kernel_nv(
        &self,
        command_buffer: CommandBuffer,
        launch_info: &CudaLaunchInfoNV,
    ) {
        unsafe { (self.cmd_cuda_launch_kernel_nv)(command_buffer, launch_info) }
    }
}

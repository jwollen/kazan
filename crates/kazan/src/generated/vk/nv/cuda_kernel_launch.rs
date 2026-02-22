#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    create_cuda_module_nv: PFN_vkCreateCudaModuleNV,
    get_cuda_module_cache_nv: PFN_vkGetCudaModuleCacheNV,
    create_cuda_function_nv: PFN_vkCreateCudaFunctionNV,
    destroy_cuda_module_nv: PFN_vkDestroyCudaModuleNV,
    destroy_cuda_function_nv: PFN_vkDestroyCudaFunctionNV,
    cmd_cuda_launch_kernel_nv: PFN_vkCmdCudaLaunchKernelNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_cuda_module_nv: transmute(
                    load(c"vkCreateCudaModuleNV").ok_or(LoadingError)?,
                ),
                get_cuda_module_cache_nv: transmute(
                    load(c"vkGetCudaModuleCacheNV").ok_or(LoadingError)?,
                ),
                create_cuda_function_nv: transmute(
                    load(c"vkCreateCudaFunctionNV").ok_or(LoadingError)?,
                ),
                destroy_cuda_module_nv: transmute(
                    load(c"vkDestroyCudaModuleNV").ok_or(LoadingError)?,
                ),
                destroy_cuda_function_nv: transmute(
                    load(c"vkDestroyCudaFunctionNV").ok_or(LoadingError)?,
                ),
                cmd_cuda_launch_kernel_nv: transmute(
                    load(c"vkCmdCudaLaunchKernelNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_cuda_module_nv(
        &self,
        device: Device,
        create_info: &CudaModuleCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<CudaModuleNV> {
        unsafe {
            let mut module = core::mem::MaybeUninit::uninit();
            let result = (self.create_cuda_module_nv)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                module.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(module.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_cuda_module_cache_nv(
        &self,
        device: Device,
        module: CudaModuleNV,
        cache_data: impl ExtendUninit<u8>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(cache_data, |cache_size, cache_data| {
                let result =
                    (self.get_cuda_module_cache_nv)(device, module, cache_size, cache_data as _);

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn create_cuda_function_nv(
        &self,
        device: Device,
        create_info: &CudaFunctionCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<CudaFunctionNV> {
        unsafe {
            let mut function = core::mem::MaybeUninit::uninit();
            let result = (self.create_cuda_function_nv)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                function.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(function.assume_init()),
                err => Err(err),
            }
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

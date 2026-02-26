#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    create_cu_module_nvx: PFN_vkCreateCuModuleNVX,
    create_cu_function_nvx: PFN_vkCreateCuFunctionNVX,
    destroy_cu_module_nvx: PFN_vkDestroyCuModuleNVX,
    destroy_cu_function_nvx: PFN_vkDestroyCuFunctionNVX,
    cmd_cu_launch_kernel_nvx: PFN_vkCmdCuLaunchKernelNVX,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_cu_module_nvx: transmute(load(c"vkCreateCuModuleNVX").ok_or(LoadingError)?),
                create_cu_function_nvx: transmute(
                    load(c"vkCreateCuFunctionNVX").ok_or(LoadingError)?,
                ),
                destroy_cu_module_nvx: transmute(
                    load(c"vkDestroyCuModuleNVX").ok_or(LoadingError)?,
                ),
                destroy_cu_function_nvx: transmute(
                    load(c"vkDestroyCuFunctionNVX").ok_or(LoadingError)?,
                ),
                cmd_cu_launch_kernel_nvx: transmute(
                    load(c"vkCmdCuLaunchKernelNVX").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_cu_module_nvx(
        &self,
        device: Device,
        create_info: &CuModuleCreateInfoNVX<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<CuModuleNVX> {
        unsafe {
            let mut module = core::mem::MaybeUninit::uninit();
            let result = (self.create_cu_module_nvx)(
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
    pub unsafe fn create_cu_function_nvx(
        &self,
        device: Device,
        create_info: &CuFunctionCreateInfoNVX<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<CuFunctionNVX> {
        unsafe {
            let mut function = core::mem::MaybeUninit::uninit();
            let result = (self.create_cu_function_nvx)(
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
    pub unsafe fn destroy_cu_module_nvx(
        &self,
        device: Device,
        module: CuModuleNVX,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_cu_module_nvx)(device, module, allocator.to_raw_ptr()) }
    }
    pub unsafe fn destroy_cu_function_nvx(
        &self,
        device: Device,
        function: CuFunctionNVX,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_cu_function_nvx)(device, function, allocator.to_raw_ptr()) }
    }
    pub unsafe fn cmd_cu_launch_kernel_nvx(
        &self,
        command_buffer: CommandBuffer,
        launch_info: &CuLaunchInfoNVX<'_>,
    ) {
        unsafe { (self.cmd_cu_launch_kernel_nvx)(command_buffer, launch_info) }
    }
}

#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    create_cu_module_nvx: PFN_vkCreateCuModuleNVX,
    create_cu_function_nvx: PFN_vkCreateCuFunctionNVX,
    destroy_cu_module_nvx: PFN_vkDestroyCuModuleNVX,
    destroy_cu_function_nvx: PFN_vkDestroyCuFunctionNVX,
    cmd_cu_launch_kernel_nvx: PFN_vkCmdCuLaunchKernelNVX,
}
impl DeviceFn {
    pub unsafe fn create_cu_module_nvx(
        &self,
        device: Device,
        create_info: &CuModuleCreateInfoNVX,
        allocator: Option<&AllocationCallbacks>,
        module: &mut CuModuleNVX,
    ) -> Result {
        unsafe { (self.create_cu_module_nvx)(device, create_info, allocator.to_raw_ptr(), module) }
    }
    pub unsafe fn create_cu_function_nvx(
        &self,
        device: Device,
        create_info: &CuFunctionCreateInfoNVX,
        allocator: Option<&AllocationCallbacks>,
        function: &mut CuFunctionNVX,
    ) -> Result {
        unsafe {
            (self.create_cu_function_nvx)(device, create_info, allocator.to_raw_ptr(), function)
        }
    }
    pub unsafe fn destroy_cu_module_nvx(
        &self,
        device: Device,
        module: CuModuleNVX,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_cu_module_nvx)(device, module, allocator.to_raw_ptr()) }
    }
    pub unsafe fn destroy_cu_function_nvx(
        &self,
        device: Device,
        function: CuFunctionNVX,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_cu_function_nvx)(device, function, allocator.to_raw_ptr()) }
    }
    pub unsafe fn cmd_cu_launch_kernel_nvx(
        &self,
        command_buffer: CommandBuffer,
        launch_info: &CuLaunchInfoNVX,
    ) {
        unsafe { (self.cmd_cu_launch_kernel_nvx)(command_buffer, launch_info) }
    }
}

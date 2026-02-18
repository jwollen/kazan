#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
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
        allocator: &AllocationCallbacks,
        module: &mut CuModuleNVX,
    ) -> Result {
        unsafe { (self.create_cu_module_nvx)(device, create_info, allocator, module) }
    }
    pub unsafe fn create_cu_function_nvx(
        &self,
        device: Device,
        create_info: &CuFunctionCreateInfoNVX,
        allocator: &AllocationCallbacks,
        function: &mut CuFunctionNVX,
    ) -> Result {
        unsafe { (self.create_cu_function_nvx)(device, create_info, allocator, function) }
    }
    pub unsafe fn destroy_cu_module_nvx(
        &self,
        device: Device,
        module: CuModuleNVX,
        allocator: &AllocationCallbacks,
    ) {
        unsafe { (self.destroy_cu_module_nvx)(device, module, allocator) }
    }
    pub unsafe fn destroy_cu_function_nvx(
        &self,
        device: Device,
        function: CuFunctionNVX,
        allocator: &AllocationCallbacks,
    ) {
        unsafe { (self.destroy_cu_function_nvx)(device, function, allocator) }
    }
    pub unsafe fn cmd_cu_launch_kernel_nvx(
        &self,
        command_buffer: CommandBuffer,
        launch_info: &CuLaunchInfoNVX,
    ) {
        unsafe { (self.cmd_cu_launch_kernel_nvx)(command_buffer, launch_info) }
    }
}

//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_internal_handle.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_internal_handle";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    pub type InternalHandleNV = *const c_void;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceInternalHandleNV.html>
    pub type PFN_vkGetPhysicalDeviceInternalHandleNV =
        unsafe extern "system" fn(physical_device: PhysicalDevice, p_handle: *mut InternalHandleNV);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceInternalHandleNV.html>
    pub type PFN_vkGetDeviceInternalHandleNV =
        unsafe extern "system" fn(device: Device, p_handle: *mut InternalHandleNV);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetQueueInternalHandleNV.html>
    pub type PFN_vkGetQueueInternalHandleNV =
        unsafe extern "system" fn(queue: Queue, p_handle: *mut InternalHandleNV);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetCommandBufferInternalHandleNV.html>
    pub type PFN_vkGetCommandBufferInternalHandleNV =
        unsafe extern "system" fn(command_buffer: CommandBuffer, p_handle: *mut InternalHandleNV);
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkInternalHandleNV = InternalHandleNV;
}

pub struct InstanceFn {
    get_physical_device_internal_handle: PFN_vkGetPhysicalDeviceInternalHandleNV,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_internal_handle: transmute(
                    load(c"vkGetPhysicalDeviceInternalHandleNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceInternalHandleNV.html>
    #[inline]
    pub unsafe fn get_physical_device_internal_handle(
        &self,
        physical_device: PhysicalDevice,
        handle: *mut InternalHandleNV,
    ) {
        unsafe { (self.get_physical_device_internal_handle)(physical_device, handle) }
    }
}

pub struct DeviceFn {
    get_device_internal_handle: PFN_vkGetDeviceInternalHandleNV,
    get_queue_internal_handle: PFN_vkGetQueueInternalHandleNV,
    get_command_buffer_internal_handle: PFN_vkGetCommandBufferInternalHandleNV,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_device_internal_handle: transmute(
                    load(c"vkGetDeviceInternalHandleNV").ok_or(MissingEntryPointError)?,
                ),
                get_queue_internal_handle: transmute(
                    load(c"vkGetQueueInternalHandleNV").ok_or(MissingEntryPointError)?,
                ),
                get_command_buffer_internal_handle: transmute(
                    load(c"vkGetCommandBufferInternalHandleNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceInternalHandleNV.html>
    #[inline]
    pub unsafe fn get_device_internal_handle(&self, device: Device, handle: *mut InternalHandleNV) {
        unsafe { (self.get_device_internal_handle)(device, handle) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetQueueInternalHandleNV.html>
    #[inline]
    pub unsafe fn get_queue_internal_handle(&self, queue: Queue, handle: *mut InternalHandleNV) {
        unsafe { (self.get_queue_internal_handle)(queue, handle) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetCommandBufferInternalHandleNV.html>
    #[inline]
    pub unsafe fn get_command_buffer_internal_handle(
        &self,
        command_buffer: CommandBuffer,
        handle: *mut InternalHandleNV,
    ) {
        unsafe { (self.get_command_buffer_internal_handle)(command_buffer, handle) }
    }
}

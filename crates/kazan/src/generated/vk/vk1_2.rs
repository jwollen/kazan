#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    reset_query_pool: PFN_vkResetQueryPool,
    get_semaphore_counter_value: PFN_vkGetSemaphoreCounterValue,
    wait_semaphores: PFN_vkWaitSemaphores,
    signal_semaphore: PFN_vkSignalSemaphore,
    get_buffer_device_address: PFN_vkGetBufferDeviceAddress,
    get_buffer_opaque_capture_address: PFN_vkGetBufferOpaqueCaptureAddress,
    get_device_memory_opaque_capture_address: PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
    cmd_draw_indirect_count: PFN_vkCmdDrawIndirectCount,
    cmd_draw_indexed_indirect_count: PFN_vkCmdDrawIndexedIndirectCount,
    create_render_pass2: PFN_vkCreateRenderPass2,
    cmd_begin_render_pass2: PFN_vkCmdBeginRenderPass2,
    cmd_next_subpass2: PFN_vkCmdNextSubpass2,
    cmd_end_render_pass2: PFN_vkCmdEndRenderPass2,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                reset_query_pool: transmute(load(c"vkResetQueryPool").ok_or(LoadingError)?),
                get_semaphore_counter_value: transmute(
                    load(c"vkGetSemaphoreCounterValue").ok_or(LoadingError)?,
                ),
                wait_semaphores: transmute(load(c"vkWaitSemaphores").ok_or(LoadingError)?),
                signal_semaphore: transmute(load(c"vkSignalSemaphore").ok_or(LoadingError)?),
                get_buffer_device_address: transmute(
                    load(c"vkGetBufferDeviceAddress").ok_or(LoadingError)?,
                ),
                get_buffer_opaque_capture_address: transmute(
                    load(c"vkGetBufferOpaqueCaptureAddress").ok_or(LoadingError)?,
                ),
                get_device_memory_opaque_capture_address: transmute(
                    load(c"vkGetDeviceMemoryOpaqueCaptureAddress").ok_or(LoadingError)?,
                ),
                cmd_draw_indirect_count: transmute(
                    load(c"vkCmdDrawIndirectCount").ok_or(LoadingError)?,
                ),
                cmd_draw_indexed_indirect_count: transmute(
                    load(c"vkCmdDrawIndexedIndirectCount").ok_or(LoadingError)?,
                ),
                create_render_pass2: transmute(load(c"vkCreateRenderPass2").ok_or(LoadingError)?),
                cmd_begin_render_pass2: transmute(
                    load(c"vkCmdBeginRenderPass2").ok_or(LoadingError)?,
                ),
                cmd_next_subpass2: transmute(load(c"vkCmdNextSubpass2").ok_or(LoadingError)?),
                cmd_end_render_pass2: transmute(load(c"vkCmdEndRenderPass2").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn reset_query_pool(
        &self,
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        unsafe { (self.reset_query_pool)(device, query_pool, first_query, query_count) }
    }
    pub unsafe fn get_semaphore_counter_value(
        &self,
        device: Device,
        semaphore: Semaphore,
    ) -> crate::Result<u64> {
        unsafe {
            let mut value = core::mem::MaybeUninit::uninit();
            let result = (self.get_semaphore_counter_value)(device, semaphore, value.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(value.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn wait_semaphores(
        &self,
        device: Device,
        wait_info: &SemaphoreWaitInfo,
        timeout: u64,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.wait_semaphores)(device, wait_info, timeout);

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::TIMEOUT => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn signal_semaphore(
        &self,
        device: Device,
        signal_info: &SemaphoreSignalInfo,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.signal_semaphore)(device, signal_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_buffer_device_address(
        &self,
        device: Device,
        info: &BufferDeviceAddressInfo,
    ) -> DeviceAddress {
        unsafe { (self.get_buffer_device_address)(device, info) }
    }
    pub unsafe fn get_buffer_opaque_capture_address(
        &self,
        device: Device,
        info: &BufferDeviceAddressInfo,
    ) -> u64 {
        unsafe { (self.get_buffer_opaque_capture_address)(device, info) }
    }
    pub unsafe fn get_device_memory_opaque_capture_address(
        &self,
        device: Device,
        info: &DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        unsafe { (self.get_device_memory_opaque_capture_address)(device, info) }
    }
    pub unsafe fn cmd_draw_indirect_count(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_indirect_count)(
                command_buffer,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        }
    }
    pub unsafe fn cmd_draw_indexed_indirect_count(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_indexed_indirect_count)(
                command_buffer,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        }
    }
    pub unsafe fn create_render_pass2(
        &self,
        device: Device,
        create_info: &RenderPassCreateInfo2,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<RenderPass> {
        unsafe {
            let mut render_pass = core::mem::MaybeUninit::uninit();
            let result = (self.create_render_pass2)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                render_pass.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(render_pass.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn cmd_begin_render_pass2(
        &self,
        command_buffer: CommandBuffer,
        render_pass_begin: &RenderPassBeginInfo,
        subpass_begin_info: &SubpassBeginInfo,
    ) {
        unsafe {
            (self.cmd_begin_render_pass2)(command_buffer, render_pass_begin, subpass_begin_info)
        }
    }
    pub unsafe fn cmd_next_subpass2(
        &self,
        command_buffer: CommandBuffer,
        subpass_begin_info: &SubpassBeginInfo,
        subpass_end_info: &SubpassEndInfo,
    ) {
        unsafe { (self.cmd_next_subpass2)(command_buffer, subpass_begin_info, subpass_end_info) }
    }
    pub unsafe fn cmd_end_render_pass2(
        &self,
        command_buffer: CommandBuffer,
        subpass_end_info: &SubpassEndInfo,
    ) {
        unsafe { (self.cmd_end_render_pass2)(command_buffer, subpass_end_info) }
    }
}

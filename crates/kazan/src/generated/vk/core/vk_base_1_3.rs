#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    get_physical_device_tool_properties: PFN_vkGetPhysicalDeviceToolProperties,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_tool_properties(
        &self,
        physical_device: PhysicalDevice,
        tool_properties: impl ExtendUninit<PhysicalDeviceToolProperties>,
    ) -> Result {
        unsafe {
            try_extend_uninit(tool_properties, |tool_count, tool_properties| {
                (self.get_physical_device_tool_properties)(
                    physical_device,
                    tool_count,
                    tool_properties as _,
                )
            })
        }
    }
}
pub struct DeviceFn {
    get_device_buffer_memory_requirements: PFN_vkGetDeviceBufferMemoryRequirements,
    get_device_image_memory_requirements: PFN_vkGetDeviceImageMemoryRequirements,
    get_device_image_sparse_memory_requirements: PFN_vkGetDeviceImageSparseMemoryRequirements,
    create_private_data_slot: PFN_vkCreatePrivateDataSlot,
    destroy_private_data_slot: PFN_vkDestroyPrivateDataSlot,
    set_private_data: PFN_vkSetPrivateData,
    get_private_data: PFN_vkGetPrivateData,
    cmd_copy_buffer2: PFN_vkCmdCopyBuffer2,
    cmd_copy_image2: PFN_vkCmdCopyImage2,
    cmd_copy_buffer_to_image2: PFN_vkCmdCopyBufferToImage2,
    cmd_copy_image_to_buffer2: PFN_vkCmdCopyImageToBuffer2,
    cmd_pipeline_barrier2: PFN_vkCmdPipelineBarrier2,
    queue_submit2: PFN_vkQueueSubmit2,
    cmd_write_timestamp2: PFN_vkCmdWriteTimestamp2,
}
impl DeviceFn {
    pub unsafe fn get_device_buffer_memory_requirements(
        &self,
        device: Device,
        info: &DeviceBufferMemoryRequirements,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe { (self.get_device_buffer_memory_requirements)(device, info, memory_requirements) }
    }
    pub unsafe fn get_device_image_memory_requirements(
        &self,
        device: Device,
        info: &DeviceImageMemoryRequirements,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe { (self.get_device_image_memory_requirements)(device, info, memory_requirements) }
    }
    pub unsafe fn get_device_image_sparse_memory_requirements(
        &self,
        device: Device,
        info: &DeviceImageMemoryRequirements,
        sparse_memory_requirements: impl ExtendUninit<SparseImageMemoryRequirements2>,
    ) {
        unsafe {
            extend_uninit(
                sparse_memory_requirements,
                |sparse_memory_requirement_count, sparse_memory_requirements| {
                    (self.get_device_image_sparse_memory_requirements)(
                        device,
                        info,
                        sparse_memory_requirement_count,
                        sparse_memory_requirements as _,
                    )
                },
            )
        }
    }
    pub unsafe fn create_private_data_slot(
        &self,
        device: Device,
        create_info: &PrivateDataSlotCreateInfo,
        allocator: &AllocationCallbacks,
        private_data_slot: &mut PrivateDataSlot,
    ) -> Result {
        unsafe {
            (self.create_private_data_slot)(device, create_info, allocator, private_data_slot)
        }
    }
    pub unsafe fn destroy_private_data_slot(
        &self,
        device: Device,
        private_data_slot: PrivateDataSlot,
        allocator: &AllocationCallbacks,
    ) {
        unsafe { (self.destroy_private_data_slot)(device, private_data_slot, allocator) }
    }
    pub unsafe fn set_private_data(
        &self,
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> Result {
        unsafe {
            (self.set_private_data)(device, object_type, object_handle, private_data_slot, data)
        }
    }
    pub unsafe fn get_private_data(
        &self,
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: &mut u64,
    ) {
        unsafe {
            (self.get_private_data)(device, object_type, object_handle, private_data_slot, data)
        }
    }
    pub unsafe fn cmd_copy_buffer2(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_info: &CopyBufferInfo2,
    ) {
        unsafe { (self.cmd_copy_buffer2)(command_buffer, copy_buffer_info) }
    }
    pub unsafe fn cmd_copy_image2(
        &self,
        command_buffer: CommandBuffer,
        copy_image_info: &CopyImageInfo2,
    ) {
        unsafe { (self.cmd_copy_image2)(command_buffer, copy_image_info) }
    }
    pub unsafe fn cmd_copy_buffer_to_image2(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_to_image_info: &CopyBufferToImageInfo2,
    ) {
        unsafe { (self.cmd_copy_buffer_to_image2)(command_buffer, copy_buffer_to_image_info) }
    }
    pub unsafe fn cmd_copy_image_to_buffer2(
        &self,
        command_buffer: CommandBuffer,
        copy_image_to_buffer_info: &CopyImageToBufferInfo2,
    ) {
        unsafe { (self.cmd_copy_image_to_buffer2)(command_buffer, copy_image_to_buffer_info) }
    }
    pub unsafe fn cmd_pipeline_barrier2(
        &self,
        command_buffer: CommandBuffer,
        dependency_info: &DependencyInfo,
    ) {
        unsafe { (self.cmd_pipeline_barrier2)(command_buffer, dependency_info) }
    }
    pub unsafe fn queue_submit2(
        &self,
        queue: Queue,
        submits: &[SubmitInfo2],
        fence: Fence,
    ) -> Result {
        unsafe {
            (self.queue_submit2)(
                queue,
                submits.len().try_into().unwrap(),
                submits.as_ptr() as _,
                fence,
            )
        }
    }
    pub unsafe fn cmd_write_timestamp2(
        &self,
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ) {
        unsafe { (self.cmd_write_timestamp2)(command_buffer, stage, query_pool, query) }
    }
}

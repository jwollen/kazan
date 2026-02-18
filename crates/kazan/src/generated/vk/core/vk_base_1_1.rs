#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct EntryFn {
    enumerate_instance_version: PFN_vkEnumerateInstanceVersion,
}
impl EntryFn {
    pub unsafe fn enumerate_instance_version(&self, api_version: &mut u32) -> Result {
        unsafe { (self.enumerate_instance_version)(api_version) }
    }
}
pub struct InstanceFn {
    get_physical_device_features2: PFN_vkGetPhysicalDeviceFeatures2,
    get_physical_device_properties2: PFN_vkGetPhysicalDeviceProperties2,
    get_physical_device_format_properties2: PFN_vkGetPhysicalDeviceFormatProperties2,
    get_physical_device_image_format_properties2: PFN_vkGetPhysicalDeviceImageFormatProperties2,
    get_physical_device_queue_family_properties2: PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
    get_physical_device_memory_properties2: PFN_vkGetPhysicalDeviceMemoryProperties2,
    get_physical_device_sparse_image_format_properties2:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
    get_physical_device_external_buffer_properties: PFN_vkGetPhysicalDeviceExternalBufferProperties,
    get_physical_device_external_semaphore_properties:
        PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
    get_physical_device_external_fence_properties: PFN_vkGetPhysicalDeviceExternalFenceProperties,
    enumerate_physical_device_groups: PFN_vkEnumeratePhysicalDeviceGroups,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_features2(
        &self,
        physical_device: PhysicalDevice,
        features: &mut PhysicalDeviceFeatures2,
    ) {
        unsafe { (self.get_physical_device_features2)(physical_device, features) }
    }
    pub unsafe fn get_physical_device_properties2(
        &self,
        physical_device: PhysicalDevice,
        properties: &mut PhysicalDeviceProperties2,
    ) {
        unsafe { (self.get_physical_device_properties2)(physical_device, properties) }
    }
    pub unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        format_properties: &mut FormatProperties2,
    ) {
        unsafe {
            (self.get_physical_device_format_properties2)(
                physical_device,
                format,
                format_properties,
            )
        }
    }
    pub unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        image_format_info: &PhysicalDeviceImageFormatInfo2,
        image_format_properties: &mut ImageFormatProperties2,
    ) -> Result {
        unsafe {
            (self.get_physical_device_image_format_properties2)(
                physical_device,
                image_format_info,
                image_format_properties,
            )
        }
    }
    pub unsafe fn get_physical_device_queue_family_properties2(
        &self,
        physical_device: PhysicalDevice,
        queue_family_properties: impl ExtendUninit<QueueFamilyProperties2>,
    ) {
        unsafe {
            extend_uninit(
                queue_family_properties,
                |queue_family_property_count, queue_family_properties| {
                    (self.get_physical_device_queue_family_properties2)(
                        physical_device,
                        queue_family_property_count,
                        queue_family_properties as _,
                    )
                },
            )
        }
    }
    pub unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: PhysicalDevice,
        memory_properties: &mut PhysicalDeviceMemoryProperties2,
    ) {
        unsafe { (self.get_physical_device_memory_properties2)(physical_device, memory_properties) }
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        format_info: &PhysicalDeviceSparseImageFormatInfo2,
        properties: impl ExtendUninit<SparseImageFormatProperties2>,
    ) {
        unsafe {
            extend_uninit(properties, |property_count, properties| {
                (self.get_physical_device_sparse_image_format_properties2)(
                    physical_device,
                    format_info,
                    property_count,
                    properties as _,
                )
            })
        }
    }
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: PhysicalDevice,
        external_buffer_info: &PhysicalDeviceExternalBufferInfo,
        external_buffer_properties: &mut ExternalBufferProperties,
    ) {
        unsafe {
            (self.get_physical_device_external_buffer_properties)(
                physical_device,
                external_buffer_info,
                external_buffer_properties,
            )
        }
    }
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: PhysicalDevice,
        external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo,
        external_semaphore_properties: &mut ExternalSemaphoreProperties,
    ) {
        unsafe {
            (self.get_physical_device_external_semaphore_properties)(
                physical_device,
                external_semaphore_info,
                external_semaphore_properties,
            )
        }
    }
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: PhysicalDevice,
        external_fence_info: &PhysicalDeviceExternalFenceInfo,
        external_fence_properties: &mut ExternalFenceProperties,
    ) {
        unsafe {
            (self.get_physical_device_external_fence_properties)(
                physical_device,
                external_fence_info,
                external_fence_properties,
            )
        }
    }
    pub unsafe fn enumerate_physical_device_groups(
        &self,
        instance: Instance,
        physical_device_group_properties: impl ExtendUninit<PhysicalDeviceGroupProperties>,
    ) -> Result {
        unsafe {
            try_extend_uninit(
                physical_device_group_properties,
                |physical_device_group_count, physical_device_group_properties| {
                    (self.enumerate_physical_device_groups)(
                        instance,
                        physical_device_group_count,
                        physical_device_group_properties as _,
                    )
                },
            )
        }
    }
}
pub struct DeviceFn {
    trim_command_pool: PFN_vkTrimCommandPool,
    get_device_group_peer_memory_features: PFN_vkGetDeviceGroupPeerMemoryFeatures,
    bind_buffer_memory2: PFN_vkBindBufferMemory2,
    bind_image_memory2: PFN_vkBindImageMemory2,
    cmd_set_device_mask: PFN_vkCmdSetDeviceMask,
    get_buffer_memory_requirements2: PFN_vkGetBufferMemoryRequirements2,
    get_image_memory_requirements2: PFN_vkGetImageMemoryRequirements2,
    get_image_sparse_memory_requirements2: PFN_vkGetImageSparseMemoryRequirements2,
    get_device_queue2: PFN_vkGetDeviceQueue2,
}
impl DeviceFn {
    pub unsafe fn trim_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ) {
        unsafe { (self.trim_command_pool)(device, command_pool, flags) }
    }
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        device: Device,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        peer_memory_features: &mut PeerMemoryFeatureFlags,
    ) {
        unsafe {
            (self.get_device_group_peer_memory_features)(
                device,
                heap_index,
                local_device_index,
                remote_device_index,
                peer_memory_features,
            )
        }
    }
    pub unsafe fn bind_buffer_memory2(
        &self,
        device: Device,
        bind_infos: &[BindBufferMemoryInfo],
    ) -> Result {
        unsafe {
            (self.bind_buffer_memory2)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            )
        }
    }
    pub unsafe fn bind_image_memory2(
        &self,
        device: Device,
        bind_infos: &[BindImageMemoryInfo],
    ) -> Result {
        unsafe {
            (self.bind_image_memory2)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_device_mask(&self, command_buffer: CommandBuffer, device_mask: u32) {
        unsafe { (self.cmd_set_device_mask)(command_buffer, device_mask) }
    }
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        device: Device,
        info: &BufferMemoryRequirementsInfo2,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe { (self.get_buffer_memory_requirements2)(device, info, memory_requirements) }
    }
    pub unsafe fn get_image_memory_requirements2(
        &self,
        device: Device,
        info: &ImageMemoryRequirementsInfo2,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe { (self.get_image_memory_requirements2)(device, info, memory_requirements) }
    }
    pub unsafe fn get_image_sparse_memory_requirements2(
        &self,
        device: Device,
        info: &ImageSparseMemoryRequirementsInfo2,
        sparse_memory_requirements: impl ExtendUninit<SparseImageMemoryRequirements2>,
    ) {
        unsafe {
            extend_uninit(
                sparse_memory_requirements,
                |sparse_memory_requirement_count, sparse_memory_requirements| {
                    (self.get_image_sparse_memory_requirements2)(
                        device,
                        info,
                        sparse_memory_requirement_count,
                        sparse_memory_requirements as _,
                    )
                },
            )
        }
    }
    pub unsafe fn get_device_queue2(
        &self,
        device: Device,
        queue_info: &DeviceQueueInfo2,
        queue: &mut Queue,
    ) {
        unsafe { (self.get_device_queue2)(device, queue_info, queue) }
    }
}

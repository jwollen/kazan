#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct EntryFn {
    enumerate_instance_version: PFN_vkEnumerateInstanceVersion,
}
impl EntryFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                enumerate_instance_version: transmute(
                    load(c"vkEnumerateInstanceVersion").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl EntryFn {
    pub unsafe fn enumerate_instance_version(&self, api_version: &mut u32) -> crate::Result<()> {
        unsafe { result((self.enumerate_instance_version)(api_version)) }
    }
}
pub struct InstanceFn {
    enumerate_physical_device_groups: PFN_vkEnumeratePhysicalDeviceGroups,
    get_physical_device_features2: PFN_vkGetPhysicalDeviceFeatures2,
    get_physical_device_properties2: PFN_vkGetPhysicalDeviceProperties2,
    get_physical_device_format_properties2: PFN_vkGetPhysicalDeviceFormatProperties2,
    get_physical_device_image_format_properties2: PFN_vkGetPhysicalDeviceImageFormatProperties2,
    get_physical_device_queue_family_properties2: PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
    get_physical_device_memory_properties2: PFN_vkGetPhysicalDeviceMemoryProperties2,
    get_physical_device_sparse_image_format_properties2:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
    get_physical_device_external_buffer_properties: PFN_vkGetPhysicalDeviceExternalBufferProperties,
    get_physical_device_external_fence_properties: PFN_vkGetPhysicalDeviceExternalFenceProperties,
    get_physical_device_external_semaphore_properties:
        PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                enumerate_physical_device_groups: transmute(
                    load(c"vkEnumeratePhysicalDeviceGroups").ok_or(LoadingError)?,
                ),
                get_physical_device_features2: transmute(
                    load(c"vkGetPhysicalDeviceFeatures2").ok_or(LoadingError)?,
                ),
                get_physical_device_properties2: transmute(
                    load(c"vkGetPhysicalDeviceProperties2").ok_or(LoadingError)?,
                ),
                get_physical_device_format_properties2: transmute(
                    load(c"vkGetPhysicalDeviceFormatProperties2").ok_or(LoadingError)?,
                ),
                get_physical_device_image_format_properties2: transmute(
                    load(c"vkGetPhysicalDeviceImageFormatProperties2").ok_or(LoadingError)?,
                ),
                get_physical_device_queue_family_properties2: transmute(
                    load(c"vkGetPhysicalDeviceQueueFamilyProperties2").ok_or(LoadingError)?,
                ),
                get_physical_device_memory_properties2: transmute(
                    load(c"vkGetPhysicalDeviceMemoryProperties2").ok_or(LoadingError)?,
                ),
                get_physical_device_sparse_image_format_properties2: transmute(
                    load(c"vkGetPhysicalDeviceSparseImageFormatProperties2").ok_or(LoadingError)?,
                ),
                get_physical_device_external_buffer_properties: transmute(
                    load(c"vkGetPhysicalDeviceExternalBufferProperties").ok_or(LoadingError)?,
                ),
                get_physical_device_external_fence_properties: transmute(
                    load(c"vkGetPhysicalDeviceExternalFenceProperties").ok_or(LoadingError)?,
                ),
                get_physical_device_external_semaphore_properties: transmute(
                    load(c"vkGetPhysicalDeviceExternalSemaphoreProperties").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn enumerate_physical_device_groups(
        &self,
        instance: Instance,
        physical_device_group_properties: impl ExtendUninit<PhysicalDeviceGroupProperties>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(
                physical_device_group_properties,
                |physical_device_group_count, physical_device_group_properties| {
                    result((self.enumerate_physical_device_groups)(
                        instance,
                        physical_device_group_count,
                        physical_device_group_properties as _,
                    ))
                },
            )
        }
    }
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
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_physical_device_image_format_properties2)(
                physical_device,
                image_format_info,
                image_format_properties,
            ))
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
}
pub struct DeviceFn {
    bind_buffer_memory2: PFN_vkBindBufferMemory2,
    bind_image_memory2: PFN_vkBindImageMemory2,
    get_device_group_peer_memory_features: PFN_vkGetDeviceGroupPeerMemoryFeatures,
    cmd_set_device_mask: PFN_vkCmdSetDeviceMask,
    get_image_memory_requirements2: PFN_vkGetImageMemoryRequirements2,
    get_buffer_memory_requirements2: PFN_vkGetBufferMemoryRequirements2,
    get_image_sparse_memory_requirements2: PFN_vkGetImageSparseMemoryRequirements2,
    trim_command_pool: PFN_vkTrimCommandPool,
    get_device_queue2: PFN_vkGetDeviceQueue2,
    cmd_dispatch_base: PFN_vkCmdDispatchBase,
    create_descriptor_update_template: PFN_vkCreateDescriptorUpdateTemplate,
    destroy_descriptor_update_template: PFN_vkDestroyDescriptorUpdateTemplate,
    update_descriptor_set_with_template: PFN_vkUpdateDescriptorSetWithTemplate,
    get_descriptor_set_layout_support: PFN_vkGetDescriptorSetLayoutSupport,
    create_sampler_ycbcr_conversion: PFN_vkCreateSamplerYcbcrConversion,
    destroy_sampler_ycbcr_conversion: PFN_vkDestroySamplerYcbcrConversion,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                bind_buffer_memory2: transmute(load(c"vkBindBufferMemory2").ok_or(LoadingError)?),
                bind_image_memory2: transmute(load(c"vkBindImageMemory2").ok_or(LoadingError)?),
                get_device_group_peer_memory_features: transmute(
                    load(c"vkGetDeviceGroupPeerMemoryFeatures").ok_or(LoadingError)?,
                ),
                cmd_set_device_mask: transmute(load(c"vkCmdSetDeviceMask").ok_or(LoadingError)?),
                get_image_memory_requirements2: transmute(
                    load(c"vkGetImageMemoryRequirements2").ok_or(LoadingError)?,
                ),
                get_buffer_memory_requirements2: transmute(
                    load(c"vkGetBufferMemoryRequirements2").ok_or(LoadingError)?,
                ),
                get_image_sparse_memory_requirements2: transmute(
                    load(c"vkGetImageSparseMemoryRequirements2").ok_or(LoadingError)?,
                ),
                trim_command_pool: transmute(load(c"vkTrimCommandPool").ok_or(LoadingError)?),
                get_device_queue2: transmute(load(c"vkGetDeviceQueue2").ok_or(LoadingError)?),
                cmd_dispatch_base: transmute(load(c"vkCmdDispatchBase").ok_or(LoadingError)?),
                create_descriptor_update_template: transmute(
                    load(c"vkCreateDescriptorUpdateTemplate").ok_or(LoadingError)?,
                ),
                destroy_descriptor_update_template: transmute(
                    load(c"vkDestroyDescriptorUpdateTemplate").ok_or(LoadingError)?,
                ),
                update_descriptor_set_with_template: transmute(
                    load(c"vkUpdateDescriptorSetWithTemplate").ok_or(LoadingError)?,
                ),
                get_descriptor_set_layout_support: transmute(
                    load(c"vkGetDescriptorSetLayoutSupport").ok_or(LoadingError)?,
                ),
                create_sampler_ycbcr_conversion: transmute(
                    load(c"vkCreateSamplerYcbcrConversion").ok_or(LoadingError)?,
                ),
                destroy_sampler_ycbcr_conversion: transmute(
                    load(c"vkDestroySamplerYcbcrConversion").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn bind_buffer_memory2(
        &self,
        device: Device,
        bind_infos: &[BindBufferMemoryInfo],
    ) -> crate::Result<()> {
        unsafe {
            result((self.bind_buffer_memory2)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            ))
        }
    }
    pub unsafe fn bind_image_memory2(
        &self,
        device: Device,
        bind_infos: &[BindImageMemoryInfo],
    ) -> crate::Result<()> {
        unsafe {
            result((self.bind_image_memory2)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            ))
        }
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
    pub unsafe fn cmd_set_device_mask(&self, command_buffer: CommandBuffer, device_mask: u32) {
        unsafe { (self.cmd_set_device_mask)(command_buffer, device_mask) }
    }
    pub unsafe fn get_image_memory_requirements2(
        &self,
        device: Device,
        info: &ImageMemoryRequirementsInfo2,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe { (self.get_image_memory_requirements2)(device, info, memory_requirements) }
    }
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        device: Device,
        info: &BufferMemoryRequirementsInfo2,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe { (self.get_buffer_memory_requirements2)(device, info, memory_requirements) }
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
    pub unsafe fn trim_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ) {
        unsafe { (self.trim_command_pool)(device, command_pool, flags) }
    }
    pub unsafe fn get_device_queue2(
        &self,
        device: Device,
        queue_info: &DeviceQueueInfo2,
        queue: &mut Queue,
    ) {
        unsafe { (self.get_device_queue2)(device, queue_info, queue) }
    }
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe {
            (self.cmd_dispatch_base)(
                command_buffer,
                base_group_x,
                base_group_y,
                base_group_z,
                group_count_x,
                group_count_y,
                group_count_z,
            )
        }
    }
    pub unsafe fn create_descriptor_update_template(
        &self,
        device: Device,
        create_info: &DescriptorUpdateTemplateCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        descriptor_update_template: &mut DescriptorUpdateTemplate,
    ) -> crate::Result<()> {
        unsafe {
            result((self.create_descriptor_update_template)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                descriptor_update_template,
            ))
        }
    }
    pub unsafe fn destroy_descriptor_update_template(
        &self,
        device: Device,
        descriptor_update_template: DescriptorUpdateTemplate,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_descriptor_update_template)(
                device,
                descriptor_update_template,
                allocator.to_raw_ptr(),
            )
        }
    }
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        device: Device,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        data: &c_void,
    ) {
        unsafe {
            (self.update_descriptor_set_with_template)(
                device,
                descriptor_set,
                descriptor_update_template,
                data,
            )
        }
    }
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        device: Device,
        create_info: &DescriptorSetLayoutCreateInfo,
        support: &mut DescriptorSetLayoutSupport,
    ) {
        unsafe { (self.get_descriptor_set_layout_support)(device, create_info, support) }
    }
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        device: Device,
        create_info: &SamplerYcbcrConversionCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        ycbcr_conversion: &mut SamplerYcbcrConversion,
    ) -> crate::Result<()> {
        unsafe {
            result((self.create_sampler_ycbcr_conversion)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                ycbcr_conversion,
            ))
        }
    }
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        device: Device,
        ycbcr_conversion: SamplerYcbcrConversion,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_sampler_ycbcr_conversion)(
                device,
                ycbcr_conversion,
                allocator.to_raw_ptr(),
            )
        }
    }
}

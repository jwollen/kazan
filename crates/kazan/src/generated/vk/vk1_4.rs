#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    map_memory2: PFN_vkMapMemory2,
    unmap_memory2: PFN_vkUnmapMemory2,
    get_device_image_subresource_layout: PFN_vkGetDeviceImageSubresourceLayout,
    get_image_subresource_layout2: PFN_vkGetImageSubresourceLayout2,
    copy_memory_to_image: PFN_vkCopyMemoryToImage,
    copy_image_to_memory: PFN_vkCopyImageToMemory,
    copy_image_to_image: PFN_vkCopyImageToImage,
    transition_image_layout: PFN_vkTransitionImageLayout,
    cmd_push_descriptor_set: PFN_vkCmdPushDescriptorSet,
    cmd_push_descriptor_set_with_template: PFN_vkCmdPushDescriptorSetWithTemplate,
    cmd_bind_descriptor_sets2: PFN_vkCmdBindDescriptorSets2,
    cmd_push_constants2: PFN_vkCmdPushConstants2,
    cmd_push_descriptor_set2: PFN_vkCmdPushDescriptorSet2,
    cmd_push_descriptor_set_with_template2: PFN_vkCmdPushDescriptorSetWithTemplate2,
    cmd_set_line_stipple: PFN_vkCmdSetLineStipple,
    cmd_bind_index_buffer2: PFN_vkCmdBindIndexBuffer2,
    get_rendering_area_granularity: PFN_vkGetRenderingAreaGranularity,
    cmd_set_rendering_attachment_locations: PFN_vkCmdSetRenderingAttachmentLocations,
    cmd_set_rendering_input_attachment_indices: PFN_vkCmdSetRenderingInputAttachmentIndices,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                map_memory2: transmute(load(c"vkMapMemory2").ok_or(LoadingError)?),
                unmap_memory2: transmute(load(c"vkUnmapMemory2").ok_or(LoadingError)?),
                get_device_image_subresource_layout: transmute(
                    load(c"vkGetDeviceImageSubresourceLayout").ok_or(LoadingError)?,
                ),
                get_image_subresource_layout2: transmute(
                    load(c"vkGetImageSubresourceLayout2").ok_or(LoadingError)?,
                ),
                copy_memory_to_image: transmute(load(c"vkCopyMemoryToImage").ok_or(LoadingError)?),
                copy_image_to_memory: transmute(load(c"vkCopyImageToMemory").ok_or(LoadingError)?),
                copy_image_to_image: transmute(load(c"vkCopyImageToImage").ok_or(LoadingError)?),
                transition_image_layout: transmute(
                    load(c"vkTransitionImageLayout").ok_or(LoadingError)?,
                ),
                cmd_push_descriptor_set: transmute(
                    load(c"vkCmdPushDescriptorSet").ok_or(LoadingError)?,
                ),
                cmd_push_descriptor_set_with_template: transmute(
                    load(c"vkCmdPushDescriptorSetWithTemplate").ok_or(LoadingError)?,
                ),
                cmd_bind_descriptor_sets2: transmute(
                    load(c"vkCmdBindDescriptorSets2").ok_or(LoadingError)?,
                ),
                cmd_push_constants2: transmute(load(c"vkCmdPushConstants2").ok_or(LoadingError)?),
                cmd_push_descriptor_set2: transmute(
                    load(c"vkCmdPushDescriptorSet2").ok_or(LoadingError)?,
                ),
                cmd_push_descriptor_set_with_template2: transmute(
                    load(c"vkCmdPushDescriptorSetWithTemplate2").ok_or(LoadingError)?,
                ),
                cmd_set_line_stipple: transmute(load(c"vkCmdSetLineStipple").ok_or(LoadingError)?),
                cmd_bind_index_buffer2: transmute(
                    load(c"vkCmdBindIndexBuffer2").ok_or(LoadingError)?,
                ),
                get_rendering_area_granularity: transmute(
                    load(c"vkGetRenderingAreaGranularity").ok_or(LoadingError)?,
                ),
                cmd_set_rendering_attachment_locations: transmute(
                    load(c"vkCmdSetRenderingAttachmentLocations").ok_or(LoadingError)?,
                ),
                cmd_set_rendering_input_attachment_indices: transmute(
                    load(c"vkCmdSetRenderingInputAttachmentIndices").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn map_memory2(
        &self,
        device: Device,
        memory_map_info: &MemoryMapInfo<'_>,
        data: &mut *mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.map_memory2)(device, memory_map_info, data);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn unmap_memory2(
        &self,
        device: Device,
        memory_unmap_info: &MemoryUnmapInfo<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.unmap_memory2)(device, memory_unmap_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_device_image_subresource_layout(
        &self,
        device: Device,
        info: &DeviceImageSubresourceInfo<'_>,
    ) -> SubresourceLayout2<'_> {
        unsafe {
            let mut layout = core::mem::MaybeUninit::uninit();
            (self.get_device_image_subresource_layout)(device, info, layout.as_mut_ptr());
            layout.assume_init()
        }
    }
    pub unsafe fn get_image_subresource_layout2(
        &self,
        device: Device,
        image: Image,
        subresource: &ImageSubresource2<'_>,
    ) -> SubresourceLayout2<'_> {
        unsafe {
            let mut layout = core::mem::MaybeUninit::uninit();
            (self.get_image_subresource_layout2)(device, image, subresource, layout.as_mut_ptr());
            layout.assume_init()
        }
    }
    pub unsafe fn copy_memory_to_image(
        &self,
        device: Device,
        copy_memory_to_image_info: &CopyMemoryToImageInfo<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.copy_memory_to_image)(device, copy_memory_to_image_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn copy_image_to_memory(
        &self,
        device: Device,
        copy_image_to_memory_info: &CopyImageToMemoryInfo<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.copy_image_to_memory)(device, copy_image_to_memory_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn copy_image_to_image(
        &self,
        device: Device,
        copy_image_to_image_info: &CopyImageToImageInfo<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.copy_image_to_image)(device, copy_image_to_image_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn transition_image_layout(
        &self,
        device: Device,
        transitions: &[HostImageLayoutTransitionInfo<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.transition_image_layout)(
                device,
                transitions.len().try_into().unwrap(),
                transitions.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn cmd_push_descriptor_set(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_writes: &[WriteDescriptorSet<'_>],
    ) {
        unsafe {
            (self.cmd_push_descriptor_set)(
                command_buffer,
                pipeline_bind_point,
                layout,
                set,
                descriptor_writes.len().try_into().unwrap(),
                descriptor_writes.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_push_descriptor_set_with_template(
        &self,
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        data: &c_void,
    ) {
        unsafe {
            (self.cmd_push_descriptor_set_with_template)(
                command_buffer,
                descriptor_update_template,
                layout,
                set,
                data,
            )
        }
    }
    pub unsafe fn cmd_bind_descriptor_sets2(
        &self,
        command_buffer: CommandBuffer,
        bind_descriptor_sets_info: &BindDescriptorSetsInfo<'_>,
    ) {
        unsafe { (self.cmd_bind_descriptor_sets2)(command_buffer, bind_descriptor_sets_info) }
    }
    pub unsafe fn cmd_push_constants2(
        &self,
        command_buffer: CommandBuffer,
        push_constants_info: &PushConstantsInfo<'_>,
    ) {
        unsafe { (self.cmd_push_constants2)(command_buffer, push_constants_info) }
    }
    pub unsafe fn cmd_push_descriptor_set2(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_info: &PushDescriptorSetInfo<'_>,
    ) {
        unsafe { (self.cmd_push_descriptor_set2)(command_buffer, push_descriptor_set_info) }
    }
    pub unsafe fn cmd_push_descriptor_set_with_template2(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo<'_>,
    ) {
        unsafe {
            (self.cmd_push_descriptor_set_with_template2)(
                command_buffer,
                push_descriptor_set_with_template_info,
            )
        }
    }
    pub unsafe fn cmd_set_line_stipple(
        &self,
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        unsafe {
            (self.cmd_set_line_stipple)(command_buffer, line_stipple_factor, line_stipple_pattern)
        }
    }
    pub unsafe fn cmd_bind_index_buffer2(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ) {
        unsafe { (self.cmd_bind_index_buffer2)(command_buffer, buffer, offset, size, index_type) }
    }
    pub unsafe fn get_rendering_area_granularity(
        &self,
        device: Device,
        rendering_area_info: &RenderingAreaInfo<'_>,
    ) -> Extent2D {
        unsafe {
            let mut granularity = core::mem::MaybeUninit::uninit();
            (self.get_rendering_area_granularity)(
                device,
                rendering_area_info,
                granularity.as_mut_ptr(),
            );
            granularity.assume_init()
        }
    }
    pub unsafe fn cmd_set_rendering_attachment_locations(
        &self,
        command_buffer: CommandBuffer,
        location_info: &RenderingAttachmentLocationInfo<'_>,
    ) {
        unsafe { (self.cmd_set_rendering_attachment_locations)(command_buffer, location_info) }
    }
    pub unsafe fn cmd_set_rendering_input_attachment_indices(
        &self,
        command_buffer: CommandBuffer,
        input_attachment_index_info: &RenderingInputAttachmentIndexInfo<'_>,
    ) {
        unsafe {
            (self.cmd_set_rendering_input_attachment_indices)(
                command_buffer,
                input_attachment_index_info,
            )
        }
    }
}

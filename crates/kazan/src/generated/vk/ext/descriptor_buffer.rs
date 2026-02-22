#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_descriptor_set_layout_size_ext: PFN_vkGetDescriptorSetLayoutSizeEXT,
    get_descriptor_set_layout_binding_offset_ext: PFN_vkGetDescriptorSetLayoutBindingOffsetEXT,
    get_descriptor_ext: PFN_vkGetDescriptorEXT,
    cmd_bind_descriptor_buffers_ext: PFN_vkCmdBindDescriptorBuffersEXT,
    cmd_set_descriptor_buffer_offsets_ext: PFN_vkCmdSetDescriptorBufferOffsetsEXT,
    cmd_bind_descriptor_buffer_embedded_samplers_ext:
        PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT,
    get_buffer_opaque_capture_descriptor_data_ext: PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT,
    get_image_opaque_capture_descriptor_data_ext: PFN_vkGetImageOpaqueCaptureDescriptorDataEXT,
    get_image_view_opaque_capture_descriptor_data_ext:
        PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT,
    get_sampler_opaque_capture_descriptor_data_ext: PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT,
    get_acceleration_structure_opaque_capture_descriptor_data_ext:
        Option<PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_descriptor_set_layout_size_ext: transmute(
                    load(c"vkGetDescriptorSetLayoutSizeEXT").ok_or(LoadingError)?,
                ),
                get_descriptor_set_layout_binding_offset_ext: transmute(
                    load(c"vkGetDescriptorSetLayoutBindingOffsetEXT").ok_or(LoadingError)?,
                ),
                get_descriptor_ext: transmute(load(c"vkGetDescriptorEXT").ok_or(LoadingError)?),
                cmd_bind_descriptor_buffers_ext: transmute(
                    load(c"vkCmdBindDescriptorBuffersEXT").ok_or(LoadingError)?,
                ),
                cmd_set_descriptor_buffer_offsets_ext: transmute(
                    load(c"vkCmdSetDescriptorBufferOffsetsEXT").ok_or(LoadingError)?,
                ),
                cmd_bind_descriptor_buffer_embedded_samplers_ext: transmute(
                    load(c"vkCmdBindDescriptorBufferEmbeddedSamplersEXT").ok_or(LoadingError)?,
                ),
                get_buffer_opaque_capture_descriptor_data_ext: transmute(
                    load(c"vkGetBufferOpaqueCaptureDescriptorDataEXT").ok_or(LoadingError)?,
                ),
                get_image_opaque_capture_descriptor_data_ext: transmute(
                    load(c"vkGetImageOpaqueCaptureDescriptorDataEXT").ok_or(LoadingError)?,
                ),
                get_image_view_opaque_capture_descriptor_data_ext: transmute(
                    load(c"vkGetImageViewOpaqueCaptureDescriptorDataEXT").ok_or(LoadingError)?,
                ),
                get_sampler_opaque_capture_descriptor_data_ext: transmute(
                    load(c"vkGetSamplerOpaqueCaptureDescriptorDataEXT").ok_or(LoadingError)?,
                ),
                get_acceleration_structure_opaque_capture_descriptor_data_ext: transmute(load(
                    c"vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT",
                )),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_descriptor_set_layout_size_ext(
        &self,
        device: Device,
        layout: DescriptorSetLayout,
        layout_size_in_bytes: &mut DeviceSize,
    ) {
        unsafe { (self.get_descriptor_set_layout_size_ext)(device, layout, layout_size_in_bytes) }
    }
    pub unsafe fn get_descriptor_set_layout_binding_offset_ext(
        &self,
        device: Device,
        layout: DescriptorSetLayout,
        binding: u32,
        offset: &mut DeviceSize,
    ) {
        unsafe {
            (self.get_descriptor_set_layout_binding_offset_ext)(device, layout, binding, offset)
        }
    }
    pub unsafe fn get_descriptor_ext(
        &self,
        device: Device,
        descriptor_info: &DescriptorGetInfoEXT,
        descriptor: &mut [u8],
    ) {
        unsafe {
            (self.get_descriptor_ext)(
                device,
                descriptor_info,
                descriptor.len().try_into().unwrap(),
                descriptor.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_bind_descriptor_buffers_ext(
        &self,
        command_buffer: CommandBuffer,
        binding_infos: &[DescriptorBufferBindingInfoEXT],
    ) {
        unsafe {
            (self.cmd_bind_descriptor_buffers_ext)(
                command_buffer,
                binding_infos.len().try_into().unwrap(),
                binding_infos.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_descriptor_buffer_offsets_ext(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        buffer_indices: &[u32],
        offsets: &[DeviceSize],
    ) {
        unsafe {
            (self.cmd_set_descriptor_buffer_offsets_ext)(
                command_buffer,
                pipeline_bind_point,
                layout,
                first_set,
                buffer_indices.len().try_into().unwrap(),
                buffer_indices.as_ptr() as _,
                offsets.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers_ext(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
    ) {
        unsafe {
            (self.cmd_bind_descriptor_buffer_embedded_samplers_ext)(
                command_buffer,
                pipeline_bind_point,
                layout,
                set,
            )
        }
    }
    pub unsafe fn get_buffer_opaque_capture_descriptor_data_ext(
        &self,
        device: Device,
        info: &BufferCaptureDescriptorDataInfoEXT,
        data: &mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_buffer_opaque_capture_descriptor_data_ext)(
                device, info, data,
            ))
        }
    }
    pub unsafe fn get_image_opaque_capture_descriptor_data_ext(
        &self,
        device: Device,
        info: &ImageCaptureDescriptorDataInfoEXT,
        data: &mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_image_opaque_capture_descriptor_data_ext)(
                device, info, data,
            ))
        }
    }
    pub unsafe fn get_image_view_opaque_capture_descriptor_data_ext(
        &self,
        device: Device,
        info: &ImageViewCaptureDescriptorDataInfoEXT,
        data: &mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_image_view_opaque_capture_descriptor_data_ext)(
                device, info, data,
            ))
        }
    }
    pub unsafe fn get_sampler_opaque_capture_descriptor_data_ext(
        &self,
        device: Device,
        info: &SamplerCaptureDescriptorDataInfoEXT,
        data: &mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_sampler_opaque_capture_descriptor_data_ext)(
                device, info, data,
            ))
        }
    }
    pub unsafe fn get_acceleration_structure_opaque_capture_descriptor_data_ext(
        &self,
        device: Device,
        info: &AccelerationStructureCaptureDescriptorDataInfoEXT,
        data: &mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            result((self
                .get_acceleration_structure_opaque_capture_descriptor_data_ext
                .unwrap())(device, info, data))
        }
    }
}

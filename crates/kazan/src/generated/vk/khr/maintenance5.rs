#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferUsageFlags2CreateInfoKHR.html>
    pub type BufferUsageFlags2CreateInfoKHR<'a> = BufferUsageFlags2CreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCreateFlags2CreateInfoKHR.html>
    pub type PipelineCreateFlags2CreateInfoKHR<'a> = PipelineCreateFlags2CreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance5FeaturesKHR.html>
    pub type PhysicalDeviceMaintenance5FeaturesKHR<'a> = PhysicalDeviceMaintenance5Features<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance5PropertiesKHR.html>
    pub type PhysicalDeviceMaintenance5PropertiesKHR<'a> = PhysicalDeviceMaintenance5Properties<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingAreaInfoKHR.html>
    pub type RenderingAreaInfoKHR<'a> = RenderingAreaInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageSubresource2KHR.html>
    pub type ImageSubresource2KHR<'a> = ImageSubresource2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubresourceLayout2KHR.html>
    pub type SubresourceLayout2KHR<'a> = SubresourceLayout2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceImageSubresourceInfoKHR.html>
    pub type DeviceImageSubresourceInfoKHR<'a> = DeviceImageSubresourceInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCreateFlags2KHR.html>
    pub type PipelineCreateFlags2KHR = PipelineCreateFlags2;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferUsageFlags2KHR.html>
    pub type BufferUsageFlags2KHR = BufferUsageFlags2;
    pub type PFN_vkGetRenderingAreaGranularityKHR = PFN_vkGetRenderingAreaGranularity;
    pub type PFN_vkCmdBindIndexBuffer2KHR = PFN_vkCmdBindIndexBuffer2;
    pub type PFN_vkGetImageSubresourceLayout2KHR = PFN_vkGetImageSubresourceLayout2;
    pub type PFN_vkGetDeviceImageSubresourceLayoutKHR = PFN_vkGetDeviceImageSubresourceLayout;
}
pub struct DeviceFn {
    cmd_bind_index_buffer2_khr: PFN_vkCmdBindIndexBuffer2,
    get_rendering_area_granularity_khr: PFN_vkGetRenderingAreaGranularity,
    get_device_image_subresource_layout_khr: PFN_vkGetDeviceImageSubresourceLayout,
    get_image_subresource_layout2_khr: PFN_vkGetImageSubresourceLayout2,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_bind_index_buffer2_khr: transmute(
                    load(c"vkCmdBindIndexBuffer2KHR").ok_or(MissingEntryPointError)?,
                ),
                get_rendering_area_granularity_khr: transmute(
                    load(c"vkGetRenderingAreaGranularityKHR").ok_or(MissingEntryPointError)?,
                ),
                get_device_image_subresource_layout_khr: transmute(
                    load(c"vkGetDeviceImageSubresourceLayoutKHR").ok_or(MissingEntryPointError)?,
                ),
                get_image_subresource_layout2_khr: transmute(
                    load(c"vkGetImageSubresourceLayout2KHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindIndexBuffer2KHR.html>
    pub unsafe fn cmd_bind_index_buffer2_khr(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ) {
        unsafe {
            (self.cmd_bind_index_buffer2_khr)(command_buffer, buffer, offset, size, index_type)
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetRenderingAreaGranularityKHR.html>
    pub unsafe fn get_rendering_area_granularity_khr(
        &self,
        device: Device,
        rendering_area_info: &RenderingAreaInfo<'_>,
    ) -> Extent2D {
        unsafe {
            let mut granularity = core::mem::MaybeUninit::uninit();
            (self.get_rendering_area_granularity_khr)(
                device,
                rendering_area_info,
                granularity.as_mut_ptr(),
            );
            granularity.assume_init()
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceImageSubresourceLayoutKHR.html>
    pub unsafe fn get_device_image_subresource_layout_khr(
        &self,
        device: Device,
        info: &DeviceImageSubresourceInfo<'_>,
        layout: &mut SubresourceLayout2<'_>,
    ) {
        unsafe { (self.get_device_image_subresource_layout_khr)(device, info, layout) }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageSubresourceLayout2KHR.html>
    pub unsafe fn get_image_subresource_layout2_khr(
        &self,
        device: Device,
        image: Image,
        subresource: &ImageSubresource2<'_>,
        layout: &mut SubresourceLayout2<'_>,
    ) {
        unsafe { (self.get_image_subresource_layout2_khr)(device, image, subresource, layout) }
    }
}

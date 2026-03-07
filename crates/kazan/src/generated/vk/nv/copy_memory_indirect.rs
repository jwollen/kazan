#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_copy_memory_indirect";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyMemoryIndirectCommandNV.html>
    pub type CopyMemoryIndirectCommandNV = CopyMemoryIndirectCommandKHR;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyMemoryToImageIndirectCommandNV.html>
    pub type CopyMemoryToImageIndirectCommandNV = CopyMemoryToImageIndirectCommandKHR;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCopyMemoryIndirectPropertiesNV.html>
    pub type PhysicalDeviceCopyMemoryIndirectPropertiesNV<'a> =
        PhysicalDeviceCopyMemoryIndirectPropertiesKHR<'a>;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCopyMemoryIndirectFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceCopyMemoryIndirectFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub indirect_copy: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceCopyMemoryIndirectFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCopyMemoryIndirectFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("indirect_copy", &self.indirect_copy)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCopyMemoryIndirectFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceCopyMemoryIndirectFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceCopyMemoryIndirectFeaturesNV<'a> {}

    impl Default for PhysicalDeviceCopyMemoryIndirectFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                indirect_copy: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCopyMemoryIndirectFeaturesNV<'a> {
        pub fn indirect_copy(mut self, indirect_copy: bool) -> Self {
            self.indirect_copy = indirect_copy.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryIndirectNV.html>
    pub type PFN_vkCmdCopyMemoryIndirectNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        copy_buffer_address: DeviceAddress,
        copy_count: u32,
        stride: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryToImageIndirectNV.html>
    pub type PFN_vkCmdCopyMemoryToImageIndirectNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        copy_buffer_address: DeviceAddress,
        copy_count: u32,
        stride: u32,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        p_image_subresources: *const ImageSubresourceLayers,
    );
}

pub struct DeviceFn {
    cmd_copy_memory_indirect_nv: PFN_vkCmdCopyMemoryIndirectNV,
    cmd_copy_memory_to_image_indirect_nv: PFN_vkCmdCopyMemoryToImageIndirectNV,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_copy_memory_indirect_nv: transmute(
                    load(c"vkCmdCopyMemoryIndirectNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_memory_to_image_indirect_nv: transmute(
                    load(c"vkCmdCopyMemoryToImageIndirectNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryIndirectNV.html>
    pub unsafe fn cmd_copy_memory_indirect_nv(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_address: DeviceAddress,
        copy_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_copy_memory_indirect_nv)(
                command_buffer,
                copy_buffer_address,
                copy_count,
                stride,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryToImageIndirectNV.html>
    pub unsafe fn cmd_copy_memory_to_image_indirect_nv(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_address: DeviceAddress,
        stride: u32,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        image_subresources: &[ImageSubresourceLayers],
    ) {
        unsafe {
            (self.cmd_copy_memory_to_image_indirect_nv)(
                command_buffer,
                copy_buffer_address,
                image_subresources.len().try_into().unwrap(),
                stride,
                dst_image,
                dst_image_layout,
                image_subresources.as_ptr() as _,
            )
        }
    }
}

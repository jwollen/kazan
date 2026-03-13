//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_copy_memory_indirect.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_copy_memory_indirect";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

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
    #[must_use]
    pub struct PhysicalDeviceCopyMemoryIndirectFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub indirect_copy: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
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

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceCopyMemoryIndirectFeaturesNV<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceCopyMemoryIndirectFeaturesNV<'_> {}

    impl Default for PhysicalDeviceCopyMemoryIndirectFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                indirect_copy: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCopyMemoryIndirectFeaturesNV<'a> {
        #[inline]
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

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceCopyMemoryIndirectFeaturesNV =
        PhysicalDeviceCopyMemoryIndirectFeaturesNV<'static>;
    pub type VkCopyMemoryIndirectCommandNV = CopyMemoryIndirectCommandNV;
    pub type VkCopyMemoryToImageIndirectCommandNV = CopyMemoryToImageIndirectCommandNV;
    pub type VkPhysicalDeviceCopyMemoryIndirectPropertiesNV =
        PhysicalDeviceCopyMemoryIndirectPropertiesNV<'static>;
    impl PhysicalDeviceCopyMemoryIndirectFeaturesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceCopyMemoryIndirectFeaturesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    cmd_copy_memory_indirect: PFN_vkCmdCopyMemoryIndirectNV,
    cmd_copy_memory_to_image_indirect: PFN_vkCmdCopyMemoryToImageIndirectNV,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_copy_memory_indirect: transmute(
                    load(c"vkCmdCopyMemoryIndirectNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_memory_to_image_indirect: transmute(
                    load(c"vkCmdCopyMemoryToImageIndirectNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryIndirectNV.html>
    #[inline]
    pub unsafe fn cmd_copy_memory_indirect(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_address: DeviceAddress,
        copy_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_copy_memory_indirect)(command_buffer, copy_buffer_address, copy_count, stride)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyMemoryToImageIndirectNV.html>
    #[inline]
    pub unsafe fn cmd_copy_memory_to_image_indirect(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_address: DeviceAddress,
        stride: u32,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        image_subresources: &[ImageSubresourceLayers],
    ) {
        unsafe {
            (self.cmd_copy_memory_to_image_indirect)(
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

//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_primitive_restart_index.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_primitive_restart_index";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePrimitiveRestartIndexFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePrimitiveRestartIndexFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub primitive_restart_index: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePrimitiveRestartIndexFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePrimitiveRestartIndexFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("primitive_restart_index", &self.primitive_restart_index)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePrimitiveRestartIndexFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PRIMITIVE_RESTART_INDEX_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDevicePrimitiveRestartIndexFeaturesEXT<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDevicePrimitiveRestartIndexFeaturesEXT<'_> {}

    impl Default for PhysicalDevicePrimitiveRestartIndexFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                primitive_restart_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePrimitiveRestartIndexFeaturesEXT<'a> {
        #[inline]
        pub fn primitive_restart_index(mut self, primitive_restart_index: bool) -> Self {
            self.primitive_restart_index = primitive_restart_index.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPrimitiveRestartIndexEXT.html>
    pub type PFN_vkCmdSetPrimitiveRestartIndexEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_restart_index: u32);
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDevicePrimitiveRestartIndexFeaturesEXT =
        PhysicalDevicePrimitiveRestartIndexFeaturesEXT<'static>;
    impl PhysicalDevicePrimitiveRestartIndexFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDevicePrimitiveRestartIndexFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    cmd_set_primitive_restart_index: PFN_vkCmdSetPrimitiveRestartIndexEXT,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_primitive_restart_index: transmute(
                    load(c"vkCmdSetPrimitiveRestartIndexEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPrimitiveRestartIndexEXT.html>
    #[inline]
    pub unsafe fn cmd_set_primitive_restart_index(
        &self,
        command_buffer: CommandBuffer,
        primitive_restart_index: u32,
    ) {
        unsafe { (self.cmd_set_primitive_restart_index)(command_buffer, primitive_restart_index) }
    }
}

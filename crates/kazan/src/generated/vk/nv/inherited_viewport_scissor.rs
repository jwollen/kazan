//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_inherited_viewport_scissor.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_inherited_viewport_scissor";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceInheritedViewportScissorFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceInheritedViewportScissorFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub inherited_viewport_scissor2_d: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceInheritedViewportScissorFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceInheritedViewportScissorFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "inherited_viewport_scissor2_d",
                    &self.inherited_viewport_scissor2_d,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceInheritedViewportScissorFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceInheritedViewportScissorFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceInheritedViewportScissorFeaturesNV<'a>
    {
    }

    impl Default for PhysicalDeviceInheritedViewportScissorFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                inherited_viewport_scissor2_d: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceInheritedViewportScissorFeaturesNV<'a> {
        #[inline]
        pub fn inherited_viewport_scissor2_d(
            mut self,
            inherited_viewport_scissor2_d: bool,
        ) -> Self {
            self.inherited_viewport_scissor2_d = inherited_viewport_scissor2_d.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCommandBufferInheritanceViewportScissorInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CommandBufferInheritanceViewportScissorInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub viewport_scissor2_d: Bool32,
        pub viewport_depth_count: u32,
        pub p_viewport_depths: *const Viewport,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CommandBufferInheritanceViewportScissorInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CommandBufferInheritanceViewportScissorInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("viewport_scissor2_d", &self.viewport_scissor2_d)
                .field("viewport_depth_count", &self.viewport_depth_count)
                .field("p_viewport_depths", &self.p_viewport_depths)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CommandBufferInheritanceViewportScissorInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV;
    }

    unsafe impl<'a> Extends<CommandBufferInheritanceInfo<'a>>
        for CommandBufferInheritanceViewportScissorInfoNV<'a>
    {
    }

    impl Default for CommandBufferInheritanceViewportScissorInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                viewport_scissor2_d: Default::default(),
                viewport_depth_count: Default::default(),
                p_viewport_depths: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CommandBufferInheritanceViewportScissorInfoNV<'a> {
        #[inline]
        pub fn viewport_scissor2_d(mut self, viewport_scissor2_d: bool) -> Self {
            self.viewport_scissor2_d = viewport_scissor2_d.into();
            self
        }

        #[inline]
        pub fn viewport_depth_count(mut self, viewport_depth_count: u32) -> Self {
            self.viewport_depth_count = viewport_depth_count;
            self
        }

        #[inline]
        pub fn viewport_depths(mut self, viewport_depths: &'a Viewport) -> Self {
            self.p_viewport_depths = viewport_depths;
            self
        }
    }
}

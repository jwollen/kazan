#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceInheritedViewportScissorFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub inherited_viewport_scissor2_d: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
        pub fn inherited_viewport_scissor2_d(
            mut self,
            inherited_viewport_scissor2_d: Bool32,
        ) -> Self {
            self.inherited_viewport_scissor2_d = inherited_viewport_scissor2_d;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CommandBufferInheritanceViewportScissorInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub viewport_scissor2_d: Bool32,
        pub viewport_depth_count: u32,
        pub p_viewport_depths: *const Viewport,
        pub _marker: PhantomData<&'a ()>,
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
        pub fn viewport_scissor2_d(mut self, viewport_scissor2_d: Bool32) -> Self {
            self.viewport_scissor2_d = viewport_scissor2_d;
            self
        }
        pub fn viewport_depth_count(mut self, viewport_depth_count: u32) -> Self {
            self.viewport_depth_count = viewport_depth_count;
            self
        }
        pub fn viewport_depths(mut self, viewport_depths: &'a Viewport) -> Self {
            self.p_viewport_depths = viewport_depths;
            self
        }
    }
}

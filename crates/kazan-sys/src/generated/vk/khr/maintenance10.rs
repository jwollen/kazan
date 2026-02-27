#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance10PropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub rgba4_opaque_black_swizzled: Bool32,
    pub resolve_srgb_format_applies_transfer_function: Bool32,
    pub resolve_srgb_format_supports_transfer_function_control: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMaintenance10PropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MAINTENANCE_10_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            rgba4_opaque_black_swizzled: Default::default(),
            resolve_srgb_format_applies_transfer_function: Default::default(),
            resolve_srgb_format_supports_transfer_function_control: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceMaintenance10PropertiesKHR<'a> {
    pub fn rgba4_opaque_black_swizzled(mut self, rgba4_opaque_black_swizzled: Bool32) -> Self {
        self.rgba4_opaque_black_swizzled = rgba4_opaque_black_swizzled;
        self
    }
    pub fn resolve_srgb_format_applies_transfer_function(
        mut self,
        resolve_srgb_format_applies_transfer_function: Bool32,
    ) -> Self {
        self.resolve_srgb_format_applies_transfer_function =
            resolve_srgb_format_applies_transfer_function;
        self
    }
    pub fn resolve_srgb_format_supports_transfer_function_control(
        mut self,
        resolve_srgb_format_supports_transfer_function_control: Bool32,
    ) -> Self {
        self.resolve_srgb_format_supports_transfer_function_control =
            resolve_srgb_format_supports_transfer_function_control;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance10FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance10: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMaintenance10FeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MAINTENANCE_10_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            maintenance10: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceMaintenance10FeaturesKHR<'a> {
    pub fn maintenance10(mut self, maintenance10: Bool32) -> Self {
        self.maintenance10 = maintenance10;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderingEndInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RenderingEndInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDERING_END_INFO_KHR,
            p_next: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> RenderingEndInfoKHR<'a> {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderingAttachmentFlagsInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderingAttachmentFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RenderingAttachmentFlagsInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDERING_ATTACHMENT_FLAGS_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> RenderingAttachmentFlagsInfoKHR<'a> {
    pub fn flags(mut self, flags: RenderingAttachmentFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ResolveImageModeInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ResolveImageFlagsKHR,
    pub resolve_mode: ResolveModeFlagBits,
    pub stencil_resolve_mode: ResolveModeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ResolveImageModeInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RESOLVE_IMAGE_MODE_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            resolve_mode: Default::default(),
            stencil_resolve_mode: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ResolveImageModeInfoKHR<'a> {
    pub fn flags(mut self, flags: ResolveImageFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    pub fn resolve_mode(mut self, resolve_mode: ResolveModeFlagBits) -> Self {
        self.resolve_mode = resolve_mode;
        self
    }
    pub fn stencil_resolve_mode(mut self, stencil_resolve_mode: ResolveModeFlagBits) -> Self {
        self.stencil_resolve_mode = stencil_resolve_mode;
        self
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RenderingAttachmentFlagsKHR: Flags {
        const INPUT_ATTACHMENT_FEEDBACK_KHR = RenderingAttachmentFlagBitsKHR::INPUT_ATTACHMENT_FEEDBACK_KHR.0;
        const RESOLVE_SKIP_TRANSFER_FUNCTION_KHR = RenderingAttachmentFlagBitsKHR::RESOLVE_SKIP_TRANSFER_FUNCTION_KHR.0;
        const RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR = RenderingAttachmentFlagBitsKHR::RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderingAttachmentFlagBitsKHR(u32);
impl RenderingAttachmentFlagBitsKHR {
    pub const INPUT_ATTACHMENT_FEEDBACK_KHR: Self = Self(1 << 0);
    pub const RESOLVE_SKIP_TRANSFER_FUNCTION_KHR: Self = Self(1 << 1);
    pub const RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ResolveImageFlagsKHR: Flags {
        const SKIP_TRANSFER_FUNCTION_KHR = ResolveImageFlagBitsKHR::SKIP_TRANSFER_FUNCTION_KHR.0;
        const ENABLE_TRANSFER_FUNCTION_KHR = ResolveImageFlagBitsKHR::ENABLE_TRANSFER_FUNCTION_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResolveImageFlagBitsKHR(u32);
impl ResolveImageFlagBitsKHR {
    pub const SKIP_TRANSFER_FUNCTION_KHR: Self = Self(1 << 0);
    pub const ENABLE_TRANSFER_FUNCTION_KHR: Self = Self(1 << 1);
}
pub type PFN_vkCmdEndRendering2KHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_rendering_end_info: *const RenderingEndInfoKHR<'_>,
);

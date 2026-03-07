//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_maintenance10.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_maintenance10";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance10PropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMaintenance10PropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub rgba4_opaque_black_swizzled: Bool32,
        pub resolve_srgb_format_applies_transfer_function: Bool32,
        pub resolve_srgb_format_supports_transfer_function_control: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMaintenance10PropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMaintenance10PropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "rgba4_opaque_black_swizzled",
                    &self.rgba4_opaque_black_swizzled,
                )
                .field(
                    "resolve_srgb_format_applies_transfer_function",
                    &self.resolve_srgb_format_applies_transfer_function,
                )
                .field(
                    "resolve_srgb_format_supports_transfer_function_control",
                    &self.resolve_srgb_format_supports_transfer_function_control,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance10PropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MAINTENANCE_10_PROPERTIES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceMaintenance10PropertiesKHR<'a>
    {
    }

    impl Default for PhysicalDeviceMaintenance10PropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                rgba4_opaque_black_swizzled: Default::default(),
                resolve_srgb_format_applies_transfer_function: Default::default(),
                resolve_srgb_format_supports_transfer_function_control: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMaintenance10PropertiesKHR<'a> {
        #[inline]
        pub fn rgba4_opaque_black_swizzled(mut self, rgba4_opaque_black_swizzled: bool) -> Self {
            self.rgba4_opaque_black_swizzled = rgba4_opaque_black_swizzled.into();
            self
        }

        #[inline]
        pub fn resolve_srgb_format_applies_transfer_function(
            mut self,
            resolve_srgb_format_applies_transfer_function: bool,
        ) -> Self {
            self.resolve_srgb_format_applies_transfer_function =
                resolve_srgb_format_applies_transfer_function.into();
            self
        }

        #[inline]
        pub fn resolve_srgb_format_supports_transfer_function_control(
            mut self,
            resolve_srgb_format_supports_transfer_function_control: bool,
        ) -> Self {
            self.resolve_srgb_format_supports_transfer_function_control =
                resolve_srgb_format_supports_transfer_function_control.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance10FeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMaintenance10FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub maintenance10: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMaintenance10FeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMaintenance10FeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("maintenance10", &self.maintenance10)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance10FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MAINTENANCE_10_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceMaintenance10FeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceMaintenance10FeaturesKHR<'a> {}

    impl Default for PhysicalDeviceMaintenance10FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                maintenance10: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMaintenance10FeaturesKHR<'a> {
        #[inline]
        pub fn maintenance10(mut self, maintenance10: bool) -> Self {
            self.maintenance10 = maintenance10.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingEndInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RenderingEndInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderingEndInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderingEndInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderingEndInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDERING_END_INFO_KHR;
    }

    impl Default for RenderingEndInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RenderingEndInfoKHR<'a> {}

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingAttachmentFlagsInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RenderingAttachmentFlagsInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: RenderingAttachmentFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderingAttachmentFlagsInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderingAttachmentFlagsInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderingAttachmentFlagsInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDERING_ATTACHMENT_FLAGS_INFO_KHR;
    }

    unsafe impl<'a> Extends<RenderingAttachmentInfo<'a>> for RenderingAttachmentFlagsInfoKHR<'a> {}

    impl Default for RenderingAttachmentFlagsInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RenderingAttachmentFlagsInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: RenderingAttachmentFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkResolveImageModeInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ResolveImageModeInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: ResolveImageFlagsKHR,
        pub resolve_mode: ResolveModeFlagBits,
        pub stencil_resolve_mode: ResolveModeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ResolveImageModeInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ResolveImageModeInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("resolve_mode", &self.resolve_mode)
                .field("stencil_resolve_mode", &self.stencil_resolve_mode)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ResolveImageModeInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RESOLVE_IMAGE_MODE_INFO_KHR;
    }

    unsafe impl<'a> Extends<ResolveImageInfo2<'a>> for ResolveImageModeInfoKHR<'a> {}

    impl Default for ResolveImageModeInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                resolve_mode: Default::default(),
                stencil_resolve_mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ResolveImageModeInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: ResolveImageFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn resolve_mode(mut self, resolve_mode: ResolveModeFlagBits) -> Self {
            self.resolve_mode = resolve_mode;
            self
        }

        #[inline]
        pub fn stencil_resolve_mode(mut self, stencil_resolve_mode: ResolveModeFlagBits) -> Self {
            self.stencil_resolve_mode = stencil_resolve_mode;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingAttachmentFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct RenderingAttachmentFlagsKHR(Flags);
    vk_bitflags_wrapped!(RenderingAttachmentFlagsKHR, Flags);

    impl RenderingAttachmentFlagsKHR {
        // VK_KHR_maintenance10
        pub const INPUT_ATTACHMENT_FEEDBACK_KHR: Self =
            Self(RenderingAttachmentFlagBitsKHR::INPUT_ATTACHMENT_FEEDBACK_KHR.0);
        pub const RESOLVE_SKIP_TRANSFER_FUNCTION_KHR: Self =
            Self(RenderingAttachmentFlagBitsKHR::RESOLVE_SKIP_TRANSFER_FUNCTION_KHR.0);
        pub const RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR: Self =
            Self(RenderingAttachmentFlagBitsKHR::RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR.0);
    }

    impl fmt::Debug for RenderingAttachmentFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    RenderingAttachmentFlagsKHR::INPUT_ATTACHMENT_FEEDBACK_KHR.0,
                    "INPUT_ATTACHMENT_FEEDBACK_KHR",
                ),
                (
                    RenderingAttachmentFlagsKHR::RESOLVE_SKIP_TRANSFER_FUNCTION_KHR.0,
                    "RESOLVE_SKIP_TRANSFER_FUNCTION_KHR",
                ),
                (
                    RenderingAttachmentFlagsKHR::RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR.0,
                    "RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingAttachmentFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct RenderingAttachmentFlagBitsKHR(u32);

    impl RenderingAttachmentFlagBitsKHR {
        // VK_KHR_maintenance10
        pub const INPUT_ATTACHMENT_FEEDBACK_KHR: Self = Self(1 << 0);
        pub const RESOLVE_SKIP_TRANSFER_FUNCTION_KHR: Self = Self(1 << 1);
        pub const RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR: Self = Self(1 << 2);
    }

    impl fmt::Debug for RenderingAttachmentFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::INPUT_ATTACHMENT_FEEDBACK_KHR => Some("INPUT_ATTACHMENT_FEEDBACK_KHR"),
                Self::RESOLVE_SKIP_TRANSFER_FUNCTION_KHR => {
                    Some("RESOLVE_SKIP_TRANSFER_FUNCTION_KHR")
                }
                Self::RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR => {
                    Some("RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR")
                }
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkResolveImageFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ResolveImageFlagsKHR(Flags);
    vk_bitflags_wrapped!(ResolveImageFlagsKHR, Flags);

    impl ResolveImageFlagsKHR {
        // VK_KHR_maintenance10
        pub const SKIP_TRANSFER_FUNCTION_KHR: Self =
            Self(ResolveImageFlagBitsKHR::SKIP_TRANSFER_FUNCTION_KHR.0);
        pub const ENABLE_TRANSFER_FUNCTION_KHR: Self =
            Self(ResolveImageFlagBitsKHR::ENABLE_TRANSFER_FUNCTION_KHR.0);
    }

    impl fmt::Debug for ResolveImageFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    ResolveImageFlagsKHR::SKIP_TRANSFER_FUNCTION_KHR.0,
                    "SKIP_TRANSFER_FUNCTION_KHR",
                ),
                (
                    ResolveImageFlagsKHR::ENABLE_TRANSFER_FUNCTION_KHR.0,
                    "ENABLE_TRANSFER_FUNCTION_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkResolveImageFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ResolveImageFlagBitsKHR(u32);

    impl ResolveImageFlagBitsKHR {
        // VK_KHR_maintenance10
        pub const SKIP_TRANSFER_FUNCTION_KHR: Self = Self(1 << 0);
        pub const ENABLE_TRANSFER_FUNCTION_KHR: Self = Self(1 << 1);
    }

    impl fmt::Debug for ResolveImageFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SKIP_TRANSFER_FUNCTION_KHR => Some("SKIP_TRANSFER_FUNCTION_KHR"),
                Self::ENABLE_TRANSFER_FUNCTION_KHR => Some("ENABLE_TRANSFER_FUNCTION_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndRendering2KHR.html>
    pub type PFN_vkCmdEndRendering2KHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_rendering_end_info: *const RenderingEndInfoKHR<'_>,
    );
}

pub struct DeviceFn {
    cmd_end_rendering2_khr: PFN_vkCmdEndRendering2KHR,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_end_rendering2_khr: transmute(
                    load(c"vkCmdEndRendering2KHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndRendering2KHR.html>
    #[inline]
    pub unsafe fn cmd_end_rendering2_khr(
        &self,
        command_buffer: CommandBuffer,
        rendering_end_info: Option<&RenderingEndInfoKHR<'_>>,
    ) {
        unsafe { (self.cmd_end_rendering2_khr)(command_buffer, rendering_end_info.to_raw_ptr()) }
    }
}

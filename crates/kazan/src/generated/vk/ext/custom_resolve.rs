//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_custom_resolve.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_custom_resolve";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBeginCustomResolveInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BeginCustomResolveInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BeginCustomResolveInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BeginCustomResolveInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BeginCustomResolveInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BEGIN_CUSTOM_RESOLVE_INFO_EXT;
    }

    impl Default for BeginCustomResolveInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BeginCustomResolveInfoEXT<'a> {}

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCustomResolveFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCustomResolveFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub custom_resolve: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCustomResolveFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCustomResolveFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("custom_resolve", &self.custom_resolve)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCustomResolveFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_CUSTOM_RESOLVE_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceCustomResolveFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceCustomResolveFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceCustomResolveFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                custom_resolve: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCustomResolveFeaturesEXT<'a> {
        #[inline]
        pub fn custom_resolve(mut self, custom_resolve: bool) -> Self {
            self.custom_resolve = custom_resolve.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCustomResolveCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CustomResolveCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub custom_resolve: Bool32,
        pub color_attachment_count: u32,
        pub p_color_attachment_formats: *const Format,
        pub depth_attachment_format: Format,
        pub stencil_attachment_format: Format,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CustomResolveCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CustomResolveCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("custom_resolve", &self.custom_resolve)
                .field("color_attachment_count", &self.color_attachment_count)
                .field(
                    "p_color_attachment_formats",
                    &self.p_color_attachment_formats,
                )
                .field("depth_attachment_format", &self.depth_attachment_format)
                .field("stencil_attachment_format", &self.stencil_attachment_format)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CustomResolveCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::CUSTOM_RESOLVE_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>> for CustomResolveCreateInfoEXT<'a> {}
    unsafe impl<'a> Extends<CommandBufferInheritanceInfo<'a>> for CustomResolveCreateInfoEXT<'a> {}
    unsafe impl<'a> Extends<ShaderCreateInfoEXT<'a>> for CustomResolveCreateInfoEXT<'a> {}

    impl Default for CustomResolveCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                custom_resolve: Default::default(),
                color_attachment_count: Default::default(),
                p_color_attachment_formats: core::ptr::null(),
                depth_attachment_format: Default::default(),
                stencil_attachment_format: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CustomResolveCreateInfoEXT<'a> {
        #[inline]
        pub fn custom_resolve(mut self, custom_resolve: bool) -> Self {
            self.custom_resolve = custom_resolve.into();
            self
        }

        #[inline]
        pub fn color_attachment_formats(mut self, color_attachment_formats: &'a [Format]) -> Self {
            self.color_attachment_count = color_attachment_formats.len().try_into().unwrap();
            self.p_color_attachment_formats = color_attachment_formats.as_ptr();
            self
        }

        #[inline]
        pub fn depth_attachment_format(mut self, depth_attachment_format: Format) -> Self {
            self.depth_attachment_format = depth_attachment_format;
            self
        }

        #[inline]
        pub fn stencil_attachment_format(mut self, stencil_attachment_format: Format) -> Self {
            self.stencil_attachment_format = stencil_attachment_format;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginCustomResolveEXT.html>
    pub type PFN_vkCmdBeginCustomResolveEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_begin_custom_resolve_info: *const BeginCustomResolveInfoEXT<'_>,
    );
}

pub struct DeviceFn {
    cmd_begin_custom_resolve_ext: Option<PFN_vkCmdBeginCustomResolveEXT>,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_begin_custom_resolve_ext: transmute(load(c"vkCmdBeginCustomResolveEXT")),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginCustomResolveEXT.html>
    #[inline]
    pub unsafe fn cmd_begin_custom_resolve_ext(
        &self,
        command_buffer: CommandBuffer,
        begin_custom_resolve_info: Option<&BeginCustomResolveInfoEXT<'_>>,
    ) {
        unsafe {
            (self.cmd_begin_custom_resolve_ext.unwrap())(
                command_buffer,
                begin_custom_resolve_info.to_raw_ptr(),
            )
        }
    }
}

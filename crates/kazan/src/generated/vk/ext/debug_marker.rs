//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_debug_marker.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_debug_marker";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDebugMarkerObjectNameInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DebugMarkerObjectNameInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub object_type: DebugReportObjectTypeEXT,
        pub object: u64,
        pub p_object_name: *const c_char,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DebugMarkerObjectNameInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DebugMarkerObjectNameInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("object_type", &self.object_type)
                .field("object", &self.object)
                .field("p_object_name", &unsafe { as_c_str(self.p_object_name) })
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DebugMarkerObjectNameInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEBUG_MARKER_OBJECT_NAME_INFO_EXT;
    }

    impl Default for DebugMarkerObjectNameInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                object_type: Default::default(),
                object: Default::default(),
                p_object_name: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DebugMarkerObjectNameInfoEXT<'a> {
        #[inline]
        pub fn object_type(mut self, object_type: DebugReportObjectTypeEXT) -> Self {
            self.object_type = object_type;
            self
        }

        #[inline]
        pub fn object(mut self, object: u64) -> Self {
            self.object = object;
            self
        }

        #[inline]
        pub fn object_name(mut self, object_name: &'a CStr) -> Self {
            self.p_object_name = object_name.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDebugMarkerObjectTagInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DebugMarkerObjectTagInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub object_type: DebugReportObjectTypeEXT,
        pub object: u64,
        pub tag_name: u64,
        pub tag_size: usize,
        pub p_tag: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DebugMarkerObjectTagInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DebugMarkerObjectTagInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("object_type", &self.object_type)
                .field("object", &self.object)
                .field("tag_name", &self.tag_name)
                .field("tag_size", &self.tag_size)
                .field("p_tag", &self.p_tag)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DebugMarkerObjectTagInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEBUG_MARKER_OBJECT_TAG_INFO_EXT;
    }

    impl Default for DebugMarkerObjectTagInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                object_type: Default::default(),
                object: Default::default(),
                tag_name: Default::default(),
                tag_size: Default::default(),
                p_tag: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DebugMarkerObjectTagInfoEXT<'a> {
        #[inline]
        pub fn object_type(mut self, object_type: DebugReportObjectTypeEXT) -> Self {
            self.object_type = object_type;
            self
        }

        #[inline]
        pub fn object(mut self, object: u64) -> Self {
            self.object = object;
            self
        }

        #[inline]
        pub fn tag_name(mut self, tag_name: u64) -> Self {
            self.tag_name = tag_name;
            self
        }

        #[inline]
        pub fn tag(mut self, tag: &'a [u8]) -> Self {
            self.tag_size = tag.len().try_into().unwrap();
            self.p_tag = tag.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDebugMarkerMarkerInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DebugMarkerMarkerInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_marker_name: *const c_char,
        pub color: [f32; 4],
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DebugMarkerMarkerInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DebugMarkerMarkerInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_marker_name", &unsafe { as_c_str(self.p_marker_name) })
                .field("color", &self.color)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DebugMarkerMarkerInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEBUG_MARKER_MARKER_INFO_EXT;
    }

    impl Default for DebugMarkerMarkerInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_marker_name: ptr::null(),
                color: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DebugMarkerMarkerInfoEXT<'a> {
        #[inline]
        pub fn marker_name(mut self, marker_name: &'a CStr) -> Self {
            self.p_marker_name = marker_name.as_ptr();
            self
        }

        #[inline]
        pub fn color(mut self, color: [f32; 4]) -> Self {
            self.color = color;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDebugMarkerSetObjectNameEXT.html>
    pub type PFN_vkDebugMarkerSetObjectNameEXT = unsafe extern "system" fn(
        device: Device,
        p_name_info: *const DebugMarkerObjectNameInfoEXT<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDebugMarkerSetObjectTagEXT.html>
    pub type PFN_vkDebugMarkerSetObjectTagEXT = unsafe extern "system" fn(
        device: Device,
        p_tag_info: *const DebugMarkerObjectTagInfoEXT<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDebugMarkerBeginEXT.html>
    pub type PFN_vkCmdDebugMarkerBeginEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDebugMarkerEndEXT.html>
    pub type PFN_vkCmdDebugMarkerEndEXT = unsafe extern "system" fn(command_buffer: CommandBuffer);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDebugMarkerInsertEXT.html>
    pub type PFN_vkCmdDebugMarkerInsertEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkDebugMarkerObjectNameInfoEXT = DebugMarkerObjectNameInfoEXT<'static>;
    pub type VkDebugMarkerObjectTagInfoEXT = DebugMarkerObjectTagInfoEXT<'static>;
    pub type VkDebugMarkerMarkerInfoEXT = DebugMarkerMarkerInfoEXT<'static>;
    impl DebugMarkerObjectNameInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDebugMarkerObjectNameInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DebugMarkerObjectTagInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDebugMarkerObjectTagInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DebugMarkerMarkerInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDebugMarkerMarkerInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    debug_marker_set_object_tag_ext: PFN_vkDebugMarkerSetObjectTagEXT,
    debug_marker_set_object_name_ext: PFN_vkDebugMarkerSetObjectNameEXT,
    cmd_debug_marker_begin_ext: PFN_vkCmdDebugMarkerBeginEXT,
    cmd_debug_marker_end_ext: PFN_vkCmdDebugMarkerEndEXT,
    cmd_debug_marker_insert_ext: PFN_vkCmdDebugMarkerInsertEXT,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                debug_marker_set_object_tag_ext: transmute(
                    load(c"vkDebugMarkerSetObjectTagEXT").ok_or(MissingEntryPointError)?,
                ),
                debug_marker_set_object_name_ext: transmute(
                    load(c"vkDebugMarkerSetObjectNameEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_debug_marker_begin_ext: transmute(
                    load(c"vkCmdDebugMarkerBeginEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_debug_marker_end_ext: transmute(
                    load(c"vkCmdDebugMarkerEndEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_debug_marker_insert_ext: transmute(
                    load(c"vkCmdDebugMarkerInsertEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDebugMarkerSetObjectTagEXT.html>
    #[inline]
    pub unsafe fn debug_marker_set_object_tag_ext(
        &self,
        device: Device,
        tag_info: &DebugMarkerObjectTagInfoEXT<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.debug_marker_set_object_tag_ext)(device, tag_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDebugMarkerSetObjectNameEXT.html>
    #[inline]
    pub unsafe fn debug_marker_set_object_name_ext(
        &self,
        device: Device,
        name_info: &DebugMarkerObjectNameInfoEXT<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.debug_marker_set_object_name_ext)(device, name_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDebugMarkerBeginEXT.html>
    #[inline]
    pub unsafe fn cmd_debug_marker_begin_ext(
        &self,
        command_buffer: CommandBuffer,
        marker_info: &DebugMarkerMarkerInfoEXT<'_>,
    ) {
        unsafe { (self.cmd_debug_marker_begin_ext)(command_buffer, marker_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDebugMarkerEndEXT.html>
    #[inline]
    pub unsafe fn cmd_debug_marker_end_ext(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_debug_marker_end_ext)(command_buffer) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDebugMarkerInsertEXT.html>
    #[inline]
    pub unsafe fn cmd_debug_marker_insert_ext(
        &self,
        command_buffer: CommandBuffer,
        marker_info: &DebugMarkerMarkerInfoEXT<'_>,
    ) {
        unsafe { (self.cmd_debug_marker_insert_ext)(command_buffer, marker_info) }
    }
}

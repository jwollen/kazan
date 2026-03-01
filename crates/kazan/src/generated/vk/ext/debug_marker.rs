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
    pub struct DebugMarkerObjectNameInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub object_type: DebugReportObjectTypeEXT,
        pub object: u64,
        pub p_object_name: *const c_char,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DebugMarkerObjectNameInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEBUG_MARKER_OBJECT_NAME_INFO_EXT;
    }
    impl Default for DebugMarkerObjectNameInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                object_type: Default::default(),
                object: Default::default(),
                p_object_name: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DebugMarkerObjectNameInfoEXT<'a> {
        pub fn object_type(mut self, object_type: DebugReportObjectTypeEXT) -> Self {
            self.object_type = object_type;
            self
        }
        pub fn object(mut self, object: u64) -> Self {
            self.object = object;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for DebugMarkerObjectTagInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEBUG_MARKER_OBJECT_TAG_INFO_EXT;
    }
    impl Default for DebugMarkerObjectTagInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                object_type: Default::default(),
                object: Default::default(),
                tag_name: Default::default(),
                tag_size: Default::default(),
                p_tag: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DebugMarkerObjectTagInfoEXT<'a> {
        pub fn object_type(mut self, object_type: DebugReportObjectTypeEXT) -> Self {
            self.object_type = object_type;
            self
        }
        pub fn object(mut self, object: u64) -> Self {
            self.object = object;
            self
        }
        pub fn tag_name(mut self, tag_name: u64) -> Self {
            self.tag_name = tag_name;
            self
        }
        pub fn tag(mut self, tag: &'a [u8]) -> Self {
            self.tag_size = tag.len().try_into().unwrap();
            self.p_tag = tag.as_ptr() as _;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DebugMarkerMarkerInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_marker_name: *const c_char,
        pub color: [f32; 4],
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DebugMarkerMarkerInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEBUG_MARKER_MARKER_INFO_EXT;
    }
    impl Default for DebugMarkerMarkerInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_marker_name: core::ptr::null(),
                color: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DebugMarkerMarkerInfoEXT<'a> {
        pub fn color(mut self, color: [f32; 4]) -> Self {
            self.color = color;
            self
        }
    }
    pub type PFN_vkDebugMarkerSetObjectNameEXT = unsafe extern "system" fn(
        device: Device,
        p_name_info: *const DebugMarkerObjectNameInfoEXT<'_>,
    ) -> vk::Result;
    pub type PFN_vkDebugMarkerSetObjectTagEXT = unsafe extern "system" fn(
        device: Device,
        p_tag_info: *const DebugMarkerObjectTagInfoEXT<'_>,
    ) -> vk::Result;
    pub type PFN_vkCmdDebugMarkerBeginEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
    );
    pub type PFN_vkCmdDebugMarkerEndEXT = unsafe extern "system" fn(command_buffer: CommandBuffer);
    pub type PFN_vkCmdDebugMarkerInsertEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
    );
}
pub struct DeviceFn {
    debug_marker_set_object_tag_ext: PFN_vkDebugMarkerSetObjectTagEXT,
    debug_marker_set_object_name_ext: PFN_vkDebugMarkerSetObjectNameEXT,
    cmd_debug_marker_begin_ext: PFN_vkCmdDebugMarkerBeginEXT,
    cmd_debug_marker_end_ext: PFN_vkCmdDebugMarkerEndEXT,
    cmd_debug_marker_insert_ext: PFN_vkCmdDebugMarkerInsertEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                debug_marker_set_object_tag_ext: transmute(
                    load(c"vkDebugMarkerSetObjectTagEXT").ok_or(LoadingError)?,
                ),
                debug_marker_set_object_name_ext: transmute(
                    load(c"vkDebugMarkerSetObjectNameEXT").ok_or(LoadingError)?,
                ),
                cmd_debug_marker_begin_ext: transmute(
                    load(c"vkCmdDebugMarkerBeginEXT").ok_or(LoadingError)?,
                ),
                cmd_debug_marker_end_ext: transmute(
                    load(c"vkCmdDebugMarkerEndEXT").ok_or(LoadingError)?,
                ),
                cmd_debug_marker_insert_ext: transmute(
                    load(c"vkCmdDebugMarkerInsertEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
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
    pub unsafe fn cmd_debug_marker_begin_ext(
        &self,
        command_buffer: CommandBuffer,
        marker_info: &DebugMarkerMarkerInfoEXT<'_>,
    ) {
        unsafe { (self.cmd_debug_marker_begin_ext)(command_buffer, marker_info) }
    }
    pub unsafe fn cmd_debug_marker_end_ext(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_debug_marker_end_ext)(command_buffer) }
    }
    pub unsafe fn cmd_debug_marker_insert_ext(
        &self,
        command_buffer: CommandBuffer,
        marker_info: &DebugMarkerMarkerInfoEXT<'_>,
    ) {
        unsafe { (self.cmd_debug_marker_insert_ext)(command_buffer, marker_info) }
    }
}

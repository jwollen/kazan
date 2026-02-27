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
impl Default for DebugMarkerObjectNameInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEBUG_MARKER_OBJECT_NAME_INFO_EXT,
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
impl Default for DebugMarkerObjectTagInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEBUG_MARKER_OBJECT_TAG_INFO_EXT,
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
impl Default for DebugMarkerMarkerInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEBUG_MARKER_MARKER_INFO_EXT,
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
) -> Result;
pub type PFN_vkDebugMarkerSetObjectTagEXT = unsafe extern "system" fn(
    device: Device,
    p_tag_info: *const DebugMarkerObjectTagInfoEXT<'_>,
) -> Result;
pub type PFN_vkCmdDebugMarkerBeginEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
);
pub type PFN_vkCmdDebugMarkerEndEXT = unsafe extern "system" fn(command_buffer: CommandBuffer);
pub type PFN_vkCmdDebugMarkerInsertEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
);

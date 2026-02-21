#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub graphics_pipeline_library: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub graphics_pipeline_library_fast_linking: Bool32,
    pub graphics_pipeline_library_independent_interpolation_decoration: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GraphicsPipelineLibraryCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: GraphicsPipelineLibraryFlagsEXT,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct GraphicsPipelineLibraryFlagsEXT: Flags {
        const VERTEX_INPUT_INTERFACE_EXT = 1 << 0;
        const PRE_RASTERIZATION_SHADERS_EXT = 1 << 1;
        const FRAGMENT_SHADER_EXT = 1 << 2;
        const FRAGMENT_OUTPUT_INTERFACE_EXT = 1 << 3;
    }
}

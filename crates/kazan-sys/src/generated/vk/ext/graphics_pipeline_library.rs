#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub graphics_pipeline_library: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub graphics_pipeline_library_fast_linking: Bool32,
    pub graphics_pipeline_library_independent_interpolation_decoration: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct GraphicsPipelineLibraryCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: GraphicsPipelineLibraryFlagsEXT,
    pub _marker: PhantomData<&'a ()>,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GraphicsPipelineLibraryFlagsEXT: Flags {
        const VERTEX_INPUT_INTERFACE_EXT = GraphicsPipelineLibraryFlagBitsEXT::VERTEX_INPUT_INTERFACE_EXT.0;
        const PRE_RASTERIZATION_SHADERS_EXT = GraphicsPipelineLibraryFlagBitsEXT::PRE_RASTERIZATION_SHADERS_EXT.0;
        const FRAGMENT_SHADER_EXT = GraphicsPipelineLibraryFlagBitsEXT::FRAGMENT_SHADER_EXT.0;
        const FRAGMENT_OUTPUT_INTERFACE_EXT = GraphicsPipelineLibraryFlagBitsEXT::FRAGMENT_OUTPUT_INTERFACE_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GraphicsPipelineLibraryFlagBitsEXT(u32);
impl GraphicsPipelineLibraryFlagBitsEXT {
    pub const VERTEX_INPUT_INTERFACE_EXT: Self = Self(1 << 0);
    pub const PRE_RASTERIZATION_SHADERS_EXT: Self = Self(1 << 1);
    pub const FRAGMENT_SHADER_EXT: Self = Self(1 << 2);
    pub const FRAGMENT_OUTPUT_INTERFACE_EXT: Self = Self(1 << 3);
}

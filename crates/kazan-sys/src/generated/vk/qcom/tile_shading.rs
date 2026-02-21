#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTileShadingFeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tile_shading: Bool32,
    pub tile_shading_fragment_stage: Bool32,
    pub tile_shading_color_attachments: Bool32,
    pub tile_shading_depth_attachments: Bool32,
    pub tile_shading_stencil_attachments: Bool32,
    pub tile_shading_input_attachments: Bool32,
    pub tile_shading_sampled_attachments: Bool32,
    pub tile_shading_per_tile_draw: Bool32,
    pub tile_shading_per_tile_dispatch: Bool32,
    pub tile_shading_dispatch_tile: Bool32,
    pub tile_shading_apron: Bool32,
    pub tile_shading_anisotropic_apron: Bool32,
    pub tile_shading_atomic_ops: Bool32,
    pub tile_shading_image_processing: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTileShadingPropertiesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_apron_size: u32,
    pub prefer_non_coherent: Bool32,
    pub tile_granularity: Extent2D,
    pub max_tile_shading_rate: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassTileShadingCreateInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: TileShadingRenderPassFlagsQCOM,
    pub tile_apron_size: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerTileBeginInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerTileEndInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DispatchTileInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct TileShadingRenderPassFlagsQCOM: Flags {
        const ENABLE_QCOM = TileShadingRenderPassFlagBitsQCOM::ENABLE_QCOM.0;
        const PER_TILE_EXECUTION_QCOM = TileShadingRenderPassFlagBitsQCOM::PER_TILE_EXECUTION_QCOM.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TileShadingRenderPassFlagBitsQCOM(u32);
impl TileShadingRenderPassFlagBitsQCOM {
    pub const ENABLE_QCOM: Self = Self(1 << 0);
    pub const PER_TILE_EXECUTION_QCOM: Self = Self(1 << 1);
}
pub type PFN_vkCmdDispatchTileQCOM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_dispatch_tile_info: *const DispatchTileInfoQCOM,
);
pub type PFN_vkCmdBeginPerTileExecutionQCOM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_per_tile_begin_info: *const PerTileBeginInfoQCOM,
);
pub type PFN_vkCmdEndPerTileExecutionQCOM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_per_tile_end_info: *const PerTileEndInfoQCOM,
);

#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTileShadingFeaturesQCOM<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceTileShadingFeaturesQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_TILE_SHADING_FEATURES_QCOM,
            p_next: core::ptr::null_mut(),
            tile_shading: Default::default(),
            tile_shading_fragment_stage: Default::default(),
            tile_shading_color_attachments: Default::default(),
            tile_shading_depth_attachments: Default::default(),
            tile_shading_stencil_attachments: Default::default(),
            tile_shading_input_attachments: Default::default(),
            tile_shading_sampled_attachments: Default::default(),
            tile_shading_per_tile_draw: Default::default(),
            tile_shading_per_tile_dispatch: Default::default(),
            tile_shading_dispatch_tile: Default::default(),
            tile_shading_apron: Default::default(),
            tile_shading_anisotropic_apron: Default::default(),
            tile_shading_atomic_ops: Default::default(),
            tile_shading_image_processing: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTileShadingPropertiesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_apron_size: u32,
    pub prefer_non_coherent: Bool32,
    pub tile_granularity: Extent2D,
    pub max_tile_shading_rate: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceTileShadingPropertiesQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_TILE_SHADING_PROPERTIES_QCOM,
            p_next: core::ptr::null_mut(),
            max_apron_size: Default::default(),
            prefer_non_coherent: Default::default(),
            tile_granularity: Default::default(),
            max_tile_shading_rate: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassTileShadingCreateInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: TileShadingRenderPassFlagsQCOM,
    pub tile_apron_size: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RenderPassTileShadingCreateInfoQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDER_PASS_TILE_SHADING_CREATE_INFO_QCOM,
            p_next: core::ptr::null(),
            flags: Default::default(),
            tile_apron_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerTileBeginInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PerTileBeginInfoQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PER_TILE_BEGIN_INFO_QCOM,
            p_next: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerTileEndInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PerTileEndInfoQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PER_TILE_END_INFO_QCOM,
            p_next: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DispatchTileInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DispatchTileInfoQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DISPATCH_TILE_INFO_QCOM,
            p_next: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TileShadingRenderPassFlagsQCOM: Flags {
        const ENABLE_QCOM = TileShadingRenderPassFlagBitsQCOM::ENABLE_QCOM.0;
        const PER_TILE_EXECUTION_QCOM = TileShadingRenderPassFlagBitsQCOM::PER_TILE_EXECUTION_QCOM.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TileShadingRenderPassFlagBitsQCOM(u32);
impl TileShadingRenderPassFlagBitsQCOM {
    pub const ENABLE_QCOM: Self = Self(1 << 0);
    pub const PER_TILE_EXECUTION_QCOM: Self = Self(1 << 1);
}
pub type PFN_vkCmdDispatchTileQCOM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_dispatch_tile_info: *const DispatchTileInfoQCOM<'_>,
);
pub type PFN_vkCmdBeginPerTileExecutionQCOM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_per_tile_begin_info: *const PerTileBeginInfoQCOM<'_>,
);
pub type PFN_vkCmdEndPerTileExecutionQCOM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_per_tile_end_info: *const PerTileEndInfoQCOM<'_>,
);

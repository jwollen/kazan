#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
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
impl<'a> PhysicalDeviceTileShadingFeaturesQCOM<'a> {
    pub fn tile_shading(mut self, tile_shading: Bool32) -> Self {
        self.tile_shading = tile_shading;
        self
    }
    pub fn tile_shading_fragment_stage(mut self, tile_shading_fragment_stage: Bool32) -> Self {
        self.tile_shading_fragment_stage = tile_shading_fragment_stage;
        self
    }
    pub fn tile_shading_color_attachments(
        mut self,
        tile_shading_color_attachments: Bool32,
    ) -> Self {
        self.tile_shading_color_attachments = tile_shading_color_attachments;
        self
    }
    pub fn tile_shading_depth_attachments(
        mut self,
        tile_shading_depth_attachments: Bool32,
    ) -> Self {
        self.tile_shading_depth_attachments = tile_shading_depth_attachments;
        self
    }
    pub fn tile_shading_stencil_attachments(
        mut self,
        tile_shading_stencil_attachments: Bool32,
    ) -> Self {
        self.tile_shading_stencil_attachments = tile_shading_stencil_attachments;
        self
    }
    pub fn tile_shading_input_attachments(
        mut self,
        tile_shading_input_attachments: Bool32,
    ) -> Self {
        self.tile_shading_input_attachments = tile_shading_input_attachments;
        self
    }
    pub fn tile_shading_sampled_attachments(
        mut self,
        tile_shading_sampled_attachments: Bool32,
    ) -> Self {
        self.tile_shading_sampled_attachments = tile_shading_sampled_attachments;
        self
    }
    pub fn tile_shading_per_tile_draw(mut self, tile_shading_per_tile_draw: Bool32) -> Self {
        self.tile_shading_per_tile_draw = tile_shading_per_tile_draw;
        self
    }
    pub fn tile_shading_per_tile_dispatch(
        mut self,
        tile_shading_per_tile_dispatch: Bool32,
    ) -> Self {
        self.tile_shading_per_tile_dispatch = tile_shading_per_tile_dispatch;
        self
    }
    pub fn tile_shading_dispatch_tile(mut self, tile_shading_dispatch_tile: Bool32) -> Self {
        self.tile_shading_dispatch_tile = tile_shading_dispatch_tile;
        self
    }
    pub fn tile_shading_apron(mut self, tile_shading_apron: Bool32) -> Self {
        self.tile_shading_apron = tile_shading_apron;
        self
    }
    pub fn tile_shading_anisotropic_apron(
        mut self,
        tile_shading_anisotropic_apron: Bool32,
    ) -> Self {
        self.tile_shading_anisotropic_apron = tile_shading_anisotropic_apron;
        self
    }
    pub fn tile_shading_atomic_ops(mut self, tile_shading_atomic_ops: Bool32) -> Self {
        self.tile_shading_atomic_ops = tile_shading_atomic_ops;
        self
    }
    pub fn tile_shading_image_processing(mut self, tile_shading_image_processing: Bool32) -> Self {
        self.tile_shading_image_processing = tile_shading_image_processing;
        self
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
impl<'a> PhysicalDeviceTileShadingPropertiesQCOM<'a> {
    pub fn max_apron_size(mut self, max_apron_size: u32) -> Self {
        self.max_apron_size = max_apron_size;
        self
    }
    pub fn prefer_non_coherent(mut self, prefer_non_coherent: Bool32) -> Self {
        self.prefer_non_coherent = prefer_non_coherent;
        self
    }
    pub fn tile_granularity(mut self, tile_granularity: Extent2D) -> Self {
        self.tile_granularity = tile_granularity;
        self
    }
    pub fn max_tile_shading_rate(mut self, max_tile_shading_rate: Extent2D) -> Self {
        self.max_tile_shading_rate = max_tile_shading_rate;
        self
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
impl<'a> RenderPassTileShadingCreateInfoQCOM<'a> {
    pub fn flags(mut self, flags: TileShadingRenderPassFlagsQCOM) -> Self {
        self.flags = flags;
        self
    }
    pub fn tile_apron_size(mut self, tile_apron_size: Extent2D) -> Self {
        self.tile_apron_size = tile_apron_size;
        self
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
impl<'a> PerTileBeginInfoQCOM<'a> {}
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
impl<'a> PerTileEndInfoQCOM<'a> {}
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
impl<'a> DispatchTileInfoQCOM<'a> {}
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

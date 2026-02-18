#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShadingRatePaletteNV {
    pub shading_rate_palette_entry_count: u32,
    pub p_shading_rate_palette_entries: *const ShadingRatePaletteEntryNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shading_rate_image_enable: Bool32,
    pub viewport_count: u32,
    pub p_shading_rate_palettes: *const ShadingRatePaletteNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShadingRateImageFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shading_rate_image: Bool32,
    pub shading_rate_coarse_sample_order: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShadingRateImagePropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shading_rate_texel_size: Extent2D,
    pub shading_rate_palette_size: u32,
    pub shading_rate_max_coarse_samples: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoarseSampleLocationNV {
    pub pixel_x: u32,
    pub pixel_y: u32,
    pub sample: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoarseSampleOrderCustomNV {
    pub shading_rate: ShadingRatePaletteEntryNV,
    pub sample_count: u32,
    pub sample_location_count: u32,
    pub p_sample_locations: *const CoarseSampleLocationNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_order_type: CoarseSampleOrderTypeNV,
    pub custom_sample_order_count: u32,
    pub p_custom_sample_orders: *const CoarseSampleOrderCustomNV,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShadingRatePaletteEntryNV(i32);
impl ShadingRatePaletteEntryNV {
    pub const NO_INVOCATIONS_NV: Self = Self(0);
    pub const _16_INVOCATIONS_PER_PIXEL_NV: Self = Self(1);
    pub const _8_INVOCATIONS_PER_PIXEL_NV: Self = Self(2);
    pub const _4_INVOCATIONS_PER_PIXEL_NV: Self = Self(3);
    pub const _2_INVOCATIONS_PER_PIXEL_NV: Self = Self(4);
    pub const _1_INVOCATION_PER_PIXEL_NV: Self = Self(5);
    pub const _1_INVOCATION_PER_2X1_PIXELS_NV: Self = Self(6);
    pub const _1_INVOCATION_PER_1X2_PIXELS_NV: Self = Self(7);
    pub const _1_INVOCATION_PER_2X2_PIXELS_NV: Self = Self(8);
    pub const _1_INVOCATION_PER_4X2_PIXELS_NV: Self = Self(9);
    pub const _1_INVOCATION_PER_2X4_PIXELS_NV: Self = Self(10);
    pub const _1_INVOCATION_PER_4X4_PIXELS_NV: Self = Self(11);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoarseSampleOrderTypeNV(i32);
impl CoarseSampleOrderTypeNV {
    pub const DEFAULT_NV: Self = Self(0);
    pub const CUSTOM_NV: Self = Self(1);
    pub const PIXEL_MAJOR_NV: Self = Self(2);
    pub const SAMPLE_MAJOR_NV: Self = Self(3);
}
pub type PFN_vkCmdBindShadingRateImageNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image_view: ImageView,
    image_layout: ImageLayout,
);
pub type PFN_vkCmdSetViewportShadingRatePaletteNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_shading_rate_palettes: *const ShadingRatePaletteNV,
);
pub type PFN_vkCmdSetCoarseSampleOrderNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    sample_order_type: CoarseSampleOrderTypeNV,
    custom_sample_order_count: u32,
    p_custom_sample_orders: *const CoarseSampleOrderCustomNV,
);

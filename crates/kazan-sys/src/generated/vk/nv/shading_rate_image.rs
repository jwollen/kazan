#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShadingRatePaletteNV<'a> {
    pub shading_rate_palette_entry_count: u32,
    pub p_shading_rate_palette_entries: *const ShadingRatePaletteEntryNV,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ShadingRatePaletteNV<'_> {
    fn default() -> Self {
        Self {
            shading_rate_palette_entry_count: Default::default(),
            p_shading_rate_palette_entries: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shading_rate_image_enable: Bool32,
    pub viewport_count: u32,
    pub p_shading_rate_palettes: *const ShadingRatePaletteNV<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineViewportShadingRateImageStateCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            shading_rate_image_enable: Default::default(),
            viewport_count: Default::default(),
            p_shading_rate_palettes: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShadingRateImageFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shading_rate_image: Bool32,
    pub shading_rate_coarse_sample_order: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShadingRateImageFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            shading_rate_image: Default::default(),
            shading_rate_coarse_sample_order: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShadingRateImagePropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shading_rate_texel_size: Extent2D,
    pub shading_rate_palette_size: u32,
    pub shading_rate_max_coarse_samples: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShadingRateImagePropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            shading_rate_texel_size: Default::default(),
            shading_rate_palette_size: Default::default(),
            shading_rate_max_coarse_samples: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct CoarseSampleLocationNV {
    pub pixel_x: u32,
    pub pixel_y: u32,
    pub sample: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CoarseSampleOrderCustomNV<'a> {
    pub shading_rate: ShadingRatePaletteEntryNV,
    pub sample_count: u32,
    pub sample_location_count: u32,
    pub p_sample_locations: *const CoarseSampleLocationNV,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CoarseSampleOrderCustomNV<'_> {
    fn default() -> Self {
        Self {
            shading_rate: Default::default(),
            sample_count: Default::default(),
            sample_location_count: Default::default(),
            p_sample_locations: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_order_type: CoarseSampleOrderTypeNV,
    pub custom_sample_order_count: u32,
    pub p_custom_sample_orders: *const CoarseSampleOrderCustomNV<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineViewportCoarseSampleOrderStateCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            sample_order_type: Default::default(),
            custom_sample_order_count: Default::default(),
            p_custom_sample_orders: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    p_shading_rate_palettes: *const ShadingRatePaletteNV<'_>,
);
pub type PFN_vkCmdSetCoarseSampleOrderNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    sample_order_type: CoarseSampleOrderTypeNV,
    custom_sample_order_count: u32,
    p_custom_sample_orders: *const CoarseSampleOrderCustomNV<'_>,
);

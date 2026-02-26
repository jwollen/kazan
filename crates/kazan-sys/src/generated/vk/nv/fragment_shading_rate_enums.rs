#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_shading_rate_enums: Bool32,
    pub supersample_fragment_shading_rates: Bool32,
    pub no_invocation_fragment_shading_rates: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            fragment_shading_rate_enums: Default::default(),
            supersample_fragment_shading_rates: Default::default(),
            no_invocation_fragment_shading_rates: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_fragment_shading_rate_invocation_count: SampleCountFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            max_fragment_shading_rate_invocation_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineFragmentShadingRateEnumStateCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shading_rate_type: FragmentShadingRateTypeNV,
    pub shading_rate: FragmentShadingRateNV,
    pub combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineFragmentShadingRateEnumStateCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            shading_rate_type: Default::default(),
            shading_rate: Default::default(),
            combiner_ops: [Default::default(); _],
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentShadingRateNV(i32);
impl FragmentShadingRateNV {
    pub const _1_INVOCATION_PER_PIXEL_NV: Self = Self(0);
    pub const _1_INVOCATION_PER_1X2_PIXELS_NV: Self = Self(1);
    pub const _1_INVOCATION_PER_2X1_PIXELS_NV: Self = Self(4);
    pub const _1_INVOCATION_PER_2X2_PIXELS_NV: Self = Self(5);
    pub const _1_INVOCATION_PER_2X4_PIXELS_NV: Self = Self(6);
    pub const _1_INVOCATION_PER_4X2_PIXELS_NV: Self = Self(9);
    pub const _1_INVOCATION_PER_4X4_PIXELS_NV: Self = Self(10);
    pub const _2_INVOCATIONS_PER_PIXEL_NV: Self = Self(11);
    pub const _4_INVOCATIONS_PER_PIXEL_NV: Self = Self(12);
    pub const _8_INVOCATIONS_PER_PIXEL_NV: Self = Self(13);
    pub const _16_INVOCATIONS_PER_PIXEL_NV: Self = Self(14);
    pub const NO_INVOCATIONS_NV: Self = Self(15);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentShadingRateTypeNV(i32);
impl FragmentShadingRateTypeNV {
    pub const FRAGMENT_SIZE_NV: Self = Self(0);
    pub const ENUMS_NV: Self = Self(1);
}
pub type PFN_vkCmdSetFragmentShadingRateEnumNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    shading_rate: FragmentShadingRateNV,
    combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2],
);

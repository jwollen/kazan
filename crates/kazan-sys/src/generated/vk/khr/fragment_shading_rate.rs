#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FragmentShadingRateAttachmentInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_fragment_shading_rate_attachment: *const AttachmentReference2<'a>,
    pub shading_rate_attachment_texel_size: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for FragmentShadingRateAttachmentInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR,
            p_next: core::ptr::null(),
            p_fragment_shading_rate_attachment: core::ptr::null(),
            shading_rate_attachment_texel_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineFragmentShadingRateStateCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fragment_size: Extent2D,
    pub combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineFragmentShadingRateStateCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            fragment_size: Default::default(),
            combiner_ops: [Default::default(); _],
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentShadingRateFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_fragment_shading_rate: Bool32,
    pub primitive_fragment_shading_rate: Bool32,
    pub attachment_fragment_shading_rate: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFragmentShadingRateFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            pipeline_fragment_shading_rate: Default::default(),
            primitive_fragment_shading_rate: Default::default(),
            attachment_fragment_shading_rate: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentShadingRatePropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_fragment_shading_rate_attachment_texel_size: Extent2D,
    pub max_fragment_shading_rate_attachment_texel_size: Extent2D,
    pub max_fragment_shading_rate_attachment_texel_size_aspect_ratio: u32,
    pub primitive_fragment_shading_rate_with_multiple_viewports: Bool32,
    pub layered_shading_rate_attachments: Bool32,
    pub fragment_shading_rate_non_trivial_combiner_ops: Bool32,
    pub max_fragment_size: Extent2D,
    pub max_fragment_size_aspect_ratio: u32,
    pub max_fragment_shading_rate_coverage_samples: u32,
    pub max_fragment_shading_rate_rasterization_samples: SampleCountFlagBits,
    pub fragment_shading_rate_with_shader_depth_stencil_writes: Bool32,
    pub fragment_shading_rate_with_sample_mask: Bool32,
    pub fragment_shading_rate_with_shader_sample_mask: Bool32,
    pub fragment_shading_rate_with_conservative_rasterization: Bool32,
    pub fragment_shading_rate_with_fragment_shader_interlock: Bool32,
    pub fragment_shading_rate_with_custom_sample_locations: Bool32,
    pub fragment_shading_rate_strict_multiply_combiner: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFragmentShadingRatePropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            min_fragment_shading_rate_attachment_texel_size: Default::default(),
            max_fragment_shading_rate_attachment_texel_size: Default::default(),
            max_fragment_shading_rate_attachment_texel_size_aspect_ratio: Default::default(),
            primitive_fragment_shading_rate_with_multiple_viewports: Default::default(),
            layered_shading_rate_attachments: Default::default(),
            fragment_shading_rate_non_trivial_combiner_ops: Default::default(),
            max_fragment_size: Default::default(),
            max_fragment_size_aspect_ratio: Default::default(),
            max_fragment_shading_rate_coverage_samples: Default::default(),
            max_fragment_shading_rate_rasterization_samples: Default::default(),
            fragment_shading_rate_with_shader_depth_stencil_writes: Default::default(),
            fragment_shading_rate_with_sample_mask: Default::default(),
            fragment_shading_rate_with_shader_sample_mask: Default::default(),
            fragment_shading_rate_with_conservative_rasterization: Default::default(),
            fragment_shading_rate_with_fragment_shader_interlock: Default::default(),
            fragment_shading_rate_with_custom_sample_locations: Default::default(),
            fragment_shading_rate_strict_multiply_combiner: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentShadingRateKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub sample_counts: SampleCountFlags,
    pub fragment_size: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFragmentShadingRateKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR,
            p_next: core::ptr::null_mut(),
            sample_counts: Default::default(),
            fragment_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderingFragmentShadingRateAttachmentInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
    pub shading_rate_attachment_texel_size: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RenderingFragmentShadingRateAttachmentInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR,
            p_next: core::ptr::null(),
            image_view: Default::default(),
            image_layout: Default::default(),
            shading_rate_attachment_texel_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentShadingRateCombinerOpKHR(i32);
impl FragmentShadingRateCombinerOpKHR {
    pub const KEEP_KHR: Self = Self(0);
    pub const REPLACE_KHR: Self = Self(1);
    pub const MIN_KHR: Self = Self(2);
    pub const MAX_KHR: Self = Self(3);
    pub const MUL_KHR: Self = Self(4);
}
pub type PFN_vkCmdSetFragmentShadingRateKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_fragment_size: *const Extent2D,
    combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2],
);
pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_fragment_shading_rate_count: *mut u32,
    p_fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR<'_>,
) -> Result;

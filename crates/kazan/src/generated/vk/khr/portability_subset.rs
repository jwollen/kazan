#![cfg(feature = "provisional")]
//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_portability_subset.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_portability_subset";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePortabilitySubsetFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePortabilitySubsetFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub constant_alpha_color_blend_factors: Bool32,
        pub events: Bool32,
        pub image_view_format_reinterpretation: Bool32,
        pub image_view_format_swizzle: Bool32,
        pub image_view2_d_on3_d_image: Bool32,
        pub multisample_array_image: Bool32,
        pub mutable_comparison_samplers: Bool32,
        pub point_polygons: Bool32,
        pub sampler_mip_lod_bias: Bool32,
        pub separate_stencil_mask_ref: Bool32,
        pub shader_sample_rate_interpolation_functions: Bool32,
        pub tessellation_isolines: Bool32,
        pub tessellation_point_mode: Bool32,
        pub triangle_fans: Bool32,
        pub vertex_attribute_access_beyond_stride: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePortabilitySubsetFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePortabilitySubsetFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "constant_alpha_color_blend_factors",
                    &self.constant_alpha_color_blend_factors,
                )
                .field("events", &self.events)
                .field(
                    "image_view_format_reinterpretation",
                    &self.image_view_format_reinterpretation,
                )
                .field("image_view_format_swizzle", &self.image_view_format_swizzle)
                .field("image_view2_d_on3_d_image", &self.image_view2_d_on3_d_image)
                .field("multisample_array_image", &self.multisample_array_image)
                .field(
                    "mutable_comparison_samplers",
                    &self.mutable_comparison_samplers,
                )
                .field("point_polygons", &self.point_polygons)
                .field("sampler_mip_lod_bias", &self.sampler_mip_lod_bias)
                .field("separate_stencil_mask_ref", &self.separate_stencil_mask_ref)
                .field(
                    "shader_sample_rate_interpolation_functions",
                    &self.shader_sample_rate_interpolation_functions,
                )
                .field("tessellation_isolines", &self.tessellation_isolines)
                .field("tessellation_point_mode", &self.tessellation_point_mode)
                .field("triangle_fans", &self.triangle_fans)
                .field(
                    "vertex_attribute_access_beyond_stride",
                    &self.vertex_attribute_access_beyond_stride,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePortabilitySubsetFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDevicePortabilitySubsetFeaturesKHR<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDevicePortabilitySubsetFeaturesKHR<'_> {}

    impl Default for PhysicalDevicePortabilitySubsetFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                constant_alpha_color_blend_factors: Default::default(),
                events: Default::default(),
                image_view_format_reinterpretation: Default::default(),
                image_view_format_swizzle: Default::default(),
                image_view2_d_on3_d_image: Default::default(),
                multisample_array_image: Default::default(),
                mutable_comparison_samplers: Default::default(),
                point_polygons: Default::default(),
                sampler_mip_lod_bias: Default::default(),
                separate_stencil_mask_ref: Default::default(),
                shader_sample_rate_interpolation_functions: Default::default(),
                tessellation_isolines: Default::default(),
                tessellation_point_mode: Default::default(),
                triangle_fans: Default::default(),
                vertex_attribute_access_beyond_stride: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePortabilitySubsetFeaturesKHR<'a> {
        #[inline]
        pub fn constant_alpha_color_blend_factors(
            mut self,
            constant_alpha_color_blend_factors: bool,
        ) -> Self {
            self.constant_alpha_color_blend_factors = constant_alpha_color_blend_factors.into();
            self
        }

        #[inline]
        pub fn events(mut self, events: bool) -> Self {
            self.events = events.into();
            self
        }

        #[inline]
        pub fn image_view_format_reinterpretation(
            mut self,
            image_view_format_reinterpretation: bool,
        ) -> Self {
            self.image_view_format_reinterpretation = image_view_format_reinterpretation.into();
            self
        }

        #[inline]
        pub fn image_view_format_swizzle(mut self, image_view_format_swizzle: bool) -> Self {
            self.image_view_format_swizzle = image_view_format_swizzle.into();
            self
        }

        #[inline]
        pub fn image_view2_d_on3_d_image(mut self, image_view2_d_on3_d_image: bool) -> Self {
            self.image_view2_d_on3_d_image = image_view2_d_on3_d_image.into();
            self
        }

        #[inline]
        pub fn multisample_array_image(mut self, multisample_array_image: bool) -> Self {
            self.multisample_array_image = multisample_array_image.into();
            self
        }

        #[inline]
        pub fn mutable_comparison_samplers(mut self, mutable_comparison_samplers: bool) -> Self {
            self.mutable_comparison_samplers = mutable_comparison_samplers.into();
            self
        }

        #[inline]
        pub fn point_polygons(mut self, point_polygons: bool) -> Self {
            self.point_polygons = point_polygons.into();
            self
        }

        #[inline]
        pub fn sampler_mip_lod_bias(mut self, sampler_mip_lod_bias: bool) -> Self {
            self.sampler_mip_lod_bias = sampler_mip_lod_bias.into();
            self
        }

        #[inline]
        pub fn separate_stencil_mask_ref(mut self, separate_stencil_mask_ref: bool) -> Self {
            self.separate_stencil_mask_ref = separate_stencil_mask_ref.into();
            self
        }

        #[inline]
        pub fn shader_sample_rate_interpolation_functions(
            mut self,
            shader_sample_rate_interpolation_functions: bool,
        ) -> Self {
            self.shader_sample_rate_interpolation_functions =
                shader_sample_rate_interpolation_functions.into();
            self
        }

        #[inline]
        pub fn tessellation_isolines(mut self, tessellation_isolines: bool) -> Self {
            self.tessellation_isolines = tessellation_isolines.into();
            self
        }

        #[inline]
        pub fn tessellation_point_mode(mut self, tessellation_point_mode: bool) -> Self {
            self.tessellation_point_mode = tessellation_point_mode.into();
            self
        }

        #[inline]
        pub fn triangle_fans(mut self, triangle_fans: bool) -> Self {
            self.triangle_fans = triangle_fans.into();
            self
        }

        #[inline]
        pub fn vertex_attribute_access_beyond_stride(
            mut self,
            vertex_attribute_access_beyond_stride: bool,
        ) -> Self {
            self.vertex_attribute_access_beyond_stride =
                vertex_attribute_access_beyond_stride.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePortabilitySubsetPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePortabilitySubsetPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_vertex_input_binding_stride_alignment: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePortabilitySubsetPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePortabilitySubsetPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "min_vertex_input_binding_stride_alignment",
                    &self.min_vertex_input_binding_stride_alignment,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePortabilitySubsetPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDevicePortabilitySubsetPropertiesKHR<'_>
    {
    }

    impl Default for PhysicalDevicePortabilitySubsetPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                min_vertex_input_binding_stride_alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePortabilitySubsetPropertiesKHR<'a> {
        #[inline]
        pub fn min_vertex_input_binding_stride_alignment(
            mut self,
            min_vertex_input_binding_stride_alignment: u32,
        ) -> Self {
            self.min_vertex_input_binding_stride_alignment =
                min_vertex_input_binding_stride_alignment;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDevicePortabilitySubsetFeaturesKHR =
        PhysicalDevicePortabilitySubsetFeaturesKHR<'static>;
    pub type VkPhysicalDevicePortabilitySubsetPropertiesKHR =
        PhysicalDevicePortabilitySubsetPropertiesKHR<'static>;
    impl PhysicalDevicePortabilitySubsetFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDevicePortabilitySubsetFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDevicePortabilitySubsetPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDevicePortabilitySubsetPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

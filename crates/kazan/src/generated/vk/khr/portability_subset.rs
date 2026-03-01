#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePortabilitySubsetFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePortabilitySubsetFeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDevicePortabilitySubsetFeaturesKHR<'a> {}
    impl Default for PhysicalDevicePortabilitySubsetFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
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
        pub fn constant_alpha_color_blend_factors(
            mut self,
            constant_alpha_color_blend_factors: Bool32,
        ) -> Self {
            self.constant_alpha_color_blend_factors = constant_alpha_color_blend_factors;
            self
        }
        pub fn events(mut self, events: Bool32) -> Self {
            self.events = events;
            self
        }
        pub fn image_view_format_reinterpretation(
            mut self,
            image_view_format_reinterpretation: Bool32,
        ) -> Self {
            self.image_view_format_reinterpretation = image_view_format_reinterpretation;
            self
        }
        pub fn image_view_format_swizzle(mut self, image_view_format_swizzle: Bool32) -> Self {
            self.image_view_format_swizzle = image_view_format_swizzle;
            self
        }
        pub fn image_view2_d_on3_d_image(mut self, image_view2_d_on3_d_image: Bool32) -> Self {
            self.image_view2_d_on3_d_image = image_view2_d_on3_d_image;
            self
        }
        pub fn multisample_array_image(mut self, multisample_array_image: Bool32) -> Self {
            self.multisample_array_image = multisample_array_image;
            self
        }
        pub fn mutable_comparison_samplers(mut self, mutable_comparison_samplers: Bool32) -> Self {
            self.mutable_comparison_samplers = mutable_comparison_samplers;
            self
        }
        pub fn point_polygons(mut self, point_polygons: Bool32) -> Self {
            self.point_polygons = point_polygons;
            self
        }
        pub fn sampler_mip_lod_bias(mut self, sampler_mip_lod_bias: Bool32) -> Self {
            self.sampler_mip_lod_bias = sampler_mip_lod_bias;
            self
        }
        pub fn separate_stencil_mask_ref(mut self, separate_stencil_mask_ref: Bool32) -> Self {
            self.separate_stencil_mask_ref = separate_stencil_mask_ref;
            self
        }
        pub fn shader_sample_rate_interpolation_functions(
            mut self,
            shader_sample_rate_interpolation_functions: Bool32,
        ) -> Self {
            self.shader_sample_rate_interpolation_functions =
                shader_sample_rate_interpolation_functions;
            self
        }
        pub fn tessellation_isolines(mut self, tessellation_isolines: Bool32) -> Self {
            self.tessellation_isolines = tessellation_isolines;
            self
        }
        pub fn tessellation_point_mode(mut self, tessellation_point_mode: Bool32) -> Self {
            self.tessellation_point_mode = tessellation_point_mode;
            self
        }
        pub fn triangle_fans(mut self, triangle_fans: Bool32) -> Self {
            self.triangle_fans = triangle_fans;
            self
        }
        pub fn vertex_attribute_access_beyond_stride(
            mut self,
            vertex_attribute_access_beyond_stride: Bool32,
        ) -> Self {
            self.vertex_attribute_access_beyond_stride = vertex_attribute_access_beyond_stride;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePortabilitySubsetPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_vertex_input_binding_stride_alignment: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePortabilitySubsetPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDevicePortabilitySubsetPropertiesKHR<'a>
    {
    }
    impl Default for PhysicalDevicePortabilitySubsetPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                min_vertex_input_binding_stride_alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePortabilitySubsetPropertiesKHR<'a> {
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

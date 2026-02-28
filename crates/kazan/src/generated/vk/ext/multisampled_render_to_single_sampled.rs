#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub multisampled_render_to_single_sampled: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
s_type: StructureType::PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT
,
p_next: core::ptr::null_mut(),
multisampled_render_to_single_sampled: Default::default(),
_marker: PhantomData
}
        }
    }
    impl<'a> PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'a> {
        pub fn multisampled_render_to_single_sampled(
            mut self,
            multisampled_render_to_single_sampled: Bool32,
        ) -> Self {
            self.multisampled_render_to_single_sampled = multisampled_render_to_single_sampled;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SubpassResolvePerformanceQueryEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub optimal: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for SubpassResolvePerformanceQueryEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT,
                p_next: core::ptr::null_mut(),
                optimal: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SubpassResolvePerformanceQueryEXT<'a> {
        pub fn optimal(mut self, optimal: Bool32) -> Self {
            self.optimal = optimal;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MultisampledRenderToSingleSampledInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub multisampled_render_to_single_sampled_enable: Bool32,
        pub rasterization_samples: SampleCountFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for MultisampledRenderToSingleSampledInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT,
                p_next: core::ptr::null(),
                multisampled_render_to_single_sampled_enable: Default::default(),
                rasterization_samples: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MultisampledRenderToSingleSampledInfoEXT<'a> {
        pub fn multisampled_render_to_single_sampled_enable(
            mut self,
            multisampled_render_to_single_sampled_enable: Bool32,
        ) -> Self {
            self.multisampled_render_to_single_sampled_enable =
                multisampled_render_to_single_sampled_enable;
            self
        }
        pub fn rasterization_samples(mut self, rasterization_samples: SampleCountFlagBits) -> Self {
            self.rasterization_samples = rasterization_samples;
            self
        }
    }
}

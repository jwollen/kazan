//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_multisampled_render_to_single_sampled.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_multisampled_render_to_single_sampled";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub multisampled_render_to_single_sampled: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "multisampled_render_to_single_sampled",
                    &self.multisampled_render_to_single_sampled,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                multisampled_render_to_single_sampled: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT<'a> {
        #[inline]
        pub fn multisampled_render_to_single_sampled(
            mut self,
            multisampled_render_to_single_sampled: bool,
        ) -> Self {
            self.multisampled_render_to_single_sampled =
                multisampled_render_to_single_sampled.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubpassResolvePerformanceQueryEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SubpassResolvePerformanceQueryEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub optimal: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SubpassResolvePerformanceQueryEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SubpassResolvePerformanceQueryEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("optimal", &self.optimal)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SubpassResolvePerformanceQueryEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT;
    }

    unsafe impl<'a> Extends<FormatProperties2<'a>> for SubpassResolvePerformanceQueryEXT<'a> {}

    impl Default for SubpassResolvePerformanceQueryEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                optimal: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SubpassResolvePerformanceQueryEXT<'a> {
        #[inline]
        pub fn optimal(mut self, optimal: bool) -> Self {
            self.optimal = optimal.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMultisampledRenderToSingleSampledInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MultisampledRenderToSingleSampledInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub multisampled_render_to_single_sampled_enable: Bool32,
        pub rasterization_samples: SampleCountFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MultisampledRenderToSingleSampledInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MultisampledRenderToSingleSampledInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "multisampled_render_to_single_sampled_enable",
                    &self.multisampled_render_to_single_sampled_enable,
                )
                .field("rasterization_samples", &self.rasterization_samples)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MultisampledRenderToSingleSampledInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT;
    }

    unsafe impl<'a> Extends<SubpassDescription2<'a>> for MultisampledRenderToSingleSampledInfoEXT<'a> {}
    unsafe impl<'a> Extends<RenderingInfo<'a>> for MultisampledRenderToSingleSampledInfoEXT<'a> {}

    impl Default for MultisampledRenderToSingleSampledInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                multisampled_render_to_single_sampled_enable: Default::default(),
                rasterization_samples: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MultisampledRenderToSingleSampledInfoEXT<'a> {
        #[inline]
        pub fn multisampled_render_to_single_sampled_enable(
            mut self,
            multisampled_render_to_single_sampled_enable: bool,
        ) -> Self {
            self.multisampled_render_to_single_sampled_enable =
                multisampled_render_to_single_sampled_enable.into();
            self
        }

        #[inline]
        pub fn rasterization_samples(mut self, rasterization_samples: SampleCountFlagBits) -> Self {
            self.rasterization_samples = rasterization_samples;
            self
        }
    }
}

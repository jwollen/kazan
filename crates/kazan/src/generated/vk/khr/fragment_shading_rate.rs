//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_fragment_shading_rate.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_fragment_shading_rate";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFragmentShadingRateAttachmentInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct FragmentShadingRateAttachmentInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_fragment_shading_rate_attachment: *const AttachmentReference2<'a>,
        pub shading_rate_attachment_texel_size: Extent2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for FragmentShadingRateAttachmentInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("FragmentShadingRateAttachmentInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "p_fragment_shading_rate_attachment",
                    &self.p_fragment_shading_rate_attachment,
                )
                .field(
                    "shading_rate_attachment_texel_size",
                    &self.shading_rate_attachment_texel_size,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for FragmentShadingRateAttachmentInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR;
    }

    unsafe impl Extends<SubpassDescription2<'_>> for FragmentShadingRateAttachmentInfoKHR<'_> {}

    impl Default for FragmentShadingRateAttachmentInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_fragment_shading_rate_attachment: ptr::null(),
                shading_rate_attachment_texel_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> FragmentShadingRateAttachmentInfoKHR<'a> {
        #[inline]
        pub fn fragment_shading_rate_attachment(
            mut self,
            fragment_shading_rate_attachment: &'a AttachmentReference2<'a>,
        ) -> Self {
            self.p_fragment_shading_rate_attachment = fragment_shading_rate_attachment;
            self
        }

        #[inline]
        pub fn shading_rate_attachment_texel_size(
            mut self,
            shading_rate_attachment_texel_size: Extent2D,
        ) -> Self {
            self.shading_rate_attachment_texel_size = shading_rate_attachment_texel_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineFragmentShadingRateStateCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineFragmentShadingRateStateCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub fragment_size: Extent2D,
        pub combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineFragmentShadingRateStateCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineFragmentShadingRateStateCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("fragment_size", &self.fragment_size)
                .field("combiner_ops", &self.combiner_ops)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineFragmentShadingRateStateCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR;
    }

    unsafe impl Extends<GraphicsPipelineCreateInfo<'_>>
        for PipelineFragmentShadingRateStateCreateInfoKHR<'_>
    {
    }

    impl Default for PipelineFragmentShadingRateStateCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                fragment_size: Default::default(),
                combiner_ops: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineFragmentShadingRateStateCreateInfoKHR<'a> {
        #[inline]
        pub fn fragment_size(mut self, fragment_size: Extent2D) -> Self {
            self.fragment_size = fragment_size;
            self
        }

        #[inline]
        pub fn combiner_ops(mut self, combiner_ops: [FragmentShadingRateCombinerOpKHR; 2]) -> Self {
            self.combiner_ops = combiner_ops;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFragmentShadingRateFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceFragmentShadingRateFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_fragment_shading_rate: Bool32,
        pub primitive_fragment_shading_rate: Bool32,
        pub attachment_fragment_shading_rate: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceFragmentShadingRateFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceFragmentShadingRateFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "pipeline_fragment_shading_rate",
                    &self.pipeline_fragment_shading_rate,
                )
                .field(
                    "primitive_fragment_shading_rate",
                    &self.primitive_fragment_shading_rate,
                )
                .field(
                    "attachment_fragment_shading_rate",
                    &self.attachment_fragment_shading_rate,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentShadingRateFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceFragmentShadingRateFeaturesKHR<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceFragmentShadingRateFeaturesKHR<'_> {}

    impl Default for PhysicalDeviceFragmentShadingRateFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                pipeline_fragment_shading_rate: Default::default(),
                primitive_fragment_shading_rate: Default::default(),
                attachment_fragment_shading_rate: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceFragmentShadingRateFeaturesKHR<'a> {
        #[inline]
        pub fn pipeline_fragment_shading_rate(
            mut self,
            pipeline_fragment_shading_rate: bool,
        ) -> Self {
            self.pipeline_fragment_shading_rate = pipeline_fragment_shading_rate.into();
            self
        }

        #[inline]
        pub fn primitive_fragment_shading_rate(
            mut self,
            primitive_fragment_shading_rate: bool,
        ) -> Self {
            self.primitive_fragment_shading_rate = primitive_fragment_shading_rate.into();
            self
        }

        #[inline]
        pub fn attachment_fragment_shading_rate(
            mut self,
            attachment_fragment_shading_rate: bool,
        ) -> Self {
            self.attachment_fragment_shading_rate = attachment_fragment_shading_rate.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFragmentShadingRatePropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceFragmentShadingRatePropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceFragmentShadingRatePropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "min_fragment_shading_rate_attachment_texel_size",
                    &self.min_fragment_shading_rate_attachment_texel_size,
                )
                .field(
                    "max_fragment_shading_rate_attachment_texel_size",
                    &self.max_fragment_shading_rate_attachment_texel_size,
                )
                .field(
                    "max_fragment_shading_rate_attachment_texel_size_aspect_ratio",
                    &self.max_fragment_shading_rate_attachment_texel_size_aspect_ratio,
                )
                .field(
                    "primitive_fragment_shading_rate_with_multiple_viewports",
                    &self.primitive_fragment_shading_rate_with_multiple_viewports,
                )
                .field(
                    "layered_shading_rate_attachments",
                    &self.layered_shading_rate_attachments,
                )
                .field(
                    "fragment_shading_rate_non_trivial_combiner_ops",
                    &self.fragment_shading_rate_non_trivial_combiner_ops,
                )
                .field("max_fragment_size", &self.max_fragment_size)
                .field(
                    "max_fragment_size_aspect_ratio",
                    &self.max_fragment_size_aspect_ratio,
                )
                .field(
                    "max_fragment_shading_rate_coverage_samples",
                    &self.max_fragment_shading_rate_coverage_samples,
                )
                .field(
                    "max_fragment_shading_rate_rasterization_samples",
                    &self.max_fragment_shading_rate_rasterization_samples,
                )
                .field(
                    "fragment_shading_rate_with_shader_depth_stencil_writes",
                    &self.fragment_shading_rate_with_shader_depth_stencil_writes,
                )
                .field(
                    "fragment_shading_rate_with_sample_mask",
                    &self.fragment_shading_rate_with_sample_mask,
                )
                .field(
                    "fragment_shading_rate_with_shader_sample_mask",
                    &self.fragment_shading_rate_with_shader_sample_mask,
                )
                .field(
                    "fragment_shading_rate_with_conservative_rasterization",
                    &self.fragment_shading_rate_with_conservative_rasterization,
                )
                .field(
                    "fragment_shading_rate_with_fragment_shader_interlock",
                    &self.fragment_shading_rate_with_fragment_shader_interlock,
                )
                .field(
                    "fragment_shading_rate_with_custom_sample_locations",
                    &self.fragment_shading_rate_with_custom_sample_locations,
                )
                .field(
                    "fragment_shading_rate_strict_multiply_combiner",
                    &self.fragment_shading_rate_strict_multiply_combiner,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentShadingRatePropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceFragmentShadingRatePropertiesKHR<'_>
    {
    }

    impl Default for PhysicalDeviceFragmentShadingRatePropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
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

    impl<'a> PhysicalDeviceFragmentShadingRatePropertiesKHR<'a> {
        #[inline]
        pub fn min_fragment_shading_rate_attachment_texel_size(
            mut self,
            min_fragment_shading_rate_attachment_texel_size: Extent2D,
        ) -> Self {
            self.min_fragment_shading_rate_attachment_texel_size =
                min_fragment_shading_rate_attachment_texel_size;
            self
        }

        #[inline]
        pub fn max_fragment_shading_rate_attachment_texel_size(
            mut self,
            max_fragment_shading_rate_attachment_texel_size: Extent2D,
        ) -> Self {
            self.max_fragment_shading_rate_attachment_texel_size =
                max_fragment_shading_rate_attachment_texel_size;
            self
        }

        #[inline]
        pub fn max_fragment_shading_rate_attachment_texel_size_aspect_ratio(
            mut self,
            max_fragment_shading_rate_attachment_texel_size_aspect_ratio: u32,
        ) -> Self {
            self.max_fragment_shading_rate_attachment_texel_size_aspect_ratio =
                max_fragment_shading_rate_attachment_texel_size_aspect_ratio;
            self
        }

        #[inline]
        pub fn primitive_fragment_shading_rate_with_multiple_viewports(
            mut self,
            primitive_fragment_shading_rate_with_multiple_viewports: bool,
        ) -> Self {
            self.primitive_fragment_shading_rate_with_multiple_viewports =
                primitive_fragment_shading_rate_with_multiple_viewports.into();
            self
        }

        #[inline]
        pub fn layered_shading_rate_attachments(
            mut self,
            layered_shading_rate_attachments: bool,
        ) -> Self {
            self.layered_shading_rate_attachments = layered_shading_rate_attachments.into();
            self
        }

        #[inline]
        pub fn fragment_shading_rate_non_trivial_combiner_ops(
            mut self,
            fragment_shading_rate_non_trivial_combiner_ops: bool,
        ) -> Self {
            self.fragment_shading_rate_non_trivial_combiner_ops =
                fragment_shading_rate_non_trivial_combiner_ops.into();
            self
        }

        #[inline]
        pub fn max_fragment_size(mut self, max_fragment_size: Extent2D) -> Self {
            self.max_fragment_size = max_fragment_size;
            self
        }

        #[inline]
        pub fn max_fragment_size_aspect_ratio(
            mut self,
            max_fragment_size_aspect_ratio: u32,
        ) -> Self {
            self.max_fragment_size_aspect_ratio = max_fragment_size_aspect_ratio;
            self
        }

        #[inline]
        pub fn max_fragment_shading_rate_coverage_samples(
            mut self,
            max_fragment_shading_rate_coverage_samples: u32,
        ) -> Self {
            self.max_fragment_shading_rate_coverage_samples =
                max_fragment_shading_rate_coverage_samples;
            self
        }

        #[inline]
        pub fn max_fragment_shading_rate_rasterization_samples(
            mut self,
            max_fragment_shading_rate_rasterization_samples: SampleCountFlagBits,
        ) -> Self {
            self.max_fragment_shading_rate_rasterization_samples =
                max_fragment_shading_rate_rasterization_samples;
            self
        }

        #[inline]
        pub fn fragment_shading_rate_with_shader_depth_stencil_writes(
            mut self,
            fragment_shading_rate_with_shader_depth_stencil_writes: bool,
        ) -> Self {
            self.fragment_shading_rate_with_shader_depth_stencil_writes =
                fragment_shading_rate_with_shader_depth_stencil_writes.into();
            self
        }

        #[inline]
        pub fn fragment_shading_rate_with_sample_mask(
            mut self,
            fragment_shading_rate_with_sample_mask: bool,
        ) -> Self {
            self.fragment_shading_rate_with_sample_mask =
                fragment_shading_rate_with_sample_mask.into();
            self
        }

        #[inline]
        pub fn fragment_shading_rate_with_shader_sample_mask(
            mut self,
            fragment_shading_rate_with_shader_sample_mask: bool,
        ) -> Self {
            self.fragment_shading_rate_with_shader_sample_mask =
                fragment_shading_rate_with_shader_sample_mask.into();
            self
        }

        #[inline]
        pub fn fragment_shading_rate_with_conservative_rasterization(
            mut self,
            fragment_shading_rate_with_conservative_rasterization: bool,
        ) -> Self {
            self.fragment_shading_rate_with_conservative_rasterization =
                fragment_shading_rate_with_conservative_rasterization.into();
            self
        }

        #[inline]
        pub fn fragment_shading_rate_with_fragment_shader_interlock(
            mut self,
            fragment_shading_rate_with_fragment_shader_interlock: bool,
        ) -> Self {
            self.fragment_shading_rate_with_fragment_shader_interlock =
                fragment_shading_rate_with_fragment_shader_interlock.into();
            self
        }

        #[inline]
        pub fn fragment_shading_rate_with_custom_sample_locations(
            mut self,
            fragment_shading_rate_with_custom_sample_locations: bool,
        ) -> Self {
            self.fragment_shading_rate_with_custom_sample_locations =
                fragment_shading_rate_with_custom_sample_locations.into();
            self
        }

        #[inline]
        pub fn fragment_shading_rate_strict_multiply_combiner(
            mut self,
            fragment_shading_rate_strict_multiply_combiner: bool,
        ) -> Self {
            self.fragment_shading_rate_strict_multiply_combiner =
                fragment_shading_rate_strict_multiply_combiner.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFragmentShadingRateKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceFragmentShadingRateKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub sample_counts: SampleCountFlags,
        pub fragment_size: Extent2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceFragmentShadingRateKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceFragmentShadingRateKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("sample_counts", &self.sample_counts)
                .field("fragment_size", &self.fragment_size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentShadingRateKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR;
    }

    impl Default for PhysicalDeviceFragmentShadingRateKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                sample_counts: Default::default(),
                fragment_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceFragmentShadingRateKHR<'a> {
        #[inline]
        pub fn sample_counts(mut self, sample_counts: SampleCountFlags) -> Self {
            self.sample_counts = sample_counts;
            self
        }

        #[inline]
        pub fn fragment_size(mut self, fragment_size: Extent2D) -> Self {
            self.fragment_size = fragment_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingFragmentShadingRateAttachmentInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RenderingFragmentShadingRateAttachmentInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image_view: ImageView,
        pub image_layout: ImageLayout,
        pub shading_rate_attachment_texel_size: Extent2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderingFragmentShadingRateAttachmentInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderingFragmentShadingRateAttachmentInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image_view", &self.image_view)
                .field("image_layout", &self.image_layout)
                .field(
                    "shading_rate_attachment_texel_size",
                    &self.shading_rate_attachment_texel_size,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderingFragmentShadingRateAttachmentInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR;
    }

    unsafe impl Extends<RenderingInfo<'_>> for RenderingFragmentShadingRateAttachmentInfoKHR<'_> {}

    impl Default for RenderingFragmentShadingRateAttachmentInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                image_view: Default::default(),
                image_layout: Default::default(),
                shading_rate_attachment_texel_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RenderingFragmentShadingRateAttachmentInfoKHR<'a> {
        #[inline]
        pub fn image_view(mut self, image_view: ImageView) -> Self {
            self.image_view = image_view;
            self
        }

        #[inline]
        pub fn image_layout(mut self, image_layout: ImageLayout) -> Self {
            self.image_layout = image_layout;
            self
        }

        #[inline]
        pub fn shading_rate_attachment_texel_size(
            mut self,
            shading_rate_attachment_texel_size: Extent2D,
        ) -> Self {
            self.shading_rate_attachment_texel_size = shading_rate_attachment_texel_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFragmentShadingRateCombinerOpKHR.html>
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

    impl fmt::Debug for FragmentShadingRateCombinerOpKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::KEEP_KHR => Some("KEEP_KHR"),
                Self::REPLACE_KHR => Some("REPLACE_KHR"),
                Self::MIN_KHR => Some("MIN_KHR"),
                Self::MAX_KHR => Some("MAX_KHR"),
                Self::MUL_KHR => Some("MUL_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetFragmentShadingRateKHR.html>
    pub type PFN_vkCmdSetFragmentShadingRateKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_fragment_size: *const Extent2D,
        combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2],
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceFragmentShadingRatesKHR.html>
    pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_fragment_shading_rate_count: *mut u32,
            p_fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR<'_>,
        ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkFragmentShadingRateAttachmentInfoKHR = FragmentShadingRateAttachmentInfoKHR<'static>;
    pub type VkPipelineFragmentShadingRateStateCreateInfoKHR =
        PipelineFragmentShadingRateStateCreateInfoKHR<'static>;
    pub type VkPhysicalDeviceFragmentShadingRateFeaturesKHR =
        PhysicalDeviceFragmentShadingRateFeaturesKHR<'static>;
    pub type VkPhysicalDeviceFragmentShadingRatePropertiesKHR =
        PhysicalDeviceFragmentShadingRatePropertiesKHR<'static>;
    pub type VkPhysicalDeviceFragmentShadingRateKHR = PhysicalDeviceFragmentShadingRateKHR<'static>;
    pub type VkRenderingFragmentShadingRateAttachmentInfoKHR =
        RenderingFragmentShadingRateAttachmentInfoKHR<'static>;
    pub type VkFragmentShadingRateCombinerOpKHR = FragmentShadingRateCombinerOpKHR;
    impl FragmentShadingRateAttachmentInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkFragmentShadingRateAttachmentInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineFragmentShadingRateStateCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPipelineFragmentShadingRateStateCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceFragmentShadingRateFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceFragmentShadingRateFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceFragmentShadingRatePropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceFragmentShadingRatePropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceFragmentShadingRateKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceFragmentShadingRateKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl RenderingFragmentShadingRateAttachmentInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkRenderingFragmentShadingRateAttachmentInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    get_physical_device_fragment_shading_rates: PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_fragment_shading_rates: transmute(
                    load(c"vkGetPhysicalDeviceFragmentShadingRatesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceFragmentShadingRatesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_fragment_shading_rates<'a>(
        &self,
        physical_device: PhysicalDevice,
        mut fragment_shading_rates: impl EnumerateInto<PhysicalDeviceFragmentShadingRateKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |fragment_shading_rate_count, fragment_shading_rates| {
                let result = (self.get_physical_device_fragment_shading_rates)(
                    physical_device,
                    fragment_shading_rate_count,
                    fragment_shading_rates as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let fragment_shading_rates_buf = fragment_shading_rates.reserve(capacity);
            len = fragment_shading_rates_buf.len().try_into().unwrap();
            let result = call(&mut len, fragment_shading_rates_buf.as_mut_ptr() as *mut _)?;
            fragment_shading_rates.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}

pub struct DeviceFn {
    cmd_set_fragment_shading_rate: PFN_vkCmdSetFragmentShadingRateKHR,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_fragment_shading_rate: transmute(
                    load(c"vkCmdSetFragmentShadingRateKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetFragmentShadingRateKHR.html>
    #[inline]
    pub unsafe fn cmd_set_fragment_shading_rate(
        &self,
        command_buffer: CommandBuffer,
        fragment_size: &Extent2D,
        combiner_ops: &[FragmentShadingRateCombinerOpKHR; 2],
    ) {
        unsafe { (self.cmd_set_fragment_shading_rate)(command_buffer, fragment_size, combiner_ops) }
    }
}

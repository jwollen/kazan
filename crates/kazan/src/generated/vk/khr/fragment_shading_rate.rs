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
    pub struct FragmentShadingRateAttachmentInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_fragment_shading_rate_attachment: *const AttachmentReference2<'a>,
        pub shading_rate_attachment_texel_size: Extent2D,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for FragmentShadingRateAttachmentInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR;
    }
    unsafe impl<'a> Extends<SubpassDescription2<'a>> for FragmentShadingRateAttachmentInfoKHR<'a> {}
    impl Default for FragmentShadingRateAttachmentInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_fragment_shading_rate_attachment: core::ptr::null(),
                shading_rate_attachment_texel_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> FragmentShadingRateAttachmentInfoKHR<'a> {
        pub fn fragment_shading_rate_attachment(
            mut self,
            fragment_shading_rate_attachment: &'a AttachmentReference2<'a>,
        ) -> Self {
            self.p_fragment_shading_rate_attachment = fragment_shading_rate_attachment;
            self
        }
        pub fn shading_rate_attachment_texel_size(
            mut self,
            shading_rate_attachment_texel_size: Extent2D,
        ) -> Self {
            self.shading_rate_attachment_texel_size = shading_rate_attachment_texel_size;
            self
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
    unsafe impl<'a> TaggedStructure<'a> for PipelineFragmentShadingRateStateCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR;
    }
    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>>
        for PipelineFragmentShadingRateStateCreateInfoKHR<'a>
    {
    }
    impl Default for PipelineFragmentShadingRateStateCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                fragment_size: Default::default(),
                combiner_ops: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineFragmentShadingRateStateCreateInfoKHR<'a> {
        pub fn fragment_size(mut self, fragment_size: Extent2D) -> Self {
            self.fragment_size = fragment_size;
            self
        }
        pub fn combiner_ops(mut self, combiner_ops: [FragmentShadingRateCombinerOpKHR; 2]) -> Self {
            self.combiner_ops = combiner_ops;
            self
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
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentShadingRateFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceFragmentShadingRateFeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceFragmentShadingRateFeaturesKHR<'a> {}
    impl Default for PhysicalDeviceFragmentShadingRateFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                pipeline_fragment_shading_rate: Default::default(),
                primitive_fragment_shading_rate: Default::default(),
                attachment_fragment_shading_rate: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceFragmentShadingRateFeaturesKHR<'a> {
        pub fn pipeline_fragment_shading_rate(
            mut self,
            pipeline_fragment_shading_rate: Bool32,
        ) -> Self {
            self.pipeline_fragment_shading_rate = pipeline_fragment_shading_rate;
            self
        }
        pub fn primitive_fragment_shading_rate(
            mut self,
            primitive_fragment_shading_rate: Bool32,
        ) -> Self {
            self.primitive_fragment_shading_rate = primitive_fragment_shading_rate;
            self
        }
        pub fn attachment_fragment_shading_rate(
            mut self,
            attachment_fragment_shading_rate: Bool32,
        ) -> Self {
            self.attachment_fragment_shading_rate = attachment_fragment_shading_rate;
            self
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
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentShadingRatePropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceFragmentShadingRatePropertiesKHR<'a>
    {
    }
    impl Default for PhysicalDeviceFragmentShadingRatePropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
    impl<'a> PhysicalDeviceFragmentShadingRatePropertiesKHR<'a> {
        pub fn min_fragment_shading_rate_attachment_texel_size(
            mut self,
            min_fragment_shading_rate_attachment_texel_size: Extent2D,
        ) -> Self {
            self.min_fragment_shading_rate_attachment_texel_size =
                min_fragment_shading_rate_attachment_texel_size;
            self
        }
        pub fn max_fragment_shading_rate_attachment_texel_size(
            mut self,
            max_fragment_shading_rate_attachment_texel_size: Extent2D,
        ) -> Self {
            self.max_fragment_shading_rate_attachment_texel_size =
                max_fragment_shading_rate_attachment_texel_size;
            self
        }
        pub fn max_fragment_shading_rate_attachment_texel_size_aspect_ratio(
            mut self,
            max_fragment_shading_rate_attachment_texel_size_aspect_ratio: u32,
        ) -> Self {
            self.max_fragment_shading_rate_attachment_texel_size_aspect_ratio =
                max_fragment_shading_rate_attachment_texel_size_aspect_ratio;
            self
        }
        pub fn primitive_fragment_shading_rate_with_multiple_viewports(
            mut self,
            primitive_fragment_shading_rate_with_multiple_viewports: Bool32,
        ) -> Self {
            self.primitive_fragment_shading_rate_with_multiple_viewports =
                primitive_fragment_shading_rate_with_multiple_viewports;
            self
        }
        pub fn layered_shading_rate_attachments(
            mut self,
            layered_shading_rate_attachments: Bool32,
        ) -> Self {
            self.layered_shading_rate_attachments = layered_shading_rate_attachments;
            self
        }
        pub fn fragment_shading_rate_non_trivial_combiner_ops(
            mut self,
            fragment_shading_rate_non_trivial_combiner_ops: Bool32,
        ) -> Self {
            self.fragment_shading_rate_non_trivial_combiner_ops =
                fragment_shading_rate_non_trivial_combiner_ops;
            self
        }
        pub fn max_fragment_size(mut self, max_fragment_size: Extent2D) -> Self {
            self.max_fragment_size = max_fragment_size;
            self
        }
        pub fn max_fragment_size_aspect_ratio(
            mut self,
            max_fragment_size_aspect_ratio: u32,
        ) -> Self {
            self.max_fragment_size_aspect_ratio = max_fragment_size_aspect_ratio;
            self
        }
        pub fn max_fragment_shading_rate_coverage_samples(
            mut self,
            max_fragment_shading_rate_coverage_samples: u32,
        ) -> Self {
            self.max_fragment_shading_rate_coverage_samples =
                max_fragment_shading_rate_coverage_samples;
            self
        }
        pub fn max_fragment_shading_rate_rasterization_samples(
            mut self,
            max_fragment_shading_rate_rasterization_samples: SampleCountFlagBits,
        ) -> Self {
            self.max_fragment_shading_rate_rasterization_samples =
                max_fragment_shading_rate_rasterization_samples;
            self
        }
        pub fn fragment_shading_rate_with_shader_depth_stencil_writes(
            mut self,
            fragment_shading_rate_with_shader_depth_stencil_writes: Bool32,
        ) -> Self {
            self.fragment_shading_rate_with_shader_depth_stencil_writes =
                fragment_shading_rate_with_shader_depth_stencil_writes;
            self
        }
        pub fn fragment_shading_rate_with_sample_mask(
            mut self,
            fragment_shading_rate_with_sample_mask: Bool32,
        ) -> Self {
            self.fragment_shading_rate_with_sample_mask = fragment_shading_rate_with_sample_mask;
            self
        }
        pub fn fragment_shading_rate_with_shader_sample_mask(
            mut self,
            fragment_shading_rate_with_shader_sample_mask: Bool32,
        ) -> Self {
            self.fragment_shading_rate_with_shader_sample_mask =
                fragment_shading_rate_with_shader_sample_mask;
            self
        }
        pub fn fragment_shading_rate_with_conservative_rasterization(
            mut self,
            fragment_shading_rate_with_conservative_rasterization: Bool32,
        ) -> Self {
            self.fragment_shading_rate_with_conservative_rasterization =
                fragment_shading_rate_with_conservative_rasterization;
            self
        }
        pub fn fragment_shading_rate_with_fragment_shader_interlock(
            mut self,
            fragment_shading_rate_with_fragment_shader_interlock: Bool32,
        ) -> Self {
            self.fragment_shading_rate_with_fragment_shader_interlock =
                fragment_shading_rate_with_fragment_shader_interlock;
            self
        }
        pub fn fragment_shading_rate_with_custom_sample_locations(
            mut self,
            fragment_shading_rate_with_custom_sample_locations: Bool32,
        ) -> Self {
            self.fragment_shading_rate_with_custom_sample_locations =
                fragment_shading_rate_with_custom_sample_locations;
            self
        }
        pub fn fragment_shading_rate_strict_multiply_combiner(
            mut self,
            fragment_shading_rate_strict_multiply_combiner: Bool32,
        ) -> Self {
            self.fragment_shading_rate_strict_multiply_combiner =
                fragment_shading_rate_strict_multiply_combiner;
            self
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
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentShadingRateKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR;
    }
    impl Default for PhysicalDeviceFragmentShadingRateKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                sample_counts: Default::default(),
                fragment_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceFragmentShadingRateKHR<'a> {
        pub fn sample_counts(mut self, sample_counts: SampleCountFlags) -> Self {
            self.sample_counts = sample_counts;
            self
        }
        pub fn fragment_size(mut self, fragment_size: Extent2D) -> Self {
            self.fragment_size = fragment_size;
            self
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
    unsafe impl<'a> TaggedStructure<'a> for RenderingFragmentShadingRateAttachmentInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR;
    }
    unsafe impl<'a> Extends<RenderingInfo<'a>> for RenderingFragmentShadingRateAttachmentInfoKHR<'a> {}
    impl Default for RenderingFragmentShadingRateAttachmentInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                image_view: Default::default(),
                image_layout: Default::default(),
                shading_rate_attachment_texel_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderingFragmentShadingRateAttachmentInfoKHR<'a> {
        pub fn image_view(mut self, image_view: ImageView) -> Self {
            self.image_view = image_view;
            self
        }
        pub fn image_layout(mut self, image_layout: ImageLayout) -> Self {
            self.image_layout = image_layout;
            self
        }
        pub fn shading_rate_attachment_texel_size(
            mut self,
            shading_rate_attachment_texel_size: Extent2D,
        ) -> Self {
            self.shading_rate_attachment_texel_size = shading_rate_attachment_texel_size;
            self
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
    pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_fragment_shading_rate_count: *mut u32,
            p_fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR<'_>,
        ) -> vk::Result;
}
pub struct InstanceFn {
    get_physical_device_fragment_shading_rates_khr: PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_fragment_shading_rates_khr: transmute(
                    load(c"vkGetPhysicalDeviceFragmentShadingRatesKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_fragment_shading_rates_khr<'a>(
        &self,
        physical_device: PhysicalDevice,
        fragment_shading_rates: impl ExtendUninit<PhysicalDeviceFragmentShadingRateKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(
                fragment_shading_rates,
                |fragment_shading_rate_count, fragment_shading_rates| {
                    let result = (self.get_physical_device_fragment_shading_rates_khr)(
                        physical_device,
                        fragment_shading_rate_count,
                        fragment_shading_rates as _,
                    );

                    match result {
                        VkResult::SUCCESS => Ok(()),
                        VkResult::INCOMPLETE => Ok(()),
                        err => Err(err),
                    }
                },
            )
        }
    }
}
pub struct DeviceFn {
    cmd_set_fragment_shading_rate_khr: PFN_vkCmdSetFragmentShadingRateKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_fragment_shading_rate_khr: transmute(
                    load(c"vkCmdSetFragmentShadingRateKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_fragment_shading_rate_khr(
        &self,
        command_buffer: CommandBuffer,
        fragment_size: &Extent2D,
        combiner_ops: &[FragmentShadingRateCombinerOpKHR; 2],
    ) {
        unsafe {
            (self.cmd_set_fragment_shading_rate_khr)(command_buffer, fragment_size, combiner_ops)
        }
    }
}

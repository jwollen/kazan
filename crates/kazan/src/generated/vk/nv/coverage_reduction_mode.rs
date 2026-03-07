#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCoverageReductionModeFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceCoverageReductionModeFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub coverage_reduction_mode: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceCoverageReductionModeFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCoverageReductionModeFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("coverage_reduction_mode", &self.coverage_reduction_mode)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCoverageReductionModeFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceCoverageReductionModeFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceCoverageReductionModeFeaturesNV<'a>
    {
    }

    impl Default for PhysicalDeviceCoverageReductionModeFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                coverage_reduction_mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCoverageReductionModeFeaturesNV<'a> {
        pub fn coverage_reduction_mode(mut self, coverage_reduction_mode: bool) -> Self {
            self.coverage_reduction_mode = coverage_reduction_mode.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCoverageReductionStateCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineCoverageReductionStateCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineCoverageReductionStateCreateFlagsNV,
        pub coverage_reduction_mode: CoverageReductionModeNV,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PipelineCoverageReductionStateCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineCoverageReductionStateCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("coverage_reduction_mode", &self.coverage_reduction_mode)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineCoverageReductionStateCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV;
    }

    unsafe impl<'a> Extends<PipelineMultisampleStateCreateInfo<'a>>
        for PipelineCoverageReductionStateCreateInfoNV<'a>
    {
    }

    impl Default for PipelineCoverageReductionStateCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                coverage_reduction_mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineCoverageReductionStateCreateInfoNV<'a> {
        pub fn flags(mut self, flags: PipelineCoverageReductionStateCreateFlagsNV) -> Self {
            self.flags = flags;
            self
        }

        pub fn coverage_reduction_mode(
            mut self,
            coverage_reduction_mode: CoverageReductionModeNV,
        ) -> Self {
            self.coverage_reduction_mode = coverage_reduction_mode;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFramebufferMixedSamplesCombinationNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct FramebufferMixedSamplesCombinationNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub coverage_reduction_mode: CoverageReductionModeNV,
        pub rasterization_samples: SampleCountFlagBits,
        pub depth_stencil_samples: SampleCountFlags,
        pub color_samples: SampleCountFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for FramebufferMixedSamplesCombinationNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("FramebufferMixedSamplesCombinationNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("coverage_reduction_mode", &self.coverage_reduction_mode)
                .field("rasterization_samples", &self.rasterization_samples)
                .field("depth_stencil_samples", &self.depth_stencil_samples)
                .field("color_samples", &self.color_samples)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for FramebufferMixedSamplesCombinationNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV;
    }

    impl Default for FramebufferMixedSamplesCombinationNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                coverage_reduction_mode: Default::default(),
                rasterization_samples: Default::default(),
                depth_stencil_samples: Default::default(),
                color_samples: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> FramebufferMixedSamplesCombinationNV<'a> {
        pub fn coverage_reduction_mode(
            mut self,
            coverage_reduction_mode: CoverageReductionModeNV,
        ) -> Self {
            self.coverage_reduction_mode = coverage_reduction_mode;
            self
        }

        pub fn rasterization_samples(mut self, rasterization_samples: SampleCountFlagBits) -> Self {
            self.rasterization_samples = rasterization_samples;
            self
        }

        pub fn depth_stencil_samples(mut self, depth_stencil_samples: SampleCountFlags) -> Self {
            self.depth_stencil_samples = depth_stencil_samples;
            self
        }

        pub fn color_samples(mut self, color_samples: SampleCountFlags) -> Self {
            self.color_samples = color_samples;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCoverageReductionModeNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CoverageReductionModeNV(i32);

    impl CoverageReductionModeNV {
        pub const MERGE_NV: Self = Self(0);
        pub const TRUNCATE_NV: Self = Self(1);
    }

    impl fmt::Debug for CoverageReductionModeNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::MERGE_NV => Some("MERGE_NV"),
                Self::TRUNCATE_NV => Some("TRUNCATE_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCoverageReductionStateCreateFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineCoverageReductionStateCreateFlagsNV(Flags);
    vk_bitflags_wrapped!(PipelineCoverageReductionStateCreateFlagsNV, Flags);

    impl fmt::Debug for PipelineCoverageReductionStateCreateFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html>
    pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_combination_count: *mut u32,
            p_combinations: *mut FramebufferMixedSamplesCombinationNV<'_>,
        ) -> vk::Result;
}

pub struct InstanceFn {
    get_physical_device_supported_framebuffer_mixed_samples_combinations_nv:
        PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_supported_framebuffer_mixed_samples_combinations_nv: transmute(
                    load(c"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html>
    pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv<'a>(
        &self,
        physical_device: PhysicalDevice,
        mut combinations: impl ExtendUninit<FramebufferMixedSamplesCombinationNV<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |combination_count, combinations| {
                let result = (self
                    .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv)(
                    physical_device,
                    combination_count,
                    combinations as _,
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
            let combinations_buf = combinations.reserve(capacity);
            len = combinations_buf.len().try_into().unwrap();
            let result = call(&mut len, combinations_buf.as_mut_ptr() as *mut _)?;
            combinations.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}

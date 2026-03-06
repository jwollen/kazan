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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV.html>
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
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'a>
    {
    }
    impl Default for PhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                fragment_shading_rate_enums: Default::default(),
                supersample_fragment_shading_rates: Default::default(),
                no_invocation_fragment_shading_rates: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'a> {
        pub fn fragment_shading_rate_enums(mut self, fragment_shading_rate_enums: bool) -> Self {
            self.fragment_shading_rate_enums = fragment_shading_rate_enums.into();
            self
        }
        pub fn supersample_fragment_shading_rates(
            mut self,
            supersample_fragment_shading_rates: bool,
        ) -> Self {
            self.supersample_fragment_shading_rates = supersample_fragment_shading_rates.into();
            self
        }
        pub fn no_invocation_fragment_shading_rates(
            mut self,
            no_invocation_fragment_shading_rates: bool,
        ) -> Self {
            self.no_invocation_fragment_shading_rates = no_invocation_fragment_shading_rates.into();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_fragment_shading_rate_invocation_count: SampleCountFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'a>
    {
    }
    impl Default for PhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_fragment_shading_rate_invocation_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'a> {
        pub fn max_fragment_shading_rate_invocation_count(
            mut self,
            max_fragment_shading_rate_invocation_count: SampleCountFlagBits,
        ) -> Self {
            self.max_fragment_shading_rate_invocation_count =
                max_fragment_shading_rate_invocation_count;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineFragmentShadingRateEnumStateCreateInfoNV.html>
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
    unsafe impl<'a> TaggedStructure<'a> for PipelineFragmentShadingRateEnumStateCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV;
    }
    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>>
        for PipelineFragmentShadingRateEnumStateCreateInfoNV<'a>
    {
    }
    impl Default for PipelineFragmentShadingRateEnumStateCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                shading_rate_type: Default::default(),
                shading_rate: Default::default(),
                combiner_ops: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineFragmentShadingRateEnumStateCreateInfoNV<'a> {
        pub fn shading_rate_type(mut self, shading_rate_type: FragmentShadingRateTypeNV) -> Self {
            self.shading_rate_type = shading_rate_type;
            self
        }
        pub fn shading_rate(mut self, shading_rate: FragmentShadingRateNV) -> Self {
            self.shading_rate = shading_rate;
            self
        }
        pub fn combiner_ops(mut self, combiner_ops: [FragmentShadingRateCombinerOpKHR; 2]) -> Self {
            self.combiner_ops = combiner_ops;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFragmentShadingRateNV.html>
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
    impl fmt::Debug for FragmentShadingRateNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_1_INVOCATION_PER_PIXEL_NV => Some("_1_INVOCATION_PER_PIXEL_NV"),
                Self::_1_INVOCATION_PER_1X2_PIXELS_NV => Some("_1_INVOCATION_PER_1X2_PIXELS_NV"),
                Self::_1_INVOCATION_PER_2X1_PIXELS_NV => Some("_1_INVOCATION_PER_2X1_PIXELS_NV"),
                Self::_1_INVOCATION_PER_2X2_PIXELS_NV => Some("_1_INVOCATION_PER_2X2_PIXELS_NV"),
                Self::_1_INVOCATION_PER_2X4_PIXELS_NV => Some("_1_INVOCATION_PER_2X4_PIXELS_NV"),
                Self::_1_INVOCATION_PER_4X2_PIXELS_NV => Some("_1_INVOCATION_PER_4X2_PIXELS_NV"),
                Self::_1_INVOCATION_PER_4X4_PIXELS_NV => Some("_1_INVOCATION_PER_4X4_PIXELS_NV"),
                Self::_2_INVOCATIONS_PER_PIXEL_NV => Some("_2_INVOCATIONS_PER_PIXEL_NV"),
                Self::_4_INVOCATIONS_PER_PIXEL_NV => Some("_4_INVOCATIONS_PER_PIXEL_NV"),
                Self::_8_INVOCATIONS_PER_PIXEL_NV => Some("_8_INVOCATIONS_PER_PIXEL_NV"),
                Self::_16_INVOCATIONS_PER_PIXEL_NV => Some("_16_INVOCATIONS_PER_PIXEL_NV"),
                Self::NO_INVOCATIONS_NV => Some("NO_INVOCATIONS_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFragmentShadingRateTypeNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FragmentShadingRateTypeNV(i32);
    impl FragmentShadingRateTypeNV {
        pub const FRAGMENT_SIZE_NV: Self = Self(0);
        pub const ENUMS_NV: Self = Self(1);
    }
    impl fmt::Debug for FragmentShadingRateTypeNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::FRAGMENT_SIZE_NV => Some("FRAGMENT_SIZE_NV"),
                Self::ENUMS_NV => Some("ENUMS_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetFragmentShadingRateEnumNV.html>
    pub type PFN_vkCmdSetFragmentShadingRateEnumNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2],
    );
}
pub struct DeviceFn {
    cmd_set_fragment_shading_rate_enum_nv: PFN_vkCmdSetFragmentShadingRateEnumNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_fragment_shading_rate_enum_nv: transmute(
                    load(c"vkCmdSetFragmentShadingRateEnumNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetFragmentShadingRateEnumNV.html>
    pub unsafe fn cmd_set_fragment_shading_rate_enum_nv(
        &self,
        command_buffer: CommandBuffer,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: &[FragmentShadingRateCombinerOpKHR; 2],
    ) {
        unsafe {
            (self.cmd_set_fragment_shading_rate_enum_nv)(command_buffer, shading_rate, combiner_ops)
        }
    }
}

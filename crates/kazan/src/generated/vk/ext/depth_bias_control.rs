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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DepthBiasInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub depth_bias_constant_factor: f32,
        pub depth_bias_clamp: f32,
        pub depth_bias_slope_factor: f32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DepthBiasInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEPTH_BIAS_INFO_EXT;
    }
    impl Default for DepthBiasInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                depth_bias_constant_factor: Default::default(),
                depth_bias_clamp: Default::default(),
                depth_bias_slope_factor: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DepthBiasInfoEXT<'a> {
        pub fn depth_bias_constant_factor(mut self, depth_bias_constant_factor: f32) -> Self {
            self.depth_bias_constant_factor = depth_bias_constant_factor;
            self
        }
        pub fn depth_bias_clamp(mut self, depth_bias_clamp: f32) -> Self {
            self.depth_bias_clamp = depth_bias_clamp;
            self
        }
        pub fn depth_bias_slope_factor(mut self, depth_bias_slope_factor: f32) -> Self {
            self.depth_bias_slope_factor = depth_bias_slope_factor;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DepthBiasRepresentationInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub depth_bias_representation: DepthBiasRepresentationEXT,
        pub depth_bias_exact: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DepthBiasRepresentationInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEPTH_BIAS_REPRESENTATION_INFO_EXT;
    }
    unsafe impl<'a> Extends<DepthBiasInfoEXT<'a>> for DepthBiasRepresentationInfoEXT<'a> {}
    unsafe impl<'a> Extends<PipelineRasterizationStateCreateInfo<'a>>
        for DepthBiasRepresentationInfoEXT<'a>
    {
    }
    impl Default for DepthBiasRepresentationInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                depth_bias_representation: Default::default(),
                depth_bias_exact: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DepthBiasRepresentationInfoEXT<'a> {
        pub fn depth_bias_representation(
            mut self,
            depth_bias_representation: DepthBiasRepresentationEXT,
        ) -> Self {
            self.depth_bias_representation = depth_bias_representation;
            self
        }
        pub fn depth_bias_exact(mut self, depth_bias_exact: Bool32) -> Self {
            self.depth_bias_exact = depth_bias_exact;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDepthBiasControlFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub depth_bias_control: Bool32,
        pub least_representable_value_force_unorm_representation: Bool32,
        pub float_representation: Bool32,
        pub depth_bias_exact: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDepthBiasControlFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDepthBiasControlFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceDepthBiasControlFeaturesEXT<'a> {}
    impl Default for PhysicalDeviceDepthBiasControlFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                depth_bias_control: Default::default(),
                least_representable_value_force_unorm_representation: Default::default(),
                float_representation: Default::default(),
                depth_bias_exact: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDepthBiasControlFeaturesEXT<'a> {
        pub fn depth_bias_control(mut self, depth_bias_control: Bool32) -> Self {
            self.depth_bias_control = depth_bias_control;
            self
        }
        pub fn least_representable_value_force_unorm_representation(
            mut self,
            least_representable_value_force_unorm_representation: Bool32,
        ) -> Self {
            self.least_representable_value_force_unorm_representation =
                least_representable_value_force_unorm_representation;
            self
        }
        pub fn float_representation(mut self, float_representation: Bool32) -> Self {
            self.float_representation = float_representation;
            self
        }
        pub fn depth_bias_exact(mut self, depth_bias_exact: Bool32) -> Self {
            self.depth_bias_exact = depth_bias_exact;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DepthBiasRepresentationEXT(i32);
    impl DepthBiasRepresentationEXT {
        pub const LEAST_REPRESENTABLE_VALUE_FORMAT_EXT: Self = Self(0);
        pub const LEAST_REPRESENTABLE_VALUE_FORCE_UNORM_EXT: Self = Self(1);
        pub const FLOAT_EXT: Self = Self(2);
    }
    impl fmt::Debug for DepthBiasRepresentationEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::LEAST_REPRESENTABLE_VALUE_FORMAT_EXT => {
                    Some("LEAST_REPRESENTABLE_VALUE_FORMAT_EXT")
                }
                Self::LEAST_REPRESENTABLE_VALUE_FORCE_UNORM_EXT => {
                    Some("LEAST_REPRESENTABLE_VALUE_FORCE_UNORM_EXT")
                }
                Self::FLOAT_EXT => Some("FLOAT_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    pub type PFN_vkCmdSetDepthBias2EXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_depth_bias_info: *const DepthBiasInfoEXT<'_>,
    );
}
pub struct DeviceFn {
    cmd_set_depth_bias2_ext: PFN_vkCmdSetDepthBias2EXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_depth_bias2_ext: transmute(
                    load(c"vkCmdSetDepthBias2EXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_depth_bias2_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_info: &DepthBiasInfoEXT<'_>,
    ) {
        unsafe { (self.cmd_set_depth_bias2_ext)(command_buffer, depth_bias_info) }
    }
}

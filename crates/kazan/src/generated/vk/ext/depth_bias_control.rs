//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_depth_bias_control.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_depth_bias_control";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDepthBiasInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DepthBiasInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub depth_bias_constant_factor: f32,
        pub depth_bias_clamp: f32,
        pub depth_bias_slope_factor: f32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DepthBiasInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DepthBiasInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "depth_bias_constant_factor",
                    &self.depth_bias_constant_factor,
                )
                .field("depth_bias_clamp", &self.depth_bias_clamp)
                .field("depth_bias_slope_factor", &self.depth_bias_slope_factor)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DepthBiasInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEPTH_BIAS_INFO_EXT;
    }

    impl Default for DepthBiasInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                depth_bias_constant_factor: Default::default(),
                depth_bias_clamp: Default::default(),
                depth_bias_slope_factor: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DepthBiasInfoEXT<'a> {
        #[inline]
        pub fn depth_bias_constant_factor(mut self, depth_bias_constant_factor: f32) -> Self {
            self.depth_bias_constant_factor = depth_bias_constant_factor;
            self
        }

        #[inline]
        pub fn depth_bias_clamp(mut self, depth_bias_clamp: f32) -> Self {
            self.depth_bias_clamp = depth_bias_clamp;
            self
        }

        #[inline]
        pub fn depth_bias_slope_factor(mut self, depth_bias_slope_factor: f32) -> Self {
            self.depth_bias_slope_factor = depth_bias_slope_factor;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDepthBiasRepresentationInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DepthBiasRepresentationInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub depth_bias_representation: DepthBiasRepresentationEXT,
        pub depth_bias_exact: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DepthBiasRepresentationInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DepthBiasRepresentationInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("depth_bias_representation", &self.depth_bias_representation)
                .field("depth_bias_exact", &self.depth_bias_exact)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DepthBiasRepresentationInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEPTH_BIAS_REPRESENTATION_INFO_EXT;
    }

    unsafe impl Extends<DepthBiasInfoEXT<'_>> for DepthBiasRepresentationInfoEXT<'_> {}
    unsafe impl Extends<PipelineRasterizationStateCreateInfo<'_>>
        for DepthBiasRepresentationInfoEXT<'_>
    {
    }

    impl Default for DepthBiasRepresentationInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                depth_bias_representation: Default::default(),
                depth_bias_exact: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DepthBiasRepresentationInfoEXT<'a> {
        #[inline]
        pub fn depth_bias_representation(
            mut self,
            depth_bias_representation: DepthBiasRepresentationEXT,
        ) -> Self {
            self.depth_bias_representation = depth_bias_representation;
            self
        }

        #[inline]
        pub fn depth_bias_exact(mut self, depth_bias_exact: bool) -> Self {
            self.depth_bias_exact = depth_bias_exact.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDepthBiasControlFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDepthBiasControlFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub depth_bias_control: Bool32,
        pub least_representable_value_force_unorm_representation: Bool32,
        pub float_representation: Bool32,
        pub depth_bias_exact: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDepthBiasControlFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDepthBiasControlFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("depth_bias_control", &self.depth_bias_control)
                .field(
                    "least_representable_value_force_unorm_representation",
                    &self.least_representable_value_force_unorm_representation,
                )
                .field("float_representation", &self.float_representation)
                .field("depth_bias_exact", &self.depth_bias_exact)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDepthBiasControlFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceDepthBiasControlFeaturesEXT<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceDepthBiasControlFeaturesEXT<'_> {}

    impl Default for PhysicalDeviceDepthBiasControlFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                depth_bias_control: Default::default(),
                least_representable_value_force_unorm_representation: Default::default(),
                float_representation: Default::default(),
                depth_bias_exact: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDepthBiasControlFeaturesEXT<'a> {
        #[inline]
        pub fn depth_bias_control(mut self, depth_bias_control: bool) -> Self {
            self.depth_bias_control = depth_bias_control.into();
            self
        }

        #[inline]
        pub fn least_representable_value_force_unorm_representation(
            mut self,
            least_representable_value_force_unorm_representation: bool,
        ) -> Self {
            self.least_representable_value_force_unorm_representation =
                least_representable_value_force_unorm_representation.into();
            self
        }

        #[inline]
        pub fn float_representation(mut self, float_representation: bool) -> Self {
            self.float_representation = float_representation.into();
            self
        }

        #[inline]
        pub fn depth_bias_exact(mut self, depth_bias_exact: bool) -> Self {
            self.depth_bias_exact = depth_bias_exact.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDepthBiasRepresentationEXT.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthBias2EXT.html>
    pub type PFN_vkCmdSetDepthBias2EXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_depth_bias_info: *const DepthBiasInfoEXT<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkDepthBiasInfoEXT = DepthBiasInfoEXT<'static>;
    pub type VkDepthBiasRepresentationInfoEXT = DepthBiasRepresentationInfoEXT<'static>;
    pub type VkPhysicalDeviceDepthBiasControlFeaturesEXT =
        PhysicalDeviceDepthBiasControlFeaturesEXT<'static>;
    pub type VkDepthBiasRepresentationEXT = DepthBiasRepresentationEXT;
    impl DepthBiasInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDepthBiasInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DepthBiasRepresentationInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDepthBiasRepresentationInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceDepthBiasControlFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceDepthBiasControlFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    cmd_set_depth_bias2_ext: PFN_vkCmdSetDepthBias2EXT,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthBias2EXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_bias2_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_info: &DepthBiasInfoEXT<'_>,
    ) {
        unsafe { (self.cmd_set_depth_bias2_ext)(command_buffer, depth_bias_info) }
    }
}

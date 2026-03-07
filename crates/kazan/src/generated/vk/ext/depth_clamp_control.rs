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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDepthClampControlFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDepthClampControlFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub depth_clamp_control: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceDepthClampControlFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDepthClampControlFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("depth_clamp_control", &self.depth_clamp_control)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDepthClampControlFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DEPTH_CLAMP_CONTROL_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDepthClampControlFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceDepthClampControlFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceDepthClampControlFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                depth_clamp_control: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDepthClampControlFeaturesEXT<'a> {
        pub fn depth_clamp_control(mut self, depth_clamp_control: bool) -> Self {
            self.depth_clamp_control = depth_clamp_control.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineViewportDepthClampControlCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineViewportDepthClampControlCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub depth_clamp_mode: DepthClampModeEXT,
        pub p_depth_clamp_range: *const DepthClampRangeEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PipelineViewportDepthClampControlCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineViewportDepthClampControlCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("depth_clamp_mode", &self.depth_clamp_mode)
                .field("p_depth_clamp_range", &self.p_depth_clamp_range)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineViewportDepthClampControlCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_VIEWPORT_DEPTH_CLAMP_CONTROL_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<PipelineViewportStateCreateInfo<'a>>
        for PipelineViewportDepthClampControlCreateInfoEXT<'a>
    {
    }

    impl Default for PipelineViewportDepthClampControlCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                depth_clamp_mode: Default::default(),
                p_depth_clamp_range: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineViewportDepthClampControlCreateInfoEXT<'a> {
        pub fn depth_clamp_mode(mut self, depth_clamp_mode: DepthClampModeEXT) -> Self {
            self.depth_clamp_mode = depth_clamp_mode;
            self
        }

        pub fn depth_clamp_range(mut self, depth_clamp_range: &'a DepthClampRangeEXT) -> Self {
            self.p_depth_clamp_range = depth_clamp_range;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDepthClampRangeEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct DepthClampRangeEXT {
        pub min_depth_clamp: f32,
        pub max_depth_clamp: f32,
    }

    impl DepthClampRangeEXT {
        pub fn min_depth_clamp(mut self, min_depth_clamp: f32) -> Self {
            self.min_depth_clamp = min_depth_clamp;
            self
        }

        pub fn max_depth_clamp(mut self, max_depth_clamp: f32) -> Self {
            self.max_depth_clamp = max_depth_clamp;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDepthClampModeEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DepthClampModeEXT(i32);

    impl DepthClampModeEXT {
        pub const VIEWPORT_RANGE_EXT: Self = Self(0);
        pub const USER_DEFINED_RANGE_EXT: Self = Self(1);
    }

    impl fmt::Debug for DepthClampModeEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::VIEWPORT_RANGE_EXT => Some("VIEWPORT_RANGE_EXT"),
                Self::USER_DEFINED_RANGE_EXT => Some("USER_DEFINED_RANGE_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
}

pub struct DeviceFn {
    cmd_set_depth_clamp_range_ext: PFN_vkCmdSetDepthClampRangeEXT,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_depth_clamp_range_ext: transmute(
                    load(c"vkCmdSetDepthClampRangeEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthClampRangeEXT.html>
    pub unsafe fn cmd_set_depth_clamp_range_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_clamp_mode: DepthClampModeEXT,
        depth_clamp_range: Option<&DepthClampRangeEXT>,
    ) {
        unsafe {
            (self.cmd_set_depth_clamp_range_ext)(
                command_buffer,
                depth_clamp_mode,
                depth_clamp_range.to_raw_ptr(),
            )
        }
    }
}

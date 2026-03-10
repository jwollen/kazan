//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_depth_clip_enable.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_depth_clip_enable";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDepthClipEnableFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDepthClipEnableFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub depth_clip_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDepthClipEnableFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDepthClipEnableFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("depth_clip_enable", &self.depth_clip_enable)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDepthClipEnableFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceDepthClipEnableFeaturesEXT<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceDepthClipEnableFeaturesEXT<'_> {}

    impl Default for PhysicalDeviceDepthClipEnableFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                depth_clip_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDepthClipEnableFeaturesEXT<'a> {
        #[inline]
        pub fn depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
            self.depth_clip_enable = depth_clip_enable.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRasterizationDepthClipStateCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineRasterizationDepthClipStateCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineRasterizationDepthClipStateCreateFlagsEXT,
        pub depth_clip_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineRasterizationDepthClipStateCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineRasterizationDepthClipStateCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("depth_clip_enable", &self.depth_clip_enable)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineRasterizationDepthClipStateCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT;
    }

    unsafe impl Extends<PipelineRasterizationStateCreateInfo<'_>>
        for PipelineRasterizationDepthClipStateCreateInfoEXT<'_>
    {
    }

    impl Default for PipelineRasterizationDepthClipStateCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                depth_clip_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineRasterizationDepthClipStateCreateInfoEXT<'a> {
        #[inline]
        pub fn flags(mut self, flags: PipelineRasterizationDepthClipStateCreateFlagsEXT) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
            self.depth_clip_enable = depth_clip_enable.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRasterizationDepthClipStateCreateFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineRasterizationDepthClipStateCreateFlagsEXT(Flags);
    vk_bitflags_wrapped!(PipelineRasterizationDepthClipStateCreateFlagsEXT, Flags);

    impl fmt::Debug for PipelineRasterizationDepthClipStateCreateFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceDepthClipEnableFeaturesEXT =
        PhysicalDeviceDepthClipEnableFeaturesEXT<'static>;
    pub type VkPipelineRasterizationDepthClipStateCreateInfoEXT =
        PipelineRasterizationDepthClipStateCreateInfoEXT<'static>;
    pub type VkPipelineRasterizationDepthClipStateCreateFlagsEXT =
        PipelineRasterizationDepthClipStateCreateFlagsEXT;
    impl PhysicalDeviceDepthClipEnableFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceDepthClipEnableFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineRasterizationDepthClipStateCreateInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPipelineRasterizationDepthClipStateCreateInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

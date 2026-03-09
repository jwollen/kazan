//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_AMD_rasterization_order.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_AMD_rasterization_order";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRasterizationStateRasterizationOrderAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineRasterizationStateRasterizationOrderAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub rasterization_order: RasterizationOrderAMD,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineRasterizationStateRasterizationOrderAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineRasterizationStateRasterizationOrderAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("rasterization_order", &self.rasterization_order)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineRasterizationStateRasterizationOrderAMD<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD;
    }

    unsafe impl<'a> Extends<PipelineRasterizationStateCreateInfo<'a>>
        for PipelineRasterizationStateRasterizationOrderAMD<'a>
    {
    }

    impl Default for PipelineRasterizationStateRasterizationOrderAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                rasterization_order: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineRasterizationStateRasterizationOrderAMD<'a> {
        #[inline]
        pub fn rasterization_order(mut self, rasterization_order: RasterizationOrderAMD) -> Self {
            self.rasterization_order = rasterization_order;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRasterizationOrderAMD.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RasterizationOrderAMD(i32);

    impl RasterizationOrderAMD {
        pub const STRICT_AMD: Self = Self(0);
        pub const RELAXED_AMD: Self = Self(1);
    }

    impl fmt::Debug for RasterizationOrderAMD {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::STRICT_AMD => Some("STRICT_AMD"),
                Self::RELAXED_AMD => Some("RELAXED_AMD"),
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

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPipelineRasterizationStateRasterizationOrderAMD =
        PipelineRasterizationStateRasterizationOrderAMD<'static>;
    pub type VkRasterizationOrderAMD = RasterizationOrderAMD;
    impl PipelineRasterizationStateRasterizationOrderAMD<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPipelineRasterizationStateRasterizationOrderAMD {
            unsafe { core::mem::transmute(self) }
        }
    }
}

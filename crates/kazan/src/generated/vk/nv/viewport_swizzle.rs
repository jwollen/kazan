//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_viewport_swizzle.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_viewport_swizzle";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkViewportSwizzleNV.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct ViewportSwizzleNV {
        pub x: ViewportCoordinateSwizzleNV,
        pub y: ViewportCoordinateSwizzleNV,
        pub z: ViewportCoordinateSwizzleNV,
        pub w: ViewportCoordinateSwizzleNV,
    }

    impl ViewportSwizzleNV {
        #[inline]
        pub fn x(mut self, x: ViewportCoordinateSwizzleNV) -> Self {
            self.x = x;
            self
        }

        #[inline]
        pub fn y(mut self, y: ViewportCoordinateSwizzleNV) -> Self {
            self.y = y;
            self
        }

        #[inline]
        pub fn z(mut self, z: ViewportCoordinateSwizzleNV) -> Self {
            self.z = z;
            self
        }

        #[inline]
        pub fn w(mut self, w: ViewportCoordinateSwizzleNV) -> Self {
            self.w = w;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineViewportSwizzleStateCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineViewportSwizzleStateCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineViewportSwizzleStateCreateFlagsNV,
        pub viewport_count: u32,
        pub p_viewport_swizzles: *const ViewportSwizzleNV,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineViewportSwizzleStateCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineViewportSwizzleStateCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("viewport_count", &self.viewport_count)
                .field("p_viewport_swizzles", &self.p_viewport_swizzles)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineViewportSwizzleStateCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV;
    }

    unsafe impl<'a> Extends<PipelineViewportStateCreateInfo<'a>>
        for PipelineViewportSwizzleStateCreateInfoNV<'a>
    {
    }

    impl Default for PipelineViewportSwizzleStateCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                viewport_count: Default::default(),
                p_viewport_swizzles: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineViewportSwizzleStateCreateInfoNV<'a> {
        #[inline]
        pub fn flags(mut self, flags: PipelineViewportSwizzleStateCreateFlagsNV) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn viewport_swizzles(mut self, viewport_swizzles: &'a [ViewportSwizzleNV]) -> Self {
            self.viewport_count = viewport_swizzles.len().try_into().unwrap();
            self.p_viewport_swizzles = viewport_swizzles.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkViewportCoordinateSwizzleNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ViewportCoordinateSwizzleNV(i32);

    impl ViewportCoordinateSwizzleNV {
        pub const POSITIVE_X_NV: Self = Self(0);
        pub const NEGATIVE_X_NV: Self = Self(1);
        pub const POSITIVE_Y_NV: Self = Self(2);
        pub const NEGATIVE_Y_NV: Self = Self(3);
        pub const POSITIVE_Z_NV: Self = Self(4);
        pub const NEGATIVE_Z_NV: Self = Self(5);
        pub const POSITIVE_W_NV: Self = Self(6);
        pub const NEGATIVE_W_NV: Self = Self(7);
    }

    impl fmt::Debug for ViewportCoordinateSwizzleNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::POSITIVE_X_NV => Some("POSITIVE_X_NV"),
                Self::NEGATIVE_X_NV => Some("NEGATIVE_X_NV"),
                Self::POSITIVE_Y_NV => Some("POSITIVE_Y_NV"),
                Self::NEGATIVE_Y_NV => Some("NEGATIVE_Y_NV"),
                Self::POSITIVE_Z_NV => Some("POSITIVE_Z_NV"),
                Self::NEGATIVE_Z_NV => Some("NEGATIVE_Z_NV"),
                Self::POSITIVE_W_NV => Some("POSITIVE_W_NV"),
                Self::NEGATIVE_W_NV => Some("NEGATIVE_W_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineViewportSwizzleStateCreateFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineViewportSwizzleStateCreateFlagsNV(Flags);
    vk_bitflags_wrapped!(PipelineViewportSwizzleStateCreateFlagsNV, Flags);

    impl fmt::Debug for PipelineViewportSwizzleStateCreateFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }
}

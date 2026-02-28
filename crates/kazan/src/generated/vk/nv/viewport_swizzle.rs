#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct ViewportSwizzleNV {
        pub x: ViewportCoordinateSwizzleNV,
        pub y: ViewportCoordinateSwizzleNV,
        pub z: ViewportCoordinateSwizzleNV,
        pub w: ViewportCoordinateSwizzleNV,
    }
    impl ViewportSwizzleNV {
        pub fn x(mut self, x: ViewportCoordinateSwizzleNV) -> Self {
            self.x = x;
            self
        }
        pub fn y(mut self, y: ViewportCoordinateSwizzleNV) -> Self {
            self.y = y;
            self
        }
        pub fn z(mut self, z: ViewportCoordinateSwizzleNV) -> Self {
            self.z = z;
            self
        }
        pub fn w(mut self, w: ViewportCoordinateSwizzleNV) -> Self {
            self.w = w;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineViewportSwizzleStateCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineViewportSwizzleStateCreateFlagsNV,
        pub viewport_count: u32,
        pub p_viewport_swizzles: *const ViewportSwizzleNV,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PipelineViewportSwizzleStateCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV,
                p_next: core::ptr::null(),
                flags: Default::default(),
                viewport_count: Default::default(),
                p_viewport_swizzles: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineViewportSwizzleStateCreateInfoNV<'a> {
        pub fn flags(mut self, flags: PipelineViewportSwizzleStateCreateFlagsNV) -> Self {
            self.flags = flags;
            self
        }
        pub fn viewport_swizzles(mut self, viewport_swizzles: &'a [ViewportSwizzleNV]) -> Self {
            self.viewport_count = viewport_swizzles.len().try_into().unwrap();
            self.p_viewport_swizzles = viewport_swizzles.as_ptr();
            self
        }
    }
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
    bitflags! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct PipelineViewportSwizzleStateCreateFlagsNV: Flags {
        }
    }
}

//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_frame_boundary.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_frame_boundary";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFrameBoundaryEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct FrameBoundaryEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: FrameBoundaryFlagsEXT,
        pub frame_id: u64,
        pub image_count: u32,
        pub p_images: *const Image,
        pub buffer_count: u32,
        pub p_buffers: *const Buffer,
        pub tag_name: u64,
        pub tag_size: usize,
        pub p_tag: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for FrameBoundaryEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("FrameBoundaryEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("frame_id", &self.frame_id)
                .field("image_count", &self.image_count)
                .field("p_images", &self.p_images)
                .field("buffer_count", &self.buffer_count)
                .field("p_buffers", &self.p_buffers)
                .field("tag_name", &self.tag_name)
                .field("tag_size", &self.tag_size)
                .field("p_tag", &self.p_tag)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for FrameBoundaryEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::FRAME_BOUNDARY_EXT;
    }

    unsafe impl Extends<SubmitInfo<'_>> for FrameBoundaryEXT<'_> {}
    unsafe impl Extends<SubmitInfo2<'_>> for FrameBoundaryEXT<'_> {}
    unsafe impl Extends<PresentInfoKHR<'_>> for FrameBoundaryEXT<'_> {}
    unsafe impl Extends<BindSparseInfo<'_>> for FrameBoundaryEXT<'_> {}

    impl Default for FrameBoundaryEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                frame_id: Default::default(),
                image_count: Default::default(),
                p_images: ptr::null(),
                buffer_count: Default::default(),
                p_buffers: ptr::null(),
                tag_name: Default::default(),
                tag_size: Default::default(),
                p_tag: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> FrameBoundaryEXT<'a> {
        #[inline]
        pub fn flags(mut self, flags: FrameBoundaryFlagsEXT) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn frame_id(mut self, frame_id: u64) -> Self {
            self.frame_id = frame_id;
            self
        }

        #[inline]
        pub fn images(mut self, images: &'a [Image]) -> Self {
            self.image_count = images.len().try_into().unwrap();
            self.p_images = images.as_ptr() as _;
            self
        }

        #[inline]
        pub fn buffers(mut self, buffers: &'a [Buffer]) -> Self {
            self.buffer_count = buffers.len().try_into().unwrap();
            self.p_buffers = buffers.as_ptr() as _;
            self
        }

        #[inline]
        pub fn tag_name(mut self, tag_name: u64) -> Self {
            self.tag_name = tag_name;
            self
        }

        #[inline]
        pub fn tag(mut self, tag: &'a [u8]) -> Self {
            self.tag_size = tag.len().try_into().unwrap();
            self.p_tag = tag.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFrameBoundaryFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceFrameBoundaryFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub frame_boundary: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceFrameBoundaryFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceFrameBoundaryFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("frame_boundary", &self.frame_boundary)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFrameBoundaryFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceFrameBoundaryFeaturesEXT<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceFrameBoundaryFeaturesEXT<'_> {}

    impl Default for PhysicalDeviceFrameBoundaryFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                frame_boundary: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceFrameBoundaryFeaturesEXT<'a> {
        #[inline]
        pub fn frame_boundary(mut self, frame_boundary: bool) -> Self {
            self.frame_boundary = frame_boundary.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFrameBoundaryFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct FrameBoundaryFlagsEXT(Flags);
    vk_bitflags_wrapped!(FrameBoundaryFlagsEXT, Flags);

    impl FrameBoundaryFlagsEXT {
        pub const FRAME_END_EXT: Self = Self(FrameBoundaryFlagBitsEXT::FRAME_END_EXT.0);
    }

    impl fmt::Debug for FrameBoundaryFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] =
                &[(FrameBoundaryFlagsEXT::FRAME_END_EXT.0, "FRAME_END_EXT")];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFrameBoundaryFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct FrameBoundaryFlagBitsEXT(u32);

    impl FrameBoundaryFlagBitsEXT {
        pub const FRAME_END_EXT: Self = Self(1 << 0);
    }

    impl fmt::Debug for FrameBoundaryFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::FRAME_END_EXT => Some("FRAME_END_EXT"),
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

    pub type VkFrameBoundaryEXT = FrameBoundaryEXT<'static>;
    pub type VkPhysicalDeviceFrameBoundaryFeaturesEXT =
        PhysicalDeviceFrameBoundaryFeaturesEXT<'static>;
    pub type VkFrameBoundaryFlagsEXT = FrameBoundaryFlagsEXT;
    pub type VkFrameBoundaryFlagBitsEXT = FrameBoundaryFlagBitsEXT;
    impl FrameBoundaryEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkFrameBoundaryEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceFrameBoundaryFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceFrameBoundaryFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

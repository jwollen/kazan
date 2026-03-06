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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFrameBoundaryEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for FrameBoundaryEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::FRAME_BOUNDARY_EXT;
    }
    unsafe impl<'a> Extends<SubmitInfo<'a>> for FrameBoundaryEXT<'a> {}
    unsafe impl<'a> Extends<SubmitInfo2<'a>> for FrameBoundaryEXT<'a> {}
    unsafe impl<'a> Extends<PresentInfoKHR<'a>> for FrameBoundaryEXT<'a> {}
    unsafe impl<'a> Extends<BindSparseInfo<'a>> for FrameBoundaryEXT<'a> {}
    impl Default for FrameBoundaryEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                frame_id: Default::default(),
                image_count: Default::default(),
                p_images: core::ptr::null(),
                buffer_count: Default::default(),
                p_buffers: core::ptr::null(),
                tag_name: Default::default(),
                tag_size: Default::default(),
                p_tag: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> FrameBoundaryEXT<'a> {
        pub fn flags(mut self, flags: FrameBoundaryFlagsEXT) -> Self {
            self.flags = flags;
            self
        }
        pub fn frame_id(mut self, frame_id: u64) -> Self {
            self.frame_id = frame_id;
            self
        }
        pub fn images(mut self, images: &'a [Image]) -> Self {
            self.image_count = images.len().try_into().unwrap();
            self.p_images = images.as_ptr();
            self
        }
        pub fn buffers(mut self, buffers: &'a [Buffer]) -> Self {
            self.buffer_count = buffers.len().try_into().unwrap();
            self.p_buffers = buffers.as_ptr();
            self
        }
        pub fn tag_name(mut self, tag_name: u64) -> Self {
            self.tag_name = tag_name;
            self
        }
        pub fn tag(mut self, tag: &'a [u8]) -> Self {
            self.tag_size = tag.len().try_into().unwrap();
            self.p_tag = tag.as_ptr() as _;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFrameBoundaryFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceFrameBoundaryFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub frame_boundary: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFrameBoundaryFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceFrameBoundaryFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceFrameBoundaryFeaturesEXT<'a> {}
    impl Default for PhysicalDeviceFrameBoundaryFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                frame_boundary: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceFrameBoundaryFeaturesEXT<'a> {
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
}

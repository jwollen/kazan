#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_image_compression_control";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageCompressionControlEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageCompressionControlEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: ImageCompressionFlagsEXT,
        pub compression_control_plane_count: u32,
        pub p_fixed_rate_flags: *mut ImageCompressionFixedRateFlagsEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for ImageCompressionControlEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageCompressionControlEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field(
                    "compression_control_plane_count",
                    &self.compression_control_plane_count,
                )
                .field("p_fixed_rate_flags", &self.p_fixed_rate_flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageCompressionControlEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_COMPRESSION_CONTROL_EXT;
    }

    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for ImageCompressionControlEXT<'a> {}
    unsafe impl<'a> Extends<SwapchainCreateInfoKHR<'a>> for ImageCompressionControlEXT<'a> {}
    unsafe impl<'a> Extends<PhysicalDeviceImageFormatInfo2<'a>> for ImageCompressionControlEXT<'a> {}

    impl Default for ImageCompressionControlEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                compression_control_plane_count: Default::default(),
                p_fixed_rate_flags: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageCompressionControlEXT<'a> {
        pub fn flags(mut self, flags: ImageCompressionFlagsEXT) -> Self {
            self.flags = flags;
            self
        }

        pub fn fixed_rate_flags(
            mut self,
            fixed_rate_flags: &'a mut [ImageCompressionFixedRateFlagsEXT],
        ) -> Self {
            self.compression_control_plane_count = fixed_rate_flags.len().try_into().unwrap();
            self.p_fixed_rate_flags = fixed_rate_flags.as_mut_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImageCompressionControlFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceImageCompressionControlFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image_compression_control: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceImageCompressionControlFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceImageCompressionControlFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image_compression_control", &self.image_compression_control)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageCompressionControlFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceImageCompressionControlFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceImageCompressionControlFeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceImageCompressionControlFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                image_compression_control: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceImageCompressionControlFeaturesEXT<'a> {
        pub fn image_compression_control(mut self, image_compression_control: bool) -> Self {
            self.image_compression_control = image_compression_control.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageCompressionPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageCompressionPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image_compression_flags: ImageCompressionFlagsEXT,
        pub image_compression_fixed_rate_flags: ImageCompressionFixedRateFlagsEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for ImageCompressionPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageCompressionPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image_compression_flags", &self.image_compression_flags)
                .field(
                    "image_compression_fixed_rate_flags",
                    &self.image_compression_fixed_rate_flags,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageCompressionPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_COMPRESSION_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<ImageFormatProperties2<'a>> for ImageCompressionPropertiesEXT<'a> {}
    unsafe impl<'a> Extends<SurfaceFormat2KHR<'a>> for ImageCompressionPropertiesEXT<'a> {}
    unsafe impl<'a> Extends<SubresourceLayout2<'a>> for ImageCompressionPropertiesEXT<'a> {}

    impl Default for ImageCompressionPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                image_compression_flags: Default::default(),
                image_compression_fixed_rate_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageCompressionPropertiesEXT<'a> {
        pub fn image_compression_flags(
            mut self,
            image_compression_flags: ImageCompressionFlagsEXT,
        ) -> Self {
            self.image_compression_flags = image_compression_flags;
            self
        }

        pub fn image_compression_fixed_rate_flags(
            mut self,
            image_compression_fixed_rate_flags: ImageCompressionFixedRateFlagsEXT,
        ) -> Self {
            self.image_compression_fixed_rate_flags = image_compression_fixed_rate_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageCompressionFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ImageCompressionFlagsEXT(Flags);
    vk_bitflags_wrapped!(ImageCompressionFlagsEXT, Flags);

    impl ImageCompressionFlagsEXT {
        pub const FIXED_RATE_DEFAULT_EXT: Self =
            Self(ImageCompressionFlagBitsEXT::FIXED_RATE_DEFAULT_EXT.0);
        pub const FIXED_RATE_EXPLICIT_EXT: Self =
            Self(ImageCompressionFlagBitsEXT::FIXED_RATE_EXPLICIT_EXT.0);
        pub const DISABLED_EXT: Self = Self(ImageCompressionFlagBitsEXT::DISABLED_EXT.0);
        pub const DEFAULT: Self = Self(0);
    }

    impl fmt::Debug for ImageCompressionFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    ImageCompressionFlagsEXT::FIXED_RATE_DEFAULT_EXT.0,
                    "FIXED_RATE_DEFAULT_EXT",
                ),
                (
                    ImageCompressionFlagsEXT::FIXED_RATE_EXPLICIT_EXT.0,
                    "FIXED_RATE_EXPLICIT_EXT",
                ),
                (ImageCompressionFlagsEXT::DISABLED_EXT.0, "DISABLED_EXT"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageCompressionFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ImageCompressionFlagBitsEXT(u32);

    impl ImageCompressionFlagBitsEXT {
        pub const FIXED_RATE_DEFAULT_EXT: Self = Self(1 << 0);
        pub const FIXED_RATE_EXPLICIT_EXT: Self = Self(1 << 1);
        pub const DISABLED_EXT: Self = Self(1 << 2);
    }

    impl fmt::Debug for ImageCompressionFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::FIXED_RATE_DEFAULT_EXT => Some("FIXED_RATE_DEFAULT_EXT"),
                Self::FIXED_RATE_EXPLICIT_EXT => Some("FIXED_RATE_EXPLICIT_EXT"),
                Self::DISABLED_EXT => Some("DISABLED_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageCompressionFixedRateFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ImageCompressionFixedRateFlagsEXT(Flags);
    vk_bitflags_wrapped!(ImageCompressionFixedRateFlagsEXT, Flags);

    impl ImageCompressionFixedRateFlagsEXT {
        pub const _1BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_1BPC_EXT.0);
        pub const _2BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_2BPC_EXT.0);
        pub const _3BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_3BPC_EXT.0);
        pub const _4BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_4BPC_EXT.0);
        pub const _5BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_5BPC_EXT.0);
        pub const _6BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_6BPC_EXT.0);
        pub const _7BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_7BPC_EXT.0);
        pub const _8BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_8BPC_EXT.0);
        pub const _9BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_9BPC_EXT.0);
        pub const _10BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_10BPC_EXT.0);
        pub const _11BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_11BPC_EXT.0);
        pub const _12BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_12BPC_EXT.0);
        pub const _13BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_13BPC_EXT.0);
        pub const _14BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_14BPC_EXT.0);
        pub const _15BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_15BPC_EXT.0);
        pub const _16BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_16BPC_EXT.0);
        pub const _17BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_17BPC_EXT.0);
        pub const _18BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_18BPC_EXT.0);
        pub const _19BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_19BPC_EXT.0);
        pub const _20BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_20BPC_EXT.0);
        pub const _21BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_21BPC_EXT.0);
        pub const _22BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_22BPC_EXT.0);
        pub const _23BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_23BPC_EXT.0);
        pub const _24BPC_EXT: Self = Self(ImageCompressionFixedRateFlagBitsEXT::_24BPC_EXT.0);
        pub const NONE: Self = Self(0);
    }

    impl fmt::Debug for ImageCompressionFixedRateFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (ImageCompressionFixedRateFlagsEXT::_1BPC_EXT.0, "_1BPC_EXT"),
                (ImageCompressionFixedRateFlagsEXT::_2BPC_EXT.0, "_2BPC_EXT"),
                (ImageCompressionFixedRateFlagsEXT::_3BPC_EXT.0, "_3BPC_EXT"),
                (ImageCompressionFixedRateFlagsEXT::_4BPC_EXT.0, "_4BPC_EXT"),
                (ImageCompressionFixedRateFlagsEXT::_5BPC_EXT.0, "_5BPC_EXT"),
                (ImageCompressionFixedRateFlagsEXT::_6BPC_EXT.0, "_6BPC_EXT"),
                (ImageCompressionFixedRateFlagsEXT::_7BPC_EXT.0, "_7BPC_EXT"),
                (ImageCompressionFixedRateFlagsEXT::_8BPC_EXT.0, "_8BPC_EXT"),
                (ImageCompressionFixedRateFlagsEXT::_9BPC_EXT.0, "_9BPC_EXT"),
                (
                    ImageCompressionFixedRateFlagsEXT::_10BPC_EXT.0,
                    "_10BPC_EXT",
                ),
                (
                    ImageCompressionFixedRateFlagsEXT::_11BPC_EXT.0,
                    "_11BPC_EXT",
                ),
                (
                    ImageCompressionFixedRateFlagsEXT::_12BPC_EXT.0,
                    "_12BPC_EXT",
                ),
                (
                    ImageCompressionFixedRateFlagsEXT::_13BPC_EXT.0,
                    "_13BPC_EXT",
                ),
                (
                    ImageCompressionFixedRateFlagsEXT::_14BPC_EXT.0,
                    "_14BPC_EXT",
                ),
                (
                    ImageCompressionFixedRateFlagsEXT::_15BPC_EXT.0,
                    "_15BPC_EXT",
                ),
                (
                    ImageCompressionFixedRateFlagsEXT::_16BPC_EXT.0,
                    "_16BPC_EXT",
                ),
                (
                    ImageCompressionFixedRateFlagsEXT::_17BPC_EXT.0,
                    "_17BPC_EXT",
                ),
                (
                    ImageCompressionFixedRateFlagsEXT::_18BPC_EXT.0,
                    "_18BPC_EXT",
                ),
                (
                    ImageCompressionFixedRateFlagsEXT::_19BPC_EXT.0,
                    "_19BPC_EXT",
                ),
                (
                    ImageCompressionFixedRateFlagsEXT::_20BPC_EXT.0,
                    "_20BPC_EXT",
                ),
                (
                    ImageCompressionFixedRateFlagsEXT::_21BPC_EXT.0,
                    "_21BPC_EXT",
                ),
                (
                    ImageCompressionFixedRateFlagsEXT::_22BPC_EXT.0,
                    "_22BPC_EXT",
                ),
                (
                    ImageCompressionFixedRateFlagsEXT::_23BPC_EXT.0,
                    "_23BPC_EXT",
                ),
                (
                    ImageCompressionFixedRateFlagsEXT::_24BPC_EXT.0,
                    "_24BPC_EXT",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageCompressionFixedRateFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ImageCompressionFixedRateFlagBitsEXT(u32);

    impl ImageCompressionFixedRateFlagBitsEXT {
        pub const _1BPC_EXT: Self = Self(1 << 0);
        pub const _2BPC_EXT: Self = Self(1 << 1);
        pub const _3BPC_EXT: Self = Self(1 << 2);
        pub const _4BPC_EXT: Self = Self(1 << 3);
        pub const _5BPC_EXT: Self = Self(1 << 4);
        pub const _6BPC_EXT: Self = Self(1 << 5);
        pub const _7BPC_EXT: Self = Self(1 << 6);
        pub const _8BPC_EXT: Self = Self(1 << 7);
        pub const _9BPC_EXT: Self = Self(1 << 8);
        pub const _10BPC_EXT: Self = Self(1 << 9);
        pub const _11BPC_EXT: Self = Self(1 << 10);
        pub const _12BPC_EXT: Self = Self(1 << 11);
        pub const _13BPC_EXT: Self = Self(1 << 12);
        pub const _14BPC_EXT: Self = Self(1 << 13);
        pub const _15BPC_EXT: Self = Self(1 << 14);
        pub const _16BPC_EXT: Self = Self(1 << 15);
        pub const _17BPC_EXT: Self = Self(1 << 16);
        pub const _18BPC_EXT: Self = Self(1 << 17);
        pub const _19BPC_EXT: Self = Self(1 << 18);
        pub const _20BPC_EXT: Self = Self(1 << 19);
        pub const _21BPC_EXT: Self = Self(1 << 20);
        pub const _22BPC_EXT: Self = Self(1 << 21);
        pub const _23BPC_EXT: Self = Self(1 << 22);
        pub const _24BPC_EXT: Self = Self(1 << 23);
    }

    impl fmt::Debug for ImageCompressionFixedRateFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_1BPC_EXT => Some("_1BPC_EXT"),
                Self::_2BPC_EXT => Some("_2BPC_EXT"),
                Self::_3BPC_EXT => Some("_3BPC_EXT"),
                Self::_4BPC_EXT => Some("_4BPC_EXT"),
                Self::_5BPC_EXT => Some("_5BPC_EXT"),
                Self::_6BPC_EXT => Some("_6BPC_EXT"),
                Self::_7BPC_EXT => Some("_7BPC_EXT"),
                Self::_8BPC_EXT => Some("_8BPC_EXT"),
                Self::_9BPC_EXT => Some("_9BPC_EXT"),
                Self::_10BPC_EXT => Some("_10BPC_EXT"),
                Self::_11BPC_EXT => Some("_11BPC_EXT"),
                Self::_12BPC_EXT => Some("_12BPC_EXT"),
                Self::_13BPC_EXT => Some("_13BPC_EXT"),
                Self::_14BPC_EXT => Some("_14BPC_EXT"),
                Self::_15BPC_EXT => Some("_15BPC_EXT"),
                Self::_16BPC_EXT => Some("_16BPC_EXT"),
                Self::_17BPC_EXT => Some("_17BPC_EXT"),
                Self::_18BPC_EXT => Some("_18BPC_EXT"),
                Self::_19BPC_EXT => Some("_19BPC_EXT"),
                Self::_20BPC_EXT => Some("_20BPC_EXT"),
                Self::_21BPC_EXT => Some("_21BPC_EXT"),
                Self::_22BPC_EXT => Some("_22BPC_EXT"),
                Self::_23BPC_EXT => Some("_23BPC_EXT"),
                Self::_24BPC_EXT => Some("_24BPC_EXT"),
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
    get_image_subresource_layout2_ext: PFN_vkGetImageSubresourceLayout2,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_image_subresource_layout2_ext: transmute(
                    load(c"vkGetImageSubresourceLayout2EXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageSubresourceLayout2EXT.html>
    pub unsafe fn get_image_subresource_layout2_ext(
        &self,
        device: Device,
        image: Image,
        subresource: &ImageSubresource2<'_>,
        layout: &mut SubresourceLayout2<'_>,
    ) {
        unsafe { (self.get_image_subresource_layout2_ext)(device, image, subresource, layout) }
    }
}

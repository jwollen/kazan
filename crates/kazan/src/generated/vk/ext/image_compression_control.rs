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
    #[derive(Copy, Clone)]
    pub struct ImageCompressionControlEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: ImageCompressionFlagsEXT,
        pub compression_control_plane_count: u32,
        pub p_fixed_rate_flags: *mut ImageCompressionFixedRateFlagsEXT,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for ImageCompressionControlEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::IMAGE_COMPRESSION_CONTROL_EXT,
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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceImageCompressionControlFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image_compression_control: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceImageCompressionControlFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT,
                p_next: core::ptr::null_mut(),
                image_compression_control: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceImageCompressionControlFeaturesEXT<'a> {
        pub fn image_compression_control(mut self, image_compression_control: Bool32) -> Self {
            self.image_compression_control = image_compression_control;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageCompressionPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image_compression_flags: ImageCompressionFlagsEXT,
        pub image_compression_fixed_rate_flags: ImageCompressionFixedRateFlagsEXT,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for ImageCompressionPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::IMAGE_COMPRESSION_PROPERTIES_EXT,
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
    bitflags! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct ImageCompressionFlagsEXT: Flags {
            const FIXED_RATE_DEFAULT_EXT = ImageCompressionFlagBitsEXT::FIXED_RATE_DEFAULT_EXT.0;
            const FIXED_RATE_EXPLICIT_EXT = ImageCompressionFlagBitsEXT::FIXED_RATE_EXPLICIT_EXT.0;
            const DISABLED_EXT = ImageCompressionFlagBitsEXT::DISABLED_EXT.0;
            const DEFAULT = 0;
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ImageCompressionFlagBitsEXT(u32);
    impl ImageCompressionFlagBitsEXT {
        pub const FIXED_RATE_DEFAULT_EXT: Self = Self(1 << 0);
        pub const FIXED_RATE_EXPLICIT_EXT: Self = Self(1 << 1);
        pub const DISABLED_EXT: Self = Self(1 << 2);
    }
    bitflags! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct ImageCompressionFixedRateFlagsEXT: Flags {
            const _1BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_1BPC_EXT.0;
            const _2BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_2BPC_EXT.0;
            const _3BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_3BPC_EXT.0;
            const _4BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_4BPC_EXT.0;
            const _5BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_5BPC_EXT.0;
            const _6BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_6BPC_EXT.0;
            const _7BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_7BPC_EXT.0;
            const _8BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_8BPC_EXT.0;
            const _9BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_9BPC_EXT.0;
            const _10BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_10BPC_EXT.0;
            const _11BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_11BPC_EXT.0;
            const _12BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_12BPC_EXT.0;
            const _13BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_13BPC_EXT.0;
            const _14BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_14BPC_EXT.0;
            const _15BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_15BPC_EXT.0;
            const _16BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_16BPC_EXT.0;
            const _17BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_17BPC_EXT.0;
            const _18BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_18BPC_EXT.0;
            const _19BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_19BPC_EXT.0;
            const _20BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_20BPC_EXT.0;
            const _21BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_21BPC_EXT.0;
            const _22BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_22BPC_EXT.0;
            const _23BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_23BPC_EXT.0;
            const _24BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_24BPC_EXT.0;
            const NONE = 0;
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
}
pub struct DeviceFn {
    get_image_subresource_layout2_ext: PFN_vkGetImageSubresourceLayout2,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_image_subresource_layout2_ext: transmute(
                    load(c"vkGetImageSubresourceLayout2EXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_image_subresource_layout2_ext(
        &self,
        device: Device,
        image: Image,
        subresource: &ImageSubresource2<'_>,
    ) -> SubresourceLayout2<'_> {
        unsafe {
            let mut layout = core::mem::MaybeUninit::uninit();
            (self.get_image_subresource_layout2_ext)(
                device,
                image,
                subresource,
                layout.as_mut_ptr(),
            );
            layout.assume_init()
        }
    }
}

#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_ANDROID_external_format_resolve";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExternalFormatResolveFeaturesANDROID.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceExternalFormatResolveFeaturesANDROID<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub external_format_resolve: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceExternalFormatResolveFeaturesANDROID<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceExternalFormatResolveFeaturesANDROID")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("external_format_resolve", &self.external_format_resolve)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExternalFormatResolveFeaturesANDROID<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceExternalFormatResolveFeaturesANDROID<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceExternalFormatResolveFeaturesANDROID<'a>
    {
    }

    impl Default for PhysicalDeviceExternalFormatResolveFeaturesANDROID<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                external_format_resolve: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceExternalFormatResolveFeaturesANDROID<'a> {
        #[inline]
        pub fn external_format_resolve(mut self, external_format_resolve: bool) -> Self {
            self.external_format_resolve = external_format_resolve.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExternalFormatResolvePropertiesANDROID.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceExternalFormatResolvePropertiesANDROID<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub null_color_attachment_with_external_format_resolve: Bool32,
        pub external_format_resolve_chroma_offset_x: ChromaLocation,
        pub external_format_resolve_chroma_offset_y: ChromaLocation,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceExternalFormatResolvePropertiesANDROID<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceExternalFormatResolvePropertiesANDROID")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "null_color_attachment_with_external_format_resolve",
                    &self.null_color_attachment_with_external_format_resolve,
                )
                .field(
                    "external_format_resolve_chroma_offset_x",
                    &self.external_format_resolve_chroma_offset_x,
                )
                .field(
                    "external_format_resolve_chroma_offset_y",
                    &self.external_format_resolve_chroma_offset_y,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExternalFormatResolvePropertiesANDROID<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceExternalFormatResolvePropertiesANDROID<'a>
    {
    }

    impl Default for PhysicalDeviceExternalFormatResolvePropertiesANDROID<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                null_color_attachment_with_external_format_resolve: Default::default(),
                external_format_resolve_chroma_offset_x: Default::default(),
                external_format_resolve_chroma_offset_y: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceExternalFormatResolvePropertiesANDROID<'a> {
        #[inline]
        pub fn null_color_attachment_with_external_format_resolve(
            mut self,
            null_color_attachment_with_external_format_resolve: bool,
        ) -> Self {
            self.null_color_attachment_with_external_format_resolve =
                null_color_attachment_with_external_format_resolve.into();
            self
        }

        #[inline]
        pub fn external_format_resolve_chroma_offset_x(
            mut self,
            external_format_resolve_chroma_offset_x: ChromaLocation,
        ) -> Self {
            self.external_format_resolve_chroma_offset_x = external_format_resolve_chroma_offset_x;
            self
        }

        #[inline]
        pub fn external_format_resolve_chroma_offset_y(
            mut self,
            external_format_resolve_chroma_offset_y: ChromaLocation,
        ) -> Self {
            self.external_format_resolve_chroma_offset_y = external_format_resolve_chroma_offset_y;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAndroidHardwareBufferFormatResolvePropertiesANDROID.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AndroidHardwareBufferFormatResolvePropertiesANDROID<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub color_attachment_format: Format,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AndroidHardwareBufferFormatResolvePropertiesANDROID<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AndroidHardwareBufferFormatResolvePropertiesANDROID")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("color_attachment_format", &self.color_attachment_format)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AndroidHardwareBufferFormatResolvePropertiesANDROID<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID;
    }

    unsafe impl<'a> Extends<AndroidHardwareBufferPropertiesANDROID<'a>>
        for AndroidHardwareBufferFormatResolvePropertiesANDROID<'a>
    {
    }

    impl Default for AndroidHardwareBufferFormatResolvePropertiesANDROID<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                color_attachment_format: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AndroidHardwareBufferFormatResolvePropertiesANDROID<'a> {
        #[inline]
        pub fn color_attachment_format(mut self, color_attachment_format: Format) -> Self {
            self.color_attachment_format = color_attachment_format;
            self
        }
    }
}

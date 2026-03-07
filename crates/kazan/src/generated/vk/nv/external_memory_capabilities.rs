//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_external_memory_capabilities.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_external_memory_capabilities";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalImageFormatPropertiesNV.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct ExternalImageFormatPropertiesNV {
        pub image_format_properties: ImageFormatProperties,
        pub external_memory_features: ExternalMemoryFeatureFlagsNV,
        pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlagsNV,
        pub compatible_handle_types: ExternalMemoryHandleTypeFlagsNV,
    }

    impl ExternalImageFormatPropertiesNV {
        #[inline]
        pub fn image_format_properties(
            mut self,
            image_format_properties: ImageFormatProperties,
        ) -> Self {
            self.image_format_properties = image_format_properties;
            self
        }

        #[inline]
        pub fn external_memory_features(
            mut self,
            external_memory_features: ExternalMemoryFeatureFlagsNV,
        ) -> Self {
            self.external_memory_features = external_memory_features;
            self
        }

        #[inline]
        pub fn export_from_imported_handle_types(
            mut self,
            export_from_imported_handle_types: ExternalMemoryHandleTypeFlagsNV,
        ) -> Self {
            self.export_from_imported_handle_types = export_from_imported_handle_types;
            self
        }

        #[inline]
        pub fn compatible_handle_types(
            mut self,
            compatible_handle_types: ExternalMemoryHandleTypeFlagsNV,
        ) -> Self {
            self.compatible_handle_types = compatible_handle_types;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryHandleTypeFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ExternalMemoryHandleTypeFlagsNV(Flags);
    vk_bitflags_wrapped!(ExternalMemoryHandleTypeFlagsNV, Flags);

    impl ExternalMemoryHandleTypeFlagsNV {
        pub const OPAQUE_WIN32_NV: Self =
            Self(ExternalMemoryHandleTypeFlagBitsNV::OPAQUE_WIN32_NV.0);
        pub const OPAQUE_WIN32_KMT_NV: Self =
            Self(ExternalMemoryHandleTypeFlagBitsNV::OPAQUE_WIN32_KMT_NV.0);
        pub const D3D11_IMAGE_NV: Self = Self(ExternalMemoryHandleTypeFlagBitsNV::D3D11_IMAGE_NV.0);
        pub const D3D11_IMAGE_KMT_NV: Self =
            Self(ExternalMemoryHandleTypeFlagBitsNV::D3D11_IMAGE_KMT_NV.0);
    }

    impl fmt::Debug for ExternalMemoryHandleTypeFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    ExternalMemoryHandleTypeFlagsNV::OPAQUE_WIN32_NV.0,
                    "OPAQUE_WIN32_NV",
                ),
                (
                    ExternalMemoryHandleTypeFlagsNV::OPAQUE_WIN32_KMT_NV.0,
                    "OPAQUE_WIN32_KMT_NV",
                ),
                (
                    ExternalMemoryHandleTypeFlagsNV::D3D11_IMAGE_NV.0,
                    "D3D11_IMAGE_NV",
                ),
                (
                    ExternalMemoryHandleTypeFlagsNV::D3D11_IMAGE_KMT_NV.0,
                    "D3D11_IMAGE_KMT_NV",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryHandleTypeFlagBitsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalMemoryHandleTypeFlagBitsNV(u32);

    impl ExternalMemoryHandleTypeFlagBitsNV {
        pub const OPAQUE_WIN32_NV: Self = Self(1 << 0);
        pub const OPAQUE_WIN32_KMT_NV: Self = Self(1 << 1);
        pub const D3D11_IMAGE_NV: Self = Self(1 << 2);
        pub const D3D11_IMAGE_KMT_NV: Self = Self(1 << 3);
    }

    impl fmt::Debug for ExternalMemoryHandleTypeFlagBitsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::OPAQUE_WIN32_NV => Some("OPAQUE_WIN32_NV"),
                Self::OPAQUE_WIN32_KMT_NV => Some("OPAQUE_WIN32_KMT_NV"),
                Self::D3D11_IMAGE_NV => Some("D3D11_IMAGE_NV"),
                Self::D3D11_IMAGE_KMT_NV => Some("D3D11_IMAGE_KMT_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryFeatureFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ExternalMemoryFeatureFlagsNV(Flags);
    vk_bitflags_wrapped!(ExternalMemoryFeatureFlagsNV, Flags);

    impl ExternalMemoryFeatureFlagsNV {
        pub const DEDICATED_ONLY_NV: Self =
            Self(ExternalMemoryFeatureFlagBitsNV::DEDICATED_ONLY_NV.0);
        pub const EXPORTABLE_NV: Self = Self(ExternalMemoryFeatureFlagBitsNV::EXPORTABLE_NV.0);
        pub const IMPORTABLE_NV: Self = Self(ExternalMemoryFeatureFlagBitsNV::IMPORTABLE_NV.0);
    }

    impl fmt::Debug for ExternalMemoryFeatureFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    ExternalMemoryFeatureFlagsNV::DEDICATED_ONLY_NV.0,
                    "DEDICATED_ONLY_NV",
                ),
                (
                    ExternalMemoryFeatureFlagsNV::EXPORTABLE_NV.0,
                    "EXPORTABLE_NV",
                ),
                (
                    ExternalMemoryFeatureFlagsNV::IMPORTABLE_NV.0,
                    "IMPORTABLE_NV",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryFeatureFlagBitsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalMemoryFeatureFlagBitsNV(u32);

    impl ExternalMemoryFeatureFlagBitsNV {
        pub const DEDICATED_ONLY_NV: Self = Self(1 << 0);
        pub const EXPORTABLE_NV: Self = Self(1 << 1);
        pub const IMPORTABLE_NV: Self = Self(1 << 2);
    }

    impl fmt::Debug for ExternalMemoryFeatureFlagBitsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEDICATED_ONLY_NV => Some("DEDICATED_ONLY_NV"),
                Self::EXPORTABLE_NV => Some("EXPORTABLE_NV"),
                Self::IMPORTABLE_NV => Some("IMPORTABLE_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html>
    pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            format: Format,
            ty: ImageType,
            tiling: ImageTiling,
            usage: ImageUsageFlags,
            flags: ImageCreateFlags,
            external_handle_type: ExternalMemoryHandleTypeFlagsNV,
            p_external_image_format_properties: *mut ExternalImageFormatPropertiesNV,
        ) -> vk::Result;
}

pub struct InstanceFn {
    get_physical_device_external_image_format_properties_nv:
        PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_external_image_format_properties_nv: transmute(
                    load(c"vkGetPhysicalDeviceExternalImageFormatPropertiesNV")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html>
    #[inline]
    pub unsafe fn get_physical_device_external_image_format_properties_nv(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        external_handle_type: ExternalMemoryHandleTypeFlagsNV,
    ) -> crate::Result<ExternalImageFormatPropertiesNV> {
        unsafe {
            let mut external_image_format_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_physical_device_external_image_format_properties_nv)(
                physical_device,
                format,
                ty,
                tiling,
                usage,
                flags,
                external_handle_type,
                external_image_format_properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(external_image_format_properties.assume_init()),
                err => Err(err),
            }
        }
    }
}

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
    pub struct ExternalImageFormatPropertiesNV {
        pub image_format_properties: ImageFormatProperties,
        pub external_memory_features: ExternalMemoryFeatureFlagsNV,
        pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlagsNV,
        pub compatible_handle_types: ExternalMemoryHandleTypeFlagsNV,
    }
    impl ExternalImageFormatPropertiesNV {
        pub fn image_format_properties(
            mut self,
            image_format_properties: ImageFormatProperties,
        ) -> Self {
            self.image_format_properties = image_format_properties;
            self
        }
        pub fn external_memory_features(
            mut self,
            external_memory_features: ExternalMemoryFeatureFlagsNV,
        ) -> Self {
            self.external_memory_features = external_memory_features;
            self
        }
        pub fn export_from_imported_handle_types(
            mut self,
            export_from_imported_handle_types: ExternalMemoryHandleTypeFlagsNV,
        ) -> Self {
            self.export_from_imported_handle_types = export_from_imported_handle_types;
            self
        }
        pub fn compatible_handle_types(
            mut self,
            compatible_handle_types: ExternalMemoryHandleTypeFlagsNV,
        ) -> Self {
            self.compatible_handle_types = compatible_handle_types;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalMemoryHandleTypeFlagsNV(Flags);
    impl ExternalMemoryHandleTypeFlagsNV {
        pub const OPAQUE_WIN32_NV: Self =
            Self(ExternalMemoryHandleTypeFlagBitsNV::OPAQUE_WIN32_NV.0);
        pub const OPAQUE_WIN32_KMT_NV: Self =
            Self(ExternalMemoryHandleTypeFlagBitsNV::OPAQUE_WIN32_KMT_NV.0);
        pub const D3D11_IMAGE_NV: Self = Self(ExternalMemoryHandleTypeFlagBitsNV::D3D11_IMAGE_NV.0);
        pub const D3D11_IMAGE_KMT_NV: Self =
            Self(ExternalMemoryHandleTypeFlagBitsNV::D3D11_IMAGE_KMT_NV.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalMemoryHandleTypeFlagBitsNV(u32);
    impl ExternalMemoryHandleTypeFlagBitsNV {
        pub const OPAQUE_WIN32_NV: Self = Self(1 << 0);
        pub const OPAQUE_WIN32_KMT_NV: Self = Self(1 << 1);
        pub const D3D11_IMAGE_NV: Self = Self(1 << 2);
        pub const D3D11_IMAGE_KMT_NV: Self = Self(1 << 3);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalMemoryFeatureFlagsNV(Flags);
    impl ExternalMemoryFeatureFlagsNV {
        pub const DEDICATED_ONLY_NV: Self =
            Self(ExternalMemoryFeatureFlagBitsNV::DEDICATED_ONLY_NV.0);
        pub const EXPORTABLE_NV: Self = Self(ExternalMemoryFeatureFlagBitsNV::EXPORTABLE_NV.0);
        pub const IMPORTABLE_NV: Self = Self(ExternalMemoryFeatureFlagBitsNV::IMPORTABLE_NV.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalMemoryFeatureFlagBitsNV(u32);
    impl ExternalMemoryFeatureFlagBitsNV {
        pub const DEDICATED_ONLY_NV: Self = Self(1 << 0);
        pub const EXPORTABLE_NV: Self = Self(1 << 1);
        pub const IMPORTABLE_NV: Self = Self(1 << 2);
    }
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
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_external_image_format_properties_nv: transmute(
                    load(c"vkGetPhysicalDeviceExternalImageFormatPropertiesNV")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
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

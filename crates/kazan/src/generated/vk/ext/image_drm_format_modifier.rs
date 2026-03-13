//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_image_drm_format_modifier.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_image_drm_format_modifier";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDrmFormatModifierPropertiesListEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DrmFormatModifierPropertiesListEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub drm_format_modifier_count: u32,
        pub p_drm_format_modifier_properties: *mut DrmFormatModifierPropertiesEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DrmFormatModifierPropertiesListEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DrmFormatModifierPropertiesListEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("drm_format_modifier_count", &self.drm_format_modifier_count)
                .field(
                    "p_drm_format_modifier_properties",
                    &self.p_drm_format_modifier_properties,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DrmFormatModifierPropertiesListEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT;
    }

    unsafe impl Extends<FormatProperties2<'_>> for DrmFormatModifierPropertiesListEXT<'_> {}

    impl Default for DrmFormatModifierPropertiesListEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                drm_format_modifier_count: Default::default(),
                p_drm_format_modifier_properties: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DrmFormatModifierPropertiesListEXT<'a> {
        #[inline]
        pub fn drm_format_modifier_properties(
            mut self,
            drm_format_modifier_properties: &'a mut [DrmFormatModifierPropertiesEXT],
        ) -> Self {
            self.drm_format_modifier_count =
                drm_format_modifier_properties.len().try_into().unwrap();
            self.p_drm_format_modifier_properties =
                drm_format_modifier_properties.as_mut_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDrmFormatModifierPropertiesEXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct DrmFormatModifierPropertiesEXT {
        pub drm_format_modifier: u64,
        pub drm_format_modifier_plane_count: u32,
        pub drm_format_modifier_tiling_features: FormatFeatureFlags,
    }

    impl DrmFormatModifierPropertiesEXT {
        #[inline]
        pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
            self.drm_format_modifier = drm_format_modifier;
            self
        }

        #[inline]
        pub fn drm_format_modifier_plane_count(
            mut self,
            drm_format_modifier_plane_count: u32,
        ) -> Self {
            self.drm_format_modifier_plane_count = drm_format_modifier_plane_count;
            self
        }

        #[inline]
        pub fn drm_format_modifier_tiling_features(
            mut self,
            drm_format_modifier_tiling_features: FormatFeatureFlags,
        ) -> Self {
            self.drm_format_modifier_tiling_features = drm_format_modifier_tiling_features;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImageDrmFormatModifierInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceImageDrmFormatModifierInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub drm_format_modifier: u64,
        pub sharing_mode: SharingMode,
        pub queue_family_index_count: u32,
        pub p_queue_family_indices: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceImageDrmFormatModifierInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceImageDrmFormatModifierInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("drm_format_modifier", &self.drm_format_modifier)
                .field("sharing_mode", &self.sharing_mode)
                .field("queue_family_index_count", &self.queue_family_index_count)
                .field("p_queue_family_indices", &self.p_queue_family_indices)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageDrmFormatModifierInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT;
    }

    unsafe impl Extends<PhysicalDeviceImageFormatInfo2<'_>>
        for PhysicalDeviceImageDrmFormatModifierInfoEXT<'_>
    {
    }

    impl Default for PhysicalDeviceImageDrmFormatModifierInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                drm_format_modifier: Default::default(),
                sharing_mode: Default::default(),
                queue_family_index_count: Default::default(),
                p_queue_family_indices: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceImageDrmFormatModifierInfoEXT<'a> {
        #[inline]
        pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
            self.drm_format_modifier = drm_format_modifier;
            self
        }

        #[inline]
        pub fn sharing_mode(mut self, sharing_mode: SharingMode) -> Self {
            self.sharing_mode = sharing_mode;
            self
        }

        #[inline]
        pub fn queue_family_indices(mut self, queue_family_indices: &'a [u32]) -> Self {
            self.queue_family_index_count = queue_family_indices.len().try_into().unwrap();
            self.p_queue_family_indices = queue_family_indices.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageDrmFormatModifierListCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageDrmFormatModifierListCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub drm_format_modifier_count: u32,
        pub p_drm_format_modifiers: *const u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageDrmFormatModifierListCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageDrmFormatModifierListCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("drm_format_modifier_count", &self.drm_format_modifier_count)
                .field("p_drm_format_modifiers", &self.p_drm_format_modifiers)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageDrmFormatModifierListCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT;
    }

    unsafe impl Extends<ImageCreateInfo<'_>> for ImageDrmFormatModifierListCreateInfoEXT<'_> {}

    impl Default for ImageDrmFormatModifierListCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                drm_format_modifier_count: Default::default(),
                p_drm_format_modifiers: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageDrmFormatModifierListCreateInfoEXT<'a> {
        #[inline]
        pub fn drm_format_modifiers(mut self, drm_format_modifiers: &'a [u64]) -> Self {
            self.drm_format_modifier_count = drm_format_modifiers.len().try_into().unwrap();
            self.p_drm_format_modifiers = drm_format_modifiers.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageDrmFormatModifierExplicitCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageDrmFormatModifierExplicitCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub drm_format_modifier: u64,
        pub drm_format_modifier_plane_count: u32,
        pub p_plane_layouts: *const SubresourceLayout,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageDrmFormatModifierExplicitCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageDrmFormatModifierExplicitCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("drm_format_modifier", &self.drm_format_modifier)
                .field(
                    "drm_format_modifier_plane_count",
                    &self.drm_format_modifier_plane_count,
                )
                .field("p_plane_layouts", &self.p_plane_layouts)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageDrmFormatModifierExplicitCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT;
    }

    unsafe impl Extends<ImageCreateInfo<'_>> for ImageDrmFormatModifierExplicitCreateInfoEXT<'_> {}

    impl Default for ImageDrmFormatModifierExplicitCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                drm_format_modifier: Default::default(),
                drm_format_modifier_plane_count: Default::default(),
                p_plane_layouts: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageDrmFormatModifierExplicitCreateInfoEXT<'a> {
        #[inline]
        pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
            self.drm_format_modifier = drm_format_modifier;
            self
        }

        #[inline]
        pub fn plane_layouts(mut self, plane_layouts: &'a [SubresourceLayout]) -> Self {
            self.drm_format_modifier_plane_count = plane_layouts.len().try_into().unwrap();
            self.p_plane_layouts = plane_layouts.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageDrmFormatModifierPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageDrmFormatModifierPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub drm_format_modifier: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageDrmFormatModifierPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageDrmFormatModifierPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("drm_format_modifier", &self.drm_format_modifier)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageDrmFormatModifierPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT;
    }

    impl Default for ImageDrmFormatModifierPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                drm_format_modifier: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageDrmFormatModifierPropertiesEXT<'a> {
        #[inline]
        pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
            self.drm_format_modifier = drm_format_modifier;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDrmFormatModifierPropertiesList2EXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DrmFormatModifierPropertiesList2EXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub drm_format_modifier_count: u32,
        pub p_drm_format_modifier_properties: *mut DrmFormatModifierProperties2EXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DrmFormatModifierPropertiesList2EXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DrmFormatModifierPropertiesList2EXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("drm_format_modifier_count", &self.drm_format_modifier_count)
                .field(
                    "p_drm_format_modifier_properties",
                    &self.p_drm_format_modifier_properties,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DrmFormatModifierPropertiesList2EXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT;
    }

    unsafe impl Extends<FormatProperties2<'_>> for DrmFormatModifierPropertiesList2EXT<'_> {}

    impl Default for DrmFormatModifierPropertiesList2EXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                drm_format_modifier_count: Default::default(),
                p_drm_format_modifier_properties: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DrmFormatModifierPropertiesList2EXT<'a> {
        #[inline]
        pub fn drm_format_modifier_properties(
            mut self,
            drm_format_modifier_properties: &'a mut [DrmFormatModifierProperties2EXT],
        ) -> Self {
            self.drm_format_modifier_count =
                drm_format_modifier_properties.len().try_into().unwrap();
            self.p_drm_format_modifier_properties =
                drm_format_modifier_properties.as_mut_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDrmFormatModifierProperties2EXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct DrmFormatModifierProperties2EXT {
        pub drm_format_modifier: u64,
        pub drm_format_modifier_plane_count: u32,
        pub drm_format_modifier_tiling_features: FormatFeatureFlags2,
    }

    impl DrmFormatModifierProperties2EXT {
        #[inline]
        pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
            self.drm_format_modifier = drm_format_modifier;
            self
        }

        #[inline]
        pub fn drm_format_modifier_plane_count(
            mut self,
            drm_format_modifier_plane_count: u32,
        ) -> Self {
            self.drm_format_modifier_plane_count = drm_format_modifier_plane_count;
            self
        }

        #[inline]
        pub fn drm_format_modifier_tiling_features(
            mut self,
            drm_format_modifier_tiling_features: FormatFeatureFlags2,
        ) -> Self {
            self.drm_format_modifier_tiling_features = drm_format_modifier_tiling_features;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html>
    pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = unsafe extern "system" fn(
        device: Device,
        image: Image,
        p_properties: *mut ImageDrmFormatModifierPropertiesEXT<'_>,
    )
        -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkDrmFormatModifierPropertiesListEXT = DrmFormatModifierPropertiesListEXT<'static>;
    pub type VkDrmFormatModifierPropertiesEXT = DrmFormatModifierPropertiesEXT;
    pub type VkPhysicalDeviceImageDrmFormatModifierInfoEXT =
        PhysicalDeviceImageDrmFormatModifierInfoEXT<'static>;
    pub type VkImageDrmFormatModifierListCreateInfoEXT =
        ImageDrmFormatModifierListCreateInfoEXT<'static>;
    pub type VkImageDrmFormatModifierExplicitCreateInfoEXT =
        ImageDrmFormatModifierExplicitCreateInfoEXT<'static>;
    pub type VkImageDrmFormatModifierPropertiesEXT = ImageDrmFormatModifierPropertiesEXT<'static>;
    pub type VkDrmFormatModifierPropertiesList2EXT = DrmFormatModifierPropertiesList2EXT<'static>;
    pub type VkDrmFormatModifierProperties2EXT = DrmFormatModifierProperties2EXT;
    impl DrmFormatModifierPropertiesListEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDrmFormatModifierPropertiesListEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceImageDrmFormatModifierInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceImageDrmFormatModifierInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageDrmFormatModifierListCreateInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageDrmFormatModifierListCreateInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageDrmFormatModifierExplicitCreateInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkImageDrmFormatModifierExplicitCreateInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageDrmFormatModifierPropertiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageDrmFormatModifierPropertiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DrmFormatModifierPropertiesList2EXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDrmFormatModifierPropertiesList2EXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_image_drm_format_modifier_properties: PFN_vkGetImageDrmFormatModifierPropertiesEXT,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_image_drm_format_modifier_properties: transmute(
                    load(c"vkGetImageDrmFormatModifierPropertiesEXT")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html>
    #[inline]
    pub unsafe fn get_image_drm_format_modifier_properties(
        &self,
        device: Device,
        image: Image,
        properties: &mut ImageDrmFormatModifierPropertiesEXT<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_image_drm_format_modifier_properties)(device, image, properties);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}

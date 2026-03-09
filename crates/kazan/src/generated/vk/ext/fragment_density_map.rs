//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_fragment_density_map.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_fragment_density_map";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFragmentDensityMapFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub fragment_density_map: Bool32,
        pub fragment_density_map_dynamic: Bool32,
        pub fragment_density_map_non_subsampled_images: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceFragmentDensityMapFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceFragmentDensityMapFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("fragment_density_map", &self.fragment_density_map)
                .field(
                    "fragment_density_map_dynamic",
                    &self.fragment_density_map_dynamic,
                )
                .field(
                    "fragment_density_map_non_subsampled_images",
                    &self.fragment_density_map_non_subsampled_images,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentDensityMapFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceFragmentDensityMapFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceFragmentDensityMapFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceFragmentDensityMapFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                fragment_density_map: Default::default(),
                fragment_density_map_dynamic: Default::default(),
                fragment_density_map_non_subsampled_images: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceFragmentDensityMapFeaturesEXT<'a> {
        #[inline]
        pub fn fragment_density_map(mut self, fragment_density_map: bool) -> Self {
            self.fragment_density_map = fragment_density_map.into();
            self
        }

        #[inline]
        pub fn fragment_density_map_dynamic(mut self, fragment_density_map_dynamic: bool) -> Self {
            self.fragment_density_map_dynamic = fragment_density_map_dynamic.into();
            self
        }

        #[inline]
        pub fn fragment_density_map_non_subsampled_images(
            mut self,
            fragment_density_map_non_subsampled_images: bool,
        ) -> Self {
            self.fragment_density_map_non_subsampled_images =
                fragment_density_map_non_subsampled_images.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFragmentDensityMapPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_fragment_density_texel_size: Extent2D,
        pub max_fragment_density_texel_size: Extent2D,
        pub fragment_density_invocations: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceFragmentDensityMapPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceFragmentDensityMapPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "min_fragment_density_texel_size",
                    &self.min_fragment_density_texel_size,
                )
                .field(
                    "max_fragment_density_texel_size",
                    &self.max_fragment_density_texel_size,
                )
                .field(
                    "fragment_density_invocations",
                    &self.fragment_density_invocations,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentDensityMapPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceFragmentDensityMapPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceFragmentDensityMapPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                min_fragment_density_texel_size: Default::default(),
                max_fragment_density_texel_size: Default::default(),
                fragment_density_invocations: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceFragmentDensityMapPropertiesEXT<'a> {
        #[inline]
        pub fn min_fragment_density_texel_size(
            mut self,
            min_fragment_density_texel_size: Extent2D,
        ) -> Self {
            self.min_fragment_density_texel_size = min_fragment_density_texel_size;
            self
        }

        #[inline]
        pub fn max_fragment_density_texel_size(
            mut self,
            max_fragment_density_texel_size: Extent2D,
        ) -> Self {
            self.max_fragment_density_texel_size = max_fragment_density_texel_size;
            self
        }

        #[inline]
        pub fn fragment_density_invocations(mut self, fragment_density_invocations: bool) -> Self {
            self.fragment_density_invocations = fragment_density_invocations.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassFragmentDensityMapCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RenderPassFragmentDensityMapCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub fragment_density_map_attachment: AttachmentReference,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderPassFragmentDensityMapCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderPassFragmentDensityMapCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "fragment_density_map_attachment",
                    &self.fragment_density_map_attachment,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderPassFragmentDensityMapCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<RenderPassCreateInfo<'a>>
        for RenderPassFragmentDensityMapCreateInfoEXT<'a>
    {
    }
    unsafe impl<'a> Extends<RenderPassCreateInfo2<'a>>
        for RenderPassFragmentDensityMapCreateInfoEXT<'a>
    {
    }

    impl Default for RenderPassFragmentDensityMapCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                fragment_density_map_attachment: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RenderPassFragmentDensityMapCreateInfoEXT<'a> {
        #[inline]
        pub fn fragment_density_map_attachment(
            mut self,
            fragment_density_map_attachment: AttachmentReference,
        ) -> Self {
            self.fragment_density_map_attachment = fragment_density_map_attachment;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderingFragmentDensityMapAttachmentInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RenderingFragmentDensityMapAttachmentInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image_view: ImageView,
        pub image_layout: ImageLayout,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderingFragmentDensityMapAttachmentInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderingFragmentDensityMapAttachmentInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image_view", &self.image_view)
                .field("image_layout", &self.image_layout)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderingFragmentDensityMapAttachmentInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT;
    }

    unsafe impl<'a> Extends<RenderingInfo<'a>> for RenderingFragmentDensityMapAttachmentInfoEXT<'a> {}

    impl Default for RenderingFragmentDensityMapAttachmentInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                image_view: Default::default(),
                image_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RenderingFragmentDensityMapAttachmentInfoEXT<'a> {
        #[inline]
        pub fn image_view(mut self, image_view: ImageView) -> Self {
            self.image_view = image_view;
            self
        }

        #[inline]
        pub fn image_layout(mut self, image_layout: ImageLayout) -> Self {
            self.image_layout = image_layout;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceFragmentDensityMapFeaturesEXT =
        PhysicalDeviceFragmentDensityMapFeaturesEXT<'static>;
    pub type VkPhysicalDeviceFragmentDensityMapPropertiesEXT =
        PhysicalDeviceFragmentDensityMapPropertiesEXT<'static>;
    pub type VkRenderPassFragmentDensityMapCreateInfoEXT =
        RenderPassFragmentDensityMapCreateInfoEXT<'static>;
    pub type VkRenderingFragmentDensityMapAttachmentInfoEXT =
        RenderingFragmentDensityMapAttachmentInfoEXT<'static>;
    impl PhysicalDeviceFragmentDensityMapFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceFragmentDensityMapFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceFragmentDensityMapPropertiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceFragmentDensityMapPropertiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl RenderPassFragmentDensityMapCreateInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkRenderPassFragmentDensityMapCreateInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl RenderingFragmentDensityMapAttachmentInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkRenderingFragmentDensityMapAttachmentInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

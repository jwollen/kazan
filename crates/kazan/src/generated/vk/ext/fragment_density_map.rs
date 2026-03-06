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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFragmentDensityMapFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub fragment_density_map: Bool32,
        pub fragment_density_map_dynamic: Bool32,
        pub fragment_density_map_non_subsampled_images: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                fragment_density_map: Default::default(),
                fragment_density_map_dynamic: Default::default(),
                fragment_density_map_non_subsampled_images: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceFragmentDensityMapFeaturesEXT<'a> {
        pub fn fragment_density_map(mut self, fragment_density_map: bool) -> Self {
            self.fragment_density_map = fragment_density_map.into();
            self
        }
        pub fn fragment_density_map_dynamic(mut self, fragment_density_map_dynamic: bool) -> Self {
            self.fragment_density_map_dynamic = fragment_density_map_dynamic.into();
            self
        }
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
    pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_fragment_density_texel_size: Extent2D,
        pub max_fragment_density_texel_size: Extent2D,
        pub fragment_density_invocations: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                min_fragment_density_texel_size: Default::default(),
                max_fragment_density_texel_size: Default::default(),
                fragment_density_invocations: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceFragmentDensityMapPropertiesEXT<'a> {
        pub fn min_fragment_density_texel_size(
            mut self,
            min_fragment_density_texel_size: Extent2D,
        ) -> Self {
            self.min_fragment_density_texel_size = min_fragment_density_texel_size;
            self
        }
        pub fn max_fragment_density_texel_size(
            mut self,
            max_fragment_density_texel_size: Extent2D,
        ) -> Self {
            self.max_fragment_density_texel_size = max_fragment_density_texel_size;
            self
        }
        pub fn fragment_density_invocations(mut self, fragment_density_invocations: bool) -> Self {
            self.fragment_density_invocations = fragment_density_invocations.into();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassFragmentDensityMapCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderPassFragmentDensityMapCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub fragment_density_map_attachment: AttachmentReference,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null(),
                fragment_density_map_attachment: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderPassFragmentDensityMapCreateInfoEXT<'a> {
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
    pub struct RenderingFragmentDensityMapAttachmentInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image_view: ImageView,
        pub image_layout: ImageLayout,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null(),
                image_view: Default::default(),
                image_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderingFragmentDensityMapAttachmentInfoEXT<'a> {
        pub fn image_view(mut self, image_view: ImageView) -> Self {
            self.image_view = image_view;
            self
        }
        pub fn image_layout(mut self, image_layout: ImageLayout) -> Self {
            self.image_layout = image_layout;
            self
        }
    }
}

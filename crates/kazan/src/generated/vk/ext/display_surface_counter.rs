//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_display_surface_counter.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_display_surface_counter";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceCapabilities2EXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SurfaceCapabilities2EXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_image_count: u32,
        pub max_image_count: u32,
        pub current_extent: Extent2D,
        pub min_image_extent: Extent2D,
        pub max_image_extent: Extent2D,
        pub max_image_array_layers: u32,
        pub supported_transforms: SurfaceTransformFlagsKHR,
        pub current_transform: SurfaceTransformFlagBitsKHR,
        pub supported_composite_alpha: CompositeAlphaFlagsKHR,
        pub supported_usage_flags: ImageUsageFlags,
        pub supported_surface_counters: SurfaceCounterFlagsEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SurfaceCapabilities2EXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SurfaceCapabilities2EXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("min_image_count", &self.min_image_count)
                .field("max_image_count", &self.max_image_count)
                .field("current_extent", &self.current_extent)
                .field("min_image_extent", &self.min_image_extent)
                .field("max_image_extent", &self.max_image_extent)
                .field("max_image_array_layers", &self.max_image_array_layers)
                .field("supported_transforms", &self.supported_transforms)
                .field("current_transform", &self.current_transform)
                .field("supported_composite_alpha", &self.supported_composite_alpha)
                .field("supported_usage_flags", &self.supported_usage_flags)
                .field(
                    "supported_surface_counters",
                    &self.supported_surface_counters,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SurfaceCapabilities2EXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SURFACE_CAPABILITIES_2_EXT;
    }

    impl Default for SurfaceCapabilities2EXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                min_image_count: Default::default(),
                max_image_count: Default::default(),
                current_extent: Default::default(),
                min_image_extent: Default::default(),
                max_image_extent: Default::default(),
                max_image_array_layers: Default::default(),
                supported_transforms: Default::default(),
                current_transform: Default::default(),
                supported_composite_alpha: Default::default(),
                supported_usage_flags: Default::default(),
                supported_surface_counters: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SurfaceCapabilities2EXT<'a> {
        #[inline]
        pub fn min_image_count(mut self, min_image_count: u32) -> Self {
            self.min_image_count = min_image_count;
            self
        }

        #[inline]
        pub fn max_image_count(mut self, max_image_count: u32) -> Self {
            self.max_image_count = max_image_count;
            self
        }

        #[inline]
        pub fn current_extent(mut self, current_extent: Extent2D) -> Self {
            self.current_extent = current_extent;
            self
        }

        #[inline]
        pub fn min_image_extent(mut self, min_image_extent: Extent2D) -> Self {
            self.min_image_extent = min_image_extent;
            self
        }

        #[inline]
        pub fn max_image_extent(mut self, max_image_extent: Extent2D) -> Self {
            self.max_image_extent = max_image_extent;
            self
        }

        #[inline]
        pub fn max_image_array_layers(mut self, max_image_array_layers: u32) -> Self {
            self.max_image_array_layers = max_image_array_layers;
            self
        }

        #[inline]
        pub fn supported_transforms(
            mut self,
            supported_transforms: SurfaceTransformFlagsKHR,
        ) -> Self {
            self.supported_transforms = supported_transforms;
            self
        }

        #[inline]
        pub fn current_transform(mut self, current_transform: SurfaceTransformFlagBitsKHR) -> Self {
            self.current_transform = current_transform;
            self
        }

        #[inline]
        pub fn supported_composite_alpha(
            mut self,
            supported_composite_alpha: CompositeAlphaFlagsKHR,
        ) -> Self {
            self.supported_composite_alpha = supported_composite_alpha;
            self
        }

        #[inline]
        pub fn supported_usage_flags(mut self, supported_usage_flags: ImageUsageFlags) -> Self {
            self.supported_usage_flags = supported_usage_flags;
            self
        }

        #[inline]
        pub fn supported_surface_counters(
            mut self,
            supported_surface_counters: SurfaceCounterFlagsEXT,
        ) -> Self {
            self.supported_surface_counters = supported_surface_counters;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceCounterFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SurfaceCounterFlagsEXT(Flags);
    vk_bitflags_wrapped!(SurfaceCounterFlagsEXT, Flags, SurfaceCounterFlagBitsEXT);

    impl fmt::Debug for SurfaceCounterFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] =
                &[(SurfaceCounterFlagBitsEXT::VBLANK_EXT.0, "VBLANK_EXT")];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceCounterFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct SurfaceCounterFlagBitsEXT(u32);

    impl SurfaceCounterFlagBitsEXT {
        pub const VBLANK_EXT: Self = Self(1 << 0);
    }

    impl fmt::Debug for SurfaceCounterFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::VBLANK_EXT => Some("VBLANK_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html>
    pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            surface: SurfaceKHR,
            p_surface_capabilities: *mut SurfaceCapabilities2EXT<'_>,
        ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkSurfaceCapabilities2EXT = SurfaceCapabilities2EXT<'static>;
    pub type VkSurfaceCounterFlagsEXT = SurfaceCounterFlagsEXT;
    pub type VkSurfaceCounterFlagBitsEXT = SurfaceCounterFlagBitsEXT;
    impl SurfaceCapabilities2EXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSurfaceCapabilities2EXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    get_physical_device_surface_capabilities2_ext: PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_surface_capabilities2_ext: transmute(
                    load(c"vkGetPhysicalDeviceSurfaceCapabilities2EXT")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html>
    #[inline]
    pub unsafe fn get_physical_device_surface_capabilities2_ext(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        surface_capabilities: &mut SurfaceCapabilities2EXT<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_physical_device_surface_capabilities2_ext)(
                physical_device,
                surface,
                surface_capabilities,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}

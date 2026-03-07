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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceCapabilities2EXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
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
                p_next: core::ptr::null_mut(),
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
        pub fn min_image_count(mut self, min_image_count: u32) -> Self {
            self.min_image_count = min_image_count;
            self
        }

        pub fn max_image_count(mut self, max_image_count: u32) -> Self {
            self.max_image_count = max_image_count;
            self
        }

        pub fn current_extent(mut self, current_extent: Extent2D) -> Self {
            self.current_extent = current_extent;
            self
        }

        pub fn min_image_extent(mut self, min_image_extent: Extent2D) -> Self {
            self.min_image_extent = min_image_extent;
            self
        }

        pub fn max_image_extent(mut self, max_image_extent: Extent2D) -> Self {
            self.max_image_extent = max_image_extent;
            self
        }

        pub fn max_image_array_layers(mut self, max_image_array_layers: u32) -> Self {
            self.max_image_array_layers = max_image_array_layers;
            self
        }

        pub fn supported_transforms(
            mut self,
            supported_transforms: SurfaceTransformFlagsKHR,
        ) -> Self {
            self.supported_transforms = supported_transforms;
            self
        }

        pub fn current_transform(mut self, current_transform: SurfaceTransformFlagBitsKHR) -> Self {
            self.current_transform = current_transform;
            self
        }

        pub fn supported_composite_alpha(
            mut self,
            supported_composite_alpha: CompositeAlphaFlagsKHR,
        ) -> Self {
            self.supported_composite_alpha = supported_composite_alpha;
            self
        }

        pub fn supported_usage_flags(mut self, supported_usage_flags: ImageUsageFlags) -> Self {
            self.supported_usage_flags = supported_usage_flags;
            self
        }

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
    vk_bitflags_wrapped!(SurfaceCounterFlagsEXT, Flags);

    impl SurfaceCounterFlagsEXT {
        pub const VBLANK_EXT: Self = Self(SurfaceCounterFlagBitsEXT::VBLANK_EXT.0);
    }

    impl fmt::Debug for SurfaceCounterFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[(SurfaceCounterFlagsEXT::VBLANK_EXT.0, "VBLANK_EXT")];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceCounterFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
    pub struct SurfaceCounterFlagBitsEXT(u32);

    impl SurfaceCounterFlagBitsEXT {
        pub const VBLANK_EXT: Self = Self(1 << 0);
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html>
    pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            surface: SurfaceKHR,
            p_surface_capabilities: *mut SurfaceCapabilities2EXT<'_>,
        ) -> vk::Result;
}

pub struct InstanceFn {
    get_physical_device_surface_capabilities2_ext: PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT,
}

impl InstanceFn {
    pub unsafe fn load(
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

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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfacePresentModeKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SurfacePresentModeKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_mode: PresentModeKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for SurfacePresentModeKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SURFACE_PRESENT_MODE_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceSurfaceInfo2KHR<'a>> for SurfacePresentModeKHR<'a> {}

    impl Default for SurfacePresentModeKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                present_mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SurfacePresentModeKHR<'a> {
        pub fn present_mode(mut self, present_mode: PresentModeKHR) -> Self {
            self.present_mode = present_mode;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfacePresentScalingCapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SurfacePresentScalingCapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub supported_present_scaling: PresentScalingFlagsKHR,
        pub supported_present_gravity_x: PresentGravityFlagsKHR,
        pub supported_present_gravity_y: PresentGravityFlagsKHR,
        pub min_scaled_image_extent: Extent2D,
        pub max_scaled_image_extent: Extent2D,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for SurfacePresentScalingCapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SURFACE_PRESENT_SCALING_CAPABILITIES_KHR;
    }

    unsafe impl<'a> Extends<SurfaceCapabilities2KHR<'a>> for SurfacePresentScalingCapabilitiesKHR<'a> {}

    impl Default for SurfacePresentScalingCapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                supported_present_scaling: Default::default(),
                supported_present_gravity_x: Default::default(),
                supported_present_gravity_y: Default::default(),
                min_scaled_image_extent: Default::default(),
                max_scaled_image_extent: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SurfacePresentScalingCapabilitiesKHR<'a> {
        pub fn supported_present_scaling(
            mut self,
            supported_present_scaling: PresentScalingFlagsKHR,
        ) -> Self {
            self.supported_present_scaling = supported_present_scaling;
            self
        }

        pub fn supported_present_gravity_x(
            mut self,
            supported_present_gravity_x: PresentGravityFlagsKHR,
        ) -> Self {
            self.supported_present_gravity_x = supported_present_gravity_x;
            self
        }

        pub fn supported_present_gravity_y(
            mut self,
            supported_present_gravity_y: PresentGravityFlagsKHR,
        ) -> Self {
            self.supported_present_gravity_y = supported_present_gravity_y;
            self
        }

        pub fn min_scaled_image_extent(mut self, min_scaled_image_extent: Extent2D) -> Self {
            self.min_scaled_image_extent = min_scaled_image_extent;
            self
        }

        pub fn max_scaled_image_extent(mut self, max_scaled_image_extent: Extent2D) -> Self {
            self.max_scaled_image_extent = max_scaled_image_extent;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfacePresentModeCompatibilityKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SurfacePresentModeCompatibilityKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_mode_count: u32,
        pub p_present_modes: *mut PresentModeKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for SurfacePresentModeCompatibilityKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SURFACE_PRESENT_MODE_COMPATIBILITY_KHR;
    }

    unsafe impl<'a> Extends<SurfaceCapabilities2KHR<'a>> for SurfacePresentModeCompatibilityKHR<'a> {}

    impl Default for SurfacePresentModeCompatibilityKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                present_mode_count: Default::default(),
                p_present_modes: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SurfacePresentModeCompatibilityKHR<'a> {
        pub fn present_modes(mut self, present_modes: &'a mut [PresentModeKHR]) -> Self {
            self.present_mode_count = present_modes.len().try_into().unwrap();
            self.p_present_modes = present_modes.as_mut_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentScalingFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PresentScalingFlagsKHR(Flags);
    vk_bitflags_wrapped!(PresentScalingFlagsKHR, Flags);

    impl PresentScalingFlagsKHR {
        pub const ONE_TO_ONE_KHR: Self = Self(PresentScalingFlagBitsKHR::ONE_TO_ONE_KHR.0);
        pub const ASPECT_RATIO_STRETCH_KHR: Self =
            Self(PresentScalingFlagBitsKHR::ASPECT_RATIO_STRETCH_KHR.0);
        pub const STRETCH_KHR: Self = Self(PresentScalingFlagBitsKHR::STRETCH_KHR.0);
        pub const ONE_TO_ONE_EXT: Self = Self::ONE_TO_ONE_KHR;
        pub const ASPECT_RATIO_STRETCH_EXT: Self = Self::ASPECT_RATIO_STRETCH_KHR;
        pub const STRETCH_EXT: Self = Self::STRETCH_KHR;
    }

    impl fmt::Debug for PresentScalingFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (PresentScalingFlagsKHR::ONE_TO_ONE_KHR.0, "ONE_TO_ONE_KHR"),
                (
                    PresentScalingFlagsKHR::ASPECT_RATIO_STRETCH_KHR.0,
                    "ASPECT_RATIO_STRETCH_KHR",
                ),
                (PresentScalingFlagsKHR::STRETCH_KHR.0, "STRETCH_KHR"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentScalingFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PresentScalingFlagBitsKHR(u32);

    impl PresentScalingFlagBitsKHR {
        pub const ONE_TO_ONE_KHR: Self = Self(1 << 0);
        pub const ASPECT_RATIO_STRETCH_KHR: Self = Self(1 << 1);
        pub const STRETCH_KHR: Self = Self(1 << 2);
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentGravityFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PresentGravityFlagsKHR(Flags);
    vk_bitflags_wrapped!(PresentGravityFlagsKHR, Flags);

    impl PresentGravityFlagsKHR {
        pub const MIN_KHR: Self = Self(PresentGravityFlagBitsKHR::MIN_KHR.0);
        pub const MAX_KHR: Self = Self(PresentGravityFlagBitsKHR::MAX_KHR.0);
        pub const CENTERED_KHR: Self = Self(PresentGravityFlagBitsKHR::CENTERED_KHR.0);
        pub const MIN_EXT: Self = Self::MIN_KHR;
        pub const MAX_EXT: Self = Self::MAX_KHR;
        pub const CENTERED_EXT: Self = Self::CENTERED_KHR;
    }

    impl fmt::Debug for PresentGravityFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (PresentGravityFlagsKHR::MIN_KHR.0, "MIN_KHR"),
                (PresentGravityFlagsKHR::MAX_KHR.0, "MAX_KHR"),
                (PresentGravityFlagsKHR::CENTERED_KHR.0, "CENTERED_KHR"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentGravityFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PresentGravityFlagBitsKHR(u32);

    impl PresentGravityFlagBitsKHR {
        pub const MIN_KHR: Self = Self(1 << 0);
        pub const MAX_KHR: Self = Self(1 << 1);
        pub const CENTERED_KHR: Self = Self(1 << 2);
    }
}

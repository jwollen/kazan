//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_display.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_display";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    handle_nondispatchable!(
        DisplayKHR,
        DISPLAY_KHR,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayKHR.html>"
    );
    handle_nondispatchable!(
        DisplayModeKHR,
        DISPLAY_MODE_KHR,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayModeKHR.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DisplayPropertiesKHR<'a> {
        pub display: DisplayKHR,
        pub display_name: *const c_char,
        pub physical_dimensions: Extent2D,
        pub physical_resolution: Extent2D,
        pub supported_transforms: SurfaceTransformFlagsKHR,
        pub plane_reorder_possible: Bool32,
        pub persistent_content: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DisplayPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DisplayPropertiesKHR")
                .field("display", &self.display)
                .field("display_name", &unsafe { as_c_str(self.display_name) })
                .field("physical_dimensions", &self.physical_dimensions)
                .field("physical_resolution", &self.physical_resolution)
                .field("supported_transforms", &self.supported_transforms)
                .field("plane_reorder_possible", &self.plane_reorder_possible)
                .field("persistent_content", &self.persistent_content)
                .finish()
        }
    }

    impl Default for DisplayPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                display: Default::default(),
                display_name: ptr::null(),
                physical_dimensions: Default::default(),
                physical_resolution: Default::default(),
                supported_transforms: Default::default(),
                plane_reorder_possible: Default::default(),
                persistent_content: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DisplayPropertiesKHR<'a> {
        #[inline]
        pub fn display(mut self, display: DisplayKHR) -> Self {
            self.display = display;
            self
        }

        #[inline]
        pub fn display_name(mut self, display_name: &'a CStr) -> Self {
            self.display_name = display_name.as_ptr();
            self
        }

        #[inline]
        pub fn physical_dimensions(mut self, physical_dimensions: Extent2D) -> Self {
            self.physical_dimensions = physical_dimensions;
            self
        }

        #[inline]
        pub fn physical_resolution(mut self, physical_resolution: Extent2D) -> Self {
            self.physical_resolution = physical_resolution;
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
        pub fn plane_reorder_possible(mut self, plane_reorder_possible: bool) -> Self {
            self.plane_reorder_possible = plane_reorder_possible.into();
            self
        }

        #[inline]
        pub fn persistent_content(mut self, persistent_content: bool) -> Self {
            self.persistent_content = persistent_content.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayPlanePropertiesKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct DisplayPlanePropertiesKHR {
        pub current_display: DisplayKHR,
        pub current_stack_index: u32,
    }

    impl DisplayPlanePropertiesKHR {
        #[inline]
        pub fn current_display(mut self, current_display: DisplayKHR) -> Self {
            self.current_display = current_display;
            self
        }

        #[inline]
        pub fn current_stack_index(mut self, current_stack_index: u32) -> Self {
            self.current_stack_index = current_stack_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayModeParametersKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct DisplayModeParametersKHR {
        pub visible_region: Extent2D,
        pub refresh_rate: u32,
    }

    impl DisplayModeParametersKHR {
        #[inline]
        pub fn visible_region(mut self, visible_region: Extent2D) -> Self {
            self.visible_region = visible_region;
            self
        }

        #[inline]
        pub fn refresh_rate(mut self, refresh_rate: u32) -> Self {
            self.refresh_rate = refresh_rate;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayModePropertiesKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct DisplayModePropertiesKHR {
        pub display_mode: DisplayModeKHR,
        pub parameters: DisplayModeParametersKHR,
    }

    impl DisplayModePropertiesKHR {
        #[inline]
        pub fn display_mode(mut self, display_mode: DisplayModeKHR) -> Self {
            self.display_mode = display_mode;
            self
        }

        #[inline]
        pub fn parameters(mut self, parameters: DisplayModeParametersKHR) -> Self {
            self.parameters = parameters;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayModeCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DisplayModeCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DisplayModeCreateFlagsKHR,
        pub parameters: DisplayModeParametersKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DisplayModeCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DisplayModeCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("parameters", &self.parameters)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DisplayModeCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DISPLAY_MODE_CREATE_INFO_KHR;
    }

    impl Default for DisplayModeCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                parameters: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DisplayModeCreateInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: DisplayModeCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn parameters(mut self, parameters: DisplayModeParametersKHR) -> Self {
            self.parameters = parameters;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayPlaneCapabilitiesKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct DisplayPlaneCapabilitiesKHR {
        pub supported_alpha: DisplayPlaneAlphaFlagsKHR,
        pub min_src_position: Offset2D,
        pub max_src_position: Offset2D,
        pub min_src_extent: Extent2D,
        pub max_src_extent: Extent2D,
        pub min_dst_position: Offset2D,
        pub max_dst_position: Offset2D,
        pub min_dst_extent: Extent2D,
        pub max_dst_extent: Extent2D,
    }

    impl DisplayPlaneCapabilitiesKHR {
        #[inline]
        pub fn supported_alpha(mut self, supported_alpha: DisplayPlaneAlphaFlagsKHR) -> Self {
            self.supported_alpha = supported_alpha;
            self
        }

        #[inline]
        pub fn min_src_position(mut self, min_src_position: Offset2D) -> Self {
            self.min_src_position = min_src_position;
            self
        }

        #[inline]
        pub fn max_src_position(mut self, max_src_position: Offset2D) -> Self {
            self.max_src_position = max_src_position;
            self
        }

        #[inline]
        pub fn min_src_extent(mut self, min_src_extent: Extent2D) -> Self {
            self.min_src_extent = min_src_extent;
            self
        }

        #[inline]
        pub fn max_src_extent(mut self, max_src_extent: Extent2D) -> Self {
            self.max_src_extent = max_src_extent;
            self
        }

        #[inline]
        pub fn min_dst_position(mut self, min_dst_position: Offset2D) -> Self {
            self.min_dst_position = min_dst_position;
            self
        }

        #[inline]
        pub fn max_dst_position(mut self, max_dst_position: Offset2D) -> Self {
            self.max_dst_position = max_dst_position;
            self
        }

        #[inline]
        pub fn min_dst_extent(mut self, min_dst_extent: Extent2D) -> Self {
            self.min_dst_extent = min_dst_extent;
            self
        }

        #[inline]
        pub fn max_dst_extent(mut self, max_dst_extent: Extent2D) -> Self {
            self.max_dst_extent = max_dst_extent;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplaySurfaceCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DisplaySurfaceCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DisplaySurfaceCreateFlagsKHR,
        pub display_mode: DisplayModeKHR,
        pub plane_index: u32,
        pub plane_stack_index: u32,
        pub transform: SurfaceTransformFlagBitsKHR,
        pub global_alpha: f32,
        pub alpha_mode: DisplayPlaneAlphaFlagBitsKHR,
        pub image_extent: Extent2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DisplaySurfaceCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DisplaySurfaceCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("display_mode", &self.display_mode)
                .field("plane_index", &self.plane_index)
                .field("plane_stack_index", &self.plane_stack_index)
                .field("transform", &self.transform)
                .field("global_alpha", &self.global_alpha)
                .field("alpha_mode", &self.alpha_mode)
                .field("image_extent", &self.image_extent)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DisplaySurfaceCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DISPLAY_SURFACE_CREATE_INFO_KHR;
    }

    impl Default for DisplaySurfaceCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                display_mode: Default::default(),
                plane_index: Default::default(),
                plane_stack_index: Default::default(),
                transform: Default::default(),
                global_alpha: Default::default(),
                alpha_mode: Default::default(),
                image_extent: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DisplaySurfaceCreateInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: DisplaySurfaceCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn display_mode(mut self, display_mode: DisplayModeKHR) -> Self {
            self.display_mode = display_mode;
            self
        }

        #[inline]
        pub fn plane_index(mut self, plane_index: u32) -> Self {
            self.plane_index = plane_index;
            self
        }

        #[inline]
        pub fn plane_stack_index(mut self, plane_stack_index: u32) -> Self {
            self.plane_stack_index = plane_stack_index;
            self
        }

        #[inline]
        pub fn transform(mut self, transform: SurfaceTransformFlagBitsKHR) -> Self {
            self.transform = transform;
            self
        }

        #[inline]
        pub fn global_alpha(mut self, global_alpha: f32) -> Self {
            self.global_alpha = global_alpha;
            self
        }

        #[inline]
        pub fn alpha_mode(mut self, alpha_mode: DisplayPlaneAlphaFlagBitsKHR) -> Self {
            self.alpha_mode = alpha_mode;
            self
        }

        #[inline]
        pub fn image_extent(mut self, image_extent: Extent2D) -> Self {
            self.image_extent = image_extent;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayPlaneAlphaFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DisplayPlaneAlphaFlagsKHR(Flags);
    vk_bitflags_wrapped!(
        DisplayPlaneAlphaFlagsKHR,
        Flags,
        DisplayPlaneAlphaFlagBitsKHR
    );

    impl fmt::Debug for DisplayPlaneAlphaFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (DisplayPlaneAlphaFlagBitsKHR::OPAQUE_KHR.0, "OPAQUE_KHR"),
                (DisplayPlaneAlphaFlagBitsKHR::GLOBAL_KHR.0, "GLOBAL_KHR"),
                (
                    DisplayPlaneAlphaFlagBitsKHR::PER_PIXEL_KHR.0,
                    "PER_PIXEL_KHR",
                ),
                (
                    DisplayPlaneAlphaFlagBitsKHR::PER_PIXEL_PREMULTIPLIED_KHR.0,
                    "PER_PIXEL_PREMULTIPLIED_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayPlaneAlphaFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DisplayPlaneAlphaFlagBitsKHR(u32);

    impl DisplayPlaneAlphaFlagBitsKHR {
        pub const OPAQUE_KHR: Self = Self(1 << 0);
        pub const GLOBAL_KHR: Self = Self(1 << 1);
        pub const PER_PIXEL_KHR: Self = Self(1 << 2);
        pub const PER_PIXEL_PREMULTIPLIED_KHR: Self = Self(1 << 3);
    }

    impl fmt::Debug for DisplayPlaneAlphaFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::OPAQUE_KHR => Some("OPAQUE_KHR"),
                Self::GLOBAL_KHR => Some("GLOBAL_KHR"),
                Self::PER_PIXEL_KHR => Some("PER_PIXEL_KHR"),
                Self::PER_PIXEL_PREMULTIPLIED_KHR => Some("PER_PIXEL_PREMULTIPLIED_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayModeCreateFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DisplayModeCreateFlagsKHR(Flags);
    vk_bitflags_wrapped!(DisplayModeCreateFlagsKHR, Flags);

    impl fmt::Debug for DisplayModeCreateFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplaySurfaceCreateFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DisplaySurfaceCreateFlagsKHR(Flags);
    vk_bitflags_wrapped!(DisplaySurfaceCreateFlagsKHR, Flags);

    impl fmt::Debug for DisplaySurfaceCreateFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html>
    pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayPropertiesKHR<'_>,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html>
    pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut DisplayPlanePropertiesKHR,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html>
    pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        plane_index: u32,
        p_display_count: *mut u32,
        p_displays: *mut DisplayKHR,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDisplayModePropertiesKHR.html>
    pub type PFN_vkGetDisplayModePropertiesKHR = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_property_count: *mut u32,
        p_properties: *mut DisplayModePropertiesKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateDisplayModeKHR.html>
    pub type PFN_vkCreateDisplayModeKHR = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_create_info: *const DisplayModeCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_mode: *mut DisplayModeKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDisplayPlaneCapabilitiesKHR.html>
    pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        mode: DisplayModeKHR,
        plane_index: u32,
        p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateDisplayPlaneSurfaceKHR.html>
    pub type PFN_vkCreateDisplayPlaneSurfaceKHR = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const DisplaySurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkDisplayKHR = DisplayKHR;
    pub type VkDisplayModeKHR = DisplayModeKHR;
    pub type VkDisplayPropertiesKHR = DisplayPropertiesKHR<'static>;
    pub type VkDisplayPlanePropertiesKHR = DisplayPlanePropertiesKHR;
    pub type VkDisplayModeParametersKHR = DisplayModeParametersKHR;
    pub type VkDisplayModePropertiesKHR = DisplayModePropertiesKHR;
    pub type VkDisplayModeCreateInfoKHR = DisplayModeCreateInfoKHR<'static>;
    pub type VkDisplayPlaneCapabilitiesKHR = DisplayPlaneCapabilitiesKHR;
    pub type VkDisplaySurfaceCreateInfoKHR = DisplaySurfaceCreateInfoKHR<'static>;
    pub type VkDisplayPlaneAlphaFlagsKHR = DisplayPlaneAlphaFlagsKHR;
    pub type VkDisplayPlaneAlphaFlagBitsKHR = DisplayPlaneAlphaFlagBitsKHR;
    pub type VkDisplayModeCreateFlagsKHR = DisplayModeCreateFlagsKHR;
    pub type VkDisplaySurfaceCreateFlagsKHR = DisplaySurfaceCreateFlagsKHR;
    impl DisplayPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDisplayPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DisplayModeCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDisplayModeCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DisplaySurfaceCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDisplaySurfaceCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    get_physical_device_display_properties: PFN_vkGetPhysicalDeviceDisplayPropertiesKHR,
    get_physical_device_display_plane_properties: PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR,
    get_display_plane_supported_displays: PFN_vkGetDisplayPlaneSupportedDisplaysKHR,
    get_display_mode_properties: PFN_vkGetDisplayModePropertiesKHR,
    create_display_mode: PFN_vkCreateDisplayModeKHR,
    get_display_plane_capabilities: PFN_vkGetDisplayPlaneCapabilitiesKHR,
    create_display_plane_surface: PFN_vkCreateDisplayPlaneSurfaceKHR,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_display_properties: transmute(
                    load(c"vkGetPhysicalDeviceDisplayPropertiesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_display_plane_properties: transmute(
                    load(c"vkGetPhysicalDeviceDisplayPlanePropertiesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_display_plane_supported_displays: transmute(
                    load(c"vkGetDisplayPlaneSupportedDisplaysKHR").ok_or(MissingEntryPointError)?,
                ),
                get_display_mode_properties: transmute(
                    load(c"vkGetDisplayModePropertiesKHR").ok_or(MissingEntryPointError)?,
                ),
                create_display_mode: transmute(
                    load(c"vkCreateDisplayModeKHR").ok_or(MissingEntryPointError)?,
                ),
                get_display_plane_capabilities: transmute(
                    load(c"vkGetDisplayPlaneCapabilitiesKHR").ok_or(MissingEntryPointError)?,
                ),
                create_display_plane_surface: transmute(
                    load(c"vkCreateDisplayPlaneSurfaceKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_display_properties<'a>(
        &self,
        physical_device: PhysicalDevice,
        mut properties: impl EnumerateInto<DisplayPropertiesKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |property_count, properties| {
                let result = (self.get_physical_device_display_properties)(
                    physical_device,
                    property_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let properties_buf = properties.reserve(capacity);
            len = properties_buf.len().try_into().unwrap();
            let result = call(&mut len, properties_buf.as_mut_ptr() as *mut _)?;
            properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_display_plane_properties(
        &self,
        physical_device: PhysicalDevice,
        mut properties: impl EnumerateInto<DisplayPlanePropertiesKHR>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |property_count, properties| {
                let result = (self.get_physical_device_display_plane_properties)(
                    physical_device,
                    property_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let properties_buf = properties.reserve(capacity);
            len = properties_buf.len().try_into().unwrap();
            let result = call(&mut len, properties_buf.as_mut_ptr() as *mut _)?;
            properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html>
    #[inline]
    pub unsafe fn get_display_plane_supported_displays(
        &self,
        physical_device: PhysicalDevice,
        plane_index: u32,
        mut displays: impl EnumerateInto<DisplayKHR>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |display_count, displays| {
                let result = (self.get_display_plane_supported_displays)(
                    physical_device,
                    plane_index,
                    display_count,
                    displays as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let displays_buf = displays.reserve(capacity);
            len = displays_buf.len().try_into().unwrap();
            let result = call(&mut len, displays_buf.as_mut_ptr() as *mut _)?;
            displays.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDisplayModePropertiesKHR.html>
    #[inline]
    pub unsafe fn get_display_mode_properties(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        mut properties: impl EnumerateInto<DisplayModePropertiesKHR>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |property_count, properties| {
                let result = (self.get_display_mode_properties)(
                    physical_device,
                    display,
                    property_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let properties_buf = properties.reserve(capacity);
            len = properties_buf.len().try_into().unwrap();
            let result = call(&mut len, properties_buf.as_mut_ptr() as *mut _)?;
            properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateDisplayModeKHR.html>
    #[inline]
    pub unsafe fn create_display_mode(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        create_info: &DisplayModeCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<DisplayModeKHR> {
        unsafe {
            let mut mode = core::mem::MaybeUninit::uninit();
            let result = (self.create_display_mode)(
                physical_device,
                display,
                create_info,
                allocator.to_raw_ptr(),
                mode.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(mode.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDisplayPlaneCapabilitiesKHR.html>
    #[inline]
    pub unsafe fn get_display_plane_capabilities(
        &self,
        physical_device: PhysicalDevice,
        mode: DisplayModeKHR,
        plane_index: u32,
    ) -> crate::Result<DisplayPlaneCapabilitiesKHR> {
        unsafe {
            let mut capabilities = core::mem::MaybeUninit::uninit();
            let result = (self.get_display_plane_capabilities)(
                physical_device,
                mode,
                plane_index,
                capabilities.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(capabilities.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateDisplayPlaneSurfaceKHR.html>
    #[inline]
    pub unsafe fn create_display_plane_surface(
        &self,
        instance: Instance,
        create_info: &DisplaySurfaceCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_display_plane_surface)(
                instance,
                create_info,
                allocator.to_raw_ptr(),
                surface.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(surface.assume_init()),
                err => Err(err),
            }
        }
    }
}

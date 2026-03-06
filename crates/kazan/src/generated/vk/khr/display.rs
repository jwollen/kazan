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
    impl Default for DisplayPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                display: Default::default(),
                display_name: core::ptr::null(),
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
        pub fn display(mut self, display: DisplayKHR) -> Self {
            self.display = display;
            self
        }
        pub fn display_name(mut self, display_name: &'a CStr) -> Self {
            self.display_name = display_name.as_ptr();
            self
        }
        pub fn physical_dimensions(mut self, physical_dimensions: Extent2D) -> Self {
            self.physical_dimensions = physical_dimensions;
            self
        }
        pub fn physical_resolution(mut self, physical_resolution: Extent2D) -> Self {
            self.physical_resolution = physical_resolution;
            self
        }
        pub fn supported_transforms(
            mut self,
            supported_transforms: SurfaceTransformFlagsKHR,
        ) -> Self {
            self.supported_transforms = supported_transforms;
            self
        }
        pub fn plane_reorder_possible(mut self, plane_reorder_possible: bool) -> Self {
            self.plane_reorder_possible = plane_reorder_possible.into();
            self
        }
        pub fn persistent_content(mut self, persistent_content: bool) -> Self {
            self.persistent_content = persistent_content.into();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayPlanePropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct DisplayPlanePropertiesKHR {
        pub current_display: DisplayKHR,
        pub current_stack_index: u32,
    }
    impl DisplayPlanePropertiesKHR {
        pub fn current_display(mut self, current_display: DisplayKHR) -> Self {
            self.current_display = current_display;
            self
        }
        pub fn current_stack_index(mut self, current_stack_index: u32) -> Self {
            self.current_stack_index = current_stack_index;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayModeParametersKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct DisplayModeParametersKHR {
        pub visible_region: Extent2D,
        pub refresh_rate: u32,
    }
    impl DisplayModeParametersKHR {
        pub fn visible_region(mut self, visible_region: Extent2D) -> Self {
            self.visible_region = visible_region;
            self
        }
        pub fn refresh_rate(mut self, refresh_rate: u32) -> Self {
            self.refresh_rate = refresh_rate;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayModePropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct DisplayModePropertiesKHR {
        pub display_mode: DisplayModeKHR,
        pub parameters: DisplayModeParametersKHR,
    }
    impl DisplayModePropertiesKHR {
        pub fn display_mode(mut self, display_mode: DisplayModeKHR) -> Self {
            self.display_mode = display_mode;
            self
        }
        pub fn parameters(mut self, parameters: DisplayModeParametersKHR) -> Self {
            self.parameters = parameters;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayModeCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DisplayModeCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DisplayModeCreateFlagsKHR,
        pub parameters: DisplayModeParametersKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DisplayModeCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DISPLAY_MODE_CREATE_INFO_KHR;
    }
    impl Default for DisplayModeCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                parameters: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DisplayModeCreateInfoKHR<'a> {
        pub fn flags(mut self, flags: DisplayModeCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn parameters(mut self, parameters: DisplayModeParametersKHR) -> Self {
            self.parameters = parameters;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayPlaneCapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
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
        pub fn supported_alpha(mut self, supported_alpha: DisplayPlaneAlphaFlagsKHR) -> Self {
            self.supported_alpha = supported_alpha;
            self
        }
        pub fn min_src_position(mut self, min_src_position: Offset2D) -> Self {
            self.min_src_position = min_src_position;
            self
        }
        pub fn max_src_position(mut self, max_src_position: Offset2D) -> Self {
            self.max_src_position = max_src_position;
            self
        }
        pub fn min_src_extent(mut self, min_src_extent: Extent2D) -> Self {
            self.min_src_extent = min_src_extent;
            self
        }
        pub fn max_src_extent(mut self, max_src_extent: Extent2D) -> Self {
            self.max_src_extent = max_src_extent;
            self
        }
        pub fn min_dst_position(mut self, min_dst_position: Offset2D) -> Self {
            self.min_dst_position = min_dst_position;
            self
        }
        pub fn max_dst_position(mut self, max_dst_position: Offset2D) -> Self {
            self.max_dst_position = max_dst_position;
            self
        }
        pub fn min_dst_extent(mut self, min_dst_extent: Extent2D) -> Self {
            self.min_dst_extent = min_dst_extent;
            self
        }
        pub fn max_dst_extent(mut self, max_dst_extent: Extent2D) -> Self {
            self.max_dst_extent = max_dst_extent;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplaySurfaceCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for DisplaySurfaceCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DISPLAY_SURFACE_CREATE_INFO_KHR;
    }
    impl Default for DisplaySurfaceCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
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
        pub fn flags(mut self, flags: DisplaySurfaceCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn display_mode(mut self, display_mode: DisplayModeKHR) -> Self {
            self.display_mode = display_mode;
            self
        }
        pub fn plane_index(mut self, plane_index: u32) -> Self {
            self.plane_index = plane_index;
            self
        }
        pub fn plane_stack_index(mut self, plane_stack_index: u32) -> Self {
            self.plane_stack_index = plane_stack_index;
            self
        }
        pub fn transform(mut self, transform: SurfaceTransformFlagBitsKHR) -> Self {
            self.transform = transform;
            self
        }
        pub fn global_alpha(mut self, global_alpha: f32) -> Self {
            self.global_alpha = global_alpha;
            self
        }
        pub fn alpha_mode(mut self, alpha_mode: DisplayPlaneAlphaFlagBitsKHR) -> Self {
            self.alpha_mode = alpha_mode;
            self
        }
        pub fn image_extent(mut self, image_extent: Extent2D) -> Self {
            self.image_extent = image_extent;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayPlaneAlphaFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DisplayPlaneAlphaFlagsKHR(Flags);
    vk_bitflags_wrapped!(DisplayPlaneAlphaFlagsKHR, Flags);
    impl DisplayPlaneAlphaFlagsKHR {
        pub const OPAQUE_KHR: Self = Self(DisplayPlaneAlphaFlagBitsKHR::OPAQUE_KHR.0);
        pub const GLOBAL_KHR: Self = Self(DisplayPlaneAlphaFlagBitsKHR::GLOBAL_KHR.0);
        pub const PER_PIXEL_KHR: Self = Self(DisplayPlaneAlphaFlagBitsKHR::PER_PIXEL_KHR.0);
        pub const PER_PIXEL_PREMULTIPLIED_KHR: Self =
            Self(DisplayPlaneAlphaFlagBitsKHR::PER_PIXEL_PREMULTIPLIED_KHR.0);
    }
    impl fmt::Debug for DisplayPlaneAlphaFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (DisplayPlaneAlphaFlagsKHR::OPAQUE_KHR.0, "OPAQUE_KHR"),
                (DisplayPlaneAlphaFlagsKHR::GLOBAL_KHR.0, "GLOBAL_KHR"),
                (DisplayPlaneAlphaFlagsKHR::PER_PIXEL_KHR.0, "PER_PIXEL_KHR"),
                (
                    DisplayPlaneAlphaFlagsKHR::PER_PIXEL_PREMULTIPLIED_KHR.0,
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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceTransformFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SurfaceTransformFlagsKHR(Flags);
    vk_bitflags_wrapped!(SurfaceTransformFlagsKHR, Flags);
    impl SurfaceTransformFlagsKHR {
        pub const IDENTITY_KHR: Self = Self(SurfaceTransformFlagBitsKHR::IDENTITY_KHR.0);
        pub const ROTATE_90_KHR: Self = Self(SurfaceTransformFlagBitsKHR::ROTATE_90_KHR.0);
        pub const ROTATE_180_KHR: Self = Self(SurfaceTransformFlagBitsKHR::ROTATE_180_KHR.0);
        pub const ROTATE_270_KHR: Self = Self(SurfaceTransformFlagBitsKHR::ROTATE_270_KHR.0);
        pub const HORIZONTAL_MIRROR_KHR: Self =
            Self(SurfaceTransformFlagBitsKHR::HORIZONTAL_MIRROR_KHR.0);
        pub const HORIZONTAL_MIRROR_ROTATE_90_KHR: Self =
            Self(SurfaceTransformFlagBitsKHR::HORIZONTAL_MIRROR_ROTATE_90_KHR.0);
        pub const HORIZONTAL_MIRROR_ROTATE_180_KHR: Self =
            Self(SurfaceTransformFlagBitsKHR::HORIZONTAL_MIRROR_ROTATE_180_KHR.0);
        pub const HORIZONTAL_MIRROR_ROTATE_270_KHR: Self =
            Self(SurfaceTransformFlagBitsKHR::HORIZONTAL_MIRROR_ROTATE_270_KHR.0);
        pub const INHERIT_KHR: Self = Self(SurfaceTransformFlagBitsKHR::INHERIT_KHR.0);
    }
    impl fmt::Debug for SurfaceTransformFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (SurfaceTransformFlagsKHR::IDENTITY_KHR.0, "IDENTITY_KHR"),
                (SurfaceTransformFlagsKHR::ROTATE_90_KHR.0, "ROTATE_90_KHR"),
                (SurfaceTransformFlagsKHR::ROTATE_180_KHR.0, "ROTATE_180_KHR"),
                (SurfaceTransformFlagsKHR::ROTATE_270_KHR.0, "ROTATE_270_KHR"),
                (
                    SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_KHR.0,
                    "HORIZONTAL_MIRROR_KHR",
                ),
                (
                    SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_90_KHR.0,
                    "HORIZONTAL_MIRROR_ROTATE_90_KHR",
                ),
                (
                    SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_180_KHR.0,
                    "HORIZONTAL_MIRROR_ROTATE_180_KHR",
                ),
                (
                    SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_270_KHR.0,
                    "HORIZONTAL_MIRROR_ROTATE_270_KHR",
                ),
                (SurfaceTransformFlagsKHR::INHERIT_KHR.0, "INHERIT_KHR"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceTransformFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct SurfaceTransformFlagBitsKHR(u32);
    impl SurfaceTransformFlagBitsKHR {
        pub const IDENTITY_KHR: Self = Self(1 << 0);
        pub const ROTATE_90_KHR: Self = Self(1 << 1);
        pub const ROTATE_180_KHR: Self = Self(1 << 2);
        pub const ROTATE_270_KHR: Self = Self(1 << 3);
        pub const HORIZONTAL_MIRROR_KHR: Self = Self(1 << 4);
        pub const HORIZONTAL_MIRROR_ROTATE_90_KHR: Self = Self(1 << 5);
        pub const HORIZONTAL_MIRROR_ROTATE_180_KHR: Self = Self(1 << 6);
        pub const HORIZONTAL_MIRROR_ROTATE_270_KHR: Self = Self(1 << 7);
        pub const INHERIT_KHR: Self = Self(1 << 8);
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
pub struct InstanceFn {
    get_physical_device_display_properties_khr: PFN_vkGetPhysicalDeviceDisplayPropertiesKHR,
    get_physical_device_display_plane_properties_khr:
        PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR,
    get_display_plane_supported_displays_khr: PFN_vkGetDisplayPlaneSupportedDisplaysKHR,
    get_display_mode_properties_khr: PFN_vkGetDisplayModePropertiesKHR,
    create_display_mode_khr: PFN_vkCreateDisplayModeKHR,
    get_display_plane_capabilities_khr: PFN_vkGetDisplayPlaneCapabilitiesKHR,
    create_display_plane_surface_khr: PFN_vkCreateDisplayPlaneSurfaceKHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_display_properties_khr: transmute(
                    load(c"vkGetPhysicalDeviceDisplayPropertiesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_display_plane_properties_khr: transmute(
                    load(c"vkGetPhysicalDeviceDisplayPlanePropertiesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_display_plane_supported_displays_khr: transmute(
                    load(c"vkGetDisplayPlaneSupportedDisplaysKHR").ok_or(MissingEntryPointError)?,
                ),
                get_display_mode_properties_khr: transmute(
                    load(c"vkGetDisplayModePropertiesKHR").ok_or(MissingEntryPointError)?,
                ),
                create_display_mode_khr: transmute(
                    load(c"vkCreateDisplayModeKHR").ok_or(MissingEntryPointError)?,
                ),
                get_display_plane_capabilities_khr: transmute(
                    load(c"vkGetDisplayPlaneCapabilitiesKHR").ok_or(MissingEntryPointError)?,
                ),
                create_display_plane_surface_khr: transmute(
                    load(c"vkCreateDisplayPlaneSurfaceKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html>
    pub unsafe fn get_physical_device_display_properties_khr<'a>(
        &self,
        physical_device: PhysicalDevice,
        mut properties: impl ExtendUninit<DisplayPropertiesKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |property_count, properties| {
                let result = (self.get_physical_device_display_properties_khr)(
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
    pub unsafe fn get_physical_device_display_plane_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        mut properties: impl ExtendUninit<DisplayPlanePropertiesKHR>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |property_count, properties| {
                let result = (self.get_physical_device_display_plane_properties_khr)(
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
    pub unsafe fn get_display_plane_supported_displays_khr(
        &self,
        physical_device: PhysicalDevice,
        plane_index: u32,
        mut displays: impl ExtendUninit<DisplayKHR>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |display_count, displays| {
                let result = (self.get_display_plane_supported_displays_khr)(
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
    pub unsafe fn get_display_mode_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        mut properties: impl ExtendUninit<DisplayModePropertiesKHR>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |property_count, properties| {
                let result = (self.get_display_mode_properties_khr)(
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
    pub unsafe fn create_display_mode_khr(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        create_info: &DisplayModeCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<DisplayModeKHR> {
        unsafe {
            let mut mode = core::mem::MaybeUninit::uninit();
            let result = (self.create_display_mode_khr)(
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
    pub unsafe fn get_display_plane_capabilities_khr(
        &self,
        physical_device: PhysicalDevice,
        mode: DisplayModeKHR,
        plane_index: u32,
    ) -> crate::Result<DisplayPlaneCapabilitiesKHR> {
        unsafe {
            let mut capabilities = core::mem::MaybeUninit::uninit();
            let result = (self.get_display_plane_capabilities_khr)(
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
    pub unsafe fn create_display_plane_surface_khr(
        &self,
        instance: Instance,
        create_info: &DisplaySurfaceCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_display_plane_surface_khr)(
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

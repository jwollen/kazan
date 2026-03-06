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
    handle_nondispatchable!(SurfaceKHR, SURFACE_KHR, doc = "");
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct SurfaceCapabilitiesKHR {
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
    }
    impl SurfaceCapabilitiesKHR {
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
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct SurfaceFormatKHR {
        pub format: Format,
        pub color_space: ColorSpaceKHR,
    }
    impl SurfaceFormatKHR {
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
        pub fn color_space(mut self, color_space: ColorSpaceKHR) -> Self {
            self.color_space = color_space;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PresentModeKHR(i32);
    impl PresentModeKHR {
        pub const IMMEDIATE_KHR: Self = Self(0);
        pub const MAILBOX_KHR: Self = Self(1);
        pub const FIFO_KHR: Self = Self(2);
        pub const FIFO_RELAXED_KHR: Self = Self(3);
        // VK_EXT_present_mode_fifo_latest_ready
        pub const FIFO_LATEST_READY_EXT: Self = Self::FIFO_LATEST_READY_KHR;
        // VK_KHR_present_mode_fifo_latest_ready
        pub const FIFO_LATEST_READY_KHR: Self = Self(1000361000);
        // VK_KHR_shared_presentable_image
        pub const SHARED_DEMAND_REFRESH_KHR: Self = Self(1000111000);
        pub const SHARED_CONTINUOUS_REFRESH_KHR: Self = Self(1000111001);
    }
    impl fmt::Debug for PresentModeKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::IMMEDIATE_KHR => Some("IMMEDIATE_KHR"),
                Self::MAILBOX_KHR => Some("MAILBOX_KHR"),
                Self::FIFO_KHR => Some("FIFO_KHR"),
                Self::FIFO_RELAXED_KHR => Some("FIFO_RELAXED_KHR"),
                Self::FIFO_LATEST_READY_KHR => Some("FIFO_LATEST_READY_KHR"),
                Self::SHARED_DEMAND_REFRESH_KHR => Some("SHARED_DEMAND_REFRESH_KHR"),
                Self::SHARED_CONTINUOUS_REFRESH_KHR => Some("SHARED_CONTINUOUS_REFRESH_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ColorSpaceKHR(i32);
    impl ColorSpaceKHR {
        pub const SRGB_NONLINEAR_KHR: Self = Self(0);
        // VK_AMD_display_native_hdr
        pub const DISPLAY_NATIVE_AMD: Self = Self(1000213000);
        // VK_EXT_swapchain_colorspace
        pub const DISPLAY_P3_NONLINEAR_EXT: Self = Self(1000104001);
        pub const EXTENDED_SRGB_LINEAR_EXT: Self = Self(1000104002);
        pub const DISPLAY_P3_LINEAR_EXT: Self = Self(1000104003);
        pub const DCI_P3_NONLINEAR_EXT: Self = Self(1000104004);
        pub const BT709_LINEAR_EXT: Self = Self(1000104005);
        pub const BT709_NONLINEAR_EXT: Self = Self(1000104006);
        pub const BT2020_LINEAR_EXT: Self = Self(1000104007);
        pub const HDR10_ST2084_EXT: Self = Self(1000104008);
        pub const DOLBYVISION_EXT: Self = Self(1000104009);
        pub const HDR10_HLG_EXT: Self = Self(1000104010);
        pub const ADOBERGB_LINEAR_EXT: Self = Self(1000104011);
        pub const ADOBERGB_NONLINEAR_EXT: Self = Self(1000104012);
        pub const PASS_THROUGH_EXT: Self = Self(1000104013);
        pub const EXTENDED_SRGB_NONLINEAR_EXT: Self = Self(1000104014);
        pub const DCI_P3_LINEAR_EXT: Self = Self::DISPLAY_P3_LINEAR_EXT;
    }
    impl fmt::Debug for ColorSpaceKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SRGB_NONLINEAR_KHR => Some("SRGB_NONLINEAR_KHR"),
                Self::DISPLAY_NATIVE_AMD => Some("DISPLAY_NATIVE_AMD"),
                Self::DISPLAY_P3_NONLINEAR_EXT => Some("DISPLAY_P3_NONLINEAR_EXT"),
                Self::EXTENDED_SRGB_LINEAR_EXT => Some("EXTENDED_SRGB_LINEAR_EXT"),
                Self::DISPLAY_P3_LINEAR_EXT => Some("DISPLAY_P3_LINEAR_EXT"),
                Self::DCI_P3_NONLINEAR_EXT => Some("DCI_P3_NONLINEAR_EXT"),
                Self::BT709_LINEAR_EXT => Some("BT709_LINEAR_EXT"),
                Self::BT709_NONLINEAR_EXT => Some("BT709_NONLINEAR_EXT"),
                Self::BT2020_LINEAR_EXT => Some("BT2020_LINEAR_EXT"),
                Self::HDR10_ST2084_EXT => Some("HDR10_ST2084_EXT"),
                Self::DOLBYVISION_EXT => Some("DOLBYVISION_EXT"),
                Self::HDR10_HLG_EXT => Some("HDR10_HLG_EXT"),
                Self::ADOBERGB_LINEAR_EXT => Some("ADOBERGB_LINEAR_EXT"),
                Self::ADOBERGB_NONLINEAR_EXT => Some("ADOBERGB_NONLINEAR_EXT"),
                Self::PASS_THROUGH_EXT => Some("PASS_THROUGH_EXT"),
                Self::EXTENDED_SRGB_NONLINEAR_EXT => Some("EXTENDED_SRGB_NONLINEAR_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CompositeAlphaFlagsKHR(Flags);
    vk_bitflags_wrapped!(CompositeAlphaFlagsKHR, Flags);
    impl CompositeAlphaFlagsKHR {
        pub const OPAQUE_KHR: Self = Self(CompositeAlphaFlagBitsKHR::OPAQUE_KHR.0);
        pub const PRE_MULTIPLIED_KHR: Self = Self(CompositeAlphaFlagBitsKHR::PRE_MULTIPLIED_KHR.0);
        pub const POST_MULTIPLIED_KHR: Self =
            Self(CompositeAlphaFlagBitsKHR::POST_MULTIPLIED_KHR.0);
        pub const INHERIT_KHR: Self = Self(CompositeAlphaFlagBitsKHR::INHERIT_KHR.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct CompositeAlphaFlagBitsKHR(u32);
    impl CompositeAlphaFlagBitsKHR {
        pub const OPAQUE_KHR: Self = Self(1 << 0);
        pub const PRE_MULTIPLIED_KHR: Self = Self(1 << 1);
        pub const POST_MULTIPLIED_KHR: Self = Self(1 << 2);
        pub const INHERIT_KHR: Self = Self(1 << 3);
    }
    pub type PFN_vkDestroySurfaceKHR = unsafe extern "system" fn(
        instance: Instance,
        surface: SurfaceKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        surface: SurfaceKHR,
        p_supported: *mut Bool32,
    )
        -> vk::Result;
    pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            surface: SurfaceKHR,
            p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
        ) -> vk::Result;
    pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut SurfaceFormatKHR,
    )
        -> vk::Result;
    pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            surface: SurfaceKHR,
            p_present_mode_count: *mut u32,
            p_present_modes: *mut PresentModeKHR,
        ) -> vk::Result;
}
pub struct InstanceFn {
    destroy_surface_khr: PFN_vkDestroySurfaceKHR,
    get_physical_device_surface_support_khr: PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    get_physical_device_surface_capabilities_khr: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    get_physical_device_surface_formats_khr: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
    get_physical_device_surface_present_modes_khr: PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                destroy_surface_khr: transmute(
                    load(c"vkDestroySurfaceKHR").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_surface_support_khr: transmute(
                    load(c"vkGetPhysicalDeviceSurfaceSupportKHR").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_surface_capabilities_khr: transmute(
                    load(c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_surface_formats_khr: transmute(
                    load(c"vkGetPhysicalDeviceSurfaceFormatsKHR").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_surface_present_modes_khr: transmute(
                    load(c"vkGetPhysicalDeviceSurfacePresentModesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn destroy_surface_khr(
        &self,
        instance: Instance,
        surface: SurfaceKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_surface_khr)(instance, surface, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_physical_device_surface_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        surface: SurfaceKHR,
    ) -> crate::Result<bool> {
        unsafe {
            let mut supported = core::mem::MaybeUninit::uninit();
            let result = (self.get_physical_device_surface_support_khr)(
                physical_device,
                queue_family_index,
                surface,
                supported.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(supported.assume_init() != 0),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_physical_device_surface_capabilities_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
    ) -> crate::Result<SurfaceCapabilitiesKHR> {
        unsafe {
            let mut surface_capabilities = core::mem::MaybeUninit::uninit();
            let result = (self.get_physical_device_surface_capabilities_khr)(
                physical_device,
                surface,
                surface_capabilities.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(surface_capabilities.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_physical_device_surface_formats_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        mut surface_formats: impl ExtendUninit<SurfaceFormatKHR>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |surface_format_count, surface_formats| {
                let result = (self.get_physical_device_surface_formats_khr)(
                    physical_device,
                    surface,
                    surface_format_count,
                    surface_formats as _,
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
            let surface_formats_buf = surface_formats.reserve(capacity);
            len = surface_formats_buf.len().try_into().unwrap();
            let result = call(&mut len, surface_formats_buf.as_mut_ptr() as *mut _)?;
            surface_formats.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
    pub unsafe fn get_physical_device_surface_present_modes_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        mut present_modes: impl ExtendUninit<PresentModeKHR>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |present_mode_count, present_modes| {
                let result = (self.get_physical_device_surface_present_modes_khr)(
                    physical_device,
                    surface,
                    present_mode_count,
                    present_modes as _,
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
            let present_modes_buf = present_modes.reserve(capacity);
            len = present_modes_buf.len().try_into().unwrap();
            let result = call(&mut len, present_modes_buf.as_mut_ptr() as *mut _)?;
            present_modes.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}

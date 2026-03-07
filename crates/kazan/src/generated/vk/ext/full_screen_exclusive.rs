#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_full_screen_exclusive";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceFullScreenExclusiveInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SurfaceFullScreenExclusiveInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub full_screen_exclusive: FullScreenExclusiveEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SurfaceFullScreenExclusiveInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SurfaceFullScreenExclusiveInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("full_screen_exclusive", &self.full_screen_exclusive)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SurfaceFullScreenExclusiveInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceSurfaceInfo2KHR<'a>>
        for SurfaceFullScreenExclusiveInfoEXT<'a>
    {
    }
    unsafe impl<'a> Extends<SwapchainCreateInfoKHR<'a>> for SurfaceFullScreenExclusiveInfoEXT<'a> {}

    impl Default for SurfaceFullScreenExclusiveInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                full_screen_exclusive: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SurfaceFullScreenExclusiveInfoEXT<'a> {
        pub fn full_screen_exclusive(
            mut self,
            full_screen_exclusive: FullScreenExclusiveEXT,
        ) -> Self {
            self.full_screen_exclusive = full_screen_exclusive;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceFullScreenExclusiveWin32InfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SurfaceFullScreenExclusiveWin32InfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub hmonitor: HMONITOR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SurfaceFullScreenExclusiveWin32InfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SurfaceFullScreenExclusiveWin32InfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("hmonitor", &self.hmonitor)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SurfaceFullScreenExclusiveWin32InfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceSurfaceInfo2KHR<'a>>
        for SurfaceFullScreenExclusiveWin32InfoEXT<'a>
    {
    }
    unsafe impl<'a> Extends<SwapchainCreateInfoKHR<'a>> for SurfaceFullScreenExclusiveWin32InfoEXT<'a> {}

    impl Default for SurfaceFullScreenExclusiveWin32InfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                hmonitor: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SurfaceFullScreenExclusiveWin32InfoEXT<'a> {
        pub fn hmonitor(mut self, hmonitor: HMONITOR) -> Self {
            self.hmonitor = hmonitor;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceCapabilitiesFullScreenExclusiveEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SurfaceCapabilitiesFullScreenExclusiveEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub full_screen_exclusive_supported: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SurfaceCapabilitiesFullScreenExclusiveEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SurfaceCapabilitiesFullScreenExclusiveEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "full_screen_exclusive_supported",
                    &self.full_screen_exclusive_supported,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SurfaceCapabilitiesFullScreenExclusiveEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT;
    }

    unsafe impl<'a> Extends<SurfaceCapabilities2KHR<'a>>
        for SurfaceCapabilitiesFullScreenExclusiveEXT<'a>
    {
    }

    impl Default for SurfaceCapabilitiesFullScreenExclusiveEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                full_screen_exclusive_supported: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SurfaceCapabilitiesFullScreenExclusiveEXT<'a> {
        pub fn full_screen_exclusive_supported(
            mut self,
            full_screen_exclusive_supported: bool,
        ) -> Self {
            self.full_screen_exclusive_supported = full_screen_exclusive_supported.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFullScreenExclusiveEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FullScreenExclusiveEXT(i32);

    impl FullScreenExclusiveEXT {
        pub const DEFAULT_EXT: Self = Self(0);
        pub const ALLOWED_EXT: Self = Self(1);
        pub const DISALLOWED_EXT: Self = Self(2);
        pub const APPLICATION_CONTROLLED_EXT: Self = Self(3);
    }

    impl fmt::Debug for FullScreenExclusiveEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEFAULT_EXT => Some("DEFAULT_EXT"),
                Self::ALLOWED_EXT => Some("ALLOWED_EXT"),
                Self::DISALLOWED_EXT => Some("DISALLOWED_EXT"),
                Self::APPLICATION_CONTROLLED_EXT => Some("APPLICATION_CONTROLLED_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html>
    pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
            p_present_mode_count: *mut u32,
            p_present_modes: *mut PresentModeKHR,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html>
    pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = unsafe extern "system" fn(
        device: Device,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
        p_modes: *mut DeviceGroupPresentModeFlagsKHR,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAcquireFullScreenExclusiveModeEXT.html>
    pub type PFN_vkAcquireFullScreenExclusiveModeEXT =
        unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkReleaseFullScreenExclusiveModeEXT.html>
    pub type PFN_vkReleaseFullScreenExclusiveModeEXT =
        unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> vk::Result;
}

pub struct InstanceFn {
    get_physical_device_surface_present_modes2_ext: PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_surface_present_modes2_ext: transmute(
                    load(c"vkGetPhysicalDeviceSurfacePresentModes2EXT")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html>
    pub unsafe fn get_physical_device_surface_present_modes2_ext<'a>(
        &self,
        physical_device: PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR<'a>,
        mut present_modes: impl ExtendUninit<PresentModeKHR>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |present_mode_count, present_modes| {
                let result = (self.get_physical_device_surface_present_modes2_ext)(
                    physical_device,
                    surface_info,
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

pub struct DeviceFn {
    acquire_full_screen_exclusive_mode_ext: PFN_vkAcquireFullScreenExclusiveModeEXT,
    release_full_screen_exclusive_mode_ext: PFN_vkReleaseFullScreenExclusiveModeEXT,
    get_device_group_surface_present_modes2_ext:
        Option<PFN_vkGetDeviceGroupSurfacePresentModes2EXT>,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                acquire_full_screen_exclusive_mode_ext: transmute(
                    load(c"vkAcquireFullScreenExclusiveModeEXT").ok_or(MissingEntryPointError)?,
                ),
                release_full_screen_exclusive_mode_ext: transmute(
                    load(c"vkReleaseFullScreenExclusiveModeEXT").ok_or(MissingEntryPointError)?,
                ),
                get_device_group_surface_present_modes2_ext: transmute(load(
                    c"vkGetDeviceGroupSurfacePresentModes2EXT",
                )),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAcquireFullScreenExclusiveModeEXT.html>
    pub unsafe fn acquire_full_screen_exclusive_mode_ext(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.acquire_full_screen_exclusive_mode_ext)(device, swapchain);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkReleaseFullScreenExclusiveModeEXT.html>
    pub unsafe fn release_full_screen_exclusive_mode_ext(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.release_full_screen_exclusive_mode_ext)(device, swapchain);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html>
    pub unsafe fn get_device_group_surface_present_modes2_ext(
        &self,
        device: Device,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR<'_>,
    ) -> crate::Result<DeviceGroupPresentModeFlagsKHR> {
        unsafe {
            let mut modes = core::mem::MaybeUninit::uninit();
            let result = (self.get_device_group_surface_present_modes2_ext.unwrap())(
                device,
                surface_info,
                modes.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(modes.assume_init()),
                err => Err(err),
            }
        }
    }
}

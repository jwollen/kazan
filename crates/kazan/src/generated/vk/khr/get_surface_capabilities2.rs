#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_get_surface_capabilities2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSurfaceInfo2KHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceSurfaceInfo2KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub surface: SurfaceKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceSurfaceInfo2KHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceSurfaceInfo2KHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("surface", &self.surface)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSurfaceInfo2KHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_SURFACE_INFO_2_KHR;
    }

    impl Default for PhysicalDeviceSurfaceInfo2KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                surface: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceSurfaceInfo2KHR<'a> {
        pub fn surface(mut self, surface: SurfaceKHR) -> Self {
            self.surface = surface;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceCapabilities2KHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SurfaceCapabilities2KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub surface_capabilities: SurfaceCapabilitiesKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SurfaceCapabilities2KHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SurfaceCapabilities2KHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("surface_capabilities", &self.surface_capabilities)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SurfaceCapabilities2KHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SURFACE_CAPABILITIES_2_KHR;
    }

    impl Default for SurfaceCapabilities2KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                surface_capabilities: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SurfaceCapabilities2KHR<'a> {
        pub fn surface_capabilities(
            mut self,
            surface_capabilities: SurfaceCapabilitiesKHR,
        ) -> Self {
            self.surface_capabilities = surface_capabilities;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceFormat2KHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SurfaceFormat2KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub surface_format: SurfaceFormatKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SurfaceFormat2KHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SurfaceFormat2KHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("surface_format", &self.surface_format)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SurfaceFormat2KHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SURFACE_FORMAT_2_KHR;
    }

    impl Default for SurfaceFormat2KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                surface_format: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SurfaceFormat2KHR<'a> {
        pub fn surface_format(mut self, surface_format: SurfaceFormatKHR) -> Self {
            self.surface_format = surface_format;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html>
    pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
            p_surface_capabilities: *mut SurfaceCapabilities2KHR<'_>,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html>
    pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut SurfaceFormat2KHR<'_>,
    )
        -> vk::Result;
}

pub struct InstanceFn {
    get_physical_device_surface_capabilities2_khr: PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR,
    get_physical_device_surface_formats2_khr: PFN_vkGetPhysicalDeviceSurfaceFormats2KHR,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_surface_capabilities2_khr: transmute(
                    load(c"vkGetPhysicalDeviceSurfaceCapabilities2KHR")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_surface_formats2_khr: transmute(
                    load(c"vkGetPhysicalDeviceSurfaceFormats2KHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html>
    pub unsafe fn get_physical_device_surface_capabilities2_khr(
        &self,
        physical_device: PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR<'_>,
        surface_capabilities: &mut SurfaceCapabilities2KHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_physical_device_surface_capabilities2_khr)(
                physical_device,
                surface_info,
                surface_capabilities,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html>
    pub unsafe fn get_physical_device_surface_formats2_khr<'a>(
        &self,
        physical_device: PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR<'a>,
        mut surface_formats: impl ExtendUninit<SurfaceFormat2KHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |surface_format_count, surface_formats| {
                let result = (self.get_physical_device_surface_formats2_khr)(
                    physical_device,
                    surface_info,
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
}

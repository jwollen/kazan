//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_SEC_ubm_surface.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_SEC_ubm_surface";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkUbmSurfaceCreateInfoSEC.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct UbmSurfaceCreateInfoSEC<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: UbmSurfaceCreateFlagsSEC,
        pub device: *mut ubm_device,
        pub surface: *mut ubm_surface,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for UbmSurfaceCreateInfoSEC<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("UbmSurfaceCreateInfoSEC")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("device", &self.device)
                .field("surface", &self.surface)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for UbmSurfaceCreateInfoSEC<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::UBM_SURFACE_CREATE_INFO_SEC;
    }

    impl Default for UbmSurfaceCreateInfoSEC<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                device: ptr::null_mut(),
                surface: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> UbmSurfaceCreateInfoSEC<'a> {
        #[inline]
        pub fn flags(mut self, flags: UbmSurfaceCreateFlagsSEC) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn device(mut self, device: *mut ubm_device) -> Self {
            self.device = device;
            self
        }

        #[inline]
        pub fn surface(mut self, surface: *mut ubm_surface) -> Self {
            self.surface = surface;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkUbmSurfaceCreateFlagsSEC.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct UbmSurfaceCreateFlagsSEC(Flags);
    vk_bitflags_wrapped!(UbmSurfaceCreateFlagsSEC, Flags);

    impl fmt::Debug for UbmSurfaceCreateFlagsSEC {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateUbmSurfaceSEC.html>
    pub type PFN_vkCreateUbmSurfaceSEC = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const UbmSurfaceCreateInfoSEC<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceUbmPresentationSupportSEC.html>
    pub type PFN_vkGetPhysicalDeviceUbmPresentationSupportSEC = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        device: *mut ubm_device,
    )
        -> Bool32;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkUbmSurfaceCreateInfoSEC = UbmSurfaceCreateInfoSEC<'static>;
    pub type VkUbmSurfaceCreateFlagsSEC = UbmSurfaceCreateFlagsSEC;
    impl UbmSurfaceCreateInfoSEC<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkUbmSurfaceCreateInfoSEC {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    create_ubm_surface_sec: PFN_vkCreateUbmSurfaceSEC,
    get_physical_device_ubm_presentation_support_sec:
        PFN_vkGetPhysicalDeviceUbmPresentationSupportSEC,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_ubm_surface_sec: transmute(
                    load(c"vkCreateUbmSurfaceSEC").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_ubm_presentation_support_sec: transmute(
                    load(c"vkGetPhysicalDeviceUbmPresentationSupportSEC")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateUbmSurfaceSEC.html>
    #[inline]
    pub unsafe fn create_ubm_surface_sec(
        &self,
        instance: Instance,
        create_info: &UbmSurfaceCreateInfoSEC<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_ubm_surface_sec)(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceUbmPresentationSupportSEC.html>
    #[inline]
    pub unsafe fn get_physical_device_ubm_presentation_support_sec(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        device: *mut ubm_device,
    ) -> bool {
        unsafe {
            (self.get_physical_device_ubm_presentation_support_sec)(
                physical_device,
                queue_family_index,
                device,
            ) != 0
        }
    }
}

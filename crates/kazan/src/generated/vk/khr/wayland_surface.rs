//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_wayland_surface.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_wayland_surface";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkWaylandSurfaceCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct WaylandSurfaceCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: WaylandSurfaceCreateFlagsKHR,
        pub display: *mut wl_display,
        pub surface: *mut wl_surface,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for WaylandSurfaceCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("WaylandSurfaceCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("display", &self.display)
                .field("surface", &self.surface)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for WaylandSurfaceCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::WAYLAND_SURFACE_CREATE_INFO_KHR;
    }

    impl Default for WaylandSurfaceCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                display: ptr::null_mut(),
                surface: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> WaylandSurfaceCreateInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: WaylandSurfaceCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn display(mut self, display: *mut wl_display) -> Self {
            self.display = display;
            self
        }

        #[inline]
        pub fn surface(mut self, surface: *mut wl_surface) -> Self {
            self.surface = surface;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkWaylandSurfaceCreateFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct WaylandSurfaceCreateFlagsKHR(Flags);
    vk_bitflags_wrapped!(WaylandSurfaceCreateFlagsKHR, Flags);

    impl fmt::Debug for WaylandSurfaceCreateFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateWaylandSurfaceKHR.html>
    pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const WaylandSurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html>
    pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
            display: *mut wl_display,
        ) -> Bool32;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkWaylandSurfaceCreateInfoKHR = WaylandSurfaceCreateInfoKHR<'static>;
    pub type VkWaylandSurfaceCreateFlagsKHR = WaylandSurfaceCreateFlagsKHR;
    impl WaylandSurfaceCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkWaylandSurfaceCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    create_wayland_surface_khr: PFN_vkCreateWaylandSurfaceKHR,
    get_physical_device_wayland_presentation_support_khr:
        PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_wayland_surface_khr: transmute(
                    load(c"vkCreateWaylandSurfaceKHR").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_wayland_presentation_support_khr: transmute(
                    load(c"vkGetPhysicalDeviceWaylandPresentationSupportKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateWaylandSurfaceKHR.html>
    #[inline]
    pub unsafe fn create_wayland_surface_khr(
        &self,
        instance: Instance,
        create_info: &WaylandSurfaceCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_wayland_surface_khr)(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_wayland_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        display: *mut wl_display,
    ) -> bool {
        unsafe {
            (self.get_physical_device_wayland_presentation_support_khr)(
                physical_device,
                queue_family_index,
                display,
            ) != 0
        }
    }
}

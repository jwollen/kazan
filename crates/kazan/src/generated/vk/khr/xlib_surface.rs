//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_xlib_surface.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_xlib_surface";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkXlibSurfaceCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct XlibSurfaceCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: XlibSurfaceCreateFlagsKHR,
        pub dpy: *mut Display,
        pub window: Window,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for XlibSurfaceCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("XlibSurfaceCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("dpy", &self.dpy)
                .field("window", &self.window)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for XlibSurfaceCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::XLIB_SURFACE_CREATE_INFO_KHR;
    }

    impl Default for XlibSurfaceCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                dpy: ptr::null_mut(),
                window: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> XlibSurfaceCreateInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: XlibSurfaceCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn dpy(mut self, dpy: *mut Display) -> Self {
            self.dpy = dpy;
            self
        }

        #[inline]
        pub fn window(mut self, window: Window) -> Self {
            self.window = window;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkXlibSurfaceCreateFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct XlibSurfaceCreateFlagsKHR(Flags);
    vk_bitflags_wrapped!(XlibSurfaceCreateFlagsKHR, Flags);

    impl fmt::Debug for XlibSurfaceCreateFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateXlibSurfaceKHR.html>
    pub type PFN_vkCreateXlibSurfaceKHR = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const XlibSurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html>
    pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
            dpy: *mut Display,
            visual_id: VisualID,
        ) -> Bool32;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkXlibSurfaceCreateInfoKHR = XlibSurfaceCreateInfoKHR<'static>;
    pub type VkXlibSurfaceCreateFlagsKHR = XlibSurfaceCreateFlagsKHR;
    impl XlibSurfaceCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkXlibSurfaceCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    create_xlib_surface_khr: PFN_vkCreateXlibSurfaceKHR,
    get_physical_device_xlib_presentation_support_khr:
        PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_xlib_surface_khr: transmute(
                    load(c"vkCreateXlibSurfaceKHR").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_xlib_presentation_support_khr: transmute(
                    load(c"vkGetPhysicalDeviceXlibPresentationSupportKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateXlibSurfaceKHR.html>
    #[inline]
    pub unsafe fn create_xlib_surface_khr(
        &self,
        instance: Instance,
        create_info: &XlibSurfaceCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_xlib_surface_khr)(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceXlibPresentationSupportKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_xlib_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dpy: *mut Display,
        visual_id: VisualID,
    ) -> bool {
        unsafe {
            (self.get_physical_device_xlib_presentation_support_khr)(
                physical_device,
                queue_family_index,
                dpy,
                visual_id,
            ) != 0
        }
    }
}

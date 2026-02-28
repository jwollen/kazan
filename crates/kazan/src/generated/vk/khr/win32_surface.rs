#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Win32SurfaceCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: Win32SurfaceCreateFlagsKHR,
        pub hinstance: HINSTANCE,
        pub hwnd: HWND,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for Win32SurfaceCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
                p_next: core::ptr::null(),
                flags: Default::default(),
                hinstance: Default::default(),
                hwnd: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> Win32SurfaceCreateInfoKHR<'a> {
        pub fn flags(mut self, flags: Win32SurfaceCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn hinstance(mut self, hinstance: HINSTANCE) -> Self {
            self.hinstance = hinstance;
            self
        }
        pub fn hwnd(mut self, hwnd: HWND) -> Self {
            self.hwnd = hwnd;
            self
        }
    }
    bitflags! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct Win32SurfaceCreateFlagsKHR: Flags {
        }
    }
    pub type PFN_vkCreateWin32SurfaceKHR = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const Win32SurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
    pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
        ) -> Bool32;
}
pub struct InstanceFn {
    create_win32_surface_khr: PFN_vkCreateWin32SurfaceKHR,
    get_physical_device_win32_presentation_support_khr:
        PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_win32_surface_khr: transmute(
                    load(c"vkCreateWin32SurfaceKHR").ok_or(LoadingError)?,
                ),
                get_physical_device_win32_presentation_support_khr: transmute(
                    load(c"vkGetPhysicalDeviceWin32PresentationSupportKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn create_win32_surface_khr(
        &self,
        instance: Instance,
        create_info: &Win32SurfaceCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_win32_surface_khr)(
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
    pub unsafe fn get_physical_device_win32_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> Bool32 {
        unsafe {
            (self.get_physical_device_win32_presentation_support_khr)(
                physical_device,
                queue_family_index,
            )
        }
    }
}

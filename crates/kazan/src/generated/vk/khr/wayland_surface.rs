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
    pub struct WaylandSurfaceCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: WaylandSurfaceCreateFlagsKHR,
        pub display: *mut wl_display,
        pub surface: *mut wl_surface,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for WaylandSurfaceCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::WAYLAND_SURFACE_CREATE_INFO_KHR,
                p_next: core::ptr::null(),
                flags: Default::default(),
                display: core::ptr::null_mut(),
                surface: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> WaylandSurfaceCreateInfoKHR<'a> {
        pub fn flags(mut self, flags: WaylandSurfaceCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn display(mut self, display: *mut wl_display) -> Self {
            self.display = display;
            self
        }
        pub fn surface(mut self, surface: *mut wl_surface) -> Self {
            self.surface = surface;
            self
        }
    }
    bitflags! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct WaylandSurfaceCreateFlagsKHR: Flags {
        }
    }
    pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const WaylandSurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
    pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
            display: *mut wl_display,
        ) -> Bool32;
}
pub struct InstanceFn {
    create_wayland_surface_khr: PFN_vkCreateWaylandSurfaceKHR,
    get_physical_device_wayland_presentation_support_khr:
        PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_wayland_surface_khr: transmute(
                    load(c"vkCreateWaylandSurfaceKHR").ok_or(LoadingError)?,
                ),
                get_physical_device_wayland_presentation_support_khr: transmute(
                    load(c"vkGetPhysicalDeviceWaylandPresentationSupportKHR")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
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
    pub unsafe fn get_physical_device_wayland_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        display: *mut wl_display,
    ) -> Bool32 {
        unsafe {
            (self.get_physical_device_wayland_presentation_support_khr)(
                physical_device,
                queue_family_index,
                display,
            )
        }
    }
}

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
    pub struct XlibSurfaceCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: XlibSurfaceCreateFlagsKHR,
        pub dpy: *mut Display,
        pub window: Window,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for XlibSurfaceCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::XLIB_SURFACE_CREATE_INFO_KHR;
    }
    impl Default for XlibSurfaceCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                dpy: core::ptr::null_mut(),
                window: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> XlibSurfaceCreateInfoKHR<'a> {
        pub fn flags(mut self, flags: XlibSurfaceCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn dpy(mut self, dpy: *mut Display) -> Self {
            self.dpy = dpy;
            self
        }
        pub fn window(mut self, window: Window) -> Self {
            self.window = window;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct XlibSurfaceCreateFlagsKHR(Flags);
    impl XlibSurfaceCreateFlagsKHR {}
    pub type PFN_vkCreateXlibSurfaceKHR = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const XlibSurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
    pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
            dpy: *mut Display,
            visual_id: VisualID,
        ) -> Bool32;
}
pub struct InstanceFn {
    create_xlib_surface_khr: PFN_vkCreateXlibSurfaceKHR,
    get_physical_device_xlib_presentation_support_khr:
        PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_xlib_surface_khr: transmute(
                    load(c"vkCreateXlibSurfaceKHR").ok_or(LoadingError)?,
                ),
                get_physical_device_xlib_presentation_support_khr: transmute(
                    load(c"vkGetPhysicalDeviceXlibPresentationSupportKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
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
    pub unsafe fn get_physical_device_xlib_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dpy: *mut Display,
        visual_id: VisualID,
    ) -> Bool32 {
        unsafe {
            (self.get_physical_device_xlib_presentation_support_khr)(
                physical_device,
                queue_family_index,
                dpy,
                visual_id,
            )
        }
    }
}

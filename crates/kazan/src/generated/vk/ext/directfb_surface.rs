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
    pub struct DirectFBSurfaceCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DirectFBSurfaceCreateFlagsEXT,
        pub dfb: *mut IDirectFB,
        pub surface: *mut IDirectFBSurface,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DirectFBSurfaceCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DIRECTFB_SURFACE_CREATE_INFO_EXT;
    }
    impl Default for DirectFBSurfaceCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                dfb: core::ptr::null_mut(),
                surface: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DirectFBSurfaceCreateInfoEXT<'a> {
        pub fn flags(mut self, flags: DirectFBSurfaceCreateFlagsEXT) -> Self {
            self.flags = flags;
            self
        }
        pub fn dfb(mut self, dfb: *mut IDirectFB) -> Self {
            self.dfb = dfb;
            self
        }
        pub fn surface(mut self, surface: *mut IDirectFBSurface) -> Self {
            self.surface = surface;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DirectFBSurfaceCreateFlagsEXT(Flags);
    impl DirectFBSurfaceCreateFlagsEXT {}
    pub type PFN_vkCreateDirectFBSurfaceEXT = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const DirectFBSurfaceCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
    pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
            dfb: *mut IDirectFB,
        ) -> Bool32;
}
pub struct InstanceFn {
    create_direct_fb_surface_ext: PFN_vkCreateDirectFBSurfaceEXT,
    get_physical_device_direct_fb_presentation_support_ext:
        PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_direct_fb_surface_ext: transmute(
                    load(c"vkCreateDirectFBSurfaceEXT").ok_or(LoadingError)?,
                ),
                get_physical_device_direct_fb_presentation_support_ext: transmute(
                    load(c"vkGetPhysicalDeviceDirectFBPresentationSupportEXT")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn create_direct_fb_surface_ext(
        &self,
        instance: Instance,
        create_info: &DirectFBSurfaceCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_direct_fb_surface_ext)(
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
    pub unsafe fn get_physical_device_direct_fb_presentation_support_ext(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dfb: *mut IDirectFB,
    ) -> Bool32 {
        unsafe {
            (self.get_physical_device_direct_fb_presentation_support_ext)(
                physical_device,
                queue_family_index,
                dfb,
            )
        }
    }
}

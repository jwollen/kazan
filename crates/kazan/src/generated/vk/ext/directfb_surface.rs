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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDirectFBSurfaceCreateInfoEXT.html>
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

    impl fmt::Debug for DirectFBSurfaceCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DirectFBSurfaceCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("dfb", &self.dfb)
                .field("surface", &self.surface)
                .finish()
        }
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDirectFBSurfaceCreateFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DirectFBSurfaceCreateFlagsEXT(Flags);
    vk_bitflags_wrapped!(DirectFBSurfaceCreateFlagsEXT, Flags);

    impl fmt::Debug for DirectFBSurfaceCreateFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateDirectFBSurfaceEXT.html>
    pub type PFN_vkCreateDirectFBSurfaceEXT = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const DirectFBSurfaceCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html>
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
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_direct_fb_surface_ext: transmute(
                    load(c"vkCreateDirectFBSurfaceEXT").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_direct_fb_presentation_support_ext: transmute(
                    load(c"vkGetPhysicalDeviceDirectFBPresentationSupportEXT")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateDirectFBSurfaceEXT.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html>
    pub unsafe fn get_physical_device_direct_fb_presentation_support_ext(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dfb: *mut IDirectFB,
    ) -> bool {
        unsafe {
            (self.get_physical_device_direct_fb_presentation_support_ext)(
                physical_device,
                queue_family_index,
                dfb,
            ) != 0
        }
    }
}

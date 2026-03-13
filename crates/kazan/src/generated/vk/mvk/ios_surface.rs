//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_MVK_ios_surface.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_MVK_ios_surface";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIOSSurfaceCreateInfoMVK.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct IOSSurfaceCreateInfoMVK<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: IOSSurfaceCreateFlagsMVK,
        pub p_view: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for IOSSurfaceCreateInfoMVK<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("IOSSurfaceCreateInfoMVK")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("p_view", &self.p_view)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for IOSSurfaceCreateInfoMVK<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IOS_SURFACE_CREATE_INFO_MVK;
    }

    impl Default for IOSSurfaceCreateInfoMVK<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                p_view: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> IOSSurfaceCreateInfoMVK<'a> {
        #[inline]
        pub fn flags(mut self, flags: IOSSurfaceCreateFlagsMVK) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn view(mut self, view: *const c_void) -> Self {
            self.p_view = view;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIOSSurfaceCreateFlagsMVK.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct IOSSurfaceCreateFlagsMVK(Flags);
    vk_bitflags_wrapped!(IOSSurfaceCreateFlagsMVK, Flags);

    impl fmt::Debug for IOSSurfaceCreateFlagsMVK {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateIOSSurfaceMVK.html>
    pub type PFN_vkCreateIOSSurfaceMVK = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const IOSSurfaceCreateInfoMVK<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkIOSSurfaceCreateInfoMVK = IOSSurfaceCreateInfoMVK<'static>;
    pub type VkIOSSurfaceCreateFlagsMVK = IOSSurfaceCreateFlagsMVK;
    impl IOSSurfaceCreateInfoMVK<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkIOSSurfaceCreateInfoMVK {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    create_ios_surface: PFN_vkCreateIOSSurfaceMVK,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_ios_surface: transmute(
                    load(c"vkCreateIOSSurfaceMVK").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateIOSSurfaceMVK.html>
    #[inline]
    pub unsafe fn create_ios_surface(
        &self,
        instance: Instance,
        create_info: &IOSSurfaceCreateInfoMVK<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_ios_surface)(
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
}

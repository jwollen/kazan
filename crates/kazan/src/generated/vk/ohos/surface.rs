//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_OHOS_surface.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_OHOS_surface";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    pub type OHNativeWindow = *const c_void;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceCreateInfoOHOS.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SurfaceCreateInfoOHOS<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: SurfaceCreateFlagsOHOS,
        pub window: *mut OHNativeWindow,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SurfaceCreateInfoOHOS<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SurfaceCreateInfoOHOS")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("window", &self.window)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SurfaceCreateInfoOHOS<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SURFACE_CREATE_INFO_OHOS;
    }

    impl Default for SurfaceCreateInfoOHOS<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                window: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SurfaceCreateInfoOHOS<'a> {
        #[inline]
        pub fn flags(mut self, flags: SurfaceCreateFlagsOHOS) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn window(mut self, window: *mut OHNativeWindow) -> Self {
            self.window = window;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceCreateFlagsOHOS.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SurfaceCreateFlagsOHOS(Flags);
    vk_bitflags_wrapped!(SurfaceCreateFlagsOHOS, Flags);

    impl fmt::Debug for SurfaceCreateFlagsOHOS {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateSurfaceOHOS.html>
    pub type PFN_vkCreateSurfaceOHOS = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const SurfaceCreateInfoOHOS<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkSurfaceCreateInfoOHOS = SurfaceCreateInfoOHOS<'static>;
    pub type VkSurfaceCreateFlagsOHOS = SurfaceCreateFlagsOHOS;
    impl SurfaceCreateInfoOHOS<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSurfaceCreateInfoOHOS {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    create_surface: PFN_vkCreateSurfaceOHOS,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_surface: transmute(
                    load(c"vkCreateSurfaceOHOS").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateSurfaceOHOS.html>
    #[inline]
    pub unsafe fn create_surface(
        &self,
        instance: Instance,
        create_info: &SurfaceCreateInfoOHOS<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_surface)(
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

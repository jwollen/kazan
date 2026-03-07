#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_headless_surface";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkHeadlessSurfaceCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct HeadlessSurfaceCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: HeadlessSurfaceCreateFlagsEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for HeadlessSurfaceCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("HeadlessSurfaceCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for HeadlessSurfaceCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::HEADLESS_SURFACE_CREATE_INFO_EXT;
    }

    impl Default for HeadlessSurfaceCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> HeadlessSurfaceCreateInfoEXT<'a> {
        #[inline]
        pub fn flags(mut self, flags: HeadlessSurfaceCreateFlagsEXT) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkHeadlessSurfaceCreateFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct HeadlessSurfaceCreateFlagsEXT(Flags);
    vk_bitflags_wrapped!(HeadlessSurfaceCreateFlagsEXT, Flags);

    impl fmt::Debug for HeadlessSurfaceCreateFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateHeadlessSurfaceEXT.html>
    pub type PFN_vkCreateHeadlessSurfaceEXT = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const HeadlessSurfaceCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
}

pub struct InstanceFn {
    create_headless_surface_ext: PFN_vkCreateHeadlessSurfaceEXT,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_headless_surface_ext: transmute(
                    load(c"vkCreateHeadlessSurfaceEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateHeadlessSurfaceEXT.html>
    #[inline]
    pub unsafe fn create_headless_surface_ext(
        &self,
        instance: Instance,
        create_info: &HeadlessSurfaceCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_headless_surface_ext)(
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

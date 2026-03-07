//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_QNX_screen_surface.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_QNX_screen_surface";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkScreenSurfaceCreateInfoQNX.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ScreenSurfaceCreateInfoQNX<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: ScreenSurfaceCreateFlagsQNX,
        pub context: *mut _screen_context,
        pub window: *mut _screen_window,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ScreenSurfaceCreateInfoQNX<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ScreenSurfaceCreateInfoQNX")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("context", &self.context)
                .field("window", &self.window)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ScreenSurfaceCreateInfoQNX<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SCREEN_SURFACE_CREATE_INFO_QNX;
    }

    impl Default for ScreenSurfaceCreateInfoQNX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                context: core::ptr::null_mut(),
                window: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ScreenSurfaceCreateInfoQNX<'a> {
        #[inline]
        pub fn flags(mut self, flags: ScreenSurfaceCreateFlagsQNX) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn context(mut self, context: *mut _screen_context) -> Self {
            self.context = context;
            self
        }

        #[inline]
        pub fn window(mut self, window: *mut _screen_window) -> Self {
            self.window = window;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkScreenSurfaceCreateFlagsQNX.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ScreenSurfaceCreateFlagsQNX(Flags);
    vk_bitflags_wrapped!(ScreenSurfaceCreateFlagsQNX, Flags);

    impl fmt::Debug for ScreenSurfaceCreateFlagsQNX {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateScreenSurfaceQNX.html>
    pub type PFN_vkCreateScreenSurfaceQNX = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const ScreenSurfaceCreateInfoQNX<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceScreenPresentationSupportQNX.html>
    pub type PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
            window: *mut _screen_window,
        ) -> Bool32;
}

pub struct InstanceFn {
    create_screen_surface_qnx: PFN_vkCreateScreenSurfaceQNX,
    get_physical_device_screen_presentation_support_qnx:
        PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_screen_surface_qnx: transmute(
                    load(c"vkCreateScreenSurfaceQNX").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_screen_presentation_support_qnx: transmute(
                    load(c"vkGetPhysicalDeviceScreenPresentationSupportQNX")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateScreenSurfaceQNX.html>
    #[inline]
    pub unsafe fn create_screen_surface_qnx(
        &self,
        instance: Instance,
        create_info: &ScreenSurfaceCreateInfoQNX<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_screen_surface_qnx)(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceScreenPresentationSupportQNX.html>
    #[inline]
    pub unsafe fn get_physical_device_screen_presentation_support_qnx(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        window: *mut _screen_window,
    ) -> bool {
        unsafe {
            (self.get_physical_device_screen_presentation_support_qnx)(
                physical_device,
                queue_family_index,
                window,
            ) != 0
        }
    }
}

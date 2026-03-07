#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_xcb_surface";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkXcbSurfaceCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct XcbSurfaceCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: XcbSurfaceCreateFlagsKHR,
        pub connection: *mut xcb_connection_t,
        pub window: xcb_window_t,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for XcbSurfaceCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("XcbSurfaceCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("connection", &self.connection)
                .field("window", &self.window)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for XcbSurfaceCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::XCB_SURFACE_CREATE_INFO_KHR;
    }

    impl Default for XcbSurfaceCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                connection: core::ptr::null_mut(),
                window: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> XcbSurfaceCreateInfoKHR<'a> {
        pub fn flags(mut self, flags: XcbSurfaceCreateFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        pub fn connection(mut self, connection: *mut xcb_connection_t) -> Self {
            self.connection = connection;
            self
        }

        pub fn window(mut self, window: xcb_window_t) -> Self {
            self.window = window;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkXcbSurfaceCreateFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct XcbSurfaceCreateFlagsKHR(Flags);
    vk_bitflags_wrapped!(XcbSurfaceCreateFlagsKHR, Flags);

    impl fmt::Debug for XcbSurfaceCreateFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateXcbSurfaceKHR.html>
    pub type PFN_vkCreateXcbSurfaceKHR = unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const XcbSurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html>
    pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    )
        -> Bool32;
}

pub struct InstanceFn {
    create_xcb_surface_khr: PFN_vkCreateXcbSurfaceKHR,
    get_physical_device_xcb_presentation_support_khr:
        PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_xcb_surface_khr: transmute(
                    load(c"vkCreateXcbSurfaceKHR").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_xcb_presentation_support_khr: transmute(
                    load(c"vkGetPhysicalDeviceXcbPresentationSupportKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateXcbSurfaceKHR.html>
    pub unsafe fn create_xcb_surface_khr(
        &self,
        instance: Instance,
        create_info: &XcbSurfaceCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_xcb_surface_khr)(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceXcbPresentationSupportKHR.html>
    pub unsafe fn get_physical_device_xcb_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) -> bool {
        unsafe {
            (self.get_physical_device_xcb_presentation_support_khr)(
                physical_device,
                queue_family_index,
                connection,
                visual_id,
            ) != 0
        }
    }
}

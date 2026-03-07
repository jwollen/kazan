#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_AMD_display_native_hdr";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayNativeHdrSurfaceCapabilitiesAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DisplayNativeHdrSurfaceCapabilitiesAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub local_dimming_support: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DisplayNativeHdrSurfaceCapabilitiesAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DisplayNativeHdrSurfaceCapabilitiesAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("local_dimming_support", &self.local_dimming_support)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DisplayNativeHdrSurfaceCapabilitiesAMD<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD;
    }

    unsafe impl<'a> Extends<SurfaceCapabilities2KHR<'a>>
        for DisplayNativeHdrSurfaceCapabilitiesAMD<'a>
    {
    }

    impl Default for DisplayNativeHdrSurfaceCapabilitiesAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                local_dimming_support: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DisplayNativeHdrSurfaceCapabilitiesAMD<'a> {
        #[inline]
        pub fn local_dimming_support(mut self, local_dimming_support: bool) -> Self {
            self.local_dimming_support = local_dimming_support.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainDisplayNativeHdrCreateInfoAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SwapchainDisplayNativeHdrCreateInfoAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub local_dimming_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SwapchainDisplayNativeHdrCreateInfoAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SwapchainDisplayNativeHdrCreateInfoAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("local_dimming_enable", &self.local_dimming_enable)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SwapchainDisplayNativeHdrCreateInfoAMD<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD;
    }

    unsafe impl<'a> Extends<SwapchainCreateInfoKHR<'a>> for SwapchainDisplayNativeHdrCreateInfoAMD<'a> {}

    impl Default for SwapchainDisplayNativeHdrCreateInfoAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                local_dimming_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SwapchainDisplayNativeHdrCreateInfoAMD<'a> {
        #[inline]
        pub fn local_dimming_enable(mut self, local_dimming_enable: bool) -> Self {
            self.local_dimming_enable = local_dimming_enable.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetLocalDimmingAMD.html>
    pub type PFN_vkSetLocalDimmingAMD = unsafe extern "system" fn(
        device: Device,
        swap_chain: SwapchainKHR,
        local_dimming_enable: Bool32,
    );
}

pub struct DeviceFn {
    set_local_dimming_amd: PFN_vkSetLocalDimmingAMD,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                set_local_dimming_amd: transmute(
                    load(c"vkSetLocalDimmingAMD").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetLocalDimmingAMD.html>
    #[inline]
    pub unsafe fn set_local_dimming_amd(
        &self,
        device: Device,
        swap_chain: SwapchainKHR,
        local_dimming_enable: bool,
    ) {
        unsafe { (self.set_local_dimming_amd)(device, swap_chain, local_dimming_enable.into()) }
    }
}

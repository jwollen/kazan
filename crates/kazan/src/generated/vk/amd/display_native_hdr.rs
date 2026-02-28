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
    pub struct DisplayNativeHdrSurfaceCapabilitiesAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub local_dimming_support: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for DisplayNativeHdrSurfaceCapabilitiesAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD,
                p_next: core::ptr::null_mut(),
                local_dimming_support: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DisplayNativeHdrSurfaceCapabilitiesAMD<'a> {
        pub fn local_dimming_support(mut self, local_dimming_support: Bool32) -> Self {
            self.local_dimming_support = local_dimming_support;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SwapchainDisplayNativeHdrCreateInfoAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub local_dimming_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for SwapchainDisplayNativeHdrCreateInfoAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD,
                p_next: core::ptr::null(),
                local_dimming_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SwapchainDisplayNativeHdrCreateInfoAMD<'a> {
        pub fn local_dimming_enable(mut self, local_dimming_enable: Bool32) -> Self {
            self.local_dimming_enable = local_dimming_enable;
            self
        }
    }
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
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                set_local_dimming_amd: transmute(
                    load(c"vkSetLocalDimmingAMD").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn set_local_dimming_amd(
        &self,
        device: Device,
        swap_chain: SwapchainKHR,
        local_dimming_enable: Bool32,
    ) {
        unsafe { (self.set_local_dimming_amd)(device, swap_chain, local_dimming_enable) }
    }
}

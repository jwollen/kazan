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
    pub struct PhysicalDeviceInvocationMaskFeaturesHUAWEI<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub invocation_mask: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceInvocationMaskFeaturesHUAWEI<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI,
                p_next: core::ptr::null_mut(),
                invocation_mask: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceInvocationMaskFeaturesHUAWEI<'a> {
        pub fn invocation_mask(mut self, invocation_mask: Bool32) -> Self {
            self.invocation_mask = invocation_mask;
            self
        }
    }
    pub type PFN_vkCmdBindInvocationMaskHUAWEI = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        image_view: ImageView,
        image_layout: ImageLayout,
    );
}
pub struct DeviceFn {
    cmd_bind_invocation_mask_huawei: PFN_vkCmdBindInvocationMaskHUAWEI,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_bind_invocation_mask_huawei: transmute(
                    load(c"vkCmdBindInvocationMaskHUAWEI").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_bind_invocation_mask_huawei(
        &self,
        command_buffer: CommandBuffer,
        image_view: ImageView,
        image_layout: ImageLayout,
    ) {
        unsafe { (self.cmd_bind_invocation_mask_huawei)(command_buffer, image_view, image_layout) }
    }
}

//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_INTEL_device_info.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_INTEL_device_info";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceInfoPropertiesINTEL.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceInfoPropertiesINTEL<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub device_ip_version_arch: u32,
        pub device_ip_version_release: u32,
        pub device_ip_version_revision: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceInfoPropertiesINTEL<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceInfoPropertiesINTEL")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("device_ip_version_arch", &self.device_ip_version_arch)
                .field("device_ip_version_release", &self.device_ip_version_release)
                .field(
                    "device_ip_version_revision",
                    &self.device_ip_version_revision,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceInfoPropertiesINTEL<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_INFO_PROPERTIES_INTEL;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>> for PhysicalDeviceInfoPropertiesINTEL<'_> {}

    impl Default for PhysicalDeviceInfoPropertiesINTEL<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                device_ip_version_arch: Default::default(),
                device_ip_version_release: Default::default(),
                device_ip_version_revision: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceInfoPropertiesINTEL<'a> {
        #[inline]
        pub fn device_ip_version_arch(mut self, device_ip_version_arch: u32) -> Self {
            self.device_ip_version_arch = device_ip_version_arch;
            self
        }

        #[inline]
        pub fn device_ip_version_release(mut self, device_ip_version_release: u32) -> Self {
            self.device_ip_version_release = device_ip_version_release;
            self
        }

        #[inline]
        pub fn device_ip_version_revision(mut self, device_ip_version_revision: u32) -> Self {
            self.device_ip_version_revision = device_ip_version_revision;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceInfoPropertiesINTEL = PhysicalDeviceInfoPropertiesINTEL<'static>;
    impl PhysicalDeviceInfoPropertiesINTEL<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceInfoPropertiesINTEL {
            unsafe { core::mem::transmute(self) }
        }
    }
}

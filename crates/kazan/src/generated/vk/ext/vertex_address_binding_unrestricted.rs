//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_vertex_address_binding_unrestricted.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_vertex_address_binding_unrestricted";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVertexAddressBindingUnrestrictedFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceVertexAddressBindingUnrestrictedFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub vertex_address_binding_unrestricted: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceVertexAddressBindingUnrestrictedFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVertexAddressBindingUnrestrictedFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "vertex_address_binding_unrestricted",
                    &self.vertex_address_binding_unrestricted,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDeviceVertexAddressBindingUnrestrictedFeaturesEXT<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_VERTEX_ADDRESS_BINDING_UNRESTRICTED_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceVertexAddressBindingUnrestrictedFeaturesEXT<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceVertexAddressBindingUnrestrictedFeaturesEXT<'_>
    {
    }

    impl Default for PhysicalDeviceVertexAddressBindingUnrestrictedFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                vertex_address_binding_unrestricted: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceVertexAddressBindingUnrestrictedFeaturesEXT<'a> {
        #[inline]
        pub fn vertex_address_binding_unrestricted(
            mut self,
            vertex_address_binding_unrestricted: bool,
        ) -> Self {
            self.vertex_address_binding_unrestricted = vertex_address_binding_unrestricted.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceVertexAddressBindingUnrestrictedFeaturesEXT =
        PhysicalDeviceVertexAddressBindingUnrestrictedFeaturesEXT<'static>;
    impl PhysicalDeviceVertexAddressBindingUnrestrictedFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceVertexAddressBindingUnrestrictedFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

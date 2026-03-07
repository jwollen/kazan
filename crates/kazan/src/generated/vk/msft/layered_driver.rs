#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_MSFT_layered_driver";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceLayeredDriverPropertiesMSFT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceLayeredDriverPropertiesMSFT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub underlying_api: LayeredDriverUnderlyingApiMSFT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceLayeredDriverPropertiesMSFT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceLayeredDriverPropertiesMSFT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("underlying_api", &self.underlying_api)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceLayeredDriverPropertiesMSFT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceLayeredDriverPropertiesMSFT<'a>
    {
    }

    impl Default for PhysicalDeviceLayeredDriverPropertiesMSFT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                underlying_api: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceLayeredDriverPropertiesMSFT<'a> {
        pub fn underlying_api(mut self, underlying_api: LayeredDriverUnderlyingApiMSFT) -> Self {
            self.underlying_api = underlying_api;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkLayeredDriverUnderlyingApiMSFT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LayeredDriverUnderlyingApiMSFT(i32);

    impl LayeredDriverUnderlyingApiMSFT {
        pub const NONE_MSFT: Self = Self(0);
        pub const D3D12_MSFT: Self = Self(1);
    }

    impl fmt::Debug for LayeredDriverUnderlyingApiMSFT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::NONE_MSFT => Some("NONE_MSFT"),
                Self::D3D12_MSFT => Some("D3D12_MSFT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
}

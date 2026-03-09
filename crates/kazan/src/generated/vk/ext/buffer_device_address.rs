//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_buffer_device_address.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_buffer_device_address";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceBufferAddressFeaturesEXT.html>
    pub type PhysicalDeviceBufferAddressFeaturesEXT<'a> =
        PhysicalDeviceBufferDeviceAddressFeaturesEXT<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferDeviceAddressInfoEXT.html>
    pub type BufferDeviceAddressInfoEXT<'a> = BufferDeviceAddressInfo<'a>;
    pub type PFN_vkGetBufferDeviceAddressEXT = PFN_vkGetBufferDeviceAddress;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceBufferDeviceAddressFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub buffer_device_address: Bool32,
        pub buffer_device_address_capture_replay: Bool32,
        pub buffer_device_address_multi_device: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceBufferDeviceAddressFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceBufferDeviceAddressFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("buffer_device_address", &self.buffer_device_address)
                .field(
                    "buffer_device_address_capture_replay",
                    &self.buffer_device_address_capture_replay,
                )
                .field(
                    "buffer_device_address_multi_device",
                    &self.buffer_device_address_multi_device,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceBufferDeviceAddressFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceBufferDeviceAddressFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceBufferDeviceAddressFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceBufferDeviceAddressFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                buffer_device_address: Default::default(),
                buffer_device_address_capture_replay: Default::default(),
                buffer_device_address_multi_device: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceBufferDeviceAddressFeaturesEXT<'a> {
        #[inline]
        pub fn buffer_device_address(mut self, buffer_device_address: bool) -> Self {
            self.buffer_device_address = buffer_device_address.into();
            self
        }

        #[inline]
        pub fn buffer_device_address_capture_replay(
            mut self,
            buffer_device_address_capture_replay: bool,
        ) -> Self {
            self.buffer_device_address_capture_replay = buffer_device_address_capture_replay.into();
            self
        }

        #[inline]
        pub fn buffer_device_address_multi_device(
            mut self,
            buffer_device_address_multi_device: bool,
        ) -> Self {
            self.buffer_device_address_multi_device = buffer_device_address_multi_device.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferDeviceAddressCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BufferDeviceAddressCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub device_address: DeviceAddress,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BufferDeviceAddressCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BufferDeviceAddressCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("device_address", &self.device_address)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BufferDeviceAddressCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<BufferCreateInfo<'a>> for BufferDeviceAddressCreateInfoEXT<'a> {}

    impl Default for BufferDeviceAddressCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                device_address: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BufferDeviceAddressCreateInfoEXT<'a> {
        #[inline]
        pub fn device_address(mut self, device_address: DeviceAddress) -> Self {
            self.device_address = device_address;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceBufferDeviceAddressFeaturesEXT =
        PhysicalDeviceBufferDeviceAddressFeaturesEXT<'static>;
    pub type VkBufferDeviceAddressCreateInfoEXT = BufferDeviceAddressCreateInfoEXT<'static>;
    pub type VkPhysicalDeviceBufferAddressFeaturesEXT =
        PhysicalDeviceBufferAddressFeaturesEXT<'static>;
    pub type VkBufferDeviceAddressInfoEXT = BufferDeviceAddressInfoEXT<'static>;
    impl PhysicalDeviceBufferDeviceAddressFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceBufferDeviceAddressFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BufferDeviceAddressCreateInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBufferDeviceAddressCreateInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_buffer_device_address_ext: PFN_vkGetBufferDeviceAddress,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_buffer_device_address_ext: transmute(
                    load(c"vkGetBufferDeviceAddressEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetBufferDeviceAddressEXT.html>
    #[inline]
    pub unsafe fn get_buffer_device_address_ext(
        &self,
        device: Device,
        info: &BufferDeviceAddressInfo<'_>,
    ) -> DeviceAddress {
        unsafe { (self.get_buffer_device_address_ext)(device, info) }
    }
}

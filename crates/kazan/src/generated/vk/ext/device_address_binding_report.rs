#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceAddressBindingReportFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceAddressBindingReportFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub report_address_binding: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceAddressBindingReportFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceAddressBindingReportFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceAddressBindingReportFeaturesEXT<'a>
    {
    }
    impl Default for PhysicalDeviceAddressBindingReportFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                report_address_binding: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceAddressBindingReportFeaturesEXT<'a> {
        pub fn report_address_binding(mut self, report_address_binding: bool) -> Self {
            self.report_address_binding = report_address_binding.into();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceAddressBindingCallbackDataEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceAddressBindingCallbackDataEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: DeviceAddressBindingFlagsEXT,
        pub base_address: DeviceAddress,
        pub size: DeviceSize,
        pub binding_type: DeviceAddressBindingTypeEXT,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DeviceAddressBindingCallbackDataEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT;
    }
    unsafe impl<'a> Extends<DebugUtilsMessengerCallbackDataEXT<'a>>
        for DeviceAddressBindingCallbackDataEXT<'a>
    {
    }
    impl Default for DeviceAddressBindingCallbackDataEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                flags: Default::default(),
                base_address: Default::default(),
                size: Default::default(),
                binding_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceAddressBindingCallbackDataEXT<'a> {
        pub fn flags(mut self, flags: DeviceAddressBindingFlagsEXT) -> Self {
            self.flags = flags;
            self
        }
        pub fn base_address(mut self, base_address: DeviceAddress) -> Self {
            self.base_address = base_address;
            self
        }
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
        pub fn binding_type(mut self, binding_type: DeviceAddressBindingTypeEXT) -> Self {
            self.binding_type = binding_type;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceAddressBindingTypeEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DeviceAddressBindingTypeEXT(i32);
    impl DeviceAddressBindingTypeEXT {
        pub const BIND_EXT: Self = Self(0);
        pub const UNBIND_EXT: Self = Self(1);
    }
    impl fmt::Debug for DeviceAddressBindingTypeEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::BIND_EXT => Some("BIND_EXT"),
                Self::UNBIND_EXT => Some("UNBIND_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceAddressBindingFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DeviceAddressBindingFlagsEXT(Flags);
    vk_bitflags_wrapped!(DeviceAddressBindingFlagsEXT, Flags);
    impl DeviceAddressBindingFlagsEXT {
        pub const INTERNAL_OBJECT_EXT: Self =
            Self(DeviceAddressBindingFlagBitsEXT::INTERNAL_OBJECT_EXT.0);
    }
    impl fmt::Debug for DeviceAddressBindingFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[(
                DeviceAddressBindingFlagsEXT::INTERNAL_OBJECT_EXT.0,
                "INTERNAL_OBJECT_EXT",
            )];
            debug_flags(f, KNOWN, self.0)
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceAddressBindingFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DeviceAddressBindingFlagBitsEXT(u32);
    impl DeviceAddressBindingFlagBitsEXT {
        pub const INTERNAL_OBJECT_EXT: Self = Self(1 << 0);
    }
}

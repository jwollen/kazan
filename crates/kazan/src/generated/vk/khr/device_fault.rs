//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_device_fault.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_device_fault";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultAddressInfoKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct DeviceFaultAddressInfoKHR {
        pub address_type: DeviceFaultAddressTypeKHR,
        pub reported_address: DeviceAddress,
        pub address_precision: DeviceSize,
    }

    impl DeviceFaultAddressInfoKHR {
        #[inline]
        pub fn address_type(mut self, address_type: DeviceFaultAddressTypeKHR) -> Self {
            self.address_type = address_type;
            self
        }

        #[inline]
        pub fn reported_address(mut self, reported_address: DeviceAddress) -> Self {
            self.reported_address = reported_address;
            self
        }

        #[inline]
        pub fn address_precision(mut self, address_precision: DeviceSize) -> Self {
            self.address_precision = address_precision;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultVendorInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceFaultVendorInfoKHR {
        pub description: ArrayCStr<{ MAX_DESCRIPTION_SIZE as usize }>,
        pub vendor_fault_code: u64,
        pub vendor_fault_data: u64,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceFaultVendorInfoKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceFaultVendorInfoKHR")
                .field("description", &self.description)
                .field("vendor_fault_code", &self.vendor_fault_code)
                .field("vendor_fault_data", &self.vendor_fault_data)
                .finish()
        }
    }

    impl Default for DeviceFaultVendorInfoKHR {
        fn default() -> Self {
            Self {
                description: Default::default(),
                vendor_fault_code: Default::default(),
                vendor_fault_data: Default::default(),
            }
        }
    }

    impl DeviceFaultVendorInfoKHR {
        #[inline]
        pub fn description(
            mut self,
            description: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            self.description.write_c_str(description)?;
            Ok(self)
        }

        #[inline]
        pub fn vendor_fault_code(mut self, vendor_fault_code: u64) -> Self {
            self.vendor_fault_code = vendor_fault_code;
            self
        }

        #[inline]
        pub fn vendor_fault_data(mut self, vendor_fault_data: u64) -> Self {
            self.vendor_fault_data = vendor_fault_data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceFaultInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: DeviceFaultFlagsKHR,
        pub group_id: u64,
        pub description: ArrayCStr<{ MAX_DESCRIPTION_SIZE as usize }>,
        pub fault_address_info: DeviceFaultAddressInfoKHR,
        pub instruction_address_info: DeviceFaultAddressInfoKHR,
        pub vendor_info: DeviceFaultVendorInfoKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceFaultInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceFaultInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("group_id", &self.group_id)
                .field("description", &self.description)
                .field("fault_address_info", &self.fault_address_info)
                .field("instruction_address_info", &self.instruction_address_info)
                .field("vendor_info", &self.vendor_info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceFaultInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_FAULT_INFO_KHR;
    }

    impl Default for DeviceFaultInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                flags: Default::default(),
                group_id: Default::default(),
                description: Default::default(),
                fault_address_info: Default::default(),
                instruction_address_info: Default::default(),
                vendor_info: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceFaultInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: DeviceFaultFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn group_id(mut self, group_id: u64) -> Self {
            self.group_id = group_id;
            self
        }

        #[inline]
        pub fn description(
            mut self,
            description: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            self.description.write_c_str(description)?;
            Ok(self)
        }

        #[inline]
        pub fn fault_address_info(mut self, fault_address_info: DeviceFaultAddressInfoKHR) -> Self {
            self.fault_address_info = fault_address_info;
            self
        }

        #[inline]
        pub fn instruction_address_info(
            mut self,
            instruction_address_info: DeviceFaultAddressInfoKHR,
        ) -> Self {
            self.instruction_address_info = instruction_address_info;
            self
        }

        #[inline]
        pub fn vendor_info(mut self, vendor_info: DeviceFaultVendorInfoKHR) -> Self {
            self.vendor_info = vendor_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultDebugInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceFaultDebugInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub vendor_binary_size: u32,
        pub p_vendor_binary_data: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceFaultDebugInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceFaultDebugInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("vendor_binary_size", &self.vendor_binary_size)
                .field("p_vendor_binary_data", &self.p_vendor_binary_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceFaultDebugInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_FAULT_DEBUG_INFO_KHR;
    }

    impl Default for DeviceFaultDebugInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                vendor_binary_size: Default::default(),
                p_vendor_binary_data: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceFaultDebugInfoKHR<'a> {
        #[inline]
        pub fn vendor_binary_data(mut self, vendor_binary_data: &'a mut [u8]) -> Self {
            self.vendor_binary_size = vendor_binary_data.len().try_into().unwrap();
            self.p_vendor_binary_data = vendor_binary_data.as_mut_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultVendorBinaryHeaderVersionOneKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceFaultVendorBinaryHeaderVersionOneKHR {
        pub header_size: u32,
        pub header_version: DeviceFaultVendorBinaryHeaderVersionKHR,
        pub vendor_id: u32,
        pub device_id: u32,
        pub driver_version: u32,
        pub pipeline_cache_uuid: [u8; UUID_SIZE as usize],
        pub application_name_offset: u32,
        pub application_version: u32,
        pub engine_name_offset: u32,
        pub engine_version: u32,
        pub api_version: ApiVersion,
    }

    impl Default for DeviceFaultVendorBinaryHeaderVersionOneKHR {
        fn default() -> Self {
            Self {
                header_size: Default::default(),
                header_version: Default::default(),
                vendor_id: Default::default(),
                device_id: Default::default(),
                driver_version: Default::default(),
                pipeline_cache_uuid: [Default::default(); _],
                application_name_offset: Default::default(),
                application_version: Default::default(),
                engine_name_offset: Default::default(),
                engine_version: Default::default(),
                api_version: Default::default(),
            }
        }
    }

    impl DeviceFaultVendorBinaryHeaderVersionOneKHR {
        #[inline]
        pub fn header_size(mut self, header_size: u32) -> Self {
            self.header_size = header_size;
            self
        }

        #[inline]
        pub fn header_version(
            mut self,
            header_version: DeviceFaultVendorBinaryHeaderVersionKHR,
        ) -> Self {
            self.header_version = header_version;
            self
        }

        #[inline]
        pub fn vendor_id(mut self, vendor_id: u32) -> Self {
            self.vendor_id = vendor_id;
            self
        }

        #[inline]
        pub fn device_id(mut self, device_id: u32) -> Self {
            self.device_id = device_id;
            self
        }

        #[inline]
        pub fn driver_version(mut self, driver_version: u32) -> Self {
            self.driver_version = driver_version;
            self
        }

        #[inline]
        pub fn pipeline_cache_uuid(
            mut self,
            pipeline_cache_uuid: [u8; UUID_SIZE as usize],
        ) -> Self {
            self.pipeline_cache_uuid = pipeline_cache_uuid;
            self
        }

        #[inline]
        pub fn application_name_offset(mut self, application_name_offset: u32) -> Self {
            self.application_name_offset = application_name_offset;
            self
        }

        #[inline]
        pub fn application_version(mut self, application_version: u32) -> Self {
            self.application_version = application_version;
            self
        }

        #[inline]
        pub fn engine_name_offset(mut self, engine_name_offset: u32) -> Self {
            self.engine_name_offset = engine_name_offset;
            self
        }

        #[inline]
        pub fn engine_version(mut self, engine_version: u32) -> Self {
            self.engine_version = engine_version;
            self
        }

        #[inline]
        pub fn api_version(mut self, api_version: ApiVersion) -> Self {
            self.api_version = api_version;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFaultFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceFaultFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub device_fault: Bool32,
        pub device_fault_vendor_binary: Bool32,
        pub device_fault_report_masked: Bool32,
        pub device_fault_device_lost_on_masked: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceFaultFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceFaultFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("device_fault", &self.device_fault)
                .field(
                    "device_fault_vendor_binary",
                    &self.device_fault_vendor_binary,
                )
                .field(
                    "device_fault_report_masked",
                    &self.device_fault_report_masked,
                )
                .field(
                    "device_fault_device_lost_on_masked",
                    &self.device_fault_device_lost_on_masked,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFaultFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_FAULT_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceFaultFeaturesKHR<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceFaultFeaturesKHR<'_> {}

    impl Default for PhysicalDeviceFaultFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                device_fault: Default::default(),
                device_fault_vendor_binary: Default::default(),
                device_fault_report_masked: Default::default(),
                device_fault_device_lost_on_masked: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceFaultFeaturesKHR<'a> {
        #[inline]
        pub fn device_fault(mut self, device_fault: bool) -> Self {
            self.device_fault = device_fault.into();
            self
        }

        #[inline]
        pub fn device_fault_vendor_binary(mut self, device_fault_vendor_binary: bool) -> Self {
            self.device_fault_vendor_binary = device_fault_vendor_binary.into();
            self
        }

        #[inline]
        pub fn device_fault_report_masked(mut self, device_fault_report_masked: bool) -> Self {
            self.device_fault_report_masked = device_fault_report_masked.into();
            self
        }

        #[inline]
        pub fn device_fault_device_lost_on_masked(
            mut self,
            device_fault_device_lost_on_masked: bool,
        ) -> Self {
            self.device_fault_device_lost_on_masked = device_fault_device_lost_on_masked.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFaultPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceFaultPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_device_fault_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceFaultPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceFaultPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_device_fault_count", &self.max_device_fault_count)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFaultPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_FAULT_PROPERTIES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>> for PhysicalDeviceFaultPropertiesKHR<'_> {}

    impl Default for PhysicalDeviceFaultPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_device_fault_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceFaultPropertiesKHR<'a> {
        #[inline]
        pub fn max_device_fault_count(mut self, max_device_fault_count: u32) -> Self {
            self.max_device_fault_count = max_device_fault_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultAddressTypeKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DeviceFaultAddressTypeKHR(i32);

    impl DeviceFaultAddressTypeKHR {
        /// Currently unused
        pub const NONE_KHR: Self = Self(0);
        pub const READ_INVALID_KHR: Self = Self(1);
        pub const WRITE_INVALID_KHR: Self = Self(2);
        pub const EXECUTE_INVALID_KHR: Self = Self(3);
        pub const INSTRUCTION_POINTER_UNKNOWN_KHR: Self = Self(4);
        pub const INSTRUCTION_POINTER_INVALID_KHR: Self = Self(5);
        pub const INSTRUCTION_POINTER_FAULT_KHR: Self = Self(6);

        // VK_EXT_device_fault
        pub const NONE_EXT: Self = Self::NONE_KHR;
        pub const READ_INVALID_EXT: Self = Self::READ_INVALID_KHR;
        pub const WRITE_INVALID_EXT: Self = Self::WRITE_INVALID_KHR;
        pub const EXECUTE_INVALID_EXT: Self = Self::EXECUTE_INVALID_KHR;
        pub const INSTRUCTION_POINTER_UNKNOWN_EXT: Self = Self::INSTRUCTION_POINTER_UNKNOWN_KHR;
        pub const INSTRUCTION_POINTER_INVALID_EXT: Self = Self::INSTRUCTION_POINTER_INVALID_KHR;
        pub const INSTRUCTION_POINTER_FAULT_EXT: Self = Self::INSTRUCTION_POINTER_FAULT_KHR;
    }

    impl fmt::Debug for DeviceFaultAddressTypeKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::NONE_KHR => Some("NONE_KHR"),
                Self::READ_INVALID_KHR => Some("READ_INVALID_KHR"),
                Self::WRITE_INVALID_KHR => Some("WRITE_INVALID_KHR"),
                Self::EXECUTE_INVALID_KHR => Some("EXECUTE_INVALID_KHR"),
                Self::INSTRUCTION_POINTER_UNKNOWN_KHR => Some("INSTRUCTION_POINTER_UNKNOWN_KHR"),
                Self::INSTRUCTION_POINTER_INVALID_KHR => Some("INSTRUCTION_POINTER_INVALID_KHR"),
                Self::INSTRUCTION_POINTER_FAULT_KHR => Some("INSTRUCTION_POINTER_FAULT_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultVendorBinaryHeaderVersionKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DeviceFaultVendorBinaryHeaderVersionKHR(i32);

    impl DeviceFaultVendorBinaryHeaderVersionKHR {
        pub const ONE_KHR: Self = Self(1);
    }

    impl fmt::Debug for DeviceFaultVendorBinaryHeaderVersionKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ONE_KHR => Some("ONE_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DeviceFaultFlagsKHR(Flags);
    vk_bitflags_wrapped!(DeviceFaultFlagsKHR, Flags, DeviceFaultFlagBitsKHR);

    impl fmt::Debug for DeviceFaultFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    DeviceFaultFlagBitsKHR::FLAG_DEVICE_LOST_KHR.0,
                    "FLAG_DEVICE_LOST_KHR",
                ),
                (
                    DeviceFaultFlagBitsKHR::FLAG_MEMORY_ADDRESS_KHR.0,
                    "FLAG_MEMORY_ADDRESS_KHR",
                ),
                (
                    DeviceFaultFlagBitsKHR::FLAG_INSTRUCTION_ADDRESS_KHR.0,
                    "FLAG_INSTRUCTION_ADDRESS_KHR",
                ),
                (DeviceFaultFlagBitsKHR::FLAG_VENDOR_KHR.0, "FLAG_VENDOR_KHR"),
                (
                    DeviceFaultFlagBitsKHR::FLAG_WATCHDOG_TIMEOUT_KHR.0,
                    "FLAG_WATCHDOG_TIMEOUT_KHR",
                ),
                (
                    DeviceFaultFlagBitsKHR::FLAG_OVERFLOW_KHR.0,
                    "FLAG_OVERFLOW_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DeviceFaultFlagBitsKHR(u32);

    impl DeviceFaultFlagBitsKHR {
        pub const FLAG_DEVICE_LOST_KHR: Self = Self(1 << 0);
        pub const FLAG_MEMORY_ADDRESS_KHR: Self = Self(1 << 1);
        pub const FLAG_INSTRUCTION_ADDRESS_KHR: Self = Self(1 << 2);
        pub const FLAG_VENDOR_KHR: Self = Self(1 << 3);
        pub const FLAG_WATCHDOG_TIMEOUT_KHR: Self = Self(1 << 4);
        pub const FLAG_OVERFLOW_KHR: Self = Self(1 << 5);
    }

    impl fmt::Debug for DeviceFaultFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::FLAG_DEVICE_LOST_KHR => Some("FLAG_DEVICE_LOST_KHR"),
                Self::FLAG_MEMORY_ADDRESS_KHR => Some("FLAG_MEMORY_ADDRESS_KHR"),
                Self::FLAG_INSTRUCTION_ADDRESS_KHR => Some("FLAG_INSTRUCTION_ADDRESS_KHR"),
                Self::FLAG_VENDOR_KHR => Some("FLAG_VENDOR_KHR"),
                Self::FLAG_WATCHDOG_TIMEOUT_KHR => Some("FLAG_WATCHDOG_TIMEOUT_KHR"),
                Self::FLAG_OVERFLOW_KHR => Some("FLAG_OVERFLOW_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceFaultReportsKHR.html>
    pub type PFN_vkGetDeviceFaultReportsKHR = unsafe extern "system" fn(
        device: Device,
        timeout: u64,
        p_fault_counts: *mut u32,
        p_fault_info: *mut DeviceFaultInfoKHR<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceFaultDebugInfoKHR.html>
    pub type PFN_vkGetDeviceFaultDebugInfoKHR = unsafe extern "system" fn(
        device: Device,
        p_debug_info: *mut DeviceFaultDebugInfoKHR<'_>,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkDeviceFaultAddressInfoKHR = DeviceFaultAddressInfoKHR;
    pub type VkDeviceFaultVendorInfoKHR = DeviceFaultVendorInfoKHR;
    pub type VkDeviceFaultInfoKHR = DeviceFaultInfoKHR<'static>;
    pub type VkDeviceFaultDebugInfoKHR = DeviceFaultDebugInfoKHR<'static>;
    pub type VkDeviceFaultVendorBinaryHeaderVersionOneKHR =
        DeviceFaultVendorBinaryHeaderVersionOneKHR;
    pub type VkPhysicalDeviceFaultFeaturesKHR = PhysicalDeviceFaultFeaturesKHR<'static>;
    pub type VkPhysicalDeviceFaultPropertiesKHR = PhysicalDeviceFaultPropertiesKHR<'static>;
    pub type VkDeviceFaultAddressTypeKHR = DeviceFaultAddressTypeKHR;
    pub type VkDeviceFaultVendorBinaryHeaderVersionKHR = DeviceFaultVendorBinaryHeaderVersionKHR;
    pub type VkDeviceFaultFlagsKHR = DeviceFaultFlagsKHR;
    pub type VkDeviceFaultFlagBitsKHR = DeviceFaultFlagBitsKHR;
    impl DeviceFaultInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceFaultInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceFaultDebugInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceFaultDebugInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceFaultFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceFaultFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceFaultPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceFaultPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_device_fault_reports: PFN_vkGetDeviceFaultReportsKHR,
    get_device_fault_debug_info: PFN_vkGetDeviceFaultDebugInfoKHR,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_device_fault_reports: transmute(
                    load(c"vkGetDeviceFaultReportsKHR").ok_or(MissingEntryPointError)?,
                ),
                get_device_fault_debug_info: transmute(
                    load(c"vkGetDeviceFaultDebugInfoKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceFaultReportsKHR.html>
    #[inline]
    pub unsafe fn get_device_fault_reports<'a>(
        &self,
        device: Device,
        timeout: u64,
        mut fault_info: impl ExtendUninit<DeviceFaultInfoKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |fault_counts, fault_info| {
                let result =
                    (self.get_device_fault_reports)(device, timeout, fault_counts, fault_info as _);

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    VkResult::TIMEOUT => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let fault_info_buf = fault_info.reserve(capacity);
            len = fault_info_buf.len().try_into().unwrap();
            let result = call(&mut len, fault_info_buf.as_mut_ptr() as *mut _)?;
            fault_info.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceFaultDebugInfoKHR.html>
    #[inline]
    pub unsafe fn get_device_fault_debug_info(
        &self,
        device: Device,
        debug_info: &mut DeviceFaultDebugInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_device_fault_debug_info)(device, debug_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}

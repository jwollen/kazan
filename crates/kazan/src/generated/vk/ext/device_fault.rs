//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_device_fault.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_device_fault";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFaultFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceFaultFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub device_fault: Bool32,
        pub device_fault_vendor_binary: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceFaultFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceFaultFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("device_fault", &self.device_fault)
                .field(
                    "device_fault_vendor_binary",
                    &self.device_fault_vendor_binary,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFaultFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_FAULT_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceFaultFeaturesEXT<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceFaultFeaturesEXT<'_> {}

    impl Default for PhysicalDeviceFaultFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                device_fault: Default::default(),
                device_fault_vendor_binary: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceFaultFeaturesEXT<'a> {
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
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultAddressInfoEXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct DeviceFaultAddressInfoEXT {
        pub address_type: DeviceFaultAddressTypeEXT,
        pub reported_address: DeviceAddress,
        pub address_precision: DeviceSize,
    }

    impl DeviceFaultAddressInfoEXT {
        #[inline]
        pub fn address_type(mut self, address_type: DeviceFaultAddressTypeEXT) -> Self {
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultVendorInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceFaultVendorInfoEXT {
        pub description: ArrayCStr<{ MAX_DESCRIPTION_SIZE as usize }>,
        pub vendor_fault_code: u64,
        pub vendor_fault_data: u64,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceFaultVendorInfoEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceFaultVendorInfoEXT")
                .field("description", &self.description)
                .field("vendor_fault_code", &self.vendor_fault_code)
                .field("vendor_fault_data", &self.vendor_fault_data)
                .finish()
        }
    }

    impl Default for DeviceFaultVendorInfoEXT {
        fn default() -> Self {
            Self {
                description: Default::default(),
                vendor_fault_code: Default::default(),
                vendor_fault_data: Default::default(),
            }
        }
    }

    impl DeviceFaultVendorInfoEXT {
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultCountsEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceFaultCountsEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub address_info_count: u32,
        pub vendor_info_count: u32,
        pub vendor_binary_size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceFaultCountsEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceFaultCountsEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("address_info_count", &self.address_info_count)
                .field("vendor_info_count", &self.vendor_info_count)
                .field("vendor_binary_size", &self.vendor_binary_size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceFaultCountsEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_FAULT_COUNTS_EXT;
    }

    impl Default for DeviceFaultCountsEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                address_info_count: Default::default(),
                vendor_info_count: Default::default(),
                vendor_binary_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceFaultCountsEXT<'a> {
        #[inline]
        pub fn address_info_count(mut self, address_info_count: u32) -> Self {
            self.address_info_count = address_info_count;
            self
        }

        #[inline]
        pub fn vendor_info_count(mut self, vendor_info_count: u32) -> Self {
            self.vendor_info_count = vendor_info_count;
            self
        }

        #[inline]
        pub fn vendor_binary_size(mut self, vendor_binary_size: DeviceSize) -> Self {
            self.vendor_binary_size = vendor_binary_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceFaultInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub description: ArrayCStr<{ MAX_DESCRIPTION_SIZE as usize }>,
        pub p_address_infos: *mut DeviceFaultAddressInfoEXT,
        pub p_vendor_infos: *mut DeviceFaultVendorInfoEXT,
        pub p_vendor_binary_data: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceFaultInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceFaultInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("description", &self.description)
                .field("p_address_infos", &self.p_address_infos)
                .field("p_vendor_infos", &self.p_vendor_infos)
                .field("p_vendor_binary_data", &self.p_vendor_binary_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceFaultInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_FAULT_INFO_EXT;
    }

    impl Default for DeviceFaultInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                description: Default::default(),
                p_address_infos: ptr::null_mut(),
                p_vendor_infos: ptr::null_mut(),
                p_vendor_binary_data: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceFaultInfoEXT<'a> {
        #[inline]
        pub fn description(
            mut self,
            description: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            self.description.write_c_str(description)?;
            Ok(self)
        }

        #[inline]
        pub fn address_infos(mut self, address_infos: &'a mut DeviceFaultAddressInfoEXT) -> Self {
            self.p_address_infos = address_infos;
            self
        }

        #[inline]
        pub fn vendor_infos(mut self, vendor_infos: &'a mut DeviceFaultVendorInfoEXT) -> Self {
            self.p_vendor_infos = vendor_infos;
            self
        }

        #[inline]
        pub fn vendor_binary_data(mut self, vendor_binary_data: *mut c_void) -> Self {
            self.p_vendor_binary_data = vendor_binary_data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultVendorBinaryHeaderVersionOneEXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceFaultVendorBinaryHeaderVersionOneEXT {
        pub header_size: u32,
        pub header_version: DeviceFaultVendorBinaryHeaderVersionEXT,
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

    impl Default for DeviceFaultVendorBinaryHeaderVersionOneEXT {
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

    impl DeviceFaultVendorBinaryHeaderVersionOneEXT {
        #[inline]
        pub fn header_size(mut self, header_size: u32) -> Self {
            self.header_size = header_size;
            self
        }

        #[inline]
        pub fn header_version(
            mut self,
            header_version: DeviceFaultVendorBinaryHeaderVersionEXT,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultAddressTypeEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DeviceFaultAddressTypeEXT(i32);

    impl DeviceFaultAddressTypeEXT {
        /// Currently unused
        pub const NONE_EXT: Self = Self(0);
        pub const READ_INVALID_EXT: Self = Self(1);
        pub const WRITE_INVALID_EXT: Self = Self(2);
        pub const EXECUTE_INVALID_EXT: Self = Self(3);
        pub const INSTRUCTION_POINTER_UNKNOWN_EXT: Self = Self(4);
        pub const INSTRUCTION_POINTER_INVALID_EXT: Self = Self(5);
        pub const INSTRUCTION_POINTER_FAULT_EXT: Self = Self(6);
    }

    impl fmt::Debug for DeviceFaultAddressTypeEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::NONE_EXT => Some("NONE_EXT"),
                Self::READ_INVALID_EXT => Some("READ_INVALID_EXT"),
                Self::WRITE_INVALID_EXT => Some("WRITE_INVALID_EXT"),
                Self::EXECUTE_INVALID_EXT => Some("EXECUTE_INVALID_EXT"),
                Self::INSTRUCTION_POINTER_UNKNOWN_EXT => Some("INSTRUCTION_POINTER_UNKNOWN_EXT"),
                Self::INSTRUCTION_POINTER_INVALID_EXT => Some("INSTRUCTION_POINTER_INVALID_EXT"),
                Self::INSTRUCTION_POINTER_FAULT_EXT => Some("INSTRUCTION_POINTER_FAULT_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceFaultVendorBinaryHeaderVersionEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DeviceFaultVendorBinaryHeaderVersionEXT(i32);

    impl DeviceFaultVendorBinaryHeaderVersionEXT {
        pub const ONE_EXT: Self = Self(1);
    }

    impl fmt::Debug for DeviceFaultVendorBinaryHeaderVersionEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ONE_EXT => Some("ONE_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceFaultInfoEXT.html>
    pub type PFN_vkGetDeviceFaultInfoEXT = unsafe extern "system" fn(
        device: Device,
        p_fault_counts: *mut DeviceFaultCountsEXT<'_>,
        p_fault_info: *mut DeviceFaultInfoEXT<'_>,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceFaultFeaturesEXT = PhysicalDeviceFaultFeaturesEXT<'static>;
    pub type VkDeviceFaultAddressInfoEXT = DeviceFaultAddressInfoEXT;
    pub type VkDeviceFaultVendorInfoEXT = DeviceFaultVendorInfoEXT;
    pub type VkDeviceFaultCountsEXT = DeviceFaultCountsEXT<'static>;
    pub type VkDeviceFaultInfoEXT = DeviceFaultInfoEXT<'static>;
    pub type VkDeviceFaultVendorBinaryHeaderVersionOneEXT =
        DeviceFaultVendorBinaryHeaderVersionOneEXT;
    pub type VkDeviceFaultAddressTypeEXT = DeviceFaultAddressTypeEXT;
    pub type VkDeviceFaultVendorBinaryHeaderVersionEXT = DeviceFaultVendorBinaryHeaderVersionEXT;
    impl PhysicalDeviceFaultFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceFaultFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceFaultCountsEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceFaultCountsEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceFaultInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceFaultInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_device_fault_info: PFN_vkGetDeviceFaultInfoEXT,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_device_fault_info: transmute(
                    load(c"vkGetDeviceFaultInfoEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceFaultInfoEXT.html>
    #[inline]
    pub unsafe fn get_device_fault_info(
        &self,
        device: Device,
        fault_counts: &mut DeviceFaultCountsEXT<'_>,
        fault_info: Option<&mut DeviceFaultInfoEXT<'_>>,
    ) -> crate::Result<()> {
        unsafe {
            let result =
                (self.get_device_fault_info)(device, fault_counts, fault_info.to_raw_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}

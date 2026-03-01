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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub extended_sparse_address_space: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'a>
    {
    }
    impl Default for PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                extended_sparse_address_space: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV<'a> {
        pub fn extended_sparse_address_space(
            mut self,
            extended_sparse_address_space: Bool32,
        ) -> Self {
            self.extended_sparse_address_space = extended_sparse_address_space;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceExtendedSparseAddressSpacePropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub extended_sparse_address_space_size: DeviceSize,
        pub extended_sparse_image_usage_flags: ImageUsageFlags,
        pub extended_sparse_buffer_usage_flags: BufferUsageFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExtendedSparseAddressSpacePropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceExtendedSparseAddressSpacePropertiesNV<'a>
    {
    }
    impl Default for PhysicalDeviceExtendedSparseAddressSpacePropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                extended_sparse_address_space_size: Default::default(),
                extended_sparse_image_usage_flags: Default::default(),
                extended_sparse_buffer_usage_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceExtendedSparseAddressSpacePropertiesNV<'a> {
        pub fn extended_sparse_address_space_size(
            mut self,
            extended_sparse_address_space_size: DeviceSize,
        ) -> Self {
            self.extended_sparse_address_space_size = extended_sparse_address_space_size;
            self
        }
        pub fn extended_sparse_image_usage_flags(
            mut self,
            extended_sparse_image_usage_flags: ImageUsageFlags,
        ) -> Self {
            self.extended_sparse_image_usage_flags = extended_sparse_image_usage_flags;
            self
        }
        pub fn extended_sparse_buffer_usage_flags(
            mut self,
            extended_sparse_buffer_usage_flags: BufferUsageFlags,
        ) -> Self {
            self.extended_sparse_buffer_usage_flags = extended_sparse_buffer_usage_flags;
            self
        }
    }
}

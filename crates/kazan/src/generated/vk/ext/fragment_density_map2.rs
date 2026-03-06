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
    pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub fragment_density_map_deferred: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentDensityMap2FeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceFragmentDensityMap2FeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceFragmentDensityMap2FeaturesEXT<'a> {}
    impl Default for PhysicalDeviceFragmentDensityMap2FeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                fragment_density_map_deferred: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceFragmentDensityMap2FeaturesEXT<'a> {
        pub fn fragment_density_map_deferred(
            mut self,
            fragment_density_map_deferred: bool,
        ) -> Self {
            self.fragment_density_map_deferred = fragment_density_map_deferred.into();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceFragmentDensityMap2PropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub subsampled_loads: Bool32,
        pub subsampled_coarse_reconstruction_early_access: Bool32,
        pub max_subsampled_array_layers: u32,
        pub max_descriptor_set_subsampled_samplers: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentDensityMap2PropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceFragmentDensityMap2PropertiesEXT<'a>
    {
    }
    impl Default for PhysicalDeviceFragmentDensityMap2PropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                subsampled_loads: Default::default(),
                subsampled_coarse_reconstruction_early_access: Default::default(),
                max_subsampled_array_layers: Default::default(),
                max_descriptor_set_subsampled_samplers: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceFragmentDensityMap2PropertiesEXT<'a> {
        pub fn subsampled_loads(mut self, subsampled_loads: bool) -> Self {
            self.subsampled_loads = subsampled_loads.into();
            self
        }
        pub fn subsampled_coarse_reconstruction_early_access(
            mut self,
            subsampled_coarse_reconstruction_early_access: bool,
        ) -> Self {
            self.subsampled_coarse_reconstruction_early_access =
                subsampled_coarse_reconstruction_early_access.into();
            self
        }
        pub fn max_subsampled_array_layers(mut self, max_subsampled_array_layers: u32) -> Self {
            self.max_subsampled_array_layers = max_subsampled_array_layers;
            self
        }
        pub fn max_descriptor_set_subsampled_samplers(
            mut self,
            max_descriptor_set_subsampled_samplers: u32,
        ) -> Self {
            self.max_descriptor_set_subsampled_samplers = max_descriptor_set_subsampled_samplers;
            self
        }
    }
}

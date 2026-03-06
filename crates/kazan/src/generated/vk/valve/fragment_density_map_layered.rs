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
    pub struct PhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_fragment_density_map_layers: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'a>
    {
    }
    impl Default for PhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_fragment_density_map_layers: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE<'a> {
        pub fn max_fragment_density_map_layers(
            mut self,
            max_fragment_density_map_layers: u32,
        ) -> Self {
            self.max_fragment_density_map_layers = max_fragment_density_map_layers;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub fragment_density_map_layered: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'a>
    {
    }
    impl Default for PhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                fragment_density_map_layered: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE<'a> {
        pub fn fragment_density_map_layered(mut self, fragment_density_map_layered: bool) -> Self {
            self.fragment_density_map_layered = fragment_density_map_layered.into();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineFragmentDensityMapLayeredCreateInfoVALVE<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub max_fragment_density_map_layers: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineFragmentDensityMapLayeredCreateInfoVALVE<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE;
    }
    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>>
        for PipelineFragmentDensityMapLayeredCreateInfoVALVE<'a>
    {
    }
    impl Default for PipelineFragmentDensityMapLayeredCreateInfoVALVE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                max_fragment_density_map_layers: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineFragmentDensityMapLayeredCreateInfoVALVE<'a> {
        pub fn max_fragment_density_map_layers(
            mut self,
            max_fragment_density_map_layers: u32,
        ) -> Self {
            self.max_fragment_density_map_layers = max_fragment_density_map_layers;
            self
        }
    }
}

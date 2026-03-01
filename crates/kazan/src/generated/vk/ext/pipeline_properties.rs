#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    pub type PipelineInfoEXT<'a> = PipelineInfoKHR<'a>;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelinePropertiesIdentifierEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_identifier: [u8; UUID_SIZE as usize],
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelinePropertiesIdentifierEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_PROPERTIES_IDENTIFIER_EXT;
    }
    impl Default for PipelinePropertiesIdentifierEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                pipeline_identifier: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelinePropertiesIdentifierEXT<'a> {
        pub fn pipeline_identifier(
            mut self,
            pipeline_identifier: [u8; UUID_SIZE as usize],
        ) -> Self {
            self.pipeline_identifier = pipeline_identifier;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePipelinePropertiesFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_properties_identifier: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePipelinePropertiesFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePipelinePropertiesFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDevicePipelinePropertiesFeaturesEXT<'a> {}
    impl Default for PhysicalDevicePipelinePropertiesFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                pipeline_properties_identifier: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePipelinePropertiesFeaturesEXT<'a> {
        pub fn pipeline_properties_identifier(
            mut self,
            pipeline_properties_identifier: Bool32,
        ) -> Self {
            self.pipeline_properties_identifier = pipeline_properties_identifier;
            self
        }
    }
    pub type PFN_vkGetPipelinePropertiesEXT = unsafe extern "system" fn(
        device: Device,
        p_pipeline_info: *const PipelineInfoEXT<'_>,
        p_pipeline_properties: *mut BaseOutStructure<'_>,
    ) -> vk::Result;
}
pub struct DeviceFn {
    get_pipeline_properties_ext: PFN_vkGetPipelinePropertiesEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_pipeline_properties_ext: transmute(
                    load(c"vkGetPipelinePropertiesEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_pipeline_properties_ext(
        &self,
        device: Device,
        pipeline_info: &PipelineInfoEXT<'_>,
    ) -> crate::Result<BaseOutStructure<'_>> {
        unsafe {
            let mut pipeline_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_pipeline_properties_ext)(
                device,
                pipeline_info,
                pipeline_properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(pipeline_properties.assume_init()),
                err => Err(err),
            }
        }
    }
}

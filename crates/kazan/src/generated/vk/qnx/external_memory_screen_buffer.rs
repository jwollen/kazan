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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImportScreenBufferInfoQNX<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub buffer: *mut _screen_buffer,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImportScreenBufferInfoQNX<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_SCREEN_BUFFER_INFO_QNX;
    }
    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for ImportScreenBufferInfoQNX<'a> {}
    impl Default for ImportScreenBufferInfoQNX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                buffer: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImportScreenBufferInfoQNX<'a> {
        pub fn buffer(mut self, buffer: *mut _screen_buffer) -> Self {
            self.buffer = buffer;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ScreenBufferPropertiesQNX<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub allocation_size: DeviceSize,
        pub memory_type_bits: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ScreenBufferPropertiesQNX<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SCREEN_BUFFER_PROPERTIES_QNX;
    }
    impl Default for ScreenBufferPropertiesQNX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                allocation_size: Default::default(),
                memory_type_bits: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ScreenBufferPropertiesQNX<'a> {
        pub fn allocation_size(mut self, allocation_size: DeviceSize) -> Self {
            self.allocation_size = allocation_size;
            self
        }
        pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
            self.memory_type_bits = memory_type_bits;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ScreenBufferFormatPropertiesQNX<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub format: Format,
        pub external_format: u64,
        pub screen_usage: u64,
        pub format_features: FormatFeatureFlags,
        pub sampler_ycbcr_conversion_components: ComponentMapping,
        pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
        pub suggested_ycbcr_range: SamplerYcbcrRange,
        pub suggested_x_chroma_offset: ChromaLocation,
        pub suggested_y_chroma_offset: ChromaLocation,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ScreenBufferFormatPropertiesQNX<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SCREEN_BUFFER_FORMAT_PROPERTIES_QNX;
    }
    unsafe impl<'a> Extends<ScreenBufferPropertiesQNX<'a>> for ScreenBufferFormatPropertiesQNX<'a> {}
    impl Default for ScreenBufferFormatPropertiesQNX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                format: Default::default(),
                external_format: Default::default(),
                screen_usage: Default::default(),
                format_features: Default::default(),
                sampler_ycbcr_conversion_components: Default::default(),
                suggested_ycbcr_model: Default::default(),
                suggested_ycbcr_range: Default::default(),
                suggested_x_chroma_offset: Default::default(),
                suggested_y_chroma_offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ScreenBufferFormatPropertiesQNX<'a> {
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
        pub fn external_format(mut self, external_format: u64) -> Self {
            self.external_format = external_format;
            self
        }
        pub fn screen_usage(mut self, screen_usage: u64) -> Self {
            self.screen_usage = screen_usage;
            self
        }
        pub fn format_features(mut self, format_features: FormatFeatureFlags) -> Self {
            self.format_features = format_features;
            self
        }
        pub fn sampler_ycbcr_conversion_components(
            mut self,
            sampler_ycbcr_conversion_components: ComponentMapping,
        ) -> Self {
            self.sampler_ycbcr_conversion_components = sampler_ycbcr_conversion_components;
            self
        }
        pub fn suggested_ycbcr_model(
            mut self,
            suggested_ycbcr_model: SamplerYcbcrModelConversion,
        ) -> Self {
            self.suggested_ycbcr_model = suggested_ycbcr_model;
            self
        }
        pub fn suggested_ycbcr_range(mut self, suggested_ycbcr_range: SamplerYcbcrRange) -> Self {
            self.suggested_ycbcr_range = suggested_ycbcr_range;
            self
        }
        pub fn suggested_x_chroma_offset(
            mut self,
            suggested_x_chroma_offset: ChromaLocation,
        ) -> Self {
            self.suggested_x_chroma_offset = suggested_x_chroma_offset;
            self
        }
        pub fn suggested_y_chroma_offset(
            mut self,
            suggested_y_chroma_offset: ChromaLocation,
        ) -> Self {
            self.suggested_y_chroma_offset = suggested_y_chroma_offset;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExternalFormatQNX<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub external_format: u64,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ExternalFormatQNX<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_FORMAT_QNX;
    }
    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for ExternalFormatQNX<'a> {}
    unsafe impl<'a> Extends<SamplerYcbcrConversionCreateInfo<'a>> for ExternalFormatQNX<'a> {}
    impl Default for ExternalFormatQNX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                external_format: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ExternalFormatQNX<'a> {
        pub fn external_format(mut self, external_format: u64) -> Self {
            self.external_format = external_format;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub screen_buffer_import: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'a>
    {
    }
    impl Default for PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                screen_buffer_import: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX<'a> {
        pub fn screen_buffer_import(mut self, screen_buffer_import: Bool32) -> Self {
            self.screen_buffer_import = screen_buffer_import;
            self
        }
    }
    pub type PFN_vkGetScreenBufferPropertiesQNX = unsafe extern "system" fn(
        device: Device,
        buffer: *const _screen_buffer,
        p_properties: *mut ScreenBufferPropertiesQNX<'_>,
    ) -> vk::Result;
}
pub struct DeviceFn {
    get_screen_buffer_properties_qnx: PFN_vkGetScreenBufferPropertiesQNX,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_screen_buffer_properties_qnx: transmute(
                    load(c"vkGetScreenBufferPropertiesQNX").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_screen_buffer_properties_qnx(
        &self,
        device: Device,
        buffer: *const _screen_buffer,
    ) -> crate::Result<ScreenBufferPropertiesQNX<'_>> {
        unsafe {
            let mut properties = core::mem::MaybeUninit::uninit();
            let result =
                (self.get_screen_buffer_properties_qnx)(device, buffer, properties.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(properties.assume_init()),
                err => Err(err),
            }
        }
    }
}

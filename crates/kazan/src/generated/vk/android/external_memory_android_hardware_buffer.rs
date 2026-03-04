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
    pub type AHardwareBuffer = *const c_void;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImportAndroidHardwareBufferInfoANDROID<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub buffer: *mut AHardwareBuffer,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImportAndroidHardwareBufferInfoANDROID<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID;
    }
    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for ImportAndroidHardwareBufferInfoANDROID<'a> {}
    impl Default for ImportAndroidHardwareBufferInfoANDROID<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                buffer: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImportAndroidHardwareBufferInfoANDROID<'a> {
        pub fn buffer(mut self, buffer: *mut AHardwareBuffer) -> Self {
            self.buffer = buffer;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AndroidHardwareBufferUsageANDROID<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub android_hardware_buffer_usage: u64,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for AndroidHardwareBufferUsageANDROID<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ANDROID_HARDWARE_BUFFER_USAGE_ANDROID;
    }
    unsafe impl<'a> Extends<ImageFormatProperties2<'a>> for AndroidHardwareBufferUsageANDROID<'a> {}
    impl Default for AndroidHardwareBufferUsageANDROID<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                android_hardware_buffer_usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> AndroidHardwareBufferUsageANDROID<'a> {
        pub fn android_hardware_buffer_usage(mut self, android_hardware_buffer_usage: u64) -> Self {
            self.android_hardware_buffer_usage = android_hardware_buffer_usage;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AndroidHardwareBufferPropertiesANDROID<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub allocation_size: DeviceSize,
        pub memory_type_bits: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for AndroidHardwareBufferPropertiesANDROID<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID;
    }
    impl Default for AndroidHardwareBufferPropertiesANDROID<'_> {
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
    impl<'a> AndroidHardwareBufferPropertiesANDROID<'a> {
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
    pub struct MemoryGetAndroidHardwareBufferInfoANDROID<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryGetAndroidHardwareBufferInfoANDROID<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID;
    }
    impl Default for MemoryGetAndroidHardwareBufferInfoANDROID<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryGetAndroidHardwareBufferInfoANDROID<'a> {
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AndroidHardwareBufferFormatPropertiesANDROID<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub format: Format,
        pub external_format: u64,
        pub format_features: FormatFeatureFlags,
        pub sampler_ycbcr_conversion_components: ComponentMapping,
        pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
        pub suggested_ycbcr_range: SamplerYcbcrRange,
        pub suggested_x_chroma_offset: ChromaLocation,
        pub suggested_y_chroma_offset: ChromaLocation,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for AndroidHardwareBufferFormatPropertiesANDROID<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID;
    }
    unsafe impl<'a> Extends<AndroidHardwareBufferPropertiesANDROID<'a>>
        for AndroidHardwareBufferFormatPropertiesANDROID<'a>
    {
    }
    impl Default for AndroidHardwareBufferFormatPropertiesANDROID<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                format: Default::default(),
                external_format: Default::default(),
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
    impl<'a> AndroidHardwareBufferFormatPropertiesANDROID<'a> {
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
        pub fn external_format(mut self, external_format: u64) -> Self {
            self.external_format = external_format;
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
    pub struct ExternalFormatANDROID<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub external_format: u64,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ExternalFormatANDROID<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_FORMAT_ANDROID;
    }
    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for ExternalFormatANDROID<'a> {}
    unsafe impl<'a> Extends<SamplerYcbcrConversionCreateInfo<'a>> for ExternalFormatANDROID<'a> {}
    unsafe impl<'a> Extends<AttachmentDescription2<'a>> for ExternalFormatANDROID<'a> {}
    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>> for ExternalFormatANDROID<'a> {}
    unsafe impl<'a> Extends<CommandBufferInheritanceInfo<'a>> for ExternalFormatANDROID<'a> {}
    impl Default for ExternalFormatANDROID<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                external_format: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ExternalFormatANDROID<'a> {
        pub fn external_format(mut self, external_format: u64) -> Self {
            self.external_format = external_format;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AndroidHardwareBufferFormatProperties2ANDROID<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub format: Format,
        pub external_format: u64,
        pub format_features: FormatFeatureFlags2,
        pub sampler_ycbcr_conversion_components: ComponentMapping,
        pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
        pub suggested_ycbcr_range: SamplerYcbcrRange,
        pub suggested_x_chroma_offset: ChromaLocation,
        pub suggested_y_chroma_offset: ChromaLocation,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for AndroidHardwareBufferFormatProperties2ANDROID<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID;
    }
    unsafe impl<'a> Extends<AndroidHardwareBufferPropertiesANDROID<'a>>
        for AndroidHardwareBufferFormatProperties2ANDROID<'a>
    {
    }
    impl Default for AndroidHardwareBufferFormatProperties2ANDROID<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                format: Default::default(),
                external_format: Default::default(),
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
    impl<'a> AndroidHardwareBufferFormatProperties2ANDROID<'a> {
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
        pub fn external_format(mut self, external_format: u64) -> Self {
            self.external_format = external_format;
            self
        }
        pub fn format_features(mut self, format_features: FormatFeatureFlags2) -> Self {
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
    pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID =
        unsafe extern "system" fn(
            device: Device,
            buffer: *const AHardwareBuffer,
            p_properties: *mut AndroidHardwareBufferPropertiesANDROID<'_>,
        ) -> vk::Result;
    pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID = unsafe extern "system" fn(
        device: Device,
        p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID<'_>,
        p_buffer: *mut *mut AHardwareBuffer,
    )
        -> vk::Result;
}
pub struct DeviceFn {
    get_android_hardware_buffer_properties_android: PFN_vkGetAndroidHardwareBufferPropertiesANDROID,
    get_memory_android_hardware_buffer_android: PFN_vkGetMemoryAndroidHardwareBufferANDROID,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_android_hardware_buffer_properties_android: transmute(
                    load(c"vkGetAndroidHardwareBufferPropertiesANDROID")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_memory_android_hardware_buffer_android: transmute(
                    load(c"vkGetMemoryAndroidHardwareBufferANDROID")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_android_hardware_buffer_properties_android(
        &self,
        device: Device,
        buffer: *const AHardwareBuffer,
    ) -> crate::Result<AndroidHardwareBufferPropertiesANDROID<'_>> {
        unsafe {
            let mut properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_android_hardware_buffer_properties_android)(
                device,
                buffer,
                properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(properties.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_memory_android_hardware_buffer_android(
        &self,
        device: Device,
        info: &MemoryGetAndroidHardwareBufferInfoANDROID<'_>,
        buffer: &mut *mut AHardwareBuffer,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_memory_android_hardware_buffer_android)(device, info, buffer);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}

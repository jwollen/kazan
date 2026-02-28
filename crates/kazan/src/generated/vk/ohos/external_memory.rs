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
    pub type OH_NativeBuffer = *const c_void;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct NativeBufferUsageOHOS<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ohos_native_buffer_usage: u64,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for NativeBufferUsageOHOS<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::NATIVE_BUFFER_USAGE_OHOS,
                p_next: core::ptr::null_mut(),
                ohos_native_buffer_usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> NativeBufferUsageOHOS<'a> {
        pub fn ohos_native_buffer_usage(mut self, ohos_native_buffer_usage: u64) -> Self {
            self.ohos_native_buffer_usage = ohos_native_buffer_usage;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct NativeBufferPropertiesOHOS<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub allocation_size: DeviceSize,
        pub memory_type_bits: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for NativeBufferPropertiesOHOS<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::NATIVE_BUFFER_PROPERTIES_OHOS,
                p_next: core::ptr::null_mut(),
                allocation_size: Default::default(),
                memory_type_bits: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> NativeBufferPropertiesOHOS<'a> {
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
    pub struct NativeBufferFormatPropertiesOHOS<'a> {
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
    impl Default for NativeBufferFormatPropertiesOHOS<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS,
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
    impl<'a> NativeBufferFormatPropertiesOHOS<'a> {
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
    pub struct ImportNativeBufferInfoOHOS<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub buffer: *mut OH_NativeBuffer,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for ImportNativeBufferInfoOHOS<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::IMPORT_NATIVE_BUFFER_INFO_OHOS,
                p_next: core::ptr::null(),
                buffer: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImportNativeBufferInfoOHOS<'a> {
        pub fn buffer(mut self, buffer: &'a mut OH_NativeBuffer) -> Self {
            self.buffer = buffer;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryGetNativeBufferInfoOHOS<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for MemoryGetNativeBufferInfoOHOS<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::MEMORY_GET_NATIVE_BUFFER_INFO_OHOS,
                p_next: core::ptr::null(),
                memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryGetNativeBufferInfoOHOS<'a> {
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExternalFormatOHOS<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub external_format: u64,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for ExternalFormatOHOS<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::EXTERNAL_FORMAT_OHOS,
                p_next: core::ptr::null_mut(),
                external_format: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ExternalFormatOHOS<'a> {
        pub fn external_format(mut self, external_format: u64) -> Self {
            self.external_format = external_format;
            self
        }
    }
    pub type PFN_vkGetNativeBufferPropertiesOHOS = unsafe extern "system" fn(
        device: Device,
        buffer: *const OH_NativeBuffer,
        p_properties: *mut NativeBufferPropertiesOHOS<'_>,
    ) -> vk::Result;
    pub type PFN_vkGetMemoryNativeBufferOHOS = unsafe extern "system" fn(
        device: Device,
        p_info: *const MemoryGetNativeBufferInfoOHOS<'_>,
        p_buffer: *mut *mut OH_NativeBuffer,
    ) -> vk::Result;
}
pub struct DeviceFn {
    get_native_buffer_properties_ohos: PFN_vkGetNativeBufferPropertiesOHOS,
    get_memory_native_buffer_ohos: PFN_vkGetMemoryNativeBufferOHOS,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_native_buffer_properties_ohos: transmute(
                    load(c"vkGetNativeBufferPropertiesOHOS").ok_or(LoadingError)?,
                ),
                get_memory_native_buffer_ohos: transmute(
                    load(c"vkGetMemoryNativeBufferOHOS").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_native_buffer_properties_ohos(
        &self,
        device: Device,
        buffer: &OH_NativeBuffer,
    ) -> crate::Result<NativeBufferPropertiesOHOS<'_>> {
        unsafe {
            let mut properties = core::mem::MaybeUninit::uninit();
            let result =
                (self.get_native_buffer_properties_ohos)(device, buffer, properties.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(properties.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_memory_native_buffer_ohos(
        &self,
        device: Device,
        info: &MemoryGetNativeBufferInfoOHOS<'_>,
        buffer: &mut *mut OH_NativeBuffer,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_memory_native_buffer_ohos)(device, info, buffer);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}

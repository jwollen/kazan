#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_ANDROID_external_memory_android_hardware_buffer";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    pub type AHardwareBuffer = *const c_void;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportAndroidHardwareBufferInfoANDROID.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImportAndroidHardwareBufferInfoANDROID<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub buffer: *mut AHardwareBuffer,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImportAndroidHardwareBufferInfoANDROID<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImportAndroidHardwareBufferInfoANDROID")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("buffer", &self.buffer)
                .finish()
        }
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
        #[inline]
        pub fn buffer(mut self, buffer: *mut AHardwareBuffer) -> Self {
            self.buffer = buffer;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAndroidHardwareBufferUsageANDROID.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AndroidHardwareBufferUsageANDROID<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub android_hardware_buffer_usage: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AndroidHardwareBufferUsageANDROID<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AndroidHardwareBufferUsageANDROID")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "android_hardware_buffer_usage",
                    &self.android_hardware_buffer_usage,
                )
                .finish()
        }
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
        #[inline]
        pub fn android_hardware_buffer_usage(mut self, android_hardware_buffer_usage: u64) -> Self {
            self.android_hardware_buffer_usage = android_hardware_buffer_usage;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAndroidHardwareBufferPropertiesANDROID.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AndroidHardwareBufferPropertiesANDROID<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub allocation_size: DeviceSize,
        pub memory_type_bits: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AndroidHardwareBufferPropertiesANDROID<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AndroidHardwareBufferPropertiesANDROID")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("allocation_size", &self.allocation_size)
                .field("memory_type_bits", &self.memory_type_bits)
                .finish()
        }
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
        #[inline]
        pub fn allocation_size(mut self, allocation_size: DeviceSize) -> Self {
            self.allocation_size = allocation_size;
            self
        }

        #[inline]
        pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
            self.memory_type_bits = memory_type_bits;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryGetAndroidHardwareBufferInfoANDROID.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryGetAndroidHardwareBufferInfoANDROID<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryGetAndroidHardwareBufferInfoANDROID<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryGetAndroidHardwareBufferInfoANDROID")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory", &self.memory)
                .finish()
        }
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
        #[inline]
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAndroidHardwareBufferFormatPropertiesANDROID.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for AndroidHardwareBufferFormatPropertiesANDROID<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AndroidHardwareBufferFormatPropertiesANDROID")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("format", &self.format)
                .field("external_format", &self.external_format)
                .field("format_features", &self.format_features)
                .field(
                    "sampler_ycbcr_conversion_components",
                    &self.sampler_ycbcr_conversion_components,
                )
                .field("suggested_ycbcr_model", &self.suggested_ycbcr_model)
                .field("suggested_ycbcr_range", &self.suggested_ycbcr_range)
                .field("suggested_x_chroma_offset", &self.suggested_x_chroma_offset)
                .field("suggested_y_chroma_offset", &self.suggested_y_chroma_offset)
                .finish()
        }
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
        #[inline]
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }

        #[inline]
        pub fn external_format(mut self, external_format: u64) -> Self {
            self.external_format = external_format;
            self
        }

        #[inline]
        pub fn format_features(mut self, format_features: FormatFeatureFlags) -> Self {
            self.format_features = format_features;
            self
        }

        #[inline]
        pub fn sampler_ycbcr_conversion_components(
            mut self,
            sampler_ycbcr_conversion_components: ComponentMapping,
        ) -> Self {
            self.sampler_ycbcr_conversion_components = sampler_ycbcr_conversion_components;
            self
        }

        #[inline]
        pub fn suggested_ycbcr_model(
            mut self,
            suggested_ycbcr_model: SamplerYcbcrModelConversion,
        ) -> Self {
            self.suggested_ycbcr_model = suggested_ycbcr_model;
            self
        }

        #[inline]
        pub fn suggested_ycbcr_range(mut self, suggested_ycbcr_range: SamplerYcbcrRange) -> Self {
            self.suggested_ycbcr_range = suggested_ycbcr_range;
            self
        }

        #[inline]
        pub fn suggested_x_chroma_offset(
            mut self,
            suggested_x_chroma_offset: ChromaLocation,
        ) -> Self {
            self.suggested_x_chroma_offset = suggested_x_chroma_offset;
            self
        }

        #[inline]
        pub fn suggested_y_chroma_offset(
            mut self,
            suggested_y_chroma_offset: ChromaLocation,
        ) -> Self {
            self.suggested_y_chroma_offset = suggested_y_chroma_offset;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalFormatANDROID.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExternalFormatANDROID<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub external_format: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExternalFormatANDROID<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExternalFormatANDROID")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("external_format", &self.external_format)
                .finish()
        }
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
        #[inline]
        pub fn external_format(mut self, external_format: u64) -> Self {
            self.external_format = external_format;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAndroidHardwareBufferFormatProperties2ANDROID.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for AndroidHardwareBufferFormatProperties2ANDROID<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AndroidHardwareBufferFormatProperties2ANDROID")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("format", &self.format)
                .field("external_format", &self.external_format)
                .field("format_features", &self.format_features)
                .field(
                    "sampler_ycbcr_conversion_components",
                    &self.sampler_ycbcr_conversion_components,
                )
                .field("suggested_ycbcr_model", &self.suggested_ycbcr_model)
                .field("suggested_ycbcr_range", &self.suggested_ycbcr_range)
                .field("suggested_x_chroma_offset", &self.suggested_x_chroma_offset)
                .field("suggested_y_chroma_offset", &self.suggested_y_chroma_offset)
                .finish()
        }
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
        #[inline]
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }

        #[inline]
        pub fn external_format(mut self, external_format: u64) -> Self {
            self.external_format = external_format;
            self
        }

        #[inline]
        pub fn format_features(mut self, format_features: FormatFeatureFlags2) -> Self {
            self.format_features = format_features;
            self
        }

        #[inline]
        pub fn sampler_ycbcr_conversion_components(
            mut self,
            sampler_ycbcr_conversion_components: ComponentMapping,
        ) -> Self {
            self.sampler_ycbcr_conversion_components = sampler_ycbcr_conversion_components;
            self
        }

        #[inline]
        pub fn suggested_ycbcr_model(
            mut self,
            suggested_ycbcr_model: SamplerYcbcrModelConversion,
        ) -> Self {
            self.suggested_ycbcr_model = suggested_ycbcr_model;
            self
        }

        #[inline]
        pub fn suggested_ycbcr_range(mut self, suggested_ycbcr_range: SamplerYcbcrRange) -> Self {
            self.suggested_ycbcr_range = suggested_ycbcr_range;
            self
        }

        #[inline]
        pub fn suggested_x_chroma_offset(
            mut self,
            suggested_x_chroma_offset: ChromaLocation,
        ) -> Self {
            self.suggested_x_chroma_offset = suggested_x_chroma_offset;
            self
        }

        #[inline]
        pub fn suggested_y_chroma_offset(
            mut self,
            suggested_y_chroma_offset: ChromaLocation,
        ) -> Self {
            self.suggested_y_chroma_offset = suggested_y_chroma_offset;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html>
    pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID =
        unsafe extern "system" fn(
            device: Device,
            buffer: *const AHardwareBuffer,
            p_properties: *mut AndroidHardwareBufferPropertiesANDROID<'_>,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html>
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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html>
    #[inline]
    pub unsafe fn get_android_hardware_buffer_properties_android(
        &self,
        device: Device,
        buffer: *const AHardwareBuffer,
        properties: &mut AndroidHardwareBufferPropertiesANDROID<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result =
                (self.get_android_hardware_buffer_properties_android)(device, buffer, properties);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html>
    #[inline]
    pub unsafe fn get_memory_android_hardware_buffer_android(
        &self,
        device: Device,
        info: &MemoryGetAndroidHardwareBufferInfoANDROID<'_>,
    ) -> crate::Result<*mut AHardwareBuffer> {
        unsafe {
            let mut buffer = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_android_hardware_buffer_android)(
                device,
                info,
                buffer.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(buffer.assume_init()),
                err => Err(err),
            }
        }
    }
}

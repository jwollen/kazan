//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_OHOS_external_memory.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_OHOS_external_memory";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    pub type OH_NativeBuffer = *const c_void;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkNativeBufferUsageOHOS.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct NativeBufferUsageOHOS<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ohos_native_buffer_usage: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for NativeBufferUsageOHOS<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("NativeBufferUsageOHOS")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("ohos_native_buffer_usage", &self.ohos_native_buffer_usage)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for NativeBufferUsageOHOS<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::NATIVE_BUFFER_USAGE_OHOS;
    }

    unsafe impl Extends<ImageFormatProperties2<'_>> for NativeBufferUsageOHOS<'_> {}

    impl Default for NativeBufferUsageOHOS<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                ohos_native_buffer_usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> NativeBufferUsageOHOS<'a> {
        #[inline]
        pub fn ohos_native_buffer_usage(mut self, ohos_native_buffer_usage: u64) -> Self {
            self.ohos_native_buffer_usage = ohos_native_buffer_usage;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkNativeBufferPropertiesOHOS.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct NativeBufferPropertiesOHOS<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub allocation_size: DeviceSize,
        pub memory_type_bits: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for NativeBufferPropertiesOHOS<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("NativeBufferPropertiesOHOS")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("allocation_size", &self.allocation_size)
                .field("memory_type_bits", &self.memory_type_bits)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for NativeBufferPropertiesOHOS<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::NATIVE_BUFFER_PROPERTIES_OHOS;
    }

    impl Default for NativeBufferPropertiesOHOS<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                allocation_size: Default::default(),
                memory_type_bits: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> NativeBufferPropertiesOHOS<'a> {
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkNativeBufferFormatPropertiesOHOS.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for NativeBufferFormatPropertiesOHOS<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("NativeBufferFormatPropertiesOHOS")
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

    unsafe impl<'a> TaggedStructure<'a> for NativeBufferFormatPropertiesOHOS<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS;
    }

    unsafe impl Extends<NativeBufferPropertiesOHOS<'_>> for NativeBufferFormatPropertiesOHOS<'_> {}

    impl Default for NativeBufferFormatPropertiesOHOS<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportNativeBufferInfoOHOS.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImportNativeBufferInfoOHOS<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub buffer: *mut OH_NativeBuffer,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImportNativeBufferInfoOHOS<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImportNativeBufferInfoOHOS")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("buffer", &self.buffer)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImportNativeBufferInfoOHOS<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_NATIVE_BUFFER_INFO_OHOS;
    }

    unsafe impl Extends<MemoryAllocateInfo<'_>> for ImportNativeBufferInfoOHOS<'_> {}

    impl Default for ImportNativeBufferInfoOHOS<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                buffer: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportNativeBufferInfoOHOS<'a> {
        #[inline]
        pub fn buffer(mut self, buffer: *mut OH_NativeBuffer) -> Self {
            self.buffer = buffer;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryGetNativeBufferInfoOHOS.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryGetNativeBufferInfoOHOS<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryGetNativeBufferInfoOHOS<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryGetNativeBufferInfoOHOS")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory", &self.memory)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryGetNativeBufferInfoOHOS<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_GET_NATIVE_BUFFER_INFO_OHOS;
    }

    impl Default for MemoryGetNativeBufferInfoOHOS<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryGetNativeBufferInfoOHOS<'a> {
        #[inline]
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalFormatOHOS.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExternalFormatOHOS<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub external_format: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExternalFormatOHOS<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExternalFormatOHOS")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("external_format", &self.external_format)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExternalFormatOHOS<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_FORMAT_OHOS;
    }

    unsafe impl Extends<ImageCreateInfo<'_>> for ExternalFormatOHOS<'_> {}
    unsafe impl Extends<SamplerYcbcrConversionCreateInfo<'_>> for ExternalFormatOHOS<'_> {}
    unsafe impl Extends<AttachmentDescription2<'_>> for ExternalFormatOHOS<'_> {}
    unsafe impl Extends<GraphicsPipelineCreateInfo<'_>> for ExternalFormatOHOS<'_> {}
    unsafe impl Extends<CommandBufferInheritanceInfo<'_>> for ExternalFormatOHOS<'_> {}

    impl Default for ExternalFormatOHOS<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                external_format: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExternalFormatOHOS<'a> {
        #[inline]
        pub fn external_format(mut self, external_format: u64) -> Self {
            self.external_format = external_format;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetNativeBufferPropertiesOHOS.html>
    pub type PFN_vkGetNativeBufferPropertiesOHOS = unsafe extern "system" fn(
        device: Device,
        buffer: *const OH_NativeBuffer,
        p_properties: *mut NativeBufferPropertiesOHOS<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryNativeBufferOHOS.html>
    pub type PFN_vkGetMemoryNativeBufferOHOS = unsafe extern "system" fn(
        device: Device,
        p_info: *const MemoryGetNativeBufferInfoOHOS<'_>,
        p_buffer: *mut *mut OH_NativeBuffer,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkNativeBufferUsageOHOS = NativeBufferUsageOHOS<'static>;
    pub type VkNativeBufferPropertiesOHOS = NativeBufferPropertiesOHOS<'static>;
    pub type VkNativeBufferFormatPropertiesOHOS = NativeBufferFormatPropertiesOHOS<'static>;
    pub type VkImportNativeBufferInfoOHOS = ImportNativeBufferInfoOHOS<'static>;
    pub type VkMemoryGetNativeBufferInfoOHOS = MemoryGetNativeBufferInfoOHOS<'static>;
    pub type VkExternalFormatOHOS = ExternalFormatOHOS<'static>;
    impl NativeBufferUsageOHOS<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkNativeBufferUsageOHOS {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl NativeBufferPropertiesOHOS<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkNativeBufferPropertiesOHOS {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl NativeBufferFormatPropertiesOHOS<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkNativeBufferFormatPropertiesOHOS {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImportNativeBufferInfoOHOS<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImportNativeBufferInfoOHOS {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MemoryGetNativeBufferInfoOHOS<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMemoryGetNativeBufferInfoOHOS {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExternalFormatOHOS<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExternalFormatOHOS {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_native_buffer_properties: PFN_vkGetNativeBufferPropertiesOHOS,
    get_memory_native_buffer: PFN_vkGetMemoryNativeBufferOHOS,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_native_buffer_properties: transmute(
                    load(c"vkGetNativeBufferPropertiesOHOS").ok_or(MissingEntryPointError)?,
                ),
                get_memory_native_buffer: transmute(
                    load(c"vkGetMemoryNativeBufferOHOS").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetNativeBufferPropertiesOHOS.html>
    #[inline]
    pub unsafe fn get_native_buffer_properties(
        &self,
        device: Device,
        buffer: *const OH_NativeBuffer,
        properties: &mut NativeBufferPropertiesOHOS<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_native_buffer_properties)(device, buffer, properties);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryNativeBufferOHOS.html>
    #[inline]
    pub unsafe fn get_memory_native_buffer(
        &self,
        device: Device,
        info: &MemoryGetNativeBufferInfoOHOS<'_>,
    ) -> crate::Result<*mut OH_NativeBuffer> {
        unsafe {
            let mut buffer = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_native_buffer)(device, info, buffer.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(buffer.assume_init()),
                err => Err(err),
            }
        }
    }
}

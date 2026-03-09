#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    pub const MAX_DEVICE_GROUP_SIZE: u32 = 32;
    pub const LUID_SIZE: u32 = 8;
    pub const QUEUE_FAMILY_EXTERNAL: u32 = !1;

    handle_nondispatchable!(
        DescriptorUpdateTemplate,
        DESCRIPTOR_UPDATE_TEMPLATE,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorUpdateTemplate.html>"
    );
    handle_nondispatchable!(
        SamplerYcbcrConversion,
        SAMPLER_YCBCR_CONVERSION,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerYcbcrConversion.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVariablePointerFeatures.html>
    pub type PhysicalDeviceVariablePointerFeatures<'a> = PhysicalDeviceVariablePointersFeatures<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderDrawParameterFeatures.html>
    pub type PhysicalDeviceShaderDrawParameterFeatures<'a> =
        PhysicalDeviceShaderDrawParametersFeatures<'a>;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFeatures2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceFeatures2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub features: PhysicalDeviceFeatures,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceFeatures2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceFeatures2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("features", &self.features)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFeatures2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_FEATURES_2;
    }

    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceFeatures2<'a> {}

    impl Default for PhysicalDeviceFeatures2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                features: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceFeatures2<'a> {
        #[inline]
        pub fn features(mut self, features: PhysicalDeviceFeatures) -> Self {
            self.features = features;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceProperties2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceProperties2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub properties: PhysicalDeviceProperties,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceProperties2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceProperties2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("properties", &self.properties)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceProperties2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_PROPERTIES_2;
    }

    impl Default for PhysicalDeviceProperties2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceProperties2<'a> {
        #[inline]
        pub fn properties(mut self, properties: PhysicalDeviceProperties) -> Self {
            self.properties = properties;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFormatProperties2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct FormatProperties2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub format_properties: FormatProperties,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for FormatProperties2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("FormatProperties2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("format_properties", &self.format_properties)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for FormatProperties2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::FORMAT_PROPERTIES_2;
    }

    impl Default for FormatProperties2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                format_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> FormatProperties2<'a> {
        #[inline]
        pub fn format_properties(mut self, format_properties: FormatProperties) -> Self {
            self.format_properties = format_properties;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageFormatProperties2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageFormatProperties2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image_format_properties: ImageFormatProperties,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageFormatProperties2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageFormatProperties2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image_format_properties", &self.image_format_properties)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageFormatProperties2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_FORMAT_PROPERTIES_2;
    }

    impl Default for ImageFormatProperties2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                image_format_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageFormatProperties2<'a> {
        #[inline]
        pub fn image_format_properties(
            mut self,
            image_format_properties: ImageFormatProperties,
        ) -> Self {
            self.image_format_properties = image_format_properties;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImageFormatInfo2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceImageFormatInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub format: Format,
        pub ty: ImageType,
        pub tiling: ImageTiling,
        pub usage: ImageUsageFlags,
        pub flags: ImageCreateFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceImageFormatInfo2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceImageFormatInfo2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("format", &self.format)
                .field("ty", &self.ty)
                .field("tiling", &self.tiling)
                .field("usage", &self.usage)
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageFormatInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2;
    }

    impl Default for PhysicalDeviceImageFormatInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                format: Default::default(),
                ty: Default::default(),
                tiling: Default::default(),
                usage: Default::default(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceImageFormatInfo2<'a> {
        #[inline]
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }

        #[inline]
        pub fn ty(mut self, ty: ImageType) -> Self {
            self.ty = ty;
            self
        }

        #[inline]
        pub fn tiling(mut self, tiling: ImageTiling) -> Self {
            self.tiling = tiling;
            self
        }

        #[inline]
        pub fn usage(mut self, usage: ImageUsageFlags) -> Self {
            self.usage = usage;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: ImageCreateFlags) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueFamilyProperties2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct QueueFamilyProperties2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub queue_family_properties: QueueFamilyProperties,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for QueueFamilyProperties2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("QueueFamilyProperties2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("queue_family_properties", &self.queue_family_properties)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for QueueFamilyProperties2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::QUEUE_FAMILY_PROPERTIES_2;
    }

    impl Default for QueueFamilyProperties2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                queue_family_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> QueueFamilyProperties2<'a> {
        #[inline]
        pub fn queue_family_properties(
            mut self,
            queue_family_properties: QueueFamilyProperties,
        ) -> Self {
            self.queue_family_properties = queue_family_properties;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMemoryProperties2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMemoryProperties2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_properties: PhysicalDeviceMemoryProperties,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMemoryProperties2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMemoryProperties2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory_properties", &self.memory_properties)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMemoryProperties2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2;
    }

    impl Default for PhysicalDeviceMemoryProperties2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                memory_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMemoryProperties2<'a> {
        #[inline]
        pub fn memory_properties(
            mut self,
            memory_properties: PhysicalDeviceMemoryProperties,
        ) -> Self {
            self.memory_properties = memory_properties;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSparseImageFormatProperties2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SparseImageFormatProperties2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub properties: SparseImageFormatProperties,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SparseImageFormatProperties2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SparseImageFormatProperties2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("properties", &self.properties)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SparseImageFormatProperties2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SPARSE_IMAGE_FORMAT_PROPERTIES_2;
    }

    impl Default for SparseImageFormatProperties2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SparseImageFormatProperties2<'a> {
        #[inline]
        pub fn properties(mut self, properties: SparseImageFormatProperties) -> Self {
            self.properties = properties;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSparseImageFormatInfo2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceSparseImageFormatInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub format: Format,
        pub ty: ImageType,
        pub samples: SampleCountFlagBits,
        pub usage: ImageUsageFlags,
        pub tiling: ImageTiling,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceSparseImageFormatInfo2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceSparseImageFormatInfo2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("format", &self.format)
                .field("ty", &self.ty)
                .field("samples", &self.samples)
                .field("usage", &self.usage)
                .field("tiling", &self.tiling)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSparseImageFormatInfo2<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2;
    }

    impl Default for PhysicalDeviceSparseImageFormatInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                format: Default::default(),
                ty: Default::default(),
                samples: Default::default(),
                usage: Default::default(),
                tiling: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceSparseImageFormatInfo2<'a> {
        #[inline]
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }

        #[inline]
        pub fn ty(mut self, ty: ImageType) -> Self {
            self.ty = ty;
            self
        }

        #[inline]
        pub fn samples(mut self, samples: SampleCountFlagBits) -> Self {
            self.samples = samples;
            self
        }

        #[inline]
        pub fn usage(mut self, usage: ImageUsageFlags) -> Self {
            self.usage = usage;
            self
        }

        #[inline]
        pub fn tiling(mut self, tiling: ImageTiling) -> Self {
            self.tiling = tiling;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVariablePointersFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceVariablePointersFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub variable_pointers_storage_buffer: Bool32,
        pub variable_pointers: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceVariablePointersFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVariablePointersFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "variable_pointers_storage_buffer",
                    &self.variable_pointers_storage_buffer,
                )
                .field("variable_pointers", &self.variable_pointers)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVariablePointersFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceVariablePointersFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceVariablePointersFeatures<'a> {}

    impl Default for PhysicalDeviceVariablePointersFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                variable_pointers_storage_buffer: Default::default(),
                variable_pointers: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceVariablePointersFeatures<'a> {
        #[inline]
        pub fn variable_pointers_storage_buffer(
            mut self,
            variable_pointers_storage_buffer: bool,
        ) -> Self {
            self.variable_pointers_storage_buffer = variable_pointers_storage_buffer.into();
            self
        }

        #[inline]
        pub fn variable_pointers(mut self, variable_pointers: bool) -> Self {
            self.variable_pointers = variable_pointers.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryProperties.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct ExternalMemoryProperties {
        pub external_memory_features: ExternalMemoryFeatureFlags,
        pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
        pub compatible_handle_types: ExternalMemoryHandleTypeFlags,
    }

    impl ExternalMemoryProperties {
        #[inline]
        pub fn external_memory_features(
            mut self,
            external_memory_features: ExternalMemoryFeatureFlags,
        ) -> Self {
            self.external_memory_features = external_memory_features;
            self
        }

        #[inline]
        pub fn export_from_imported_handle_types(
            mut self,
            export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
        ) -> Self {
            self.export_from_imported_handle_types = export_from_imported_handle_types;
            self
        }

        #[inline]
        pub fn compatible_handle_types(
            mut self,
            compatible_handle_types: ExternalMemoryHandleTypeFlags,
        ) -> Self {
            self.compatible_handle_types = compatible_handle_types;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExternalImageFormatInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceExternalImageFormatInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceExternalImageFormatInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceExternalImageFormatInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("handle_type", &self.handle_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExternalImageFormatInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO;
    }

    unsafe impl<'a> Extends<PhysicalDeviceImageFormatInfo2<'a>>
        for PhysicalDeviceExternalImageFormatInfo<'a>
    {
    }

    impl Default for PhysicalDeviceExternalImageFormatInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceExternalImageFormatInfo<'a> {
        #[inline]
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalImageFormatProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExternalImageFormatProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub external_memory_properties: ExternalMemoryProperties,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExternalImageFormatProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExternalImageFormatProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "external_memory_properties",
                    &self.external_memory_properties,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExternalImageFormatProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_IMAGE_FORMAT_PROPERTIES;
    }

    unsafe impl<'a> Extends<ImageFormatProperties2<'a>> for ExternalImageFormatProperties<'a> {}

    impl Default for ExternalImageFormatProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                external_memory_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExternalImageFormatProperties<'a> {
        #[inline]
        pub fn external_memory_properties(
            mut self,
            external_memory_properties: ExternalMemoryProperties,
        ) -> Self {
            self.external_memory_properties = external_memory_properties;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExternalBufferInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceExternalBufferInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: BufferCreateFlags,
        pub usage: BufferUsageFlags,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceExternalBufferInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceExternalBufferInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("usage", &self.usage)
                .field("handle_type", &self.handle_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExternalBufferInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO;
    }

    impl Default for PhysicalDeviceExternalBufferInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                usage: Default::default(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceExternalBufferInfo<'a> {
        #[inline]
        pub fn flags(mut self, flags: BufferCreateFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn usage(mut self, usage: BufferUsageFlags) -> Self {
            self.usage = usage;
            self
        }

        #[inline]
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalBufferProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExternalBufferProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub external_memory_properties: ExternalMemoryProperties,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExternalBufferProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExternalBufferProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "external_memory_properties",
                    &self.external_memory_properties,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExternalBufferProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_BUFFER_PROPERTIES;
    }

    impl Default for ExternalBufferProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                external_memory_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExternalBufferProperties<'a> {
        #[inline]
        pub fn external_memory_properties(
            mut self,
            external_memory_properties: ExternalMemoryProperties,
        ) -> Self {
            self.external_memory_properties = external_memory_properties;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceIDProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceIDProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub device_uuid: [u8; UUID_SIZE as usize],
        pub driver_uuid: [u8; UUID_SIZE as usize],
        pub device_luid: [u8; LUID_SIZE as usize],
        pub device_node_mask: u32,
        pub device_luid_valid: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceIDProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceIDProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("device_uuid", &self.device_uuid)
                .field("driver_uuid", &self.driver_uuid)
                .field("device_luid", &self.device_luid)
                .field("device_node_mask", &self.device_node_mask)
                .field("device_luid_valid", &self.device_luid_valid)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceIDProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_ID_PROPERTIES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>> for PhysicalDeviceIDProperties<'a> {}

    impl Default for PhysicalDeviceIDProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                device_uuid: [Default::default(); _],
                driver_uuid: [Default::default(); _],
                device_luid: [Default::default(); _],
                device_node_mask: Default::default(),
                device_luid_valid: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceIDProperties<'a> {
        #[inline]
        pub fn device_uuid(mut self, device_uuid: [u8; UUID_SIZE as usize]) -> Self {
            self.device_uuid = device_uuid;
            self
        }

        #[inline]
        pub fn driver_uuid(mut self, driver_uuid: [u8; UUID_SIZE as usize]) -> Self {
            self.driver_uuid = driver_uuid;
            self
        }

        #[inline]
        pub fn device_luid(mut self, device_luid: [u8; LUID_SIZE as usize]) -> Self {
            self.device_luid = device_luid;
            self
        }

        #[inline]
        pub fn device_node_mask(mut self, device_node_mask: u32) -> Self {
            self.device_node_mask = device_node_mask;
            self
        }

        #[inline]
        pub fn device_luid_valid(mut self, device_luid_valid: bool) -> Self {
            self.device_luid_valid = device_luid_valid.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryImageCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExternalMemoryImageCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_types: ExternalMemoryHandleTypeFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExternalMemoryImageCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExternalMemoryImageCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("handle_types", &self.handle_types)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExternalMemoryImageCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO;
    }

    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for ExternalMemoryImageCreateInfo<'a> {}

    impl Default for ExternalMemoryImageCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                handle_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExternalMemoryImageCreateInfo<'a> {
        #[inline]
        pub fn handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlags) -> Self {
            self.handle_types = handle_types;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryBufferCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExternalMemoryBufferCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_types: ExternalMemoryHandleTypeFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExternalMemoryBufferCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExternalMemoryBufferCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("handle_types", &self.handle_types)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExternalMemoryBufferCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_MEMORY_BUFFER_CREATE_INFO;
    }

    unsafe impl<'a> Extends<BufferCreateInfo<'a>> for ExternalMemoryBufferCreateInfo<'a> {}

    impl Default for ExternalMemoryBufferCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                handle_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExternalMemoryBufferCreateInfo<'a> {
        #[inline]
        pub fn handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlags) -> Self {
            self.handle_types = handle_types;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMemoryAllocateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExportMemoryAllocateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_types: ExternalMemoryHandleTypeFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExportMemoryAllocateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExportMemoryAllocateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("handle_types", &self.handle_types)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExportMemoryAllocateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_MEMORY_ALLOCATE_INFO;
    }

    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for ExportMemoryAllocateInfo<'a> {}

    impl Default for ExportMemoryAllocateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                handle_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMemoryAllocateInfo<'a> {
        #[inline]
        pub fn handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlags) -> Self {
            self.handle_types = handle_types;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExternalSemaphoreInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceExternalSemaphoreInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceExternalSemaphoreInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceExternalSemaphoreInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("handle_type", &self.handle_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExternalSemaphoreInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO;
    }

    impl Default for PhysicalDeviceExternalSemaphoreInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceExternalSemaphoreInfo<'a> {
        #[inline]
        pub fn handle_type(mut self, handle_type: ExternalSemaphoreHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalSemaphoreProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExternalSemaphoreProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
        pub compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
        pub external_semaphore_features: ExternalSemaphoreFeatureFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExternalSemaphoreProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExternalSemaphoreProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "export_from_imported_handle_types",
                    &self.export_from_imported_handle_types,
                )
                .field("compatible_handle_types", &self.compatible_handle_types)
                .field(
                    "external_semaphore_features",
                    &self.external_semaphore_features,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExternalSemaphoreProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_SEMAPHORE_PROPERTIES;
    }

    impl Default for ExternalSemaphoreProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                export_from_imported_handle_types: Default::default(),
                compatible_handle_types: Default::default(),
                external_semaphore_features: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExternalSemaphoreProperties<'a> {
        #[inline]
        pub fn export_from_imported_handle_types(
            mut self,
            export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
        ) -> Self {
            self.export_from_imported_handle_types = export_from_imported_handle_types;
            self
        }

        #[inline]
        pub fn compatible_handle_types(
            mut self,
            compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
        ) -> Self {
            self.compatible_handle_types = compatible_handle_types;
            self
        }

        #[inline]
        pub fn external_semaphore_features(
            mut self,
            external_semaphore_features: ExternalSemaphoreFeatureFlags,
        ) -> Self {
            self.external_semaphore_features = external_semaphore_features;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportSemaphoreCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExportSemaphoreCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_types: ExternalSemaphoreHandleTypeFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExportSemaphoreCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExportSemaphoreCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("handle_types", &self.handle_types)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExportSemaphoreCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_SEMAPHORE_CREATE_INFO;
    }

    unsafe impl<'a> Extends<SemaphoreCreateInfo<'a>> for ExportSemaphoreCreateInfo<'a> {}

    impl Default for ExportSemaphoreCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                handle_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportSemaphoreCreateInfo<'a> {
        #[inline]
        pub fn handle_types(mut self, handle_types: ExternalSemaphoreHandleTypeFlags) -> Self {
            self.handle_types = handle_types;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceExternalFenceInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceExternalFenceInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_type: ExternalFenceHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceExternalFenceInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceExternalFenceInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("handle_type", &self.handle_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExternalFenceInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO;
    }

    impl Default for PhysicalDeviceExternalFenceInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceExternalFenceInfo<'a> {
        #[inline]
        pub fn handle_type(mut self, handle_type: ExternalFenceHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalFenceProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExternalFenceProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
        pub compatible_handle_types: ExternalFenceHandleTypeFlags,
        pub external_fence_features: ExternalFenceFeatureFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExternalFenceProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExternalFenceProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "export_from_imported_handle_types",
                    &self.export_from_imported_handle_types,
                )
                .field("compatible_handle_types", &self.compatible_handle_types)
                .field("external_fence_features", &self.external_fence_features)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExternalFenceProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_FENCE_PROPERTIES;
    }

    impl Default for ExternalFenceProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                export_from_imported_handle_types: Default::default(),
                compatible_handle_types: Default::default(),
                external_fence_features: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExternalFenceProperties<'a> {
        #[inline]
        pub fn export_from_imported_handle_types(
            mut self,
            export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
        ) -> Self {
            self.export_from_imported_handle_types = export_from_imported_handle_types;
            self
        }

        #[inline]
        pub fn compatible_handle_types(
            mut self,
            compatible_handle_types: ExternalFenceHandleTypeFlags,
        ) -> Self {
            self.compatible_handle_types = compatible_handle_types;
            self
        }

        #[inline]
        pub fn external_fence_features(
            mut self,
            external_fence_features: ExternalFenceFeatureFlags,
        ) -> Self {
            self.external_fence_features = external_fence_features;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportFenceCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExportFenceCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_types: ExternalFenceHandleTypeFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExportFenceCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExportFenceCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("handle_types", &self.handle_types)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExportFenceCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_FENCE_CREATE_INFO;
    }

    unsafe impl<'a> Extends<FenceCreateInfo<'a>> for ExportFenceCreateInfo<'a> {}

    impl Default for ExportFenceCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                handle_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportFenceCreateInfo<'a> {
        #[inline]
        pub fn handle_types(mut self, handle_types: ExternalFenceHandleTypeFlags) -> Self {
            self.handle_types = handle_types;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMultiviewFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMultiviewFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub multiview: Bool32,
        pub multiview_geometry_shader: Bool32,
        pub multiview_tessellation_shader: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMultiviewFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMultiviewFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("multiview", &self.multiview)
                .field("multiview_geometry_shader", &self.multiview_geometry_shader)
                .field(
                    "multiview_tessellation_shader",
                    &self.multiview_tessellation_shader,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMultiviewFeatures<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_MULTIVIEW_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceMultiviewFeatures<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceMultiviewFeatures<'a> {}

    impl Default for PhysicalDeviceMultiviewFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                multiview: Default::default(),
                multiview_geometry_shader: Default::default(),
                multiview_tessellation_shader: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMultiviewFeatures<'a> {
        #[inline]
        pub fn multiview(mut self, multiview: bool) -> Self {
            self.multiview = multiview.into();
            self
        }

        #[inline]
        pub fn multiview_geometry_shader(mut self, multiview_geometry_shader: bool) -> Self {
            self.multiview_geometry_shader = multiview_geometry_shader.into();
            self
        }

        #[inline]
        pub fn multiview_tessellation_shader(
            mut self,
            multiview_tessellation_shader: bool,
        ) -> Self {
            self.multiview_tessellation_shader = multiview_tessellation_shader.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMultiviewProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMultiviewProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_multiview_view_count: u32,
        pub max_multiview_instance_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMultiviewProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMultiviewProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_multiview_view_count", &self.max_multiview_view_count)
                .field(
                    "max_multiview_instance_index",
                    &self.max_multiview_instance_index,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMultiviewProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>> for PhysicalDeviceMultiviewProperties<'a> {}

    impl Default for PhysicalDeviceMultiviewProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_multiview_view_count: Default::default(),
                max_multiview_instance_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMultiviewProperties<'a> {
        #[inline]
        pub fn max_multiview_view_count(mut self, max_multiview_view_count: u32) -> Self {
            self.max_multiview_view_count = max_multiview_view_count;
            self
        }

        #[inline]
        pub fn max_multiview_instance_index(mut self, max_multiview_instance_index: u32) -> Self {
            self.max_multiview_instance_index = max_multiview_instance_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassMultiviewCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RenderPassMultiviewCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub subpass_count: u32,
        pub p_view_masks: *const u32,
        pub dependency_count: u32,
        pub p_view_offsets: *const i32,
        pub correlation_mask_count: u32,
        pub p_correlation_masks: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderPassMultiviewCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderPassMultiviewCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("subpass_count", &self.subpass_count)
                .field("p_view_masks", &self.p_view_masks)
                .field("dependency_count", &self.dependency_count)
                .field("p_view_offsets", &self.p_view_offsets)
                .field("correlation_mask_count", &self.correlation_mask_count)
                .field("p_correlation_masks", &self.p_correlation_masks)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderPassMultiviewCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDER_PASS_MULTIVIEW_CREATE_INFO;
    }

    unsafe impl<'a> Extends<RenderPassCreateInfo<'a>> for RenderPassMultiviewCreateInfo<'a> {}

    impl Default for RenderPassMultiviewCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                subpass_count: Default::default(),
                p_view_masks: ptr::null(),
                dependency_count: Default::default(),
                p_view_offsets: ptr::null(),
                correlation_mask_count: Default::default(),
                p_correlation_masks: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RenderPassMultiviewCreateInfo<'a> {
        #[inline]
        pub fn view_masks(mut self, view_masks: &'a [u32]) -> Self {
            self.subpass_count = view_masks.len().try_into().unwrap();
            self.p_view_masks = view_masks.as_ptr();
            self
        }

        #[inline]
        pub fn view_offsets(mut self, view_offsets: &'a [i32]) -> Self {
            self.dependency_count = view_offsets.len().try_into().unwrap();
            self.p_view_offsets = view_offsets.as_ptr();
            self
        }

        #[inline]
        pub fn correlation_masks(mut self, correlation_masks: &'a [u32]) -> Self {
            self.correlation_mask_count = correlation_masks.len().try_into().unwrap();
            self.p_correlation_masks = correlation_masks.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceGroupProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceGroupProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub physical_device_count: u32,
        pub physical_devices: [PhysicalDevice; MAX_DEVICE_GROUP_SIZE as usize],
        pub subset_allocation: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceGroupProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceGroupProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("physical_device_count", &self.physical_device_count)
                .field("physical_devices", &self.physical_devices)
                .field("subset_allocation", &self.subset_allocation)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceGroupProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_GROUP_PROPERTIES;
    }

    impl Default for PhysicalDeviceGroupProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                physical_device_count: Default::default(),
                physical_devices: [Default::default(); _],
                subset_allocation: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceGroupProperties<'a> {
        #[inline]
        pub fn physical_devices(mut self, physical_devices: &[PhysicalDevice]) -> Self {
            self.physical_device_count = physical_devices.len().try_into().unwrap();
            self.physical_devices[..physical_devices.len()].copy_from_slice(physical_devices);
            self
        }

        #[inline]
        pub fn subset_allocation(mut self, subset_allocation: bool) -> Self {
            self.subset_allocation = subset_allocation.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryAllocateFlagsInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryAllocateFlagsInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: MemoryAllocateFlags,
        pub device_mask: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryAllocateFlagsInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryAllocateFlagsInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("device_mask", &self.device_mask)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryAllocateFlagsInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_ALLOCATE_FLAGS_INFO;
    }

    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for MemoryAllocateFlagsInfo<'a> {}

    impl Default for MemoryAllocateFlagsInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                device_mask: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryAllocateFlagsInfo<'a> {
        #[inline]
        pub fn flags(mut self, flags: MemoryAllocateFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn device_mask(mut self, device_mask: u32) -> Self {
            self.device_mask = device_mask;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindBufferMemoryInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BindBufferMemoryInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub buffer: Buffer,
        pub memory: DeviceMemory,
        pub memory_offset: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BindBufferMemoryInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BindBufferMemoryInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("buffer", &self.buffer)
                .field("memory", &self.memory)
                .field("memory_offset", &self.memory_offset)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BindBufferMemoryInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_BUFFER_MEMORY_INFO;
    }

    impl Default for BindBufferMemoryInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                buffer: Default::default(),
                memory: Default::default(),
                memory_offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BindBufferMemoryInfo<'a> {
        #[inline]
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }

        #[inline]
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }

        #[inline]
        pub fn memory_offset(mut self, memory_offset: DeviceSize) -> Self {
            self.memory_offset = memory_offset;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindBufferMemoryDeviceGroupInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BindBufferMemoryDeviceGroupInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub device_index_count: u32,
        pub p_device_indices: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BindBufferMemoryDeviceGroupInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BindBufferMemoryDeviceGroupInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("device_index_count", &self.device_index_count)
                .field("p_device_indices", &self.p_device_indices)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BindBufferMemoryDeviceGroupInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO;
    }

    unsafe impl<'a> Extends<BindBufferMemoryInfo<'a>> for BindBufferMemoryDeviceGroupInfo<'a> {}

    impl Default for BindBufferMemoryDeviceGroupInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                device_index_count: Default::default(),
                p_device_indices: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BindBufferMemoryDeviceGroupInfo<'a> {
        #[inline]
        pub fn device_indices(mut self, device_indices: &'a [u32]) -> Self {
            self.device_index_count = device_indices.len().try_into().unwrap();
            self.p_device_indices = device_indices.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindImageMemoryInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BindImageMemoryInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image: Image,
        pub memory: DeviceMemory,
        pub memory_offset: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BindImageMemoryInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BindImageMemoryInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image", &self.image)
                .field("memory", &self.memory)
                .field("memory_offset", &self.memory_offset)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BindImageMemoryInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_IMAGE_MEMORY_INFO;
    }

    impl Default for BindImageMemoryInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                image: Default::default(),
                memory: Default::default(),
                memory_offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BindImageMemoryInfo<'a> {
        #[inline]
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }

        #[inline]
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }

        #[inline]
        pub fn memory_offset(mut self, memory_offset: DeviceSize) -> Self {
            self.memory_offset = memory_offset;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindImageMemoryDeviceGroupInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BindImageMemoryDeviceGroupInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub device_index_count: u32,
        pub p_device_indices: *const u32,
        pub split_instance_bind_region_count: u32,
        pub p_split_instance_bind_regions: *const Rect2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BindImageMemoryDeviceGroupInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BindImageMemoryDeviceGroupInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("device_index_count", &self.device_index_count)
                .field("p_device_indices", &self.p_device_indices)
                .field(
                    "split_instance_bind_region_count",
                    &self.split_instance_bind_region_count,
                )
                .field(
                    "p_split_instance_bind_regions",
                    &self.p_split_instance_bind_regions,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BindImageMemoryDeviceGroupInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO;
    }

    unsafe impl<'a> Extends<BindImageMemoryInfo<'a>> for BindImageMemoryDeviceGroupInfo<'a> {}

    impl Default for BindImageMemoryDeviceGroupInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                device_index_count: Default::default(),
                p_device_indices: ptr::null(),
                split_instance_bind_region_count: Default::default(),
                p_split_instance_bind_regions: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BindImageMemoryDeviceGroupInfo<'a> {
        #[inline]
        pub fn device_indices(mut self, device_indices: &'a [u32]) -> Self {
            self.device_index_count = device_indices.len().try_into().unwrap();
            self.p_device_indices = device_indices.as_ptr();
            self
        }

        #[inline]
        pub fn split_instance_bind_regions(
            mut self,
            split_instance_bind_regions: &'a [Rect2D],
        ) -> Self {
            self.split_instance_bind_region_count =
                split_instance_bind_regions.len().try_into().unwrap();
            self.p_split_instance_bind_regions = split_instance_bind_regions.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceGroupRenderPassBeginInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceGroupRenderPassBeginInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub device_mask: u32,
        pub device_render_area_count: u32,
        pub p_device_render_areas: *const Rect2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceGroupRenderPassBeginInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceGroupRenderPassBeginInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("device_mask", &self.device_mask)
                .field("device_render_area_count", &self.device_render_area_count)
                .field("p_device_render_areas", &self.p_device_render_areas)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceGroupRenderPassBeginInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO;
    }

    unsafe impl<'a> Extends<RenderPassBeginInfo<'a>> for DeviceGroupRenderPassBeginInfo<'a> {}
    unsafe impl<'a> Extends<RenderingInfo<'a>> for DeviceGroupRenderPassBeginInfo<'a> {}

    impl Default for DeviceGroupRenderPassBeginInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                device_mask: Default::default(),
                device_render_area_count: Default::default(),
                p_device_render_areas: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceGroupRenderPassBeginInfo<'a> {
        #[inline]
        pub fn device_mask(mut self, device_mask: u32) -> Self {
            self.device_mask = device_mask;
            self
        }

        #[inline]
        pub fn device_render_areas(mut self, device_render_areas: &'a [Rect2D]) -> Self {
            self.device_render_area_count = device_render_areas.len().try_into().unwrap();
            self.p_device_render_areas = device_render_areas.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceGroupCommandBufferBeginInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceGroupCommandBufferBeginInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub device_mask: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceGroupCommandBufferBeginInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceGroupCommandBufferBeginInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("device_mask", &self.device_mask)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceGroupCommandBufferBeginInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO;
    }

    unsafe impl<'a> Extends<CommandBufferBeginInfo<'a>> for DeviceGroupCommandBufferBeginInfo<'a> {}

    impl Default for DeviceGroupCommandBufferBeginInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                device_mask: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceGroupCommandBufferBeginInfo<'a> {
        #[inline]
        pub fn device_mask(mut self, device_mask: u32) -> Self {
            self.device_mask = device_mask;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceGroupSubmitInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceGroupSubmitInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub wait_semaphore_count: u32,
        pub p_wait_semaphore_device_indices: *const u32,
        pub command_buffer_count: u32,
        pub p_command_buffer_device_masks: *const u32,
        pub signal_semaphore_count: u32,
        pub p_signal_semaphore_device_indices: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceGroupSubmitInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceGroupSubmitInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("wait_semaphore_count", &self.wait_semaphore_count)
                .field(
                    "p_wait_semaphore_device_indices",
                    &self.p_wait_semaphore_device_indices,
                )
                .field("command_buffer_count", &self.command_buffer_count)
                .field(
                    "p_command_buffer_device_masks",
                    &self.p_command_buffer_device_masks,
                )
                .field("signal_semaphore_count", &self.signal_semaphore_count)
                .field(
                    "p_signal_semaphore_device_indices",
                    &self.p_signal_semaphore_device_indices,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceGroupSubmitInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_GROUP_SUBMIT_INFO;
    }

    unsafe impl<'a> Extends<SubmitInfo<'a>> for DeviceGroupSubmitInfo<'a> {}

    impl Default for DeviceGroupSubmitInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                wait_semaphore_count: Default::default(),
                p_wait_semaphore_device_indices: ptr::null(),
                command_buffer_count: Default::default(),
                p_command_buffer_device_masks: ptr::null(),
                signal_semaphore_count: Default::default(),
                p_signal_semaphore_device_indices: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceGroupSubmitInfo<'a> {
        #[inline]
        pub fn wait_semaphore_device_indices(
            mut self,
            wait_semaphore_device_indices: &'a [u32],
        ) -> Self {
            self.wait_semaphore_count = wait_semaphore_device_indices.len().try_into().unwrap();
            self.p_wait_semaphore_device_indices = wait_semaphore_device_indices.as_ptr();
            self
        }

        #[inline]
        pub fn command_buffer_device_masks(
            mut self,
            command_buffer_device_masks: &'a [u32],
        ) -> Self {
            self.command_buffer_count = command_buffer_device_masks.len().try_into().unwrap();
            self.p_command_buffer_device_masks = command_buffer_device_masks.as_ptr();
            self
        }

        #[inline]
        pub fn signal_semaphore_device_indices(
            mut self,
            signal_semaphore_device_indices: &'a [u32],
        ) -> Self {
            self.signal_semaphore_count = signal_semaphore_device_indices.len().try_into().unwrap();
            self.p_signal_semaphore_device_indices = signal_semaphore_device_indices.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceGroupBindSparseInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceGroupBindSparseInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub resource_device_index: u32,
        pub memory_device_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceGroupBindSparseInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceGroupBindSparseInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("resource_device_index", &self.resource_device_index)
                .field("memory_device_index", &self.memory_device_index)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceGroupBindSparseInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_GROUP_BIND_SPARSE_INFO;
    }

    unsafe impl<'a> Extends<BindSparseInfo<'a>> for DeviceGroupBindSparseInfo<'a> {}

    impl Default for DeviceGroupBindSparseInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                resource_device_index: Default::default(),
                memory_device_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceGroupBindSparseInfo<'a> {
        #[inline]
        pub fn resource_device_index(mut self, resource_device_index: u32) -> Self {
            self.resource_device_index = resource_device_index;
            self
        }

        #[inline]
        pub fn memory_device_index(mut self, memory_device_index: u32) -> Self {
            self.memory_device_index = memory_device_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceGroupDeviceCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceGroupDeviceCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub physical_device_count: u32,
        pub p_physical_devices: *const PhysicalDevice,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceGroupDeviceCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceGroupDeviceCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("physical_device_count", &self.physical_device_count)
                .field("p_physical_devices", &self.p_physical_devices)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceGroupDeviceCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_GROUP_DEVICE_CREATE_INFO;
    }

    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for DeviceGroupDeviceCreateInfo<'a> {}

    impl Default for DeviceGroupDeviceCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                physical_device_count: Default::default(),
                p_physical_devices: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceGroupDeviceCreateInfo<'a> {
        #[inline]
        pub fn physical_devices(mut self, physical_devices: &'a [PhysicalDevice]) -> Self {
            self.physical_device_count = physical_devices.len().try_into().unwrap();
            self.p_physical_devices = physical_devices.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorUpdateTemplateEntry.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct DescriptorUpdateTemplateEntry {
        pub dst_binding: u32,
        pub dst_array_element: u32,
        pub descriptor_count: u32,
        pub descriptor_type: DescriptorType,
        pub offset: usize,
        pub stride: usize,
    }

    impl DescriptorUpdateTemplateEntry {
        #[inline]
        pub fn dst_binding(mut self, dst_binding: u32) -> Self {
            self.dst_binding = dst_binding;
            self
        }

        #[inline]
        pub fn dst_array_element(mut self, dst_array_element: u32) -> Self {
            self.dst_array_element = dst_array_element;
            self
        }

        #[inline]
        pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
            self.descriptor_count = descriptor_count;
            self
        }

        #[inline]
        pub fn descriptor_type(mut self, descriptor_type: DescriptorType) -> Self {
            self.descriptor_type = descriptor_type;
            self
        }

        #[inline]
        pub fn offset(mut self, offset: usize) -> Self {
            self.offset = offset;
            self
        }

        #[inline]
        pub fn stride(mut self, stride: usize) -> Self {
            self.stride = stride;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorUpdateTemplateCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DescriptorUpdateTemplateCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DescriptorUpdateTemplateCreateFlags,
        pub descriptor_update_entry_count: u32,
        pub p_descriptor_update_entries: *const DescriptorUpdateTemplateEntry,
        pub template_type: DescriptorUpdateTemplateType,
        pub descriptor_set_layout: DescriptorSetLayout,
        pub pipeline_bind_point: PipelineBindPoint,
        pub pipeline_layout: PipelineLayout,
        pub set: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorUpdateTemplateCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorUpdateTemplateCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field(
                    "descriptor_update_entry_count",
                    &self.descriptor_update_entry_count,
                )
                .field(
                    "p_descriptor_update_entries",
                    &self.p_descriptor_update_entries,
                )
                .field("template_type", &self.template_type)
                .field("descriptor_set_layout", &self.descriptor_set_layout)
                .field("pipeline_bind_point", &self.pipeline_bind_point)
                .field("pipeline_layout", &self.pipeline_layout)
                .field("set", &self.set)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DescriptorUpdateTemplateCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO;
    }

    impl Default for DescriptorUpdateTemplateCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                descriptor_update_entry_count: Default::default(),
                p_descriptor_update_entries: ptr::null(),
                template_type: Default::default(),
                descriptor_set_layout: Default::default(),
                pipeline_bind_point: Default::default(),
                pipeline_layout: Default::default(),
                set: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DescriptorUpdateTemplateCreateInfo<'a> {
        #[inline]
        pub fn flags(mut self, flags: DescriptorUpdateTemplateCreateFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn descriptor_update_entries(
            mut self,
            descriptor_update_entries: &'a [DescriptorUpdateTemplateEntry],
        ) -> Self {
            self.descriptor_update_entry_count =
                descriptor_update_entries.len().try_into().unwrap();
            self.p_descriptor_update_entries = descriptor_update_entries.as_ptr();
            self
        }

        #[inline]
        pub fn template_type(mut self, template_type: DescriptorUpdateTemplateType) -> Self {
            self.template_type = template_type;
            self
        }

        #[inline]
        pub fn descriptor_set_layout(mut self, descriptor_set_layout: DescriptorSetLayout) -> Self {
            self.descriptor_set_layout = descriptor_set_layout;
            self
        }

        #[inline]
        pub fn pipeline_bind_point(mut self, pipeline_bind_point: PipelineBindPoint) -> Self {
            self.pipeline_bind_point = pipeline_bind_point;
            self
        }

        #[inline]
        pub fn pipeline_layout(mut self, pipeline_layout: PipelineLayout) -> Self {
            self.pipeline_layout = pipeline_layout;
            self
        }

        #[inline]
        pub fn set(mut self, set: u32) -> Self {
            self.set = set;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkInputAttachmentAspectReference.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct InputAttachmentAspectReference {
        pub subpass: u32,
        pub input_attachment_index: u32,
        pub aspect_mask: ImageAspectFlags,
    }

    impl InputAttachmentAspectReference {
        #[inline]
        pub fn subpass(mut self, subpass: u32) -> Self {
            self.subpass = subpass;
            self
        }

        #[inline]
        pub fn input_attachment_index(mut self, input_attachment_index: u32) -> Self {
            self.input_attachment_index = input_attachment_index;
            self
        }

        #[inline]
        pub fn aspect_mask(mut self, aspect_mask: ImageAspectFlags) -> Self {
            self.aspect_mask = aspect_mask;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassInputAttachmentAspectCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RenderPassInputAttachmentAspectCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub aspect_reference_count: u32,
        pub p_aspect_references: *const InputAttachmentAspectReference,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderPassInputAttachmentAspectCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderPassInputAttachmentAspectCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("aspect_reference_count", &self.aspect_reference_count)
                .field("p_aspect_references", &self.p_aspect_references)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderPassInputAttachmentAspectCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO;
    }

    unsafe impl<'a> Extends<RenderPassCreateInfo<'a>>
        for RenderPassInputAttachmentAspectCreateInfo<'a>
    {
    }

    impl Default for RenderPassInputAttachmentAspectCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                aspect_reference_count: Default::default(),
                p_aspect_references: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RenderPassInputAttachmentAspectCreateInfo<'a> {
        #[inline]
        pub fn aspect_references(
            mut self,
            aspect_references: &'a [InputAttachmentAspectReference],
        ) -> Self {
            self.aspect_reference_count = aspect_references.len().try_into().unwrap();
            self.p_aspect_references = aspect_references.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevice16BitStorageFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevice16BitStorageFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub storage_buffer16_bit_access: Bool32,
        pub uniform_and_storage_buffer16_bit_access: Bool32,
        pub storage_push_constant16: Bool32,
        pub storage_input_output16: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevice16BitStorageFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevice16BitStorageFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "storage_buffer16_bit_access",
                    &self.storage_buffer16_bit_access,
                )
                .field(
                    "uniform_and_storage_buffer16_bit_access",
                    &self.uniform_and_storage_buffer16_bit_access,
                )
                .field("storage_push_constant16", &self.storage_push_constant16)
                .field("storage_input_output16", &self.storage_input_output16)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevice16BitStorageFeatures<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDevice16BitStorageFeatures<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDevice16BitStorageFeatures<'a> {}

    impl Default for PhysicalDevice16BitStorageFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                storage_buffer16_bit_access: Default::default(),
                uniform_and_storage_buffer16_bit_access: Default::default(),
                storage_push_constant16: Default::default(),
                storage_input_output16: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevice16BitStorageFeatures<'a> {
        #[inline]
        pub fn storage_buffer16_bit_access(mut self, storage_buffer16_bit_access: bool) -> Self {
            self.storage_buffer16_bit_access = storage_buffer16_bit_access.into();
            self
        }

        #[inline]
        pub fn uniform_and_storage_buffer16_bit_access(
            mut self,
            uniform_and_storage_buffer16_bit_access: bool,
        ) -> Self {
            self.uniform_and_storage_buffer16_bit_access =
                uniform_and_storage_buffer16_bit_access.into();
            self
        }

        #[inline]
        pub fn storage_push_constant16(mut self, storage_push_constant16: bool) -> Self {
            self.storage_push_constant16 = storage_push_constant16.into();
            self
        }

        #[inline]
        pub fn storage_input_output16(mut self, storage_input_output16: bool) -> Self {
            self.storage_input_output16 = storage_input_output16.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSubgroupProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceSubgroupProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub subgroup_size: u32,
        pub supported_stages: ShaderStageFlags,
        pub supported_operations: SubgroupFeatureFlags,
        pub quad_operations_in_all_stages: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceSubgroupProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceSubgroupProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("subgroup_size", &self.subgroup_size)
                .field("supported_stages", &self.supported_stages)
                .field("supported_operations", &self.supported_operations)
                .field(
                    "quad_operations_in_all_stages",
                    &self.quad_operations_in_all_stages,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSubgroupProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>> for PhysicalDeviceSubgroupProperties<'a> {}

    impl Default for PhysicalDeviceSubgroupProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                subgroup_size: Default::default(),
                supported_stages: Default::default(),
                supported_operations: Default::default(),
                quad_operations_in_all_stages: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceSubgroupProperties<'a> {
        #[inline]
        pub fn subgroup_size(mut self, subgroup_size: u32) -> Self {
            self.subgroup_size = subgroup_size;
            self
        }

        #[inline]
        pub fn supported_stages(mut self, supported_stages: ShaderStageFlags) -> Self {
            self.supported_stages = supported_stages;
            self
        }

        #[inline]
        pub fn supported_operations(mut self, supported_operations: SubgroupFeatureFlags) -> Self {
            self.supported_operations = supported_operations;
            self
        }

        #[inline]
        pub fn quad_operations_in_all_stages(
            mut self,
            quad_operations_in_all_stages: bool,
        ) -> Self {
            self.quad_operations_in_all_stages = quad_operations_in_all_stages.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferMemoryRequirementsInfo2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BufferMemoryRequirementsInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub buffer: Buffer,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BufferMemoryRequirementsInfo2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BufferMemoryRequirementsInfo2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("buffer", &self.buffer)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BufferMemoryRequirementsInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_MEMORY_REQUIREMENTS_INFO_2;
    }

    impl Default for BufferMemoryRequirementsInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                buffer: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BufferMemoryRequirementsInfo2<'a> {
        #[inline]
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageMemoryRequirementsInfo2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageMemoryRequirementsInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image: Image,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageMemoryRequirementsInfo2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageMemoryRequirementsInfo2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image", &self.image)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageMemoryRequirementsInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_MEMORY_REQUIREMENTS_INFO_2;
    }

    impl Default for ImageMemoryRequirementsInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                image: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageMemoryRequirementsInfo2<'a> {
        #[inline]
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageSparseMemoryRequirementsInfo2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageSparseMemoryRequirementsInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image: Image,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageSparseMemoryRequirementsInfo2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageSparseMemoryRequirementsInfo2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image", &self.image)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageSparseMemoryRequirementsInfo2<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2;
    }

    impl Default for ImageSparseMemoryRequirementsInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                image: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageSparseMemoryRequirementsInfo2<'a> {
        #[inline]
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryRequirements2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryRequirements2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_requirements: MemoryRequirements,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryRequirements2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryRequirements2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory_requirements", &self.memory_requirements)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryRequirements2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_REQUIREMENTS_2;
    }

    impl Default for MemoryRequirements2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                memory_requirements: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryRequirements2<'a> {
        #[inline]
        pub fn memory_requirements(mut self, memory_requirements: MemoryRequirements) -> Self {
            self.memory_requirements = memory_requirements;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSparseImageMemoryRequirements2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SparseImageMemoryRequirements2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_requirements: SparseImageMemoryRequirements,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SparseImageMemoryRequirements2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SparseImageMemoryRequirements2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory_requirements", &self.memory_requirements)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SparseImageMemoryRequirements2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2;
    }

    impl Default for SparseImageMemoryRequirements2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                memory_requirements: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SparseImageMemoryRequirements2<'a> {
        #[inline]
        pub fn memory_requirements(
            mut self,
            memory_requirements: SparseImageMemoryRequirements,
        ) -> Self {
            self.memory_requirements = memory_requirements;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePointClippingProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePointClippingProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub point_clipping_behavior: PointClippingBehavior,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePointClippingProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePointClippingProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("point_clipping_behavior", &self.point_clipping_behavior)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePointClippingProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDevicePointClippingProperties<'a>
    {
    }

    impl Default for PhysicalDevicePointClippingProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                point_clipping_behavior: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePointClippingProperties<'a> {
        #[inline]
        pub fn point_clipping_behavior(
            mut self,
            point_clipping_behavior: PointClippingBehavior,
        ) -> Self {
            self.point_clipping_behavior = point_clipping_behavior;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryDedicatedRequirements.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryDedicatedRequirements<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub prefers_dedicated_allocation: Bool32,
        pub requires_dedicated_allocation: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryDedicatedRequirements<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryDedicatedRequirements")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "prefers_dedicated_allocation",
                    &self.prefers_dedicated_allocation,
                )
                .field(
                    "requires_dedicated_allocation",
                    &self.requires_dedicated_allocation,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryDedicatedRequirements<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_DEDICATED_REQUIREMENTS;
    }

    unsafe impl<'a> Extends<MemoryRequirements2<'a>> for MemoryDedicatedRequirements<'a> {}

    impl Default for MemoryDedicatedRequirements<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                prefers_dedicated_allocation: Default::default(),
                requires_dedicated_allocation: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryDedicatedRequirements<'a> {
        #[inline]
        pub fn prefers_dedicated_allocation(mut self, prefers_dedicated_allocation: bool) -> Self {
            self.prefers_dedicated_allocation = prefers_dedicated_allocation.into();
            self
        }

        #[inline]
        pub fn requires_dedicated_allocation(
            mut self,
            requires_dedicated_allocation: bool,
        ) -> Self {
            self.requires_dedicated_allocation = requires_dedicated_allocation.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryDedicatedAllocateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryDedicatedAllocateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image: Image,
        pub buffer: Buffer,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryDedicatedAllocateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryDedicatedAllocateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image", &self.image)
                .field("buffer", &self.buffer)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryDedicatedAllocateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_DEDICATED_ALLOCATE_INFO;
    }

    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for MemoryDedicatedAllocateInfo<'a> {}

    impl Default for MemoryDedicatedAllocateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                image: Default::default(),
                buffer: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryDedicatedAllocateInfo<'a> {
        #[inline]
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }

        #[inline]
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageViewUsageCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageViewUsageCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub usage: ImageUsageFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageViewUsageCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageViewUsageCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("usage", &self.usage)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageViewUsageCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_VIEW_USAGE_CREATE_INFO;
    }

    unsafe impl<'a> Extends<ImageViewCreateInfo<'a>> for ImageViewUsageCreateInfo<'a> {}

    impl Default for ImageViewUsageCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageViewUsageCreateInfo<'a> {
        #[inline]
        pub fn usage(mut self, usage: ImageUsageFlags) -> Self {
            self.usage = usage;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineTessellationDomainOriginStateCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineTessellationDomainOriginStateCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub domain_origin: TessellationDomainOrigin,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineTessellationDomainOriginStateCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineTessellationDomainOriginStateCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("domain_origin", &self.domain_origin)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineTessellationDomainOriginStateCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO;
    }

    unsafe impl<'a> Extends<PipelineTessellationStateCreateInfo<'a>>
        for PipelineTessellationDomainOriginStateCreateInfo<'a>
    {
    }

    impl Default for PipelineTessellationDomainOriginStateCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                domain_origin: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineTessellationDomainOriginStateCreateInfo<'a> {
        #[inline]
        pub fn domain_origin(mut self, domain_origin: TessellationDomainOrigin) -> Self {
            self.domain_origin = domain_origin;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerYcbcrConversionInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SamplerYcbcrConversionInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub conversion: SamplerYcbcrConversion,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SamplerYcbcrConversionInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SamplerYcbcrConversionInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("conversion", &self.conversion)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SamplerYcbcrConversionInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SAMPLER_YCBCR_CONVERSION_INFO;
    }

    unsafe impl<'a> Extends<SamplerCreateInfo<'a>> for SamplerYcbcrConversionInfo<'a> {}
    unsafe impl<'a> Extends<ImageViewCreateInfo<'a>> for SamplerYcbcrConversionInfo<'a> {}

    impl Default for SamplerYcbcrConversionInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                conversion: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SamplerYcbcrConversionInfo<'a> {
        #[inline]
        pub fn conversion(mut self, conversion: SamplerYcbcrConversion) -> Self {
            self.conversion = conversion;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerYcbcrConversionCreateInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SamplerYcbcrConversionCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub format: Format,
        pub ycbcr_model: SamplerYcbcrModelConversion,
        pub ycbcr_range: SamplerYcbcrRange,
        pub components: ComponentMapping,
        pub x_chroma_offset: ChromaLocation,
        pub y_chroma_offset: ChromaLocation,
        pub chroma_filter: Filter,
        pub force_explicit_reconstruction: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SamplerYcbcrConversionCreateInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SamplerYcbcrConversionCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("format", &self.format)
                .field("ycbcr_model", &self.ycbcr_model)
                .field("ycbcr_range", &self.ycbcr_range)
                .field("components", &self.components)
                .field("x_chroma_offset", &self.x_chroma_offset)
                .field("y_chroma_offset", &self.y_chroma_offset)
                .field("chroma_filter", &self.chroma_filter)
                .field(
                    "force_explicit_reconstruction",
                    &self.force_explicit_reconstruction,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SamplerYcbcrConversionCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SAMPLER_YCBCR_CONVERSION_CREATE_INFO;
    }

    impl Default for SamplerYcbcrConversionCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                format: Default::default(),
                ycbcr_model: Default::default(),
                ycbcr_range: Default::default(),
                components: Default::default(),
                x_chroma_offset: Default::default(),
                y_chroma_offset: Default::default(),
                chroma_filter: Default::default(),
                force_explicit_reconstruction: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SamplerYcbcrConversionCreateInfo<'a> {
        #[inline]
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }

        #[inline]
        pub fn ycbcr_model(mut self, ycbcr_model: SamplerYcbcrModelConversion) -> Self {
            self.ycbcr_model = ycbcr_model;
            self
        }

        #[inline]
        pub fn ycbcr_range(mut self, ycbcr_range: SamplerYcbcrRange) -> Self {
            self.ycbcr_range = ycbcr_range;
            self
        }

        #[inline]
        pub fn components(mut self, components: ComponentMapping) -> Self {
            self.components = components;
            self
        }

        #[inline]
        pub fn x_chroma_offset(mut self, x_chroma_offset: ChromaLocation) -> Self {
            self.x_chroma_offset = x_chroma_offset;
            self
        }

        #[inline]
        pub fn y_chroma_offset(mut self, y_chroma_offset: ChromaLocation) -> Self {
            self.y_chroma_offset = y_chroma_offset;
            self
        }

        #[inline]
        pub fn chroma_filter(mut self, chroma_filter: Filter) -> Self {
            self.chroma_filter = chroma_filter;
            self
        }

        #[inline]
        pub fn force_explicit_reconstruction(
            mut self,
            force_explicit_reconstruction: bool,
        ) -> Self {
            self.force_explicit_reconstruction = force_explicit_reconstruction.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindImagePlaneMemoryInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BindImagePlaneMemoryInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub plane_aspect: ImageAspectFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BindImagePlaneMemoryInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BindImagePlaneMemoryInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("plane_aspect", &self.plane_aspect)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BindImagePlaneMemoryInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_IMAGE_PLANE_MEMORY_INFO;
    }

    unsafe impl<'a> Extends<BindImageMemoryInfo<'a>> for BindImagePlaneMemoryInfo<'a> {}

    impl Default for BindImagePlaneMemoryInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                plane_aspect: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BindImagePlaneMemoryInfo<'a> {
        #[inline]
        pub fn plane_aspect(mut self, plane_aspect: ImageAspectFlagBits) -> Self {
            self.plane_aspect = plane_aspect;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImagePlaneMemoryRequirementsInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImagePlaneMemoryRequirementsInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub plane_aspect: ImageAspectFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImagePlaneMemoryRequirementsInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImagePlaneMemoryRequirementsInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("plane_aspect", &self.plane_aspect)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImagePlaneMemoryRequirementsInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO;
    }

    unsafe impl<'a> Extends<ImageMemoryRequirementsInfo2<'a>> for ImagePlaneMemoryRequirementsInfo<'a> {}

    impl Default for ImagePlaneMemoryRequirementsInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                plane_aspect: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImagePlaneMemoryRequirementsInfo<'a> {
        #[inline]
        pub fn plane_aspect(mut self, plane_aspect: ImageAspectFlagBits) -> Self {
            self.plane_aspect = plane_aspect;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSamplerYcbcrConversionFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceSamplerYcbcrConversionFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub sampler_ycbcr_conversion: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceSamplerYcbcrConversionFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceSamplerYcbcrConversionFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("sampler_ycbcr_conversion", &self.sampler_ycbcr_conversion)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSamplerYcbcrConversionFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceSamplerYcbcrConversionFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceSamplerYcbcrConversionFeatures<'a> {}

    impl Default for PhysicalDeviceSamplerYcbcrConversionFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                sampler_ycbcr_conversion: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceSamplerYcbcrConversionFeatures<'a> {
        #[inline]
        pub fn sampler_ycbcr_conversion(mut self, sampler_ycbcr_conversion: bool) -> Self {
            self.sampler_ycbcr_conversion = sampler_ycbcr_conversion.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerYcbcrConversionImageFormatProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SamplerYcbcrConversionImageFormatProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub combined_image_sampler_descriptor_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SamplerYcbcrConversionImageFormatProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SamplerYcbcrConversionImageFormatProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "combined_image_sampler_descriptor_count",
                    &self.combined_image_sampler_descriptor_count,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SamplerYcbcrConversionImageFormatProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES;
    }

    unsafe impl<'a> Extends<ImageFormatProperties2<'a>>
        for SamplerYcbcrConversionImageFormatProperties<'a>
    {
    }

    impl Default for SamplerYcbcrConversionImageFormatProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                combined_image_sampler_descriptor_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SamplerYcbcrConversionImageFormatProperties<'a> {
        #[inline]
        pub fn combined_image_sampler_descriptor_count(
            mut self,
            combined_image_sampler_descriptor_count: u32,
        ) -> Self {
            self.combined_image_sampler_descriptor_count = combined_image_sampler_descriptor_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkProtectedSubmitInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ProtectedSubmitInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub protected_submit: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ProtectedSubmitInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ProtectedSubmitInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("protected_submit", &self.protected_submit)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ProtectedSubmitInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PROTECTED_SUBMIT_INFO;
    }

    unsafe impl<'a> Extends<SubmitInfo<'a>> for ProtectedSubmitInfo<'a> {}

    impl Default for ProtectedSubmitInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                protected_submit: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ProtectedSubmitInfo<'a> {
        #[inline]
        pub fn protected_submit(mut self, protected_submit: bool) -> Self {
            self.protected_submit = protected_submit.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceProtectedMemoryFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceProtectedMemoryFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub protected_memory: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceProtectedMemoryFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceProtectedMemoryFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("protected_memory", &self.protected_memory)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceProtectedMemoryFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceProtectedMemoryFeatures<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceProtectedMemoryFeatures<'a> {}

    impl Default for PhysicalDeviceProtectedMemoryFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                protected_memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceProtectedMemoryFeatures<'a> {
        #[inline]
        pub fn protected_memory(mut self, protected_memory: bool) -> Self {
            self.protected_memory = protected_memory.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceProtectedMemoryProperties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceProtectedMemoryProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub protected_no_fault: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceProtectedMemoryProperties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceProtectedMemoryProperties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("protected_no_fault", &self.protected_no_fault)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceProtectedMemoryProperties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceProtectedMemoryProperties<'a>
    {
    }

    impl Default for PhysicalDeviceProtectedMemoryProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                protected_no_fault: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceProtectedMemoryProperties<'a> {
        #[inline]
        pub fn protected_no_fault(mut self, protected_no_fault: bool) -> Self {
            self.protected_no_fault = protected_no_fault.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceQueueInfo2.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DeviceQueueInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DeviceQueueCreateFlags,
        pub queue_family_index: u32,
        pub queue_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DeviceQueueInfo2<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DeviceQueueInfo2")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("queue_family_index", &self.queue_family_index)
                .field("queue_index", &self.queue_index)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DeviceQueueInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_QUEUE_INFO_2;
    }

    impl Default for DeviceQueueInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                queue_family_index: Default::default(),
                queue_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DeviceQueueInfo2<'a> {
        #[inline]
        pub fn flags(mut self, flags: DeviceQueueCreateFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
            self.queue_family_index = queue_family_index;
            self
        }

        #[inline]
        pub fn queue_index(mut self, queue_index: u32) -> Self {
            self.queue_index = queue_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance3Properties.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMaintenance3Properties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_per_set_descriptors: u32,
        pub max_memory_allocation_size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMaintenance3Properties<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMaintenance3Properties")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_per_set_descriptors", &self.max_per_set_descriptors)
                .field(
                    "max_memory_allocation_size",
                    &self.max_memory_allocation_size,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance3Properties<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceMaintenance3Properties<'a>
    {
    }

    impl Default for PhysicalDeviceMaintenance3Properties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_per_set_descriptors: Default::default(),
                max_memory_allocation_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMaintenance3Properties<'a> {
        #[inline]
        pub fn max_per_set_descriptors(mut self, max_per_set_descriptors: u32) -> Self {
            self.max_per_set_descriptors = max_per_set_descriptors;
            self
        }

        #[inline]
        pub fn max_memory_allocation_size(
            mut self,
            max_memory_allocation_size: DeviceSize,
        ) -> Self {
            self.max_memory_allocation_size = max_memory_allocation_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorSetLayoutSupport.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DescriptorSetLayoutSupport<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub supported: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorSetLayoutSupport<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorSetLayoutSupport")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("supported", &self.supported)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DescriptorSetLayoutSupport<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DESCRIPTOR_SET_LAYOUT_SUPPORT;
    }

    impl Default for DescriptorSetLayoutSupport<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                supported: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DescriptorSetLayoutSupport<'a> {
        #[inline]
        pub fn supported(mut self, supported: bool) -> Self {
            self.supported = supported.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderDrawParametersFeatures.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderDrawParametersFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_draw_parameters: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderDrawParametersFeatures<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderDrawParametersFeatures")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_draw_parameters", &self.shader_draw_parameters)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderDrawParametersFeatures<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderDrawParametersFeatures<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShaderDrawParametersFeatures<'a> {}

    impl Default for PhysicalDeviceShaderDrawParametersFeatures<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_draw_parameters: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderDrawParametersFeatures<'a> {
        #[inline]
        pub fn shader_draw_parameters(mut self, shader_draw_parameters: bool) -> Self {
            self.shader_draw_parameters = shader_draw_parameters.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorUpdateTemplateType.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DescriptorUpdateTemplateType(i32);

    impl DescriptorUpdateTemplateType {
        /// Create descriptor update template for descriptor set updates
        pub const DESCRIPTOR_SET: Self = Self(0);

        // VK_KHR_descriptor_update_template
        pub const DESCRIPTOR_SET_KHR: Self = Self::DESCRIPTOR_SET;
        pub const PUSH_DESCRIPTORS_KHR: Self = Self::PUSH_DESCRIPTORS;

        // VK_VERSION_1_4
        pub const PUSH_DESCRIPTORS: Self = Self(1);
    }

    impl fmt::Debug for DescriptorUpdateTemplateType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DESCRIPTOR_SET => Some("DESCRIPTOR_SET"),
                Self::PUSH_DESCRIPTORS => Some("PUSH_DESCRIPTORS"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPointClippingBehavior.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PointClippingBehavior(i32);

    impl PointClippingBehavior {
        pub const ALL_CLIP_PLANES: Self = Self(0);
        pub const USER_CLIP_PLANES_ONLY: Self = Self(1);

        // VK_KHR_maintenance2
        pub const ALL_CLIP_PLANES_KHR: Self = Self::ALL_CLIP_PLANES;
        pub const USER_CLIP_PLANES_ONLY_KHR: Self = Self::USER_CLIP_PLANES_ONLY;
    }

    impl fmt::Debug for PointClippingBehavior {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ALL_CLIP_PLANES => Some("ALL_CLIP_PLANES"),
                Self::USER_CLIP_PLANES_ONLY => Some("USER_CLIP_PLANES_ONLY"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTessellationDomainOrigin.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TessellationDomainOrigin(i32);

    impl TessellationDomainOrigin {
        pub const UPPER_LEFT: Self = Self(0);
        pub const LOWER_LEFT: Self = Self(1);

        // VK_KHR_maintenance2
        pub const UPPER_LEFT_KHR: Self = Self::UPPER_LEFT;
        pub const LOWER_LEFT_KHR: Self = Self::LOWER_LEFT;
    }

    impl fmt::Debug for TessellationDomainOrigin {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::UPPER_LEFT => Some("UPPER_LEFT"),
                Self::LOWER_LEFT => Some("LOWER_LEFT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerYcbcrModelConversion.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SamplerYcbcrModelConversion(i32);

    impl SamplerYcbcrModelConversion {
        pub const RGB_IDENTITY: Self = Self(0);
        /// just range expansion
        pub const YCBCR_IDENTITY: Self = Self(1);
        /// aka HD YUV
        pub const YCBCR_709: Self = Self(2);
        /// aka SD YUV
        pub const YCBCR_601: Self = Self(3);
        /// aka UHD YUV
        pub const YCBCR_2020: Self = Self(4);

        // VK_KHR_sampler_ycbcr_conversion
        pub const RGB_IDENTITY_KHR: Self = Self::RGB_IDENTITY;
        pub const YCBCR_IDENTITY_KHR: Self = Self::YCBCR_IDENTITY;
        pub const YCBCR_709_KHR: Self = Self::YCBCR_709;
        pub const YCBCR_601_KHR: Self = Self::YCBCR_601;
        pub const YCBCR_2020_KHR: Self = Self::YCBCR_2020;
    }

    impl fmt::Debug for SamplerYcbcrModelConversion {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::RGB_IDENTITY => Some("RGB_IDENTITY"),
                Self::YCBCR_IDENTITY => Some("YCBCR_IDENTITY"),
                Self::YCBCR_709 => Some("YCBCR_709"),
                Self::YCBCR_601 => Some("YCBCR_601"),
                Self::YCBCR_2020 => Some("YCBCR_2020"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerYcbcrRange.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SamplerYcbcrRange(i32);

    impl SamplerYcbcrRange {
        /// Luma 0..1 maps to 0..255, chroma -0.5..0.5 to 1..255 (clamped)
        pub const ITU_FULL: Self = Self(0);
        /// Luma 0..1 maps to 16..235, chroma -0.5..0.5 to 16..240
        pub const ITU_NARROW: Self = Self(1);

        // VK_KHR_sampler_ycbcr_conversion
        pub const ITU_FULL_KHR: Self = Self::ITU_FULL;
        pub const ITU_NARROW_KHR: Self = Self::ITU_NARROW;
    }

    impl fmt::Debug for SamplerYcbcrRange {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ITU_FULL => Some("ITU_FULL"),
                Self::ITU_NARROW => Some("ITU_NARROW"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkChromaLocation.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ChromaLocation(i32);

    impl ChromaLocation {
        pub const COSITED_EVEN: Self = Self(0);
        pub const MIDPOINT: Self = Self(1);

        // VK_KHR_sampler_ycbcr_conversion
        pub const COSITED_EVEN_KHR: Self = Self::COSITED_EVEN;
        pub const MIDPOINT_KHR: Self = Self::MIDPOINT;
    }

    impl fmt::Debug for ChromaLocation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::COSITED_EVEN => Some("COSITED_EVEN"),
                Self::MIDPOINT => Some("MIDPOINT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubgroupFeatureFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SubgroupFeatureFlags(Flags);
    vk_bitflags_wrapped!(SubgroupFeatureFlags, Flags);

    impl SubgroupFeatureFlags {
        /// Basic subgroup operations
        pub const BASIC: Self = Self(SubgroupFeatureFlagBits::BASIC.0);
        /// Vote subgroup operations
        pub const VOTE: Self = Self(SubgroupFeatureFlagBits::VOTE.0);
        /// Arithmetic subgroup operations
        pub const ARITHMETIC: Self = Self(SubgroupFeatureFlagBits::ARITHMETIC.0);
        /// Ballot subgroup operations
        pub const BALLOT: Self = Self(SubgroupFeatureFlagBits::BALLOT.0);
        /// Shuffle subgroup operations
        pub const SHUFFLE: Self = Self(SubgroupFeatureFlagBits::SHUFFLE.0);
        /// Shuffle relative subgroup operations
        pub const SHUFFLE_RELATIVE: Self = Self(SubgroupFeatureFlagBits::SHUFFLE_RELATIVE.0);
        /// Clustered subgroup operations
        pub const CLUSTERED: Self = Self(SubgroupFeatureFlagBits::CLUSTERED.0);
        /// Quad subgroup operations
        pub const QUAD: Self = Self(SubgroupFeatureFlagBits::QUAD.0);

        // VK_EXT_shader_subgroup_partitioned
        pub const PARTITIONED_EXT: Self = Self(SubgroupFeatureFlagBits::PARTITIONED_EXT.0);

        // VK_KHR_shader_subgroup_rotate
        pub const ROTATE_KHR: Self = Self::ROTATE;
        pub const ROTATE_CLUSTERED_KHR: Self = Self::ROTATE_CLUSTERED;

        // VK_NV_shader_subgroup_partitioned
        pub const PARTITIONED_NV: Self = Self::PARTITIONED_EXT;

        // VK_VERSION_1_4
        pub const ROTATE: Self = Self(SubgroupFeatureFlagBits::ROTATE.0);
        pub const ROTATE_CLUSTERED: Self = Self(SubgroupFeatureFlagBits::ROTATE_CLUSTERED.0);
    }

    impl fmt::Debug for SubgroupFeatureFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (SubgroupFeatureFlags::BASIC.0, "BASIC"),
                (SubgroupFeatureFlags::VOTE.0, "VOTE"),
                (SubgroupFeatureFlags::ARITHMETIC.0, "ARITHMETIC"),
                (SubgroupFeatureFlags::BALLOT.0, "BALLOT"),
                (SubgroupFeatureFlags::SHUFFLE.0, "SHUFFLE"),
                (SubgroupFeatureFlags::SHUFFLE_RELATIVE.0, "SHUFFLE_RELATIVE"),
                (SubgroupFeatureFlags::CLUSTERED.0, "CLUSTERED"),
                (SubgroupFeatureFlags::QUAD.0, "QUAD"),
                (SubgroupFeatureFlags::PARTITIONED_EXT.0, "PARTITIONED_EXT"),
                (SubgroupFeatureFlags::ROTATE.0, "ROTATE"),
                (SubgroupFeatureFlags::ROTATE_CLUSTERED.0, "ROTATE_CLUSTERED"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubgroupFeatureFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct SubgroupFeatureFlagBits(u32);

    impl SubgroupFeatureFlagBits {
        /// Basic subgroup operations
        pub const BASIC: Self = Self(1 << 0);
        /// Vote subgroup operations
        pub const VOTE: Self = Self(1 << 1);
        /// Arithmetic subgroup operations
        pub const ARITHMETIC: Self = Self(1 << 2);
        /// Ballot subgroup operations
        pub const BALLOT: Self = Self(1 << 3);
        /// Shuffle subgroup operations
        pub const SHUFFLE: Self = Self(1 << 4);
        /// Shuffle relative subgroup operations
        pub const SHUFFLE_RELATIVE: Self = Self(1 << 5);
        /// Clustered subgroup operations
        pub const CLUSTERED: Self = Self(1 << 6);
        /// Quad subgroup operations
        pub const QUAD: Self = Self(1 << 7);
        // VK_EXT_shader_subgroup_partitioned
        pub const PARTITIONED_EXT: Self = Self(1 << 8);

        // VK_KHR_shader_subgroup_rotate
        pub const ROTATE_KHR: Self = Self::ROTATE;
        pub const ROTATE_CLUSTERED_KHR: Self = Self::ROTATE_CLUSTERED;

        // VK_NV_shader_subgroup_partitioned
        pub const PARTITIONED_NV: Self = Self::PARTITIONED_EXT;

        // VK_VERSION_1_4
        pub const ROTATE: Self = Self(1 << 9);
        pub const ROTATE_CLUSTERED: Self = Self(1 << 10);
    }

    impl fmt::Debug for SubgroupFeatureFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::BASIC => Some("BASIC"),
                Self::VOTE => Some("VOTE"),
                Self::ARITHMETIC => Some("ARITHMETIC"),
                Self::BALLOT => Some("BALLOT"),
                Self::SHUFFLE => Some("SHUFFLE"),
                Self::SHUFFLE_RELATIVE => Some("SHUFFLE_RELATIVE"),
                Self::CLUSTERED => Some("CLUSTERED"),
                Self::QUAD => Some("QUAD"),
                Self::PARTITIONED_EXT => Some("PARTITIONED_EXT"),
                Self::ROTATE => Some("ROTATE"),
                Self::ROTATE_CLUSTERED => Some("ROTATE_CLUSTERED"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorUpdateTemplateCreateFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DescriptorUpdateTemplateCreateFlags(Flags);
    vk_bitflags_wrapped!(DescriptorUpdateTemplateCreateFlags, Flags);

    impl fmt::Debug for DescriptorUpdateTemplateCreateFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPeerMemoryFeatureFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PeerMemoryFeatureFlags(Flags);
    vk_bitflags_wrapped!(PeerMemoryFeatureFlags, Flags);

    impl PeerMemoryFeatureFlags {
        /// Can read with vkCmdCopy commands
        pub const COPY_SRC: Self = Self(PeerMemoryFeatureFlagBits::COPY_SRC.0);
        /// Can write with vkCmdCopy commands
        pub const COPY_DST: Self = Self(PeerMemoryFeatureFlagBits::COPY_DST.0);
        /// Can read with any access type/command
        pub const GENERIC_SRC: Self = Self(PeerMemoryFeatureFlagBits::GENERIC_SRC.0);
        /// Can write with and access type/command
        pub const GENERIC_DST: Self = Self(PeerMemoryFeatureFlagBits::GENERIC_DST.0);

        // VK_KHR_device_group
        pub const COPY_SRC_KHR: Self = Self::COPY_SRC;
        pub const COPY_DST_KHR: Self = Self::COPY_DST;
        pub const GENERIC_SRC_KHR: Self = Self::GENERIC_SRC;
        pub const GENERIC_DST_KHR: Self = Self::GENERIC_DST;
    }

    impl fmt::Debug for PeerMemoryFeatureFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (PeerMemoryFeatureFlags::COPY_SRC.0, "COPY_SRC"),
                (PeerMemoryFeatureFlags::COPY_DST.0, "COPY_DST"),
                (PeerMemoryFeatureFlags::GENERIC_SRC.0, "GENERIC_SRC"),
                (PeerMemoryFeatureFlags::GENERIC_DST.0, "GENERIC_DST"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPeerMemoryFeatureFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PeerMemoryFeatureFlagBits(u32);

    impl PeerMemoryFeatureFlagBits {
        /// Can read with vkCmdCopy commands
        pub const COPY_SRC: Self = Self(1 << 0);
        /// Can write with vkCmdCopy commands
        pub const COPY_DST: Self = Self(1 << 1);
        /// Can read with any access type/command
        pub const GENERIC_SRC: Self = Self(1 << 2);
        /// Can write with and access type/command
        pub const GENERIC_DST: Self = Self(1 << 3);
        // VK_KHR_device_group
        pub const COPY_SRC_KHR: Self = Self::COPY_SRC;
        pub const COPY_DST_KHR: Self = Self::COPY_DST;
        pub const GENERIC_SRC_KHR: Self = Self::GENERIC_SRC;
        pub const GENERIC_DST_KHR: Self = Self::GENERIC_DST;
    }

    impl fmt::Debug for PeerMemoryFeatureFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::COPY_SRC => Some("COPY_SRC"),
                Self::COPY_DST => Some("COPY_DST"),
                Self::GENERIC_SRC => Some("GENERIC_SRC"),
                Self::GENERIC_DST => Some("GENERIC_DST"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryAllocateFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MemoryAllocateFlags(Flags);
    vk_bitflags_wrapped!(MemoryAllocateFlags, Flags);

    impl MemoryAllocateFlags {
        /// Force allocation on specific devices
        pub const DEVICE_MASK: Self = Self(MemoryAllocateFlagBits::DEVICE_MASK.0);

        // VK_EXT_zero_initialize_device_memory
        pub const ZERO_INITIALIZE_EXT: Self = Self(MemoryAllocateFlagBits::ZERO_INITIALIZE_EXT.0);

        // VK_KHR_buffer_device_address
        pub const DEVICE_ADDRESS_KHR: Self = Self::DEVICE_ADDRESS;
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;

        // VK_KHR_device_group
        pub const DEVICE_MASK_KHR: Self = Self::DEVICE_MASK;

        // VK_VERSION_1_2
        pub const DEVICE_ADDRESS: Self = Self(MemoryAllocateFlagBits::DEVICE_ADDRESS.0);
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self =
            Self(MemoryAllocateFlagBits::DEVICE_ADDRESS_CAPTURE_REPLAY.0);
    }

    impl fmt::Debug for MemoryAllocateFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (MemoryAllocateFlags::DEVICE_MASK.0, "DEVICE_MASK"),
                (
                    MemoryAllocateFlags::ZERO_INITIALIZE_EXT.0,
                    "ZERO_INITIALIZE_EXT",
                ),
                (MemoryAllocateFlags::DEVICE_ADDRESS.0, "DEVICE_ADDRESS"),
                (
                    MemoryAllocateFlags::DEVICE_ADDRESS_CAPTURE_REPLAY.0,
                    "DEVICE_ADDRESS_CAPTURE_REPLAY",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryAllocateFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct MemoryAllocateFlagBits(u32);

    impl MemoryAllocateFlagBits {
        /// Force allocation on specific devices
        pub const DEVICE_MASK: Self = Self(1 << 0);
        // VK_EXT_zero_initialize_device_memory
        pub const ZERO_INITIALIZE_EXT: Self = Self(1 << 3);

        // VK_KHR_buffer_device_address
        pub const DEVICE_ADDRESS_KHR: Self = Self::DEVICE_ADDRESS;
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;

        // VK_KHR_device_group
        pub const DEVICE_MASK_KHR: Self = Self::DEVICE_MASK;

        // VK_VERSION_1_2
        pub const DEVICE_ADDRESS: Self = Self(1 << 1);
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(1 << 2);
    }

    impl fmt::Debug for MemoryAllocateFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEVICE_MASK => Some("DEVICE_MASK"),
                Self::ZERO_INITIALIZE_EXT => Some("ZERO_INITIALIZE_EXT"),
                Self::DEVICE_ADDRESS => Some("DEVICE_ADDRESS"),
                Self::DEVICE_ADDRESS_CAPTURE_REPLAY => Some("DEVICE_ADDRESS_CAPTURE_REPLAY"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCommandPoolTrimFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CommandPoolTrimFlags(Flags);
    vk_bitflags_wrapped!(CommandPoolTrimFlags, Flags);

    impl fmt::Debug for CommandPoolTrimFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryHandleTypeFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ExternalMemoryHandleTypeFlags(Flags);
    vk_bitflags_wrapped!(ExternalMemoryHandleTypeFlags, Flags);

    impl ExternalMemoryHandleTypeFlags {
        pub const OPAQUE_FD: Self = Self(ExternalMemoryHandleTypeFlagBits::OPAQUE_FD.0);
        pub const OPAQUE_WIN32: Self = Self(ExternalMemoryHandleTypeFlagBits::OPAQUE_WIN32.0);
        pub const OPAQUE_WIN32_KMT: Self =
            Self(ExternalMemoryHandleTypeFlagBits::OPAQUE_WIN32_KMT.0);
        pub const D3D11_TEXTURE: Self = Self(ExternalMemoryHandleTypeFlagBits::D3D11_TEXTURE.0);
        pub const D3D11_TEXTURE_KMT: Self =
            Self(ExternalMemoryHandleTypeFlagBits::D3D11_TEXTURE_KMT.0);
        pub const D3D12_HEAP: Self = Self(ExternalMemoryHandleTypeFlagBits::D3D12_HEAP.0);
        pub const D3D12_RESOURCE: Self = Self(ExternalMemoryHandleTypeFlagBits::D3D12_RESOURCE.0);

        // VK_ANDROID_external_memory_android_hardware_buffer
        pub const ANDROID_HARDWARE_BUFFER_ANDROID: Self =
            Self(ExternalMemoryHandleTypeFlagBits::ANDROID_HARDWARE_BUFFER_ANDROID.0);

        // VK_EXT_external_memory_dma_buf
        pub const DMA_BUF_EXT: Self = Self(ExternalMemoryHandleTypeFlagBits::DMA_BUF_EXT.0);

        // VK_EXT_external_memory_host
        pub const HOST_ALLOCATION_EXT: Self =
            Self(ExternalMemoryHandleTypeFlagBits::HOST_ALLOCATION_EXT.0);
        pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self =
            Self(ExternalMemoryHandleTypeFlagBits::HOST_MAPPED_FOREIGN_MEMORY_EXT.0);

        // VK_EXT_external_memory_metal
        pub const MTLBUFFER_EXT: Self = Self(ExternalMemoryHandleTypeFlagBits::MTLBUFFER_EXT.0);
        pub const MTLTEXTURE_EXT: Self = Self(ExternalMemoryHandleTypeFlagBits::MTLTEXTURE_EXT.0);
        pub const MTLHEAP_EXT: Self = Self(ExternalMemoryHandleTypeFlagBits::MTLHEAP_EXT.0);

        // VK_FUCHSIA_external_memory
        pub const ZIRCON_VMO_FUCHSIA: Self =
            Self(ExternalMemoryHandleTypeFlagBits::ZIRCON_VMO_FUCHSIA.0);

        // VK_KHR_external_memory_capabilities
        pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
        pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
        pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
        pub const D3D11_TEXTURE_KHR: Self = Self::D3D11_TEXTURE;
        pub const D3D11_TEXTURE_KMT_KHR: Self = Self::D3D11_TEXTURE_KMT;
        pub const D3D12_HEAP_KHR: Self = Self::D3D12_HEAP;
        pub const D3D12_RESOURCE_KHR: Self = Self::D3D12_RESOURCE;

        // VK_NV_external_memory_rdma
        pub const RDMA_ADDRESS_NV: Self = Self(ExternalMemoryHandleTypeFlagBits::RDMA_ADDRESS_NV.0);

        // VK_OHOS_external_memory
        pub const OH_NATIVE_BUFFER_OHOS: Self =
            Self(ExternalMemoryHandleTypeFlagBits::OH_NATIVE_BUFFER_OHOS.0);

        // VK_QNX_external_memory_screen_buffer
        pub const SCREEN_BUFFER_QNX: Self =
            Self(ExternalMemoryHandleTypeFlagBits::SCREEN_BUFFER_QNX.0);
    }

    impl fmt::Debug for ExternalMemoryHandleTypeFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (ExternalMemoryHandleTypeFlags::OPAQUE_FD.0, "OPAQUE_FD"),
                (
                    ExternalMemoryHandleTypeFlags::OPAQUE_WIN32.0,
                    "OPAQUE_WIN32",
                ),
                (
                    ExternalMemoryHandleTypeFlags::OPAQUE_WIN32_KMT.0,
                    "OPAQUE_WIN32_KMT",
                ),
                (
                    ExternalMemoryHandleTypeFlags::D3D11_TEXTURE.0,
                    "D3D11_TEXTURE",
                ),
                (
                    ExternalMemoryHandleTypeFlags::D3D11_TEXTURE_KMT.0,
                    "D3D11_TEXTURE_KMT",
                ),
                (ExternalMemoryHandleTypeFlags::D3D12_HEAP.0, "D3D12_HEAP"),
                (
                    ExternalMemoryHandleTypeFlags::D3D12_RESOURCE.0,
                    "D3D12_RESOURCE",
                ),
                (
                    ExternalMemoryHandleTypeFlags::ANDROID_HARDWARE_BUFFER_ANDROID.0,
                    "ANDROID_HARDWARE_BUFFER_ANDROID",
                ),
                (ExternalMemoryHandleTypeFlags::DMA_BUF_EXT.0, "DMA_BUF_EXT"),
                (
                    ExternalMemoryHandleTypeFlags::HOST_ALLOCATION_EXT.0,
                    "HOST_ALLOCATION_EXT",
                ),
                (
                    ExternalMemoryHandleTypeFlags::HOST_MAPPED_FOREIGN_MEMORY_EXT.0,
                    "HOST_MAPPED_FOREIGN_MEMORY_EXT",
                ),
                (
                    ExternalMemoryHandleTypeFlags::MTLBUFFER_EXT.0,
                    "MTLBUFFER_EXT",
                ),
                (
                    ExternalMemoryHandleTypeFlags::MTLTEXTURE_EXT.0,
                    "MTLTEXTURE_EXT",
                ),
                (ExternalMemoryHandleTypeFlags::MTLHEAP_EXT.0, "MTLHEAP_EXT"),
                (
                    ExternalMemoryHandleTypeFlags::ZIRCON_VMO_FUCHSIA.0,
                    "ZIRCON_VMO_FUCHSIA",
                ),
                (
                    ExternalMemoryHandleTypeFlags::RDMA_ADDRESS_NV.0,
                    "RDMA_ADDRESS_NV",
                ),
                (
                    ExternalMemoryHandleTypeFlags::OH_NATIVE_BUFFER_OHOS.0,
                    "OH_NATIVE_BUFFER_OHOS",
                ),
                (
                    ExternalMemoryHandleTypeFlags::SCREEN_BUFFER_QNX.0,
                    "SCREEN_BUFFER_QNX",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryHandleTypeFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalMemoryHandleTypeFlagBits(u32);

    impl ExternalMemoryHandleTypeFlagBits {
        pub const OPAQUE_FD: Self = Self(1 << 0);
        pub const OPAQUE_WIN32: Self = Self(1 << 1);
        pub const OPAQUE_WIN32_KMT: Self = Self(1 << 2);
        pub const D3D11_TEXTURE: Self = Self(1 << 3);
        pub const D3D11_TEXTURE_KMT: Self = Self(1 << 4);
        pub const D3D12_HEAP: Self = Self(1 << 5);
        pub const D3D12_RESOURCE: Self = Self(1 << 6);
        // VK_ANDROID_external_memory_android_hardware_buffer
        pub const ANDROID_HARDWARE_BUFFER_ANDROID: Self = Self(1 << 10);

        // VK_EXT_external_memory_dma_buf
        pub const DMA_BUF_EXT: Self = Self(1 << 9);

        // VK_EXT_external_memory_host
        pub const HOST_ALLOCATION_EXT: Self = Self(1 << 7);
        pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self = Self(1 << 8);

        // VK_EXT_external_memory_metal
        pub const MTLBUFFER_EXT: Self = Self(1 << 16);
        pub const MTLTEXTURE_EXT: Self = Self(1 << 17);
        pub const MTLHEAP_EXT: Self = Self(1 << 18);

        // VK_FUCHSIA_external_memory
        pub const ZIRCON_VMO_FUCHSIA: Self = Self(1 << 11);

        // VK_KHR_external_memory_capabilities
        pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
        pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
        pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
        pub const D3D11_TEXTURE_KHR: Self = Self::D3D11_TEXTURE;
        pub const D3D11_TEXTURE_KMT_KHR: Self = Self::D3D11_TEXTURE_KMT;
        pub const D3D12_HEAP_KHR: Self = Self::D3D12_HEAP;
        pub const D3D12_RESOURCE_KHR: Self = Self::D3D12_RESOURCE;

        // VK_NV_external_memory_rdma
        pub const RDMA_ADDRESS_NV: Self = Self(1 << 12);

        // VK_OHOS_external_memory
        pub const OH_NATIVE_BUFFER_OHOS: Self = Self(1 << 15);

        // VK_QNX_external_memory_screen_buffer
        pub const SCREEN_BUFFER_QNX: Self = Self(1 << 14);
    }

    impl fmt::Debug for ExternalMemoryHandleTypeFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::OPAQUE_FD => Some("OPAQUE_FD"),
                Self::OPAQUE_WIN32 => Some("OPAQUE_WIN32"),
                Self::OPAQUE_WIN32_KMT => Some("OPAQUE_WIN32_KMT"),
                Self::D3D11_TEXTURE => Some("D3D11_TEXTURE"),
                Self::D3D11_TEXTURE_KMT => Some("D3D11_TEXTURE_KMT"),
                Self::D3D12_HEAP => Some("D3D12_HEAP"),
                Self::D3D12_RESOURCE => Some("D3D12_RESOURCE"),
                Self::ANDROID_HARDWARE_BUFFER_ANDROID => Some("ANDROID_HARDWARE_BUFFER_ANDROID"),
                Self::DMA_BUF_EXT => Some("DMA_BUF_EXT"),
                Self::HOST_ALLOCATION_EXT => Some("HOST_ALLOCATION_EXT"),
                Self::HOST_MAPPED_FOREIGN_MEMORY_EXT => Some("HOST_MAPPED_FOREIGN_MEMORY_EXT"),
                Self::MTLBUFFER_EXT => Some("MTLBUFFER_EXT"),
                Self::MTLTEXTURE_EXT => Some("MTLTEXTURE_EXT"),
                Self::MTLHEAP_EXT => Some("MTLHEAP_EXT"),
                Self::ZIRCON_VMO_FUCHSIA => Some("ZIRCON_VMO_FUCHSIA"),
                Self::RDMA_ADDRESS_NV => Some("RDMA_ADDRESS_NV"),
                Self::OH_NATIVE_BUFFER_OHOS => Some("OH_NATIVE_BUFFER_OHOS"),
                Self::SCREEN_BUFFER_QNX => Some("SCREEN_BUFFER_QNX"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryFeatureFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ExternalMemoryFeatureFlags(Flags);
    vk_bitflags_wrapped!(ExternalMemoryFeatureFlags, Flags);

    impl ExternalMemoryFeatureFlags {
        pub const DEDICATED_ONLY: Self = Self(ExternalMemoryFeatureFlagBits::DEDICATED_ONLY.0);
        pub const EXPORTABLE: Self = Self(ExternalMemoryFeatureFlagBits::EXPORTABLE.0);
        pub const IMPORTABLE: Self = Self(ExternalMemoryFeatureFlagBits::IMPORTABLE.0);

        // VK_KHR_external_memory_capabilities
        pub const DEDICATED_ONLY_KHR: Self = Self::DEDICATED_ONLY;
        pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
        pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    }

    impl fmt::Debug for ExternalMemoryFeatureFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    ExternalMemoryFeatureFlags::DEDICATED_ONLY.0,
                    "DEDICATED_ONLY",
                ),
                (ExternalMemoryFeatureFlags::EXPORTABLE.0, "EXPORTABLE"),
                (ExternalMemoryFeatureFlags::IMPORTABLE.0, "IMPORTABLE"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalMemoryFeatureFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalMemoryFeatureFlagBits(u32);

    impl ExternalMemoryFeatureFlagBits {
        pub const DEDICATED_ONLY: Self = Self(1 << 0);
        pub const EXPORTABLE: Self = Self(1 << 1);
        pub const IMPORTABLE: Self = Self(1 << 2);
        // VK_KHR_external_memory_capabilities
        pub const DEDICATED_ONLY_KHR: Self = Self::DEDICATED_ONLY;
        pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
        pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    }

    impl fmt::Debug for ExternalMemoryFeatureFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEDICATED_ONLY => Some("DEDICATED_ONLY"),
                Self::EXPORTABLE => Some("EXPORTABLE"),
                Self::IMPORTABLE => Some("IMPORTABLE"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalSemaphoreHandleTypeFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ExternalSemaphoreHandleTypeFlags(Flags);
    vk_bitflags_wrapped!(ExternalSemaphoreHandleTypeFlags, Flags);

    impl ExternalSemaphoreHandleTypeFlags {
        pub const OPAQUE_FD: Self = Self(ExternalSemaphoreHandleTypeFlagBits::OPAQUE_FD.0);
        pub const OPAQUE_WIN32: Self = Self(ExternalSemaphoreHandleTypeFlagBits::OPAQUE_WIN32.0);
        pub const OPAQUE_WIN32_KMT: Self =
            Self(ExternalSemaphoreHandleTypeFlagBits::OPAQUE_WIN32_KMT.0);
        pub const D3D12_FENCE: Self = Self(ExternalSemaphoreHandleTypeFlagBits::D3D12_FENCE.0);
        pub const SYNC_FD: Self = Self(ExternalSemaphoreHandleTypeFlagBits::SYNC_FD.0);
        pub const D3D11_FENCE: Self = Self::D3D12_FENCE;

        // VK_FUCHSIA_external_semaphore
        pub const ZIRCON_EVENT_FUCHSIA: Self =
            Self(ExternalSemaphoreHandleTypeFlagBits::ZIRCON_EVENT_FUCHSIA.0);

        // VK_KHR_external_semaphore_capabilities
        pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
        pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
        pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
        pub const D3D12_FENCE_KHR: Self = Self::D3D12_FENCE;
        pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
    }

    impl fmt::Debug for ExternalSemaphoreHandleTypeFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (ExternalSemaphoreHandleTypeFlags::OPAQUE_FD.0, "OPAQUE_FD"),
                (
                    ExternalSemaphoreHandleTypeFlags::OPAQUE_WIN32.0,
                    "OPAQUE_WIN32",
                ),
                (
                    ExternalSemaphoreHandleTypeFlags::OPAQUE_WIN32_KMT.0,
                    "OPAQUE_WIN32_KMT",
                ),
                (
                    ExternalSemaphoreHandleTypeFlags::D3D12_FENCE.0,
                    "D3D12_FENCE",
                ),
                (ExternalSemaphoreHandleTypeFlags::SYNC_FD.0, "SYNC_FD"),
                (
                    ExternalSemaphoreHandleTypeFlags::ZIRCON_EVENT_FUCHSIA.0,
                    "ZIRCON_EVENT_FUCHSIA",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalSemaphoreHandleTypeFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalSemaphoreHandleTypeFlagBits(u32);

    impl ExternalSemaphoreHandleTypeFlagBits {
        pub const OPAQUE_FD: Self = Self(1 << 0);
        pub const OPAQUE_WIN32: Self = Self(1 << 1);
        pub const OPAQUE_WIN32_KMT: Self = Self(1 << 2);
        pub const D3D12_FENCE: Self = Self(1 << 3);
        pub const SYNC_FD: Self = Self(1 << 4);
        // VK_FUCHSIA_external_semaphore
        pub const ZIRCON_EVENT_FUCHSIA: Self = Self(1 << 7);

        // VK_KHR_external_semaphore_capabilities
        pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
        pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
        pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
        pub const D3D12_FENCE_KHR: Self = Self::D3D12_FENCE;
        pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
    }

    impl fmt::Debug for ExternalSemaphoreHandleTypeFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::OPAQUE_FD => Some("OPAQUE_FD"),
                Self::OPAQUE_WIN32 => Some("OPAQUE_WIN32"),
                Self::OPAQUE_WIN32_KMT => Some("OPAQUE_WIN32_KMT"),
                Self::D3D12_FENCE => Some("D3D12_FENCE"),
                Self::SYNC_FD => Some("SYNC_FD"),
                Self::ZIRCON_EVENT_FUCHSIA => Some("ZIRCON_EVENT_FUCHSIA"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalSemaphoreFeatureFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ExternalSemaphoreFeatureFlags(Flags);
    vk_bitflags_wrapped!(ExternalSemaphoreFeatureFlags, Flags);

    impl ExternalSemaphoreFeatureFlags {
        pub const EXPORTABLE: Self = Self(ExternalSemaphoreFeatureFlagBits::EXPORTABLE.0);
        pub const IMPORTABLE: Self = Self(ExternalSemaphoreFeatureFlagBits::IMPORTABLE.0);

        // VK_KHR_external_semaphore_capabilities
        pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
        pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    }

    impl fmt::Debug for ExternalSemaphoreFeatureFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (ExternalSemaphoreFeatureFlags::EXPORTABLE.0, "EXPORTABLE"),
                (ExternalSemaphoreFeatureFlags::IMPORTABLE.0, "IMPORTABLE"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalSemaphoreFeatureFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalSemaphoreFeatureFlagBits(u32);

    impl ExternalSemaphoreFeatureFlagBits {
        pub const EXPORTABLE: Self = Self(1 << 0);
        pub const IMPORTABLE: Self = Self(1 << 1);
        // VK_KHR_external_semaphore_capabilities
        pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
        pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    }

    impl fmt::Debug for ExternalSemaphoreFeatureFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::EXPORTABLE => Some("EXPORTABLE"),
                Self::IMPORTABLE => Some("IMPORTABLE"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSemaphoreImportFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SemaphoreImportFlags(Flags);
    vk_bitflags_wrapped!(SemaphoreImportFlags, Flags);

    impl SemaphoreImportFlags {
        pub const TEMPORARY: Self = Self(SemaphoreImportFlagBits::TEMPORARY.0);

        // VK_KHR_external_semaphore
        pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
    }

    impl fmt::Debug for SemaphoreImportFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[(SemaphoreImportFlags::TEMPORARY.0, "TEMPORARY")];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSemaphoreImportFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct SemaphoreImportFlagBits(u32);

    impl SemaphoreImportFlagBits {
        pub const TEMPORARY: Self = Self(1 << 0);
        // VK_KHR_external_semaphore
        pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
    }

    impl fmt::Debug for SemaphoreImportFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::TEMPORARY => Some("TEMPORARY"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalFenceHandleTypeFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ExternalFenceHandleTypeFlags(Flags);
    vk_bitflags_wrapped!(ExternalFenceHandleTypeFlags, Flags);

    impl ExternalFenceHandleTypeFlags {
        pub const OPAQUE_FD: Self = Self(ExternalFenceHandleTypeFlagBits::OPAQUE_FD.0);
        pub const OPAQUE_WIN32: Self = Self(ExternalFenceHandleTypeFlagBits::OPAQUE_WIN32.0);
        pub const OPAQUE_WIN32_KMT: Self =
            Self(ExternalFenceHandleTypeFlagBits::OPAQUE_WIN32_KMT.0);
        pub const SYNC_FD: Self = Self(ExternalFenceHandleTypeFlagBits::SYNC_FD.0);

        // VK_KHR_external_fence_capabilities
        pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
        pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
        pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
        pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
    }

    impl fmt::Debug for ExternalFenceHandleTypeFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (ExternalFenceHandleTypeFlags::OPAQUE_FD.0, "OPAQUE_FD"),
                (ExternalFenceHandleTypeFlags::OPAQUE_WIN32.0, "OPAQUE_WIN32"),
                (
                    ExternalFenceHandleTypeFlags::OPAQUE_WIN32_KMT.0,
                    "OPAQUE_WIN32_KMT",
                ),
                (ExternalFenceHandleTypeFlags::SYNC_FD.0, "SYNC_FD"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalFenceHandleTypeFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalFenceHandleTypeFlagBits(u32);

    impl ExternalFenceHandleTypeFlagBits {
        pub const OPAQUE_FD: Self = Self(1 << 0);
        pub const OPAQUE_WIN32: Self = Self(1 << 1);
        pub const OPAQUE_WIN32_KMT: Self = Self(1 << 2);
        pub const SYNC_FD: Self = Self(1 << 3);
        // VK_KHR_external_fence_capabilities
        pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
        pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
        pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
        pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
    }

    impl fmt::Debug for ExternalFenceHandleTypeFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::OPAQUE_FD => Some("OPAQUE_FD"),
                Self::OPAQUE_WIN32 => Some("OPAQUE_WIN32"),
                Self::OPAQUE_WIN32_KMT => Some("OPAQUE_WIN32_KMT"),
                Self::SYNC_FD => Some("SYNC_FD"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalFenceFeatureFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ExternalFenceFeatureFlags(Flags);
    vk_bitflags_wrapped!(ExternalFenceFeatureFlags, Flags);

    impl ExternalFenceFeatureFlags {
        pub const EXPORTABLE: Self = Self(ExternalFenceFeatureFlagBits::EXPORTABLE.0);
        pub const IMPORTABLE: Self = Self(ExternalFenceFeatureFlagBits::IMPORTABLE.0);

        // VK_KHR_external_fence_capabilities
        pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
        pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    }

    impl fmt::Debug for ExternalFenceFeatureFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (ExternalFenceFeatureFlags::EXPORTABLE.0, "EXPORTABLE"),
                (ExternalFenceFeatureFlags::IMPORTABLE.0, "IMPORTABLE"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExternalFenceFeatureFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalFenceFeatureFlagBits(u32);

    impl ExternalFenceFeatureFlagBits {
        pub const EXPORTABLE: Self = Self(1 << 0);
        pub const IMPORTABLE: Self = Self(1 << 1);
        // VK_KHR_external_fence_capabilities
        pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
        pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    }

    impl fmt::Debug for ExternalFenceFeatureFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::EXPORTABLE => Some("EXPORTABLE"),
                Self::IMPORTABLE => Some("IMPORTABLE"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFenceImportFlags.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct FenceImportFlags(Flags);
    vk_bitflags_wrapped!(FenceImportFlags, Flags);

    impl FenceImportFlags {
        pub const TEMPORARY: Self = Self(FenceImportFlagBits::TEMPORARY.0);

        // VK_KHR_external_fence
        pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
    }

    impl fmt::Debug for FenceImportFlags {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[(FenceImportFlags::TEMPORARY.0, "TEMPORARY")];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFenceImportFlagBits.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct FenceImportFlagBits(u32);

    impl FenceImportFlagBits {
        pub const TEMPORARY: Self = Self(1 << 0);
        // VK_KHR_external_fence
        pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
    }

    impl fmt::Debug for FenceImportFlagBits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::TEMPORARY => Some("TEMPORARY"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkEnumerateInstanceVersion.html>
    pub type PFN_vkEnumerateInstanceVersion =
        unsafe extern "system" fn(p_api_version: *mut u32) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceFeatures2.html>
    pub type PFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceProperties2.html>
    pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_properties: *mut PhysicalDeviceProperties2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceFormatProperties2.html>
    pub type PFN_vkGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *mut FormatProperties2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceImageFormatProperties2.html>
    pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_image_format_info: *const PhysicalDeviceImageFormatInfo2<'_>,
            p_image_format_properties: *mut ImageFormatProperties2<'_>,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html>
    pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceMemoryProperties2.html>
    pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html>
    pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_format_info: *const PhysicalDeviceSparseImageFormatInfo2<'_>,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkTrimCommandPool.html>
    pub type PFN_vkTrimCommandPool = unsafe extern "system" fn(
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceExternalBufferProperties.html>
    pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo<'_>,
        p_external_buffer_properties: *mut ExternalBufferProperties<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html>
    pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo<'_>,
        p_external_semaphore_properties: *mut ExternalSemaphoreProperties<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceExternalFenceProperties.html>
    pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_external_fence_info: *const PhysicalDeviceExternalFenceInfo<'_>,
        p_external_fence_properties: *mut ExternalFenceProperties<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkEnumeratePhysicalDeviceGroups.html>
    pub type PFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
        instance: Instance,
        p_physical_device_group_count: *mut u32,
        p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceGroupPeerMemoryFeatures.html>
    pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(
        device: Device,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        p_peer_memory_features: *mut PeerMemoryFeatureFlags,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBindBufferMemory2.html>
    pub type PFN_vkBindBufferMemory2 = unsafe extern "system" fn(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindBufferMemoryInfo<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBindImageMemory2.html>
    pub type PFN_vkBindImageMemory2 = unsafe extern "system" fn(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindImageMemoryInfo<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDeviceMask.html>
    pub type PFN_vkCmdSetDeviceMask =
        unsafe extern "system" fn(command_buffer: CommandBuffer, device_mask: u32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDispatchBase.html>
    pub type PFN_vkCmdDispatchBase = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateDescriptorUpdateTemplate.html>
    pub type PFN_vkCreateDescriptorUpdateTemplate = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const DescriptorUpdateTemplateCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_descriptor_update_template: *mut DescriptorUpdateTemplate,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyDescriptorUpdateTemplate.html>
    pub type PFN_vkDestroyDescriptorUpdateTemplate = unsafe extern "system" fn(
        device: Device,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUpdateDescriptorSetWithTemplate.html>
    pub type PFN_vkUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(
        device: Device,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_data: *const c_void,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetBufferMemoryRequirements2.html>
    pub type PFN_vkGetBufferMemoryRequirements2 = unsafe extern "system" fn(
        device: Device,
        p_info: *const BufferMemoryRequirementsInfo2<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageMemoryRequirements2.html>
    pub type PFN_vkGetImageMemoryRequirements2 = unsafe extern "system" fn(
        device: Device,
        p_info: *const ImageMemoryRequirementsInfo2<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageSparseMemoryRequirements2.html>
    pub type PFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(
        device: Device,
        p_info: *const ImageSparseMemoryRequirementsInfo2<'_>,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateSamplerYcbcrConversion.html>
    pub type PFN_vkCreateSamplerYcbcrConversion = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const SamplerYcbcrConversionCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_ycbcr_conversion: *mut SamplerYcbcrConversion,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroySamplerYcbcrConversion.html>
    pub type PFN_vkDestroySamplerYcbcrConversion = unsafe extern "system" fn(
        device: Device,
        ycbcr_conversion: SamplerYcbcrConversion,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceQueue2.html>
    pub type PFN_vkGetDeviceQueue2 = unsafe extern "system" fn(
        device: Device,
        p_queue_info: *const DeviceQueueInfo2<'_>,
        p_queue: *mut Queue,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDescriptorSetLayoutSupport.html>
    pub type PFN_vkGetDescriptorSetLayoutSupport = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
        p_support: *mut DescriptorSetLayoutSupport<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkDescriptorUpdateTemplate = DescriptorUpdateTemplate;
    pub type VkSamplerYcbcrConversion = SamplerYcbcrConversion;
    pub type VkPhysicalDeviceFeatures2 = PhysicalDeviceFeatures2<'static>;
    pub type VkPhysicalDeviceProperties2 = PhysicalDeviceProperties2<'static>;
    pub type VkFormatProperties2 = FormatProperties2<'static>;
    pub type VkImageFormatProperties2 = ImageFormatProperties2<'static>;
    pub type VkPhysicalDeviceImageFormatInfo2 = PhysicalDeviceImageFormatInfo2<'static>;
    pub type VkQueueFamilyProperties2 = QueueFamilyProperties2<'static>;
    pub type VkPhysicalDeviceMemoryProperties2 = PhysicalDeviceMemoryProperties2<'static>;
    pub type VkSparseImageFormatProperties2 = SparseImageFormatProperties2<'static>;
    pub type VkPhysicalDeviceSparseImageFormatInfo2 = PhysicalDeviceSparseImageFormatInfo2<'static>;
    pub type VkPhysicalDeviceVariablePointersFeatures =
        PhysicalDeviceVariablePointersFeatures<'static>;
    pub type VkExternalMemoryProperties = ExternalMemoryProperties;
    pub type VkPhysicalDeviceExternalImageFormatInfo =
        PhysicalDeviceExternalImageFormatInfo<'static>;
    pub type VkExternalImageFormatProperties = ExternalImageFormatProperties<'static>;
    pub type VkPhysicalDeviceExternalBufferInfo = PhysicalDeviceExternalBufferInfo<'static>;
    pub type VkExternalBufferProperties = ExternalBufferProperties<'static>;
    pub type VkPhysicalDeviceIDProperties = PhysicalDeviceIDProperties<'static>;
    pub type VkExternalMemoryImageCreateInfo = ExternalMemoryImageCreateInfo<'static>;
    pub type VkExternalMemoryBufferCreateInfo = ExternalMemoryBufferCreateInfo<'static>;
    pub type VkExportMemoryAllocateInfo = ExportMemoryAllocateInfo<'static>;
    pub type VkPhysicalDeviceExternalSemaphoreInfo = PhysicalDeviceExternalSemaphoreInfo<'static>;
    pub type VkExternalSemaphoreProperties = ExternalSemaphoreProperties<'static>;
    pub type VkExportSemaphoreCreateInfo = ExportSemaphoreCreateInfo<'static>;
    pub type VkPhysicalDeviceExternalFenceInfo = PhysicalDeviceExternalFenceInfo<'static>;
    pub type VkExternalFenceProperties = ExternalFenceProperties<'static>;
    pub type VkExportFenceCreateInfo = ExportFenceCreateInfo<'static>;
    pub type VkPhysicalDeviceMultiviewFeatures = PhysicalDeviceMultiviewFeatures<'static>;
    pub type VkPhysicalDeviceMultiviewProperties = PhysicalDeviceMultiviewProperties<'static>;
    pub type VkRenderPassMultiviewCreateInfo = RenderPassMultiviewCreateInfo<'static>;
    pub type VkPhysicalDeviceGroupProperties = PhysicalDeviceGroupProperties<'static>;
    pub type VkMemoryAllocateFlagsInfo = MemoryAllocateFlagsInfo<'static>;
    pub type VkBindBufferMemoryInfo = BindBufferMemoryInfo<'static>;
    pub type VkBindBufferMemoryDeviceGroupInfo = BindBufferMemoryDeviceGroupInfo<'static>;
    pub type VkBindImageMemoryInfo = BindImageMemoryInfo<'static>;
    pub type VkBindImageMemoryDeviceGroupInfo = BindImageMemoryDeviceGroupInfo<'static>;
    pub type VkDeviceGroupRenderPassBeginInfo = DeviceGroupRenderPassBeginInfo<'static>;
    pub type VkDeviceGroupCommandBufferBeginInfo = DeviceGroupCommandBufferBeginInfo<'static>;
    pub type VkDeviceGroupSubmitInfo = DeviceGroupSubmitInfo<'static>;
    pub type VkDeviceGroupBindSparseInfo = DeviceGroupBindSparseInfo<'static>;
    pub type VkDeviceGroupDeviceCreateInfo = DeviceGroupDeviceCreateInfo<'static>;
    pub type VkDescriptorUpdateTemplateEntry = DescriptorUpdateTemplateEntry;
    pub type VkDescriptorUpdateTemplateCreateInfo = DescriptorUpdateTemplateCreateInfo<'static>;
    pub type VkInputAttachmentAspectReference = InputAttachmentAspectReference;
    pub type VkRenderPassInputAttachmentAspectCreateInfo =
        RenderPassInputAttachmentAspectCreateInfo<'static>;
    pub type VkPhysicalDevice16BitStorageFeatures = PhysicalDevice16BitStorageFeatures<'static>;
    pub type VkPhysicalDeviceSubgroupProperties = PhysicalDeviceSubgroupProperties<'static>;
    pub type VkBufferMemoryRequirementsInfo2 = BufferMemoryRequirementsInfo2<'static>;
    pub type VkImageMemoryRequirementsInfo2 = ImageMemoryRequirementsInfo2<'static>;
    pub type VkImageSparseMemoryRequirementsInfo2 = ImageSparseMemoryRequirementsInfo2<'static>;
    pub type VkMemoryRequirements2 = MemoryRequirements2<'static>;
    pub type VkSparseImageMemoryRequirements2 = SparseImageMemoryRequirements2<'static>;
    pub type VkPhysicalDevicePointClippingProperties =
        PhysicalDevicePointClippingProperties<'static>;
    pub type VkMemoryDedicatedRequirements = MemoryDedicatedRequirements<'static>;
    pub type VkMemoryDedicatedAllocateInfo = MemoryDedicatedAllocateInfo<'static>;
    pub type VkImageViewUsageCreateInfo = ImageViewUsageCreateInfo<'static>;
    pub type VkPipelineTessellationDomainOriginStateCreateInfo =
        PipelineTessellationDomainOriginStateCreateInfo<'static>;
    pub type VkSamplerYcbcrConversionInfo = SamplerYcbcrConversionInfo<'static>;
    pub type VkSamplerYcbcrConversionCreateInfo = SamplerYcbcrConversionCreateInfo<'static>;
    pub type VkBindImagePlaneMemoryInfo = BindImagePlaneMemoryInfo<'static>;
    pub type VkImagePlaneMemoryRequirementsInfo = ImagePlaneMemoryRequirementsInfo<'static>;
    pub type VkPhysicalDeviceSamplerYcbcrConversionFeatures =
        PhysicalDeviceSamplerYcbcrConversionFeatures<'static>;
    pub type VkSamplerYcbcrConversionImageFormatProperties =
        SamplerYcbcrConversionImageFormatProperties<'static>;
    pub type VkProtectedSubmitInfo = ProtectedSubmitInfo<'static>;
    pub type VkPhysicalDeviceProtectedMemoryFeatures =
        PhysicalDeviceProtectedMemoryFeatures<'static>;
    pub type VkPhysicalDeviceProtectedMemoryProperties =
        PhysicalDeviceProtectedMemoryProperties<'static>;
    pub type VkDeviceQueueInfo2 = DeviceQueueInfo2<'static>;
    pub type VkPhysicalDeviceMaintenance3Properties = PhysicalDeviceMaintenance3Properties<'static>;
    pub type VkDescriptorSetLayoutSupport = DescriptorSetLayoutSupport<'static>;
    pub type VkPhysicalDeviceShaderDrawParametersFeatures =
        PhysicalDeviceShaderDrawParametersFeatures<'static>;
    pub type VkDescriptorUpdateTemplateType = DescriptorUpdateTemplateType;
    pub type VkPointClippingBehavior = PointClippingBehavior;
    pub type VkTessellationDomainOrigin = TessellationDomainOrigin;
    pub type VkSamplerYcbcrModelConversion = SamplerYcbcrModelConversion;
    pub type VkSamplerYcbcrRange = SamplerYcbcrRange;
    pub type VkChromaLocation = ChromaLocation;
    pub type VkSubgroupFeatureFlags = SubgroupFeatureFlags;
    pub type VkSubgroupFeatureFlagBits = SubgroupFeatureFlagBits;
    pub type VkDescriptorUpdateTemplateCreateFlags = DescriptorUpdateTemplateCreateFlags;
    pub type VkPeerMemoryFeatureFlags = PeerMemoryFeatureFlags;
    pub type VkPeerMemoryFeatureFlagBits = PeerMemoryFeatureFlagBits;
    pub type VkMemoryAllocateFlags = MemoryAllocateFlags;
    pub type VkMemoryAllocateFlagBits = MemoryAllocateFlagBits;
    pub type VkCommandPoolTrimFlags = CommandPoolTrimFlags;
    pub type VkExternalMemoryHandleTypeFlags = ExternalMemoryHandleTypeFlags;
    pub type VkExternalMemoryHandleTypeFlagBits = ExternalMemoryHandleTypeFlagBits;
    pub type VkExternalMemoryFeatureFlags = ExternalMemoryFeatureFlags;
    pub type VkExternalMemoryFeatureFlagBits = ExternalMemoryFeatureFlagBits;
    pub type VkExternalSemaphoreHandleTypeFlags = ExternalSemaphoreHandleTypeFlags;
    pub type VkExternalSemaphoreHandleTypeFlagBits = ExternalSemaphoreHandleTypeFlagBits;
    pub type VkExternalSemaphoreFeatureFlags = ExternalSemaphoreFeatureFlags;
    pub type VkExternalSemaphoreFeatureFlagBits = ExternalSemaphoreFeatureFlagBits;
    pub type VkSemaphoreImportFlags = SemaphoreImportFlags;
    pub type VkSemaphoreImportFlagBits = SemaphoreImportFlagBits;
    pub type VkExternalFenceHandleTypeFlags = ExternalFenceHandleTypeFlags;
    pub type VkExternalFenceHandleTypeFlagBits = ExternalFenceHandleTypeFlagBits;
    pub type VkExternalFenceFeatureFlags = ExternalFenceFeatureFlags;
    pub type VkExternalFenceFeatureFlagBits = ExternalFenceFeatureFlagBits;
    pub type VkFenceImportFlags = FenceImportFlags;
    pub type VkFenceImportFlagBits = FenceImportFlagBits;
    pub type VkPhysicalDeviceVariablePointerFeatures =
        PhysicalDeviceVariablePointerFeatures<'static>;
    pub type VkPhysicalDeviceShaderDrawParameterFeatures =
        PhysicalDeviceShaderDrawParameterFeatures<'static>;
    impl PhysicalDeviceFeatures2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceFeatures2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceProperties2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceProperties2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl FormatProperties2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkFormatProperties2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageFormatProperties2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageFormatProperties2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceImageFormatInfo2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceImageFormatInfo2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl QueueFamilyProperties2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkQueueFamilyProperties2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceMemoryProperties2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceMemoryProperties2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SparseImageFormatProperties2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSparseImageFormatProperties2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceSparseImageFormatInfo2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceSparseImageFormatInfo2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceVariablePointersFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceVariablePointersFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceExternalImageFormatInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceExternalImageFormatInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExternalImageFormatProperties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExternalImageFormatProperties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceExternalBufferInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceExternalBufferInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExternalBufferProperties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExternalBufferProperties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceIDProperties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceIDProperties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExternalMemoryImageCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExternalMemoryImageCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExternalMemoryBufferCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExternalMemoryBufferCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExportMemoryAllocateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExportMemoryAllocateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceExternalSemaphoreInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceExternalSemaphoreInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExternalSemaphoreProperties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExternalSemaphoreProperties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExportSemaphoreCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExportSemaphoreCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceExternalFenceInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceExternalFenceInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExternalFenceProperties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExternalFenceProperties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExportFenceCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExportFenceCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceMultiviewFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceMultiviewFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceMultiviewProperties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceMultiviewProperties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl RenderPassMultiviewCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkRenderPassMultiviewCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceGroupProperties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceGroupProperties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MemoryAllocateFlagsInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMemoryAllocateFlagsInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BindBufferMemoryInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBindBufferMemoryInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BindBufferMemoryDeviceGroupInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBindBufferMemoryDeviceGroupInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BindImageMemoryInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBindImageMemoryInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BindImageMemoryDeviceGroupInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBindImageMemoryDeviceGroupInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceGroupRenderPassBeginInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceGroupRenderPassBeginInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceGroupCommandBufferBeginInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceGroupCommandBufferBeginInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceGroupSubmitInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceGroupSubmitInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceGroupBindSparseInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceGroupBindSparseInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceGroupDeviceCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceGroupDeviceCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DescriptorUpdateTemplateCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDescriptorUpdateTemplateCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl RenderPassInputAttachmentAspectCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkRenderPassInputAttachmentAspectCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDevice16BitStorageFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDevice16BitStorageFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceSubgroupProperties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceSubgroupProperties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BufferMemoryRequirementsInfo2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBufferMemoryRequirementsInfo2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageMemoryRequirementsInfo2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageMemoryRequirementsInfo2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageSparseMemoryRequirementsInfo2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageSparseMemoryRequirementsInfo2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MemoryRequirements2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMemoryRequirements2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SparseImageMemoryRequirements2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSparseImageMemoryRequirements2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDevicePointClippingProperties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDevicePointClippingProperties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MemoryDedicatedRequirements<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMemoryDedicatedRequirements {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MemoryDedicatedAllocateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMemoryDedicatedAllocateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageViewUsageCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageViewUsageCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineTessellationDomainOriginStateCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPipelineTessellationDomainOriginStateCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SamplerYcbcrConversionInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSamplerYcbcrConversionInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SamplerYcbcrConversionCreateInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSamplerYcbcrConversionCreateInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BindImagePlaneMemoryInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBindImagePlaneMemoryInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImagePlaneMemoryRequirementsInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImagePlaneMemoryRequirementsInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceSamplerYcbcrConversionFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceSamplerYcbcrConversionFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SamplerYcbcrConversionImageFormatProperties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkSamplerYcbcrConversionImageFormatProperties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ProtectedSubmitInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkProtectedSubmitInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceProtectedMemoryFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceProtectedMemoryFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceProtectedMemoryProperties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceProtectedMemoryProperties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DeviceQueueInfo2<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDeviceQueueInfo2 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceMaintenance3Properties<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceMaintenance3Properties {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DescriptorSetLayoutSupport<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDescriptorSetLayoutSupport {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceShaderDrawParametersFeatures<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderDrawParametersFeatures {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct EntryFn {
    enumerate_instance_version: PFN_vkEnumerateInstanceVersion,
}

impl EntryFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                enumerate_instance_version: transmute(
                    load(c"vkEnumerateInstanceVersion").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl EntryFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkEnumerateInstanceVersion.html>
    #[inline]
    pub unsafe fn enumerate_instance_version(&self) -> crate::Result<u32> {
        unsafe {
            let mut api_version = core::mem::MaybeUninit::uninit();
            let result = (self.enumerate_instance_version)(api_version.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(api_version.assume_init()),
                err => Err(err),
            }
        }
    }
}

pub struct InstanceFn {
    enumerate_physical_device_groups: PFN_vkEnumeratePhysicalDeviceGroups,
    get_physical_device_features2: PFN_vkGetPhysicalDeviceFeatures2,
    get_physical_device_properties2: PFN_vkGetPhysicalDeviceProperties2,
    get_physical_device_format_properties2: PFN_vkGetPhysicalDeviceFormatProperties2,
    get_physical_device_image_format_properties2: PFN_vkGetPhysicalDeviceImageFormatProperties2,
    get_physical_device_queue_family_properties2: PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
    get_physical_device_memory_properties2: PFN_vkGetPhysicalDeviceMemoryProperties2,
    get_physical_device_sparse_image_format_properties2:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
    get_physical_device_external_buffer_properties: PFN_vkGetPhysicalDeviceExternalBufferProperties,
    get_physical_device_external_fence_properties: PFN_vkGetPhysicalDeviceExternalFenceProperties,
    get_physical_device_external_semaphore_properties:
        PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                enumerate_physical_device_groups: transmute(
                    load(c"vkEnumeratePhysicalDeviceGroups").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_features2: transmute(
                    load(c"vkGetPhysicalDeviceFeatures2").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_properties2: transmute(
                    load(c"vkGetPhysicalDeviceProperties2").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_format_properties2: transmute(
                    load(c"vkGetPhysicalDeviceFormatProperties2").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_image_format_properties2: transmute(
                    load(c"vkGetPhysicalDeviceImageFormatProperties2")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_queue_family_properties2: transmute(
                    load(c"vkGetPhysicalDeviceQueueFamilyProperties2")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_memory_properties2: transmute(
                    load(c"vkGetPhysicalDeviceMemoryProperties2").ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_sparse_image_format_properties2: transmute(
                    load(c"vkGetPhysicalDeviceSparseImageFormatProperties2")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_external_buffer_properties: transmute(
                    load(c"vkGetPhysicalDeviceExternalBufferProperties")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_external_fence_properties: transmute(
                    load(c"vkGetPhysicalDeviceExternalFenceProperties")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_external_semaphore_properties: transmute(
                    load(c"vkGetPhysicalDeviceExternalSemaphoreProperties")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkEnumeratePhysicalDeviceGroups.html>
    #[inline]
    pub unsafe fn enumerate_physical_device_groups<'a>(
        &self,
        instance: Instance,
        mut physical_device_group_properties: impl ExtendUninit<PhysicalDeviceGroupProperties<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |physical_device_group_count, physical_device_group_properties| {
                let result = (self.enumerate_physical_device_groups)(
                    instance,
                    physical_device_group_count,
                    physical_device_group_properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let physical_device_group_properties_buf =
                physical_device_group_properties.reserve(capacity);
            len = physical_device_group_properties_buf
                .len()
                .try_into()
                .unwrap();
            let result = call(
                &mut len,
                physical_device_group_properties_buf.as_mut_ptr() as *mut _,
            )?;
            physical_device_group_properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceFeatures2.html>
    #[inline]
    pub unsafe fn get_physical_device_features2(
        &self,
        physical_device: PhysicalDevice,
        features: &mut PhysicalDeviceFeatures2<'_>,
    ) {
        unsafe { (self.get_physical_device_features2)(physical_device, features) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceProperties2.html>
    #[inline]
    pub unsafe fn get_physical_device_properties2(
        &self,
        physical_device: PhysicalDevice,
        properties: &mut PhysicalDeviceProperties2<'_>,
    ) {
        unsafe { (self.get_physical_device_properties2)(physical_device, properties) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceFormatProperties2.html>
    #[inline]
    pub unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        format_properties: &mut FormatProperties2<'_>,
    ) {
        unsafe {
            (self.get_physical_device_format_properties2)(
                physical_device,
                format,
                format_properties,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceImageFormatProperties2.html>
    #[inline]
    pub unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        image_format_info: &PhysicalDeviceImageFormatInfo2<'_>,
        image_format_properties: &mut ImageFormatProperties2<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_physical_device_image_format_properties2)(
                physical_device,
                image_format_info,
                image_format_properties,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html>
    #[inline]
    pub unsafe fn get_physical_device_queue_family_properties2<'a>(
        &self,
        physical_device: PhysicalDevice,
        mut queue_family_properties: impl ExtendUninit<QueueFamilyProperties2<'a>>,
    ) {
        unsafe {
            let call = |queue_family_property_count, queue_family_properties| {
                (self.get_physical_device_queue_family_properties2)(
                    physical_device,
                    queue_family_property_count,
                    queue_family_properties as _,
                )
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut());
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let queue_family_properties_buf = queue_family_properties.reserve(capacity);
            len = queue_family_properties_buf.len().try_into().unwrap();
            call(&mut len, queue_family_properties_buf.as_mut_ptr() as *mut _);
            queue_family_properties.set_len(len.try_into().unwrap());
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceMemoryProperties2.html>
    #[inline]
    pub unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: PhysicalDevice,
        memory_properties: &mut PhysicalDeviceMemoryProperties2<'_>,
    ) {
        unsafe { (self.get_physical_device_memory_properties2)(physical_device, memory_properties) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html>
    #[inline]
    pub unsafe fn get_physical_device_sparse_image_format_properties2<'a>(
        &self,
        physical_device: PhysicalDevice,
        format_info: &PhysicalDeviceSparseImageFormatInfo2<'a>,
        mut properties: impl ExtendUninit<SparseImageFormatProperties2<'a>>,
    ) {
        unsafe {
            let call = |property_count, properties| {
                (self.get_physical_device_sparse_image_format_properties2)(
                    physical_device,
                    format_info,
                    property_count,
                    properties as _,
                )
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut());
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let properties_buf = properties.reserve(capacity);
            len = properties_buf.len().try_into().unwrap();
            call(&mut len, properties_buf.as_mut_ptr() as *mut _);
            properties.set_len(len.try_into().unwrap());
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceExternalBufferProperties.html>
    #[inline]
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: PhysicalDevice,
        external_buffer_info: &PhysicalDeviceExternalBufferInfo<'_>,
        external_buffer_properties: &mut ExternalBufferProperties<'_>,
    ) {
        unsafe {
            (self.get_physical_device_external_buffer_properties)(
                physical_device,
                external_buffer_info,
                external_buffer_properties,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceExternalFenceProperties.html>
    #[inline]
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: PhysicalDevice,
        external_fence_info: &PhysicalDeviceExternalFenceInfo<'_>,
        external_fence_properties: &mut ExternalFenceProperties<'_>,
    ) {
        unsafe {
            (self.get_physical_device_external_fence_properties)(
                physical_device,
                external_fence_info,
                external_fence_properties,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html>
    #[inline]
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: PhysicalDevice,
        external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo<'_>,
        external_semaphore_properties: &mut ExternalSemaphoreProperties<'_>,
    ) {
        unsafe {
            (self.get_physical_device_external_semaphore_properties)(
                physical_device,
                external_semaphore_info,
                external_semaphore_properties,
            )
        }
    }
}

pub struct DeviceFn {
    bind_buffer_memory2: PFN_vkBindBufferMemory2,
    bind_image_memory2: PFN_vkBindImageMemory2,
    get_device_group_peer_memory_features: PFN_vkGetDeviceGroupPeerMemoryFeatures,
    cmd_set_device_mask: PFN_vkCmdSetDeviceMask,
    get_image_memory_requirements2: PFN_vkGetImageMemoryRequirements2,
    get_buffer_memory_requirements2: PFN_vkGetBufferMemoryRequirements2,
    get_image_sparse_memory_requirements2: PFN_vkGetImageSparseMemoryRequirements2,
    trim_command_pool: PFN_vkTrimCommandPool,
    get_device_queue2: PFN_vkGetDeviceQueue2,
    cmd_dispatch_base: PFN_vkCmdDispatchBase,
    create_descriptor_update_template: PFN_vkCreateDescriptorUpdateTemplate,
    destroy_descriptor_update_template: PFN_vkDestroyDescriptorUpdateTemplate,
    update_descriptor_set_with_template: PFN_vkUpdateDescriptorSetWithTemplate,
    get_descriptor_set_layout_support: PFN_vkGetDescriptorSetLayoutSupport,
    create_sampler_ycbcr_conversion: PFN_vkCreateSamplerYcbcrConversion,
    destroy_sampler_ycbcr_conversion: PFN_vkDestroySamplerYcbcrConversion,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                bind_buffer_memory2: transmute(
                    load(c"vkBindBufferMemory2").ok_or(MissingEntryPointError)?,
                ),
                bind_image_memory2: transmute(
                    load(c"vkBindImageMemory2").ok_or(MissingEntryPointError)?,
                ),
                get_device_group_peer_memory_features: transmute(
                    load(c"vkGetDeviceGroupPeerMemoryFeatures").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_device_mask: transmute(
                    load(c"vkCmdSetDeviceMask").ok_or(MissingEntryPointError)?,
                ),
                get_image_memory_requirements2: transmute(
                    load(c"vkGetImageMemoryRequirements2").ok_or(MissingEntryPointError)?,
                ),
                get_buffer_memory_requirements2: transmute(
                    load(c"vkGetBufferMemoryRequirements2").ok_or(MissingEntryPointError)?,
                ),
                get_image_sparse_memory_requirements2: transmute(
                    load(c"vkGetImageSparseMemoryRequirements2").ok_or(MissingEntryPointError)?,
                ),
                trim_command_pool: transmute(
                    load(c"vkTrimCommandPool").ok_or(MissingEntryPointError)?,
                ),
                get_device_queue2: transmute(
                    load(c"vkGetDeviceQueue2").ok_or(MissingEntryPointError)?,
                ),
                cmd_dispatch_base: transmute(
                    load(c"vkCmdDispatchBase").ok_or(MissingEntryPointError)?,
                ),
                create_descriptor_update_template: transmute(
                    load(c"vkCreateDescriptorUpdateTemplate").ok_or(MissingEntryPointError)?,
                ),
                destroy_descriptor_update_template: transmute(
                    load(c"vkDestroyDescriptorUpdateTemplate").ok_or(MissingEntryPointError)?,
                ),
                update_descriptor_set_with_template: transmute(
                    load(c"vkUpdateDescriptorSetWithTemplate").ok_or(MissingEntryPointError)?,
                ),
                get_descriptor_set_layout_support: transmute(
                    load(c"vkGetDescriptorSetLayoutSupport").ok_or(MissingEntryPointError)?,
                ),
                create_sampler_ycbcr_conversion: transmute(
                    load(c"vkCreateSamplerYcbcrConversion").ok_or(MissingEntryPointError)?,
                ),
                destroy_sampler_ycbcr_conversion: transmute(
                    load(c"vkDestroySamplerYcbcrConversion").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBindBufferMemory2.html>
    #[inline]
    pub unsafe fn bind_buffer_memory2(
        &self,
        device: Device,
        bind_infos: &[BindBufferMemoryInfo<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.bind_buffer_memory2)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBindImageMemory2.html>
    #[inline]
    pub unsafe fn bind_image_memory2(
        &self,
        device: Device,
        bind_infos: &[BindImageMemoryInfo<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.bind_image_memory2)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceGroupPeerMemoryFeatures.html>
    #[inline]
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        device: Device,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> PeerMemoryFeatureFlags {
        unsafe {
            let mut peer_memory_features = core::mem::MaybeUninit::uninit();
            (self.get_device_group_peer_memory_features)(
                device,
                heap_index,
                local_device_index,
                remote_device_index,
                peer_memory_features.as_mut_ptr(),
            );
            peer_memory_features.assume_init()
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDeviceMask.html>
    #[inline]
    pub unsafe fn cmd_set_device_mask(&self, command_buffer: CommandBuffer, device_mask: u32) {
        unsafe { (self.cmd_set_device_mask)(command_buffer, device_mask) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageMemoryRequirements2.html>
    #[inline]
    pub unsafe fn get_image_memory_requirements2(
        &self,
        device: Device,
        info: &ImageMemoryRequirementsInfo2<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe { (self.get_image_memory_requirements2)(device, info, memory_requirements) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetBufferMemoryRequirements2.html>
    #[inline]
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        device: Device,
        info: &BufferMemoryRequirementsInfo2<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe { (self.get_buffer_memory_requirements2)(device, info, memory_requirements) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageSparseMemoryRequirements2.html>
    #[inline]
    pub unsafe fn get_image_sparse_memory_requirements2<'a>(
        &self,
        device: Device,
        info: &ImageSparseMemoryRequirementsInfo2<'a>,
        mut sparse_memory_requirements: impl ExtendUninit<SparseImageMemoryRequirements2<'a>>,
    ) {
        unsafe {
            let call = |sparse_memory_requirement_count, sparse_memory_requirements| {
                (self.get_image_sparse_memory_requirements2)(
                    device,
                    info,
                    sparse_memory_requirement_count,
                    sparse_memory_requirements as _,
                )
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut());
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let sparse_memory_requirements_buf = sparse_memory_requirements.reserve(capacity);
            len = sparse_memory_requirements_buf.len().try_into().unwrap();
            call(
                &mut len,
                sparse_memory_requirements_buf.as_mut_ptr() as *mut _,
            );
            sparse_memory_requirements.set_len(len.try_into().unwrap());
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkTrimCommandPool.html>
    #[inline]
    pub unsafe fn trim_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ) {
        unsafe { (self.trim_command_pool)(device, command_pool, flags) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceQueue2.html>
    #[inline]
    pub unsafe fn get_device_queue2(
        &self,
        device: Device,
        queue_info: &DeviceQueueInfo2<'_>,
    ) -> Queue {
        unsafe {
            let mut queue = core::mem::MaybeUninit::uninit();
            (self.get_device_queue2)(device, queue_info, queue.as_mut_ptr());
            queue.assume_init()
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDispatchBase.html>
    #[inline]
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe {
            (self.cmd_dispatch_base)(
                command_buffer,
                base_group_x,
                base_group_y,
                base_group_z,
                group_count_x,
                group_count_y,
                group_count_z,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateDescriptorUpdateTemplate.html>
    #[inline]
    pub unsafe fn create_descriptor_update_template(
        &self,
        device: Device,
        create_info: &DescriptorUpdateTemplateCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<DescriptorUpdateTemplate> {
        unsafe {
            let mut descriptor_update_template = core::mem::MaybeUninit::uninit();
            let result = (self.create_descriptor_update_template)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                descriptor_update_template.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(descriptor_update_template.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyDescriptorUpdateTemplate.html>
    #[inline]
    pub unsafe fn destroy_descriptor_update_template(
        &self,
        device: Device,
        descriptor_update_template: DescriptorUpdateTemplate,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_descriptor_update_template)(
                device,
                descriptor_update_template,
                allocator.to_raw_ptr(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUpdateDescriptorSetWithTemplate.html>
    #[inline]
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        device: Device,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        data: *const c_void,
    ) {
        unsafe {
            (self.update_descriptor_set_with_template)(
                device,
                descriptor_set,
                descriptor_update_template,
                data,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDescriptorSetLayoutSupport.html>
    #[inline]
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        device: Device,
        create_info: &DescriptorSetLayoutCreateInfo<'_>,
        support: &mut DescriptorSetLayoutSupport<'_>,
    ) {
        unsafe { (self.get_descriptor_set_layout_support)(device, create_info, support) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateSamplerYcbcrConversion.html>
    #[inline]
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        device: Device,
        create_info: &SamplerYcbcrConversionCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<SamplerYcbcrConversion> {
        unsafe {
            let mut ycbcr_conversion = core::mem::MaybeUninit::uninit();
            let result = (self.create_sampler_ycbcr_conversion)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                ycbcr_conversion.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(ycbcr_conversion.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroySamplerYcbcrConversion.html>
    #[inline]
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        device: Device,
        ycbcr_conversion: SamplerYcbcrConversion,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_sampler_ycbcr_conversion)(
                device,
                ycbcr_conversion,
                allocator.to_raw_ptr(),
            )
        }
    }
}

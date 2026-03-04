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
    pub const MAX_DEVICE_GROUP_SIZE: u32 = 32;
    pub const LUID_SIZE: u32 = 8;
    pub const QUEUE_FAMILY_EXTERNAL: u32 = !1;
    handle_nondispatchable!(
        DescriptorUpdateTemplate,
        DESCRIPTOR_UPDATE_TEMPLATE,
        doc = ""
    );
    handle_nondispatchable!(SamplerYcbcrConversion, SAMPLER_YCBCR_CONVERSION, doc = "");
    pub type PhysicalDeviceVariablePointerFeatures<'a> = PhysicalDeviceVariablePointersFeatures<'a>;
    pub type PhysicalDeviceShaderDrawParameterFeatures<'a> =
        PhysicalDeviceShaderDrawParametersFeatures<'a>;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceFeatures2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub features: PhysicalDeviceFeatures,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFeatures2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_FEATURES_2;
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceFeatures2<'a> {}
    impl Default for PhysicalDeviceFeatures2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                features: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceFeatures2<'a> {
        pub fn features(mut self, features: PhysicalDeviceFeatures) -> Self {
            self.features = features;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceProperties2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub properties: PhysicalDeviceProperties,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceProperties2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_PROPERTIES_2;
    }
    impl Default for PhysicalDeviceProperties2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceProperties2<'a> {
        pub fn properties(mut self, properties: PhysicalDeviceProperties) -> Self {
            self.properties = properties;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct FormatProperties2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub format_properties: FormatProperties,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for FormatProperties2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::FORMAT_PROPERTIES_2;
    }
    impl Default for FormatProperties2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                format_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> FormatProperties2<'a> {
        pub fn format_properties(mut self, format_properties: FormatProperties) -> Self {
            self.format_properties = format_properties;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageFormatProperties2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image_format_properties: ImageFormatProperties,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageFormatProperties2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_FORMAT_PROPERTIES_2;
    }
    impl Default for ImageFormatProperties2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                image_format_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageFormatProperties2<'a> {
        pub fn image_format_properties(
            mut self,
            image_format_properties: ImageFormatProperties,
        ) -> Self {
            self.image_format_properties = image_format_properties;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageFormatInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2;
    }
    impl Default for PhysicalDeviceImageFormatInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
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
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
        pub fn ty(mut self, ty: ImageType) -> Self {
            self.ty = ty;
            self
        }
        pub fn tiling(mut self, tiling: ImageTiling) -> Self {
            self.tiling = tiling;
            self
        }
        pub fn usage(mut self, usage: ImageUsageFlags) -> Self {
            self.usage = usage;
            self
        }
        pub fn flags(mut self, flags: ImageCreateFlags) -> Self {
            self.flags = flags;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct QueueFamilyProperties2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub queue_family_properties: QueueFamilyProperties,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for QueueFamilyProperties2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::QUEUE_FAMILY_PROPERTIES_2;
    }
    impl Default for QueueFamilyProperties2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                queue_family_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> QueueFamilyProperties2<'a> {
        pub fn queue_family_properties(
            mut self,
            queue_family_properties: QueueFamilyProperties,
        ) -> Self {
            self.queue_family_properties = queue_family_properties;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMemoryProperties2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_properties: PhysicalDeviceMemoryProperties,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMemoryProperties2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2;
    }
    impl Default for PhysicalDeviceMemoryProperties2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                memory_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMemoryProperties2<'a> {
        pub fn memory_properties(
            mut self,
            memory_properties: PhysicalDeviceMemoryProperties,
        ) -> Self {
            self.memory_properties = memory_properties;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SparseImageFormatProperties2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub properties: SparseImageFormatProperties,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SparseImageFormatProperties2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SPARSE_IMAGE_FORMAT_PROPERTIES_2;
    }
    impl Default for SparseImageFormatProperties2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SparseImageFormatProperties2<'a> {
        pub fn properties(mut self, properties: SparseImageFormatProperties) -> Self {
            self.properties = properties;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSparseImageFormatInfo2<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2;
    }
    impl Default for PhysicalDeviceSparseImageFormatInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
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
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
        pub fn ty(mut self, ty: ImageType) -> Self {
            self.ty = ty;
            self
        }
        pub fn samples(mut self, samples: SampleCountFlagBits) -> Self {
            self.samples = samples;
            self
        }
        pub fn usage(mut self, usage: ImageUsageFlags) -> Self {
            self.usage = usage;
            self
        }
        pub fn tiling(mut self, tiling: ImageTiling) -> Self {
            self.tiling = tiling;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVariablePointersFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub variable_pointers_storage_buffer: Bool32,
        pub variable_pointers: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                variable_pointers_storage_buffer: Default::default(),
                variable_pointers: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceVariablePointersFeatures<'a> {
        pub fn variable_pointers_storage_buffer(
            mut self,
            variable_pointers_storage_buffer: Bool32,
        ) -> Self {
            self.variable_pointers_storage_buffer = variable_pointers_storage_buffer;
            self
        }
        pub fn variable_pointers(mut self, variable_pointers: Bool32) -> Self {
            self.variable_pointers = variable_pointers;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct ExternalMemoryProperties {
        pub external_memory_features: ExternalMemoryFeatureFlags,
        pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
        pub compatible_handle_types: ExternalMemoryHandleTypeFlags,
    }
    impl ExternalMemoryProperties {
        pub fn external_memory_features(
            mut self,
            external_memory_features: ExternalMemoryFeatureFlags,
        ) -> Self {
            self.external_memory_features = external_memory_features;
            self
        }
        pub fn export_from_imported_handle_types(
            mut self,
            export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
        ) -> Self {
            self.export_from_imported_handle_types = export_from_imported_handle_types;
            self
        }
        pub fn compatible_handle_types(
            mut self,
            compatible_handle_types: ExternalMemoryHandleTypeFlags,
        ) -> Self {
            self.compatible_handle_types = compatible_handle_types;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceExternalImageFormatInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceExternalImageFormatInfo<'a> {
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExternalImageFormatProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub external_memory_properties: ExternalMemoryProperties,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ExternalImageFormatProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_IMAGE_FORMAT_PROPERTIES;
    }
    unsafe impl<'a> Extends<ImageFormatProperties2<'a>> for ExternalImageFormatProperties<'a> {}
    impl Default for ExternalImageFormatProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                external_memory_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ExternalImageFormatProperties<'a> {
        pub fn external_memory_properties(
            mut self,
            external_memory_properties: ExternalMemoryProperties,
        ) -> Self {
            self.external_memory_properties = external_memory_properties;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceExternalBufferInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: BufferCreateFlags,
        pub usage: BufferUsageFlags,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExternalBufferInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO;
    }
    impl Default for PhysicalDeviceExternalBufferInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                usage: Default::default(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceExternalBufferInfo<'a> {
        pub fn flags(mut self, flags: BufferCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn usage(mut self, usage: BufferUsageFlags) -> Self {
            self.usage = usage;
            self
        }
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExternalBufferProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub external_memory_properties: ExternalMemoryProperties,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ExternalBufferProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_BUFFER_PROPERTIES;
    }
    impl Default for ExternalBufferProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                external_memory_properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ExternalBufferProperties<'a> {
        pub fn external_memory_properties(
            mut self,
            external_memory_properties: ExternalMemoryProperties,
        ) -> Self {
            self.external_memory_properties = external_memory_properties;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceIDProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_ID_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>> for PhysicalDeviceIDProperties<'a> {}
    impl Default for PhysicalDeviceIDProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
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
        pub fn device_uuid(mut self, device_uuid: [u8; UUID_SIZE as usize]) -> Self {
            self.device_uuid = device_uuid;
            self
        }
        pub fn driver_uuid(mut self, driver_uuid: [u8; UUID_SIZE as usize]) -> Self {
            self.driver_uuid = driver_uuid;
            self
        }
        pub fn device_luid(mut self, device_luid: [u8; LUID_SIZE as usize]) -> Self {
            self.device_luid = device_luid;
            self
        }
        pub fn device_node_mask(mut self, device_node_mask: u32) -> Self {
            self.device_node_mask = device_node_mask;
            self
        }
        pub fn device_luid_valid(mut self, device_luid_valid: Bool32) -> Self {
            self.device_luid_valid = device_luid_valid;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExternalMemoryImageCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_types: ExternalMemoryHandleTypeFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ExternalMemoryImageCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO;
    }
    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for ExternalMemoryImageCreateInfo<'a> {}
    impl Default for ExternalMemoryImageCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                handle_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ExternalMemoryImageCreateInfo<'a> {
        pub fn handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlags) -> Self {
            self.handle_types = handle_types;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExternalMemoryBufferCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_types: ExternalMemoryHandleTypeFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ExternalMemoryBufferCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_MEMORY_BUFFER_CREATE_INFO;
    }
    unsafe impl<'a> Extends<BufferCreateInfo<'a>> for ExternalMemoryBufferCreateInfo<'a> {}
    impl Default for ExternalMemoryBufferCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                handle_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ExternalMemoryBufferCreateInfo<'a> {
        pub fn handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlags) -> Self {
            self.handle_types = handle_types;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExportMemoryAllocateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_types: ExternalMemoryHandleTypeFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ExportMemoryAllocateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_MEMORY_ALLOCATE_INFO;
    }
    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for ExportMemoryAllocateInfo<'a> {}
    impl Default for ExportMemoryAllocateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                handle_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ExportMemoryAllocateInfo<'a> {
        pub fn handle_types(mut self, handle_types: ExternalMemoryHandleTypeFlags) -> Self {
            self.handle_types = handle_types;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceExternalSemaphoreInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExternalSemaphoreInfo<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO;
    }
    impl Default for PhysicalDeviceExternalSemaphoreInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceExternalSemaphoreInfo<'a> {
        pub fn handle_type(mut self, handle_type: ExternalSemaphoreHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExternalSemaphoreProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
        pub compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
        pub external_semaphore_features: ExternalSemaphoreFeatureFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ExternalSemaphoreProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_SEMAPHORE_PROPERTIES;
    }
    impl Default for ExternalSemaphoreProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                export_from_imported_handle_types: Default::default(),
                compatible_handle_types: Default::default(),
                external_semaphore_features: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ExternalSemaphoreProperties<'a> {
        pub fn export_from_imported_handle_types(
            mut self,
            export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
        ) -> Self {
            self.export_from_imported_handle_types = export_from_imported_handle_types;
            self
        }
        pub fn compatible_handle_types(
            mut self,
            compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
        ) -> Self {
            self.compatible_handle_types = compatible_handle_types;
            self
        }
        pub fn external_semaphore_features(
            mut self,
            external_semaphore_features: ExternalSemaphoreFeatureFlags,
        ) -> Self {
            self.external_semaphore_features = external_semaphore_features;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExportSemaphoreCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_types: ExternalSemaphoreHandleTypeFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ExportSemaphoreCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_SEMAPHORE_CREATE_INFO;
    }
    unsafe impl<'a> Extends<SemaphoreCreateInfo<'a>> for ExportSemaphoreCreateInfo<'a> {}
    impl Default for ExportSemaphoreCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                handle_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ExportSemaphoreCreateInfo<'a> {
        pub fn handle_types(mut self, handle_types: ExternalSemaphoreHandleTypeFlags) -> Self {
            self.handle_types = handle_types;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceExternalFenceInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_type: ExternalFenceHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceExternalFenceInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO;
    }
    impl Default for PhysicalDeviceExternalFenceInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceExternalFenceInfo<'a> {
        pub fn handle_type(mut self, handle_type: ExternalFenceHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExternalFenceProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
        pub compatible_handle_types: ExternalFenceHandleTypeFlags,
        pub external_fence_features: ExternalFenceFeatureFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ExternalFenceProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXTERNAL_FENCE_PROPERTIES;
    }
    impl Default for ExternalFenceProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                export_from_imported_handle_types: Default::default(),
                compatible_handle_types: Default::default(),
                external_fence_features: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ExternalFenceProperties<'a> {
        pub fn export_from_imported_handle_types(
            mut self,
            export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
        ) -> Self {
            self.export_from_imported_handle_types = export_from_imported_handle_types;
            self
        }
        pub fn compatible_handle_types(
            mut self,
            compatible_handle_types: ExternalFenceHandleTypeFlags,
        ) -> Self {
            self.compatible_handle_types = compatible_handle_types;
            self
        }
        pub fn external_fence_features(
            mut self,
            external_fence_features: ExternalFenceFeatureFlags,
        ) -> Self {
            self.external_fence_features = external_fence_features;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExportFenceCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_types: ExternalFenceHandleTypeFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ExportFenceCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_FENCE_CREATE_INFO;
    }
    unsafe impl<'a> Extends<FenceCreateInfo<'a>> for ExportFenceCreateInfo<'a> {}
    impl Default for ExportFenceCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                handle_types: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ExportFenceCreateInfo<'a> {
        pub fn handle_types(mut self, handle_types: ExternalFenceHandleTypeFlags) -> Self {
            self.handle_types = handle_types;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMultiviewFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub multiview: Bool32,
        pub multiview_geometry_shader: Bool32,
        pub multiview_tessellation_shader: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                multiview: Default::default(),
                multiview_geometry_shader: Default::default(),
                multiview_tessellation_shader: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMultiviewFeatures<'a> {
        pub fn multiview(mut self, multiview: Bool32) -> Self {
            self.multiview = multiview;
            self
        }
        pub fn multiview_geometry_shader(mut self, multiview_geometry_shader: Bool32) -> Self {
            self.multiview_geometry_shader = multiview_geometry_shader;
            self
        }
        pub fn multiview_tessellation_shader(
            mut self,
            multiview_tessellation_shader: Bool32,
        ) -> Self {
            self.multiview_tessellation_shader = multiview_tessellation_shader;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMultiviewProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_multiview_view_count: u32,
        pub max_multiview_instance_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMultiviewProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>> for PhysicalDeviceMultiviewProperties<'a> {}
    impl Default for PhysicalDeviceMultiviewProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_multiview_view_count: Default::default(),
                max_multiview_instance_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMultiviewProperties<'a> {
        pub fn max_multiview_view_count(mut self, max_multiview_view_count: u32) -> Self {
            self.max_multiview_view_count = max_multiview_view_count;
            self
        }
        pub fn max_multiview_instance_index(mut self, max_multiview_instance_index: u32) -> Self {
            self.max_multiview_instance_index = max_multiview_instance_index;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for RenderPassMultiviewCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDER_PASS_MULTIVIEW_CREATE_INFO;
    }
    unsafe impl<'a> Extends<RenderPassCreateInfo<'a>> for RenderPassMultiviewCreateInfo<'a> {}
    impl Default for RenderPassMultiviewCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                subpass_count: Default::default(),
                p_view_masks: core::ptr::null(),
                dependency_count: Default::default(),
                p_view_offsets: core::ptr::null(),
                correlation_mask_count: Default::default(),
                p_correlation_masks: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderPassMultiviewCreateInfo<'a> {
        pub fn view_masks(mut self, view_masks: &'a [u32]) -> Self {
            self.subpass_count = view_masks.len().try_into().unwrap();
            self.p_view_masks = view_masks.as_ptr();
            self
        }
        pub fn view_offsets(mut self, view_offsets: &'a [i32]) -> Self {
            self.dependency_count = view_offsets.len().try_into().unwrap();
            self.p_view_offsets = view_offsets.as_ptr();
            self
        }
        pub fn correlation_masks(mut self, correlation_masks: &'a [u32]) -> Self {
            self.correlation_mask_count = correlation_masks.len().try_into().unwrap();
            self.p_correlation_masks = correlation_masks.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceGroupProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub physical_device_count: u32,
        pub physical_devices: [PhysicalDevice; MAX_DEVICE_GROUP_SIZE as usize],
        pub subset_allocation: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceGroupProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_GROUP_PROPERTIES;
    }
    impl Default for PhysicalDeviceGroupProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                physical_device_count: Default::default(),
                physical_devices: [Default::default(); _],
                subset_allocation: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceGroupProperties<'a> {
        pub fn physical_devices(mut self, physical_devices: &[PhysicalDevice]) -> Self {
            self.physical_device_count = physical_devices.len().try_into().unwrap();
            self.physical_devices[..physical_devices.len()].copy_from_slice(physical_devices);
            self
        }
        pub fn subset_allocation(mut self, subset_allocation: Bool32) -> Self {
            self.subset_allocation = subset_allocation;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryAllocateFlagsInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: MemoryAllocateFlags,
        pub device_mask: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryAllocateFlagsInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_ALLOCATE_FLAGS_INFO;
    }
    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for MemoryAllocateFlagsInfo<'a> {}
    impl Default for MemoryAllocateFlagsInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                device_mask: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryAllocateFlagsInfo<'a> {
        pub fn flags(mut self, flags: MemoryAllocateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn device_mask(mut self, device_mask: u32) -> Self {
            self.device_mask = device_mask;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BindBufferMemoryInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub buffer: Buffer,
        pub memory: DeviceMemory,
        pub memory_offset: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BindBufferMemoryInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_BUFFER_MEMORY_INFO;
    }
    impl Default for BindBufferMemoryInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                buffer: Default::default(),
                memory: Default::default(),
                memory_offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BindBufferMemoryInfo<'a> {
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
        pub fn memory_offset(mut self, memory_offset: DeviceSize) -> Self {
            self.memory_offset = memory_offset;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BindBufferMemoryDeviceGroupInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub device_index_count: u32,
        pub p_device_indices: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BindBufferMemoryDeviceGroupInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO;
    }
    unsafe impl<'a> Extends<BindBufferMemoryInfo<'a>> for BindBufferMemoryDeviceGroupInfo<'a> {}
    impl Default for BindBufferMemoryDeviceGroupInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                device_index_count: Default::default(),
                p_device_indices: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BindBufferMemoryDeviceGroupInfo<'a> {
        pub fn device_indices(mut self, device_indices: &'a [u32]) -> Self {
            self.device_index_count = device_indices.len().try_into().unwrap();
            self.p_device_indices = device_indices.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BindImageMemoryInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image: Image,
        pub memory: DeviceMemory,
        pub memory_offset: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BindImageMemoryInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_IMAGE_MEMORY_INFO;
    }
    impl Default for BindImageMemoryInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                image: Default::default(),
                memory: Default::default(),
                memory_offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BindImageMemoryInfo<'a> {
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
        pub fn memory_offset(mut self, memory_offset: DeviceSize) -> Self {
            self.memory_offset = memory_offset;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BindImageMemoryDeviceGroupInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub device_index_count: u32,
        pub p_device_indices: *const u32,
        pub split_instance_bind_region_count: u32,
        pub p_split_instance_bind_regions: *const Rect2D,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BindImageMemoryDeviceGroupInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO;
    }
    unsafe impl<'a> Extends<BindImageMemoryInfo<'a>> for BindImageMemoryDeviceGroupInfo<'a> {}
    impl Default for BindImageMemoryDeviceGroupInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                device_index_count: Default::default(),
                p_device_indices: core::ptr::null(),
                split_instance_bind_region_count: Default::default(),
                p_split_instance_bind_regions: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BindImageMemoryDeviceGroupInfo<'a> {
        pub fn device_indices(mut self, device_indices: &'a [u32]) -> Self {
            self.device_index_count = device_indices.len().try_into().unwrap();
            self.p_device_indices = device_indices.as_ptr();
            self
        }
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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceGroupRenderPassBeginInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub device_mask: u32,
        pub device_render_area_count: u32,
        pub p_device_render_areas: *const Rect2D,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null(),
                device_mask: Default::default(),
                device_render_area_count: Default::default(),
                p_device_render_areas: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceGroupRenderPassBeginInfo<'a> {
        pub fn device_mask(mut self, device_mask: u32) -> Self {
            self.device_mask = device_mask;
            self
        }
        pub fn device_render_areas(mut self, device_render_areas: &'a [Rect2D]) -> Self {
            self.device_render_area_count = device_render_areas.len().try_into().unwrap();
            self.p_device_render_areas = device_render_areas.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceGroupCommandBufferBeginInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub device_mask: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DeviceGroupCommandBufferBeginInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO;
    }
    unsafe impl<'a> Extends<CommandBufferBeginInfo<'a>> for DeviceGroupCommandBufferBeginInfo<'a> {}
    impl Default for DeviceGroupCommandBufferBeginInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                device_mask: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceGroupCommandBufferBeginInfo<'a> {
        pub fn device_mask(mut self, device_mask: u32) -> Self {
            self.device_mask = device_mask;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for DeviceGroupSubmitInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_GROUP_SUBMIT_INFO;
    }
    unsafe impl<'a> Extends<SubmitInfo<'a>> for DeviceGroupSubmitInfo<'a> {}
    impl Default for DeviceGroupSubmitInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                wait_semaphore_count: Default::default(),
                p_wait_semaphore_device_indices: core::ptr::null(),
                command_buffer_count: Default::default(),
                p_command_buffer_device_masks: core::ptr::null(),
                signal_semaphore_count: Default::default(),
                p_signal_semaphore_device_indices: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceGroupSubmitInfo<'a> {
        pub fn wait_semaphore_device_indices(
            mut self,
            wait_semaphore_device_indices: &'a [u32],
        ) -> Self {
            self.wait_semaphore_count = wait_semaphore_device_indices.len().try_into().unwrap();
            self.p_wait_semaphore_device_indices = wait_semaphore_device_indices.as_ptr();
            self
        }
        pub fn command_buffer_device_masks(
            mut self,
            command_buffer_device_masks: &'a [u32],
        ) -> Self {
            self.command_buffer_count = command_buffer_device_masks.len().try_into().unwrap();
            self.p_command_buffer_device_masks = command_buffer_device_masks.as_ptr();
            self
        }
        pub fn signal_semaphore_device_indices(
            mut self,
            signal_semaphore_device_indices: &'a [u32],
        ) -> Self {
            self.signal_semaphore_count = signal_semaphore_device_indices.len().try_into().unwrap();
            self.p_signal_semaphore_device_indices = signal_semaphore_device_indices.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceGroupBindSparseInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub resource_device_index: u32,
        pub memory_device_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DeviceGroupBindSparseInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_GROUP_BIND_SPARSE_INFO;
    }
    unsafe impl<'a> Extends<BindSparseInfo<'a>> for DeviceGroupBindSparseInfo<'a> {}
    impl Default for DeviceGroupBindSparseInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                resource_device_index: Default::default(),
                memory_device_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceGroupBindSparseInfo<'a> {
        pub fn resource_device_index(mut self, resource_device_index: u32) -> Self {
            self.resource_device_index = resource_device_index;
            self
        }
        pub fn memory_device_index(mut self, memory_device_index: u32) -> Self {
            self.memory_device_index = memory_device_index;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceGroupDeviceCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub physical_device_count: u32,
        pub p_physical_devices: *const PhysicalDevice,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DeviceGroupDeviceCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_GROUP_DEVICE_CREATE_INFO;
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for DeviceGroupDeviceCreateInfo<'a> {}
    impl Default for DeviceGroupDeviceCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                physical_device_count: Default::default(),
                p_physical_devices: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceGroupDeviceCreateInfo<'a> {
        pub fn physical_devices(mut self, physical_devices: &'a [PhysicalDevice]) -> Self {
            self.physical_device_count = physical_devices.len().try_into().unwrap();
            self.p_physical_devices = physical_devices.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct DescriptorUpdateTemplateEntry {
        pub dst_binding: u32,
        pub dst_array_element: u32,
        pub descriptor_count: u32,
        pub descriptor_type: DescriptorType,
        pub offset: usize,
        pub stride: usize,
    }
    impl DescriptorUpdateTemplateEntry {
        pub fn dst_binding(mut self, dst_binding: u32) -> Self {
            self.dst_binding = dst_binding;
            self
        }
        pub fn dst_array_element(mut self, dst_array_element: u32) -> Self {
            self.dst_array_element = dst_array_element;
            self
        }
        pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
            self.descriptor_count = descriptor_count;
            self
        }
        pub fn descriptor_type(mut self, descriptor_type: DescriptorType) -> Self {
            self.descriptor_type = descriptor_type;
            self
        }
        pub fn offset(mut self, offset: usize) -> Self {
            self.offset = offset;
            self
        }
        pub fn stride(mut self, stride: usize) -> Self {
            self.stride = stride;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for DescriptorUpdateTemplateCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO;
    }
    impl Default for DescriptorUpdateTemplateCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                descriptor_update_entry_count: Default::default(),
                p_descriptor_update_entries: core::ptr::null(),
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
        pub fn flags(mut self, flags: DescriptorUpdateTemplateCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn descriptor_update_entries(
            mut self,
            descriptor_update_entries: &'a [DescriptorUpdateTemplateEntry],
        ) -> Self {
            self.descriptor_update_entry_count =
                descriptor_update_entries.len().try_into().unwrap();
            self.p_descriptor_update_entries = descriptor_update_entries.as_ptr();
            self
        }
        pub fn template_type(mut self, template_type: DescriptorUpdateTemplateType) -> Self {
            self.template_type = template_type;
            self
        }
        pub fn descriptor_set_layout(mut self, descriptor_set_layout: DescriptorSetLayout) -> Self {
            self.descriptor_set_layout = descriptor_set_layout;
            self
        }
        pub fn pipeline_bind_point(mut self, pipeline_bind_point: PipelineBindPoint) -> Self {
            self.pipeline_bind_point = pipeline_bind_point;
            self
        }
        pub fn pipeline_layout(mut self, pipeline_layout: PipelineLayout) -> Self {
            self.pipeline_layout = pipeline_layout;
            self
        }
        pub fn set(mut self, set: u32) -> Self {
            self.set = set;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct InputAttachmentAspectReference {
        pub subpass: u32,
        pub input_attachment_index: u32,
        pub aspect_mask: ImageAspectFlags,
    }
    impl InputAttachmentAspectReference {
        pub fn subpass(mut self, subpass: u32) -> Self {
            self.subpass = subpass;
            self
        }
        pub fn input_attachment_index(mut self, input_attachment_index: u32) -> Self {
            self.input_attachment_index = input_attachment_index;
            self
        }
        pub fn aspect_mask(mut self, aspect_mask: ImageAspectFlags) -> Self {
            self.aspect_mask = aspect_mask;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderPassInputAttachmentAspectCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub aspect_reference_count: u32,
        pub p_aspect_references: *const InputAttachmentAspectReference,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null(),
                aspect_reference_count: Default::default(),
                p_aspect_references: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderPassInputAttachmentAspectCreateInfo<'a> {
        pub fn aspect_references(
            mut self,
            aspect_references: &'a [InputAttachmentAspectReference],
        ) -> Self {
            self.aspect_reference_count = aspect_references.len().try_into().unwrap();
            self.p_aspect_references = aspect_references.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevice16BitStorageFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub storage_buffer16_bit_access: Bool32,
        pub uniform_and_storage_buffer16_bit_access: Bool32,
        pub storage_push_constant16: Bool32,
        pub storage_input_output16: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                storage_buffer16_bit_access: Default::default(),
                uniform_and_storage_buffer16_bit_access: Default::default(),
                storage_push_constant16: Default::default(),
                storage_input_output16: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevice16BitStorageFeatures<'a> {
        pub fn storage_buffer16_bit_access(mut self, storage_buffer16_bit_access: Bool32) -> Self {
            self.storage_buffer16_bit_access = storage_buffer16_bit_access;
            self
        }
        pub fn uniform_and_storage_buffer16_bit_access(
            mut self,
            uniform_and_storage_buffer16_bit_access: Bool32,
        ) -> Self {
            self.uniform_and_storage_buffer16_bit_access = uniform_and_storage_buffer16_bit_access;
            self
        }
        pub fn storage_push_constant16(mut self, storage_push_constant16: Bool32) -> Self {
            self.storage_push_constant16 = storage_push_constant16;
            self
        }
        pub fn storage_input_output16(mut self, storage_input_output16: Bool32) -> Self {
            self.storage_input_output16 = storage_input_output16;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceSubgroupProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub subgroup_size: u32,
        pub supported_stages: ShaderStageFlags,
        pub supported_operations: SubgroupFeatureFlags,
        pub quad_operations_in_all_stages: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSubgroupProperties<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>> for PhysicalDeviceSubgroupProperties<'a> {}
    impl Default for PhysicalDeviceSubgroupProperties<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                subgroup_size: Default::default(),
                supported_stages: Default::default(),
                supported_operations: Default::default(),
                quad_operations_in_all_stages: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceSubgroupProperties<'a> {
        pub fn subgroup_size(mut self, subgroup_size: u32) -> Self {
            self.subgroup_size = subgroup_size;
            self
        }
        pub fn supported_stages(mut self, supported_stages: ShaderStageFlags) -> Self {
            self.supported_stages = supported_stages;
            self
        }
        pub fn supported_operations(mut self, supported_operations: SubgroupFeatureFlags) -> Self {
            self.supported_operations = supported_operations;
            self
        }
        pub fn quad_operations_in_all_stages(
            mut self,
            quad_operations_in_all_stages: Bool32,
        ) -> Self {
            self.quad_operations_in_all_stages = quad_operations_in_all_stages;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BufferMemoryRequirementsInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub buffer: Buffer,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BufferMemoryRequirementsInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BUFFER_MEMORY_REQUIREMENTS_INFO_2;
    }
    impl Default for BufferMemoryRequirementsInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                buffer: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BufferMemoryRequirementsInfo2<'a> {
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageMemoryRequirementsInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image: Image,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageMemoryRequirementsInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_MEMORY_REQUIREMENTS_INFO_2;
    }
    impl Default for ImageMemoryRequirementsInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                image: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageMemoryRequirementsInfo2<'a> {
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageSparseMemoryRequirementsInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image: Image,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageSparseMemoryRequirementsInfo2<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2;
    }
    impl Default for ImageSparseMemoryRequirementsInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                image: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageSparseMemoryRequirementsInfo2<'a> {
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryRequirements2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_requirements: MemoryRequirements,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryRequirements2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_REQUIREMENTS_2;
    }
    impl Default for MemoryRequirements2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                memory_requirements: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryRequirements2<'a> {
        pub fn memory_requirements(mut self, memory_requirements: MemoryRequirements) -> Self {
            self.memory_requirements = memory_requirements;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SparseImageMemoryRequirements2<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_requirements: SparseImageMemoryRequirements,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SparseImageMemoryRequirements2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2;
    }
    impl Default for SparseImageMemoryRequirements2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                memory_requirements: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SparseImageMemoryRequirements2<'a> {
        pub fn memory_requirements(
            mut self,
            memory_requirements: SparseImageMemoryRequirements,
        ) -> Self {
            self.memory_requirements = memory_requirements;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePointClippingProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub point_clipping_behavior: PointClippingBehavior,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                point_clipping_behavior: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePointClippingProperties<'a> {
        pub fn point_clipping_behavior(
            mut self,
            point_clipping_behavior: PointClippingBehavior,
        ) -> Self {
            self.point_clipping_behavior = point_clipping_behavior;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryDedicatedRequirements<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub prefers_dedicated_allocation: Bool32,
        pub requires_dedicated_allocation: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryDedicatedRequirements<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_DEDICATED_REQUIREMENTS;
    }
    unsafe impl<'a> Extends<MemoryRequirements2<'a>> for MemoryDedicatedRequirements<'a> {}
    impl Default for MemoryDedicatedRequirements<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                prefers_dedicated_allocation: Default::default(),
                requires_dedicated_allocation: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryDedicatedRequirements<'a> {
        pub fn prefers_dedicated_allocation(
            mut self,
            prefers_dedicated_allocation: Bool32,
        ) -> Self {
            self.prefers_dedicated_allocation = prefers_dedicated_allocation;
            self
        }
        pub fn requires_dedicated_allocation(
            mut self,
            requires_dedicated_allocation: Bool32,
        ) -> Self {
            self.requires_dedicated_allocation = requires_dedicated_allocation;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryDedicatedAllocateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image: Image,
        pub buffer: Buffer,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryDedicatedAllocateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_DEDICATED_ALLOCATE_INFO;
    }
    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for MemoryDedicatedAllocateInfo<'a> {}
    impl Default for MemoryDedicatedAllocateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                image: Default::default(),
                buffer: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryDedicatedAllocateInfo<'a> {
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageViewUsageCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub usage: ImageUsageFlags,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImageViewUsageCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_VIEW_USAGE_CREATE_INFO;
    }
    unsafe impl<'a> Extends<ImageViewCreateInfo<'a>> for ImageViewUsageCreateInfo<'a> {}
    impl Default for ImageViewUsageCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImageViewUsageCreateInfo<'a> {
        pub fn usage(mut self, usage: ImageUsageFlags) -> Self {
            self.usage = usage;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineTessellationDomainOriginStateCreateInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub domain_origin: TessellationDomainOrigin,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null(),
                domain_origin: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineTessellationDomainOriginStateCreateInfo<'a> {
        pub fn domain_origin(mut self, domain_origin: TessellationDomainOrigin) -> Self {
            self.domain_origin = domain_origin;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SamplerYcbcrConversionInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub conversion: SamplerYcbcrConversion,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null(),
                conversion: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SamplerYcbcrConversionInfo<'a> {
        pub fn conversion(mut self, conversion: SamplerYcbcrConversion) -> Self {
            self.conversion = conversion;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for SamplerYcbcrConversionCreateInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SAMPLER_YCBCR_CONVERSION_CREATE_INFO;
    }
    impl Default for SamplerYcbcrConversionCreateInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
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
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
        pub fn ycbcr_model(mut self, ycbcr_model: SamplerYcbcrModelConversion) -> Self {
            self.ycbcr_model = ycbcr_model;
            self
        }
        pub fn ycbcr_range(mut self, ycbcr_range: SamplerYcbcrRange) -> Self {
            self.ycbcr_range = ycbcr_range;
            self
        }
        pub fn components(mut self, components: ComponentMapping) -> Self {
            self.components = components;
            self
        }
        pub fn x_chroma_offset(mut self, x_chroma_offset: ChromaLocation) -> Self {
            self.x_chroma_offset = x_chroma_offset;
            self
        }
        pub fn y_chroma_offset(mut self, y_chroma_offset: ChromaLocation) -> Self {
            self.y_chroma_offset = y_chroma_offset;
            self
        }
        pub fn chroma_filter(mut self, chroma_filter: Filter) -> Self {
            self.chroma_filter = chroma_filter;
            self
        }
        pub fn force_explicit_reconstruction(
            mut self,
            force_explicit_reconstruction: Bool32,
        ) -> Self {
            self.force_explicit_reconstruction = force_explicit_reconstruction;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BindImagePlaneMemoryInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub plane_aspect: ImageAspectFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for BindImagePlaneMemoryInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_IMAGE_PLANE_MEMORY_INFO;
    }
    unsafe impl<'a> Extends<BindImageMemoryInfo<'a>> for BindImagePlaneMemoryInfo<'a> {}
    impl Default for BindImagePlaneMemoryInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                plane_aspect: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> BindImagePlaneMemoryInfo<'a> {
        pub fn plane_aspect(mut self, plane_aspect: ImageAspectFlagBits) -> Self {
            self.plane_aspect = plane_aspect;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImagePlaneMemoryRequirementsInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub plane_aspect: ImageAspectFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImagePlaneMemoryRequirementsInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO;
    }
    unsafe impl<'a> Extends<ImageMemoryRequirementsInfo2<'a>> for ImagePlaneMemoryRequirementsInfo<'a> {}
    impl Default for ImagePlaneMemoryRequirementsInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                plane_aspect: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImagePlaneMemoryRequirementsInfo<'a> {
        pub fn plane_aspect(mut self, plane_aspect: ImageAspectFlagBits) -> Self {
            self.plane_aspect = plane_aspect;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceSamplerYcbcrConversionFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub sampler_ycbcr_conversion: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                sampler_ycbcr_conversion: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceSamplerYcbcrConversionFeatures<'a> {
        pub fn sampler_ycbcr_conversion(mut self, sampler_ycbcr_conversion: Bool32) -> Self {
            self.sampler_ycbcr_conversion = sampler_ycbcr_conversion;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SamplerYcbcrConversionImageFormatProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub combined_image_sampler_descriptor_count: u32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                combined_image_sampler_descriptor_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SamplerYcbcrConversionImageFormatProperties<'a> {
        pub fn combined_image_sampler_descriptor_count(
            mut self,
            combined_image_sampler_descriptor_count: u32,
        ) -> Self {
            self.combined_image_sampler_descriptor_count = combined_image_sampler_descriptor_count;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ProtectedSubmitInfo<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub protected_submit: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ProtectedSubmitInfo<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PROTECTED_SUBMIT_INFO;
    }
    unsafe impl<'a> Extends<SubmitInfo<'a>> for ProtectedSubmitInfo<'a> {}
    impl Default for ProtectedSubmitInfo<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                protected_submit: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ProtectedSubmitInfo<'a> {
        pub fn protected_submit(mut self, protected_submit: Bool32) -> Self {
            self.protected_submit = protected_submit;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceProtectedMemoryFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub protected_memory: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                protected_memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceProtectedMemoryFeatures<'a> {
        pub fn protected_memory(mut self, protected_memory: Bool32) -> Self {
            self.protected_memory = protected_memory;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceProtectedMemoryProperties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub protected_no_fault: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                protected_no_fault: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceProtectedMemoryProperties<'a> {
        pub fn protected_no_fault(mut self, protected_no_fault: Bool32) -> Self {
            self.protected_no_fault = protected_no_fault;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DeviceQueueInfo2<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DeviceQueueCreateFlags,
        pub queue_family_index: u32,
        pub queue_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DeviceQueueInfo2<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_QUEUE_INFO_2;
    }
    impl Default for DeviceQueueInfo2<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                queue_family_index: Default::default(),
                queue_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DeviceQueueInfo2<'a> {
        pub fn flags(mut self, flags: DeviceQueueCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
            self.queue_family_index = queue_family_index;
            self
        }
        pub fn queue_index(mut self, queue_index: u32) -> Self {
            self.queue_index = queue_index;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMaintenance3Properties<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_per_set_descriptors: u32,
        pub max_memory_allocation_size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                max_per_set_descriptors: Default::default(),
                max_memory_allocation_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMaintenance3Properties<'a> {
        pub fn max_per_set_descriptors(mut self, max_per_set_descriptors: u32) -> Self {
            self.max_per_set_descriptors = max_per_set_descriptors;
            self
        }
        pub fn max_memory_allocation_size(
            mut self,
            max_memory_allocation_size: DeviceSize,
        ) -> Self {
            self.max_memory_allocation_size = max_memory_allocation_size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DescriptorSetLayoutSupport<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub supported: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for DescriptorSetLayoutSupport<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DESCRIPTOR_SET_LAYOUT_SUPPORT;
    }
    impl Default for DescriptorSetLayoutSupport<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                supported: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DescriptorSetLayoutSupport<'a> {
        pub fn supported(mut self, supported: Bool32) -> Self {
            self.supported = supported;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderDrawParametersFeatures<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_draw_parameters: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                shader_draw_parameters: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderDrawParametersFeatures<'a> {
        pub fn shader_draw_parameters(mut self, shader_draw_parameters: Bool32) -> Self {
            self.shader_draw_parameters = shader_draw_parameters;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DescriptorUpdateTemplateType(i32);
    impl DescriptorUpdateTemplateType {
        pub const DESCRIPTOR_SET: Self = Self(0);
        pub const PUSH_DESCRIPTORS: Self = Self(1);
        pub const DESCRIPTOR_SET_KHR: Self = Self::DESCRIPTOR_SET;
        pub const PUSH_DESCRIPTORS_KHR: Self = Self::PUSH_DESCRIPTORS;
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PointClippingBehavior(i32);
    impl PointClippingBehavior {
        pub const ALL_CLIP_PLANES: Self = Self(0);
        pub const USER_CLIP_PLANES_ONLY: Self = Self(1);
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TessellationDomainOrigin(i32);
    impl TessellationDomainOrigin {
        pub const UPPER_LEFT: Self = Self(0);
        pub const LOWER_LEFT: Self = Self(1);
        pub const LOWER_LEFT_KHR: Self = Self::LOWER_LEFT;
        pub const UPPER_LEFT_KHR: Self = Self::UPPER_LEFT;
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SamplerYcbcrModelConversion(i32);
    impl SamplerYcbcrModelConversion {
        pub const RGB_IDENTITY: Self = Self(0);
        pub const YCBCR_IDENTITY: Self = Self(1);
        pub const YCBCR_709: Self = Self(2);
        pub const YCBCR_601: Self = Self(3);
        pub const YCBCR_2020: Self = Self(4);
        pub const RGB_IDENTITY_KHR: Self = Self::RGB_IDENTITY;
        pub const YCBCR_2020_KHR: Self = Self::YCBCR_2020;
        pub const YCBCR_601_KHR: Self = Self::YCBCR_601;
        pub const YCBCR_709_KHR: Self = Self::YCBCR_709;
        pub const YCBCR_IDENTITY_KHR: Self = Self::YCBCR_IDENTITY;
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SamplerYcbcrRange(i32);
    impl SamplerYcbcrRange {
        pub const ITU_FULL: Self = Self(0);
        pub const ITU_NARROW: Self = Self(1);
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ChromaLocation(i32);
    impl ChromaLocation {
        pub const COSITED_EVEN: Self = Self(0);
        pub const MIDPOINT: Self = Self(1);
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
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SubgroupFeatureFlags(Flags);
    vk_bitflags_wrapped!(SubgroupFeatureFlags, Flags);
    impl SubgroupFeatureFlags {
        pub const BASIC: Self = Self(SubgroupFeatureFlagBits::BASIC.0);
        pub const VOTE: Self = Self(SubgroupFeatureFlagBits::VOTE.0);
        pub const ARITHMETIC: Self = Self(SubgroupFeatureFlagBits::ARITHMETIC.0);
        pub const BALLOT: Self = Self(SubgroupFeatureFlagBits::BALLOT.0);
        pub const SHUFFLE: Self = Self(SubgroupFeatureFlagBits::SHUFFLE.0);
        pub const SHUFFLE_RELATIVE: Self = Self(SubgroupFeatureFlagBits::SHUFFLE_RELATIVE.0);
        pub const CLUSTERED: Self = Self(SubgroupFeatureFlagBits::CLUSTERED.0);
        pub const QUAD: Self = Self(SubgroupFeatureFlagBits::QUAD.0);
        pub const PARTITIONED_EXT: Self = Self(SubgroupFeatureFlagBits::PARTITIONED_EXT.0);
        pub const ROTATE: Self = Self(SubgroupFeatureFlagBits::ROTATE.0);
        pub const ROTATE_CLUSTERED: Self = Self(SubgroupFeatureFlagBits::ROTATE_CLUSTERED.0);
        pub const PARTITIONED_NV: Self = Self::PARTITIONED_EXT;
        pub const ROTATE_KHR: Self = Self::ROTATE;
        pub const ROTATE_CLUSTERED_KHR: Self = Self::ROTATE_CLUSTERED;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct SubgroupFeatureFlagBits(u32);
    impl SubgroupFeatureFlagBits {
        pub const BASIC: Self = Self(1 << 0);
        pub const VOTE: Self = Self(1 << 1);
        pub const ARITHMETIC: Self = Self(1 << 2);
        pub const BALLOT: Self = Self(1 << 3);
        pub const SHUFFLE: Self = Self(1 << 4);
        pub const SHUFFLE_RELATIVE: Self = Self(1 << 5);
        pub const CLUSTERED: Self = Self(1 << 6);
        pub const QUAD: Self = Self(1 << 7);
        pub const PARTITIONED_EXT: Self = Self(1 << 8);
        pub const ROTATE: Self = Self(1 << 9);
        pub const ROTATE_CLUSTERED: Self = Self(1 << 10);
        pub const PARTITIONED_NV: Self = Self::PARTITIONED_EXT;
        pub const ROTATE_KHR: Self = Self::ROTATE;
        pub const ROTATE_CLUSTERED_KHR: Self = Self::ROTATE_CLUSTERED;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DescriptorUpdateTemplateCreateFlags(Flags);
    vk_bitflags_wrapped!(DescriptorUpdateTemplateCreateFlags, Flags);
    impl DescriptorUpdateTemplateCreateFlags {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PeerMemoryFeatureFlags(Flags);
    vk_bitflags_wrapped!(PeerMemoryFeatureFlags, Flags);
    impl PeerMemoryFeatureFlags {
        pub const COPY_SRC: Self = Self(PeerMemoryFeatureFlagBits::COPY_SRC.0);
        pub const COPY_DST: Self = Self(PeerMemoryFeatureFlagBits::COPY_DST.0);
        pub const GENERIC_SRC: Self = Self(PeerMemoryFeatureFlagBits::GENERIC_SRC.0);
        pub const GENERIC_DST: Self = Self(PeerMemoryFeatureFlagBits::GENERIC_DST.0);
        pub const COPY_DST_KHR: Self = Self::COPY_DST;
        pub const COPY_SRC_KHR: Self = Self::COPY_SRC;
        pub const GENERIC_DST_KHR: Self = Self::GENERIC_DST;
        pub const GENERIC_SRC_KHR: Self = Self::GENERIC_SRC;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PeerMemoryFeatureFlagBits(u32);
    impl PeerMemoryFeatureFlagBits {
        pub const COPY_SRC: Self = Self(1 << 0);
        pub const COPY_DST: Self = Self(1 << 1);
        pub const GENERIC_SRC: Self = Self(1 << 2);
        pub const GENERIC_DST: Self = Self(1 << 3);
        pub const COPY_DST_KHR: Self = Self::COPY_DST;
        pub const COPY_SRC_KHR: Self = Self::COPY_SRC;
        pub const GENERIC_DST_KHR: Self = Self::GENERIC_DST;
        pub const GENERIC_SRC_KHR: Self = Self::GENERIC_SRC;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct MemoryAllocateFlags(Flags);
    vk_bitflags_wrapped!(MemoryAllocateFlags, Flags);
    impl MemoryAllocateFlags {
        pub const DEVICE_MASK: Self = Self(MemoryAllocateFlagBits::DEVICE_MASK.0);
        pub const DEVICE_ADDRESS: Self = Self(MemoryAllocateFlagBits::DEVICE_ADDRESS.0);
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self =
            Self(MemoryAllocateFlagBits::DEVICE_ADDRESS_CAPTURE_REPLAY.0);
        pub const ZERO_INITIALIZE_EXT: Self = Self(MemoryAllocateFlagBits::ZERO_INITIALIZE_EXT.0);
        pub const DEVICE_ADDRESS_KHR: Self = Self::DEVICE_ADDRESS;
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
        pub const DEVICE_MASK_KHR: Self = Self::DEVICE_MASK;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct MemoryAllocateFlagBits(u32);
    impl MemoryAllocateFlagBits {
        pub const DEVICE_MASK: Self = Self(1 << 0);
        pub const DEVICE_ADDRESS: Self = Self(1 << 1);
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(1 << 2);
        pub const ZERO_INITIALIZE_EXT: Self = Self(1 << 3);
        pub const DEVICE_ADDRESS_KHR: Self = Self::DEVICE_ADDRESS;
        pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
        pub const DEVICE_MASK_KHR: Self = Self::DEVICE_MASK;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct CommandPoolTrimFlags(Flags);
    vk_bitflags_wrapped!(CommandPoolTrimFlags, Flags);
    impl CommandPoolTrimFlags {}
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
        pub const HOST_ALLOCATION_EXT: Self =
            Self(ExternalMemoryHandleTypeFlagBits::HOST_ALLOCATION_EXT.0);
        pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self =
            Self(ExternalMemoryHandleTypeFlagBits::HOST_MAPPED_FOREIGN_MEMORY_EXT.0);
        pub const DMA_BUF_EXT: Self = Self(ExternalMemoryHandleTypeFlagBits::DMA_BUF_EXT.0);
        pub const ANDROID_HARDWARE_BUFFER_ANDROID: Self =
            Self(ExternalMemoryHandleTypeFlagBits::ANDROID_HARDWARE_BUFFER_ANDROID.0);
        pub const ZIRCON_VMO_FUCHSIA: Self =
            Self(ExternalMemoryHandleTypeFlagBits::ZIRCON_VMO_FUCHSIA.0);
        pub const RDMA_ADDRESS_NV: Self = Self(ExternalMemoryHandleTypeFlagBits::RDMA_ADDRESS_NV.0);
        pub const SCREEN_BUFFER_QNX: Self =
            Self(ExternalMemoryHandleTypeFlagBits::SCREEN_BUFFER_QNX.0);
        pub const OH_NATIVE_BUFFER_OHOS: Self =
            Self(ExternalMemoryHandleTypeFlagBits::OH_NATIVE_BUFFER_OHOS.0);
        pub const MTLBUFFER_EXT: Self = Self(ExternalMemoryHandleTypeFlagBits::MTLBUFFER_EXT.0);
        pub const MTLTEXTURE_EXT: Self = Self(ExternalMemoryHandleTypeFlagBits::MTLTEXTURE_EXT.0);
        pub const MTLHEAP_EXT: Self = Self(ExternalMemoryHandleTypeFlagBits::MTLHEAP_EXT.0);
        pub const D3D11_TEXTURE_KHR: Self = Self::D3D11_TEXTURE;
        pub const D3D11_TEXTURE_KMT_KHR: Self = Self::D3D11_TEXTURE_KMT;
        pub const D3D12_HEAP_KHR: Self = Self::D3D12_HEAP;
        pub const D3D12_RESOURCE_KHR: Self = Self::D3D12_RESOURCE;
        pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
        pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
        pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    }
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
        pub const HOST_ALLOCATION_EXT: Self = Self(1 << 7);
        pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self = Self(1 << 8);
        pub const DMA_BUF_EXT: Self = Self(1 << 9);
        pub const ANDROID_HARDWARE_BUFFER_ANDROID: Self = Self(1 << 10);
        pub const ZIRCON_VMO_FUCHSIA: Self = Self(1 << 11);
        pub const RDMA_ADDRESS_NV: Self = Self(1 << 12);
        pub const SCREEN_BUFFER_QNX: Self = Self(1 << 14);
        pub const OH_NATIVE_BUFFER_OHOS: Self = Self(1 << 15);
        pub const MTLBUFFER_EXT: Self = Self(1 << 16);
        pub const MTLTEXTURE_EXT: Self = Self(1 << 17);
        pub const MTLHEAP_EXT: Self = Self(1 << 18);
        pub const D3D11_TEXTURE_KHR: Self = Self::D3D11_TEXTURE;
        pub const D3D11_TEXTURE_KMT_KHR: Self = Self::D3D11_TEXTURE_KMT;
        pub const D3D12_HEAP_KHR: Self = Self::D3D12_HEAP;
        pub const D3D12_RESOURCE_KHR: Self = Self::D3D12_RESOURCE;
        pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
        pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
        pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ExternalMemoryFeatureFlags(Flags);
    vk_bitflags_wrapped!(ExternalMemoryFeatureFlags, Flags);
    impl ExternalMemoryFeatureFlags {
        pub const DEDICATED_ONLY: Self = Self(ExternalMemoryFeatureFlagBits::DEDICATED_ONLY.0);
        pub const EXPORTABLE: Self = Self(ExternalMemoryFeatureFlagBits::EXPORTABLE.0);
        pub const IMPORTABLE: Self = Self(ExternalMemoryFeatureFlagBits::IMPORTABLE.0);
        pub const DEDICATED_ONLY_KHR: Self = Self::DEDICATED_ONLY;
        pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
        pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalMemoryFeatureFlagBits(u32);
    impl ExternalMemoryFeatureFlagBits {
        pub const DEDICATED_ONLY: Self = Self(1 << 0);
        pub const EXPORTABLE: Self = Self(1 << 1);
        pub const IMPORTABLE: Self = Self(1 << 2);
        pub const DEDICATED_ONLY_KHR: Self = Self::DEDICATED_ONLY;
        pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
        pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    }
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
        pub const ZIRCON_EVENT_FUCHSIA: Self =
            Self(ExternalSemaphoreHandleTypeFlagBits::ZIRCON_EVENT_FUCHSIA.0);
        pub const D3D11_FENCE: Self = Self::D3D12_FENCE;
        pub const D3D12_FENCE_KHR: Self = Self::D3D12_FENCE;
        pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
        pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
        pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
        pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalSemaphoreHandleTypeFlagBits(u32);
    impl ExternalSemaphoreHandleTypeFlagBits {
        pub const OPAQUE_FD: Self = Self(1 << 0);
        pub const OPAQUE_WIN32: Self = Self(1 << 1);
        pub const OPAQUE_WIN32_KMT: Self = Self(1 << 2);
        pub const D3D12_FENCE: Self = Self(1 << 3);
        pub const SYNC_FD: Self = Self(1 << 4);
        pub const ZIRCON_EVENT_FUCHSIA: Self = Self(1 << 7);
        pub const D3D12_FENCE_KHR: Self = Self::D3D12_FENCE;
        pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
        pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
        pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
        pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ExternalSemaphoreFeatureFlags(Flags);
    vk_bitflags_wrapped!(ExternalSemaphoreFeatureFlags, Flags);
    impl ExternalSemaphoreFeatureFlags {
        pub const EXPORTABLE: Self = Self(ExternalSemaphoreFeatureFlagBits::EXPORTABLE.0);
        pub const IMPORTABLE: Self = Self(ExternalSemaphoreFeatureFlagBits::IMPORTABLE.0);
        pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
        pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalSemaphoreFeatureFlagBits(u32);
    impl ExternalSemaphoreFeatureFlagBits {
        pub const EXPORTABLE: Self = Self(1 << 0);
        pub const IMPORTABLE: Self = Self(1 << 1);
        pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
        pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SemaphoreImportFlags(Flags);
    vk_bitflags_wrapped!(SemaphoreImportFlags, Flags);
    impl SemaphoreImportFlags {
        pub const TEMPORARY: Self = Self(SemaphoreImportFlagBits::TEMPORARY.0);
        pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct SemaphoreImportFlagBits(u32);
    impl SemaphoreImportFlagBits {
        pub const TEMPORARY: Self = Self(1 << 0);
        pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
    }
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
        pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
        pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
        pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
        pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalFenceHandleTypeFlagBits(u32);
    impl ExternalFenceHandleTypeFlagBits {
        pub const OPAQUE_FD: Self = Self(1 << 0);
        pub const OPAQUE_WIN32: Self = Self(1 << 1);
        pub const OPAQUE_WIN32_KMT: Self = Self(1 << 2);
        pub const SYNC_FD: Self = Self(1 << 3);
        pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
        pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
        pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
        pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ExternalFenceFeatureFlags(Flags);
    vk_bitflags_wrapped!(ExternalFenceFeatureFlags, Flags);
    impl ExternalFenceFeatureFlags {
        pub const EXPORTABLE: Self = Self(ExternalFenceFeatureFlagBits::EXPORTABLE.0);
        pub const IMPORTABLE: Self = Self(ExternalFenceFeatureFlagBits::IMPORTABLE.0);
        pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
        pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExternalFenceFeatureFlagBits(u32);
    impl ExternalFenceFeatureFlagBits {
        pub const EXPORTABLE: Self = Self(1 << 0);
        pub const IMPORTABLE: Self = Self(1 << 1);
        pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
        pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct FenceImportFlags(Flags);
    vk_bitflags_wrapped!(FenceImportFlags, Flags);
    impl FenceImportFlags {
        pub const TEMPORARY: Self = Self(FenceImportFlagBits::TEMPORARY.0);
        pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct FenceImportFlagBits(u32);
    impl FenceImportFlagBits {
        pub const TEMPORARY: Self = Self(1 << 0);
        pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
    }
    pub type PFN_vkEnumerateInstanceVersion =
        unsafe extern "system" fn(p_api_version: *mut u32) -> vk::Result;
    pub type PFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures2<'_>,
    );
    pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_properties: *mut PhysicalDeviceProperties2<'_>,
    );
    pub type PFN_vkGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *mut FormatProperties2<'_>,
    );
    pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_image_format_info: *const PhysicalDeviceImageFormatInfo2<'_>,
            p_image_format_properties: *mut ImageFormatProperties2<'_>,
        ) -> vk::Result;
    pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties2<'_>,
    );
    pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties2<'_>,
    );
    pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_format_info: *const PhysicalDeviceSparseImageFormatInfo2<'_>,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties2<'_>,
    );
    pub type PFN_vkTrimCommandPool = unsafe extern "system" fn(
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    );
    pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo<'_>,
        p_external_buffer_properties: *mut ExternalBufferProperties<'_>,
    );
    pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo<'_>,
        p_external_semaphore_properties: *mut ExternalSemaphoreProperties<'_>,
    );
    pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_external_fence_info: *const PhysicalDeviceExternalFenceInfo<'_>,
        p_external_fence_properties: *mut ExternalFenceProperties<'_>,
    );
    pub type PFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
        instance: Instance,
        p_physical_device_group_count: *mut u32,
        p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties<'_>,
    ) -> vk::Result;
    pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(
        device: Device,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        p_peer_memory_features: *mut PeerMemoryFeatureFlags,
    );
    pub type PFN_vkBindBufferMemory2 = unsafe extern "system" fn(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindBufferMemoryInfo<'_>,
    ) -> vk::Result;
    pub type PFN_vkBindImageMemory2 = unsafe extern "system" fn(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindImageMemoryInfo<'_>,
    ) -> vk::Result;
    pub type PFN_vkCmdSetDeviceMask =
        unsafe extern "system" fn(command_buffer: CommandBuffer, device_mask: u32);
    pub type PFN_vkCmdDispatchBase = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    );
    pub type PFN_vkCreateDescriptorUpdateTemplate = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const DescriptorUpdateTemplateCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_descriptor_update_template: *mut DescriptorUpdateTemplate,
    ) -> vk::Result;
    pub type PFN_vkDestroyDescriptorUpdateTemplate = unsafe extern "system" fn(
        device: Device,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(
        device: Device,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_data: *const c_void,
    );
    pub type PFN_vkGetBufferMemoryRequirements2 = unsafe extern "system" fn(
        device: Device,
        p_info: *const BufferMemoryRequirementsInfo2<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    );
    pub type PFN_vkGetImageMemoryRequirements2 = unsafe extern "system" fn(
        device: Device,
        p_info: *const ImageMemoryRequirementsInfo2<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    );
    pub type PFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(
        device: Device,
        p_info: *const ImageSparseMemoryRequirementsInfo2<'_>,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
    );
    pub type PFN_vkCreateSamplerYcbcrConversion = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const SamplerYcbcrConversionCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_ycbcr_conversion: *mut SamplerYcbcrConversion,
    ) -> vk::Result;
    pub type PFN_vkDestroySamplerYcbcrConversion = unsafe extern "system" fn(
        device: Device,
        ycbcr_conversion: SamplerYcbcrConversion,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkGetDeviceQueue2 = unsafe extern "system" fn(
        device: Device,
        p_queue_info: *const DeviceQueueInfo2<'_>,
        p_queue: *mut Queue,
    );
    pub type PFN_vkGetDescriptorSetLayoutSupport = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
        p_support: *mut DescriptorSetLayoutSupport<'_>,
    );
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
    pub unsafe fn enumerate_physical_device_groups<'a>(
        &self,
        instance: Instance,
        physical_device_group_properties: impl ExtendUninit<PhysicalDeviceGroupProperties<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(
                physical_device_group_properties,
                |physical_device_group_count, physical_device_group_properties| {
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
                },
            )
        }
    }
    pub unsafe fn get_physical_device_features2(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceFeatures2<'_> {
        unsafe {
            let mut features = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_features2)(physical_device, features.as_mut_ptr());
            features.assume_init()
        }
    }
    pub unsafe fn get_physical_device_properties2(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceProperties2<'_> {
        unsafe {
            let mut properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_properties2)(physical_device, properties.as_mut_ptr());
            properties.assume_init()
        }
    }
    pub unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
    ) -> FormatProperties2<'_> {
        unsafe {
            let mut format_properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_format_properties2)(
                physical_device,
                format,
                format_properties.as_mut_ptr(),
            );
            format_properties.assume_init()
        }
    }
    pub unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        image_format_info: &PhysicalDeviceImageFormatInfo2<'_>,
    ) -> crate::Result<ImageFormatProperties2<'_>> {
        unsafe {
            let mut image_format_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_physical_device_image_format_properties2)(
                physical_device,
                image_format_info,
                image_format_properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(image_format_properties.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_physical_device_queue_family_properties2<'a>(
        &self,
        physical_device: PhysicalDevice,
        queue_family_properties: impl ExtendUninit<QueueFamilyProperties2<'a>>,
    ) {
        unsafe {
            extend_uninit(
                queue_family_properties,
                |queue_family_property_count, queue_family_properties| {
                    (self.get_physical_device_queue_family_properties2)(
                        physical_device,
                        queue_family_property_count,
                        queue_family_properties as _,
                    )
                },
            )
        }
    }
    pub unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceMemoryProperties2<'_> {
        unsafe {
            let mut memory_properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_memory_properties2)(
                physical_device,
                memory_properties.as_mut_ptr(),
            );
            memory_properties.assume_init()
        }
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2<'a>(
        &self,
        physical_device: PhysicalDevice,
        format_info: &PhysicalDeviceSparseImageFormatInfo2<'_>,
        properties: impl ExtendUninit<SparseImageFormatProperties2<'a>>,
    ) {
        unsafe {
            extend_uninit(properties, |property_count, properties| {
                (self.get_physical_device_sparse_image_format_properties2)(
                    physical_device,
                    format_info,
                    property_count,
                    properties as _,
                )
            })
        }
    }
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: PhysicalDevice,
        external_buffer_info: &PhysicalDeviceExternalBufferInfo<'_>,
    ) -> ExternalBufferProperties<'_> {
        unsafe {
            let mut external_buffer_properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_external_buffer_properties)(
                physical_device,
                external_buffer_info,
                external_buffer_properties.as_mut_ptr(),
            );
            external_buffer_properties.assume_init()
        }
    }
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: PhysicalDevice,
        external_fence_info: &PhysicalDeviceExternalFenceInfo<'_>,
    ) -> ExternalFenceProperties<'_> {
        unsafe {
            let mut external_fence_properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_external_fence_properties)(
                physical_device,
                external_fence_info,
                external_fence_properties.as_mut_ptr(),
            );
            external_fence_properties.assume_init()
        }
    }
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: PhysicalDevice,
        external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo<'_>,
    ) -> ExternalSemaphoreProperties<'_> {
        unsafe {
            let mut external_semaphore_properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_external_semaphore_properties)(
                physical_device,
                external_semaphore_info,
                external_semaphore_properties.as_mut_ptr(),
            );
            external_semaphore_properties.assume_init()
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
    pub unsafe fn cmd_set_device_mask(&self, command_buffer: CommandBuffer, device_mask: u32) {
        unsafe { (self.cmd_set_device_mask)(command_buffer, device_mask) }
    }
    pub unsafe fn get_image_memory_requirements2(
        &self,
        device: Device,
        info: &ImageMemoryRequirementsInfo2<'_>,
    ) -> MemoryRequirements2<'_> {
        unsafe {
            let mut memory_requirements = core::mem::MaybeUninit::uninit();
            (self.get_image_memory_requirements2)(device, info, memory_requirements.as_mut_ptr());
            memory_requirements.assume_init()
        }
    }
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        device: Device,
        info: &BufferMemoryRequirementsInfo2<'_>,
    ) -> MemoryRequirements2<'_> {
        unsafe {
            let mut memory_requirements = core::mem::MaybeUninit::uninit();
            (self.get_buffer_memory_requirements2)(device, info, memory_requirements.as_mut_ptr());
            memory_requirements.assume_init()
        }
    }
    pub unsafe fn get_image_sparse_memory_requirements2<'a>(
        &self,
        device: Device,
        info: &ImageSparseMemoryRequirementsInfo2<'_>,
        sparse_memory_requirements: impl ExtendUninit<SparseImageMemoryRequirements2<'a>>,
    ) {
        unsafe {
            extend_uninit(
                sparse_memory_requirements,
                |sparse_memory_requirement_count, sparse_memory_requirements| {
                    (self.get_image_sparse_memory_requirements2)(
                        device,
                        info,
                        sparse_memory_requirement_count,
                        sparse_memory_requirements as _,
                    )
                },
            )
        }
    }
    pub unsafe fn trim_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ) {
        unsafe { (self.trim_command_pool)(device, command_pool, flags) }
    }
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
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        device: Device,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        data: &c_void,
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
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        device: Device,
        create_info: &DescriptorSetLayoutCreateInfo<'_>,
    ) -> DescriptorSetLayoutSupport<'_> {
        unsafe {
            let mut support = core::mem::MaybeUninit::uninit();
            (self.get_descriptor_set_layout_support)(device, create_info, support.as_mut_ptr());
            support.assume_init()
        }
    }
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

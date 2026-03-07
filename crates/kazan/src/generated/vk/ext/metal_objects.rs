#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_metal_objects";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    pub type MTLDevice_id = *const c_void;
    pub type MTLCommandQueue_id = *const c_void;
    pub type MTLBuffer_id = *const c_void;
    pub type MTLTexture_id = *const c_void;
    pub type MTLSharedEvent_id = *const c_void;
    pub type IOSurfaceRef = *const c_void;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalObjectCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExportMetalObjectCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub export_object_type: ExportMetalObjectTypeFlagBitsEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExportMetalObjectCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExportMetalObjectCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("export_object_type", &self.export_object_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExportMetalObjectCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_METAL_OBJECT_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<InstanceCreateInfo<'a>> for ExportMetalObjectCreateInfoEXT<'a> {}
    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for ExportMetalObjectCreateInfoEXT<'a> {}
    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for ExportMetalObjectCreateInfoEXT<'a> {}
    unsafe impl<'a> Extends<ImageViewCreateInfo<'a>> for ExportMetalObjectCreateInfoEXT<'a> {}
    unsafe impl<'a> Extends<BufferViewCreateInfo<'a>> for ExportMetalObjectCreateInfoEXT<'a> {}
    unsafe impl<'a> Extends<SemaphoreCreateInfo<'a>> for ExportMetalObjectCreateInfoEXT<'a> {}
    unsafe impl<'a> Extends<EventCreateInfo<'a>> for ExportMetalObjectCreateInfoEXT<'a> {}

    impl Default for ExportMetalObjectCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                export_object_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMetalObjectCreateInfoEXT<'a> {
        pub fn export_object_type(
            mut self,
            export_object_type: ExportMetalObjectTypeFlagBitsEXT,
        ) -> Self {
            self.export_object_type = export_object_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalObjectsInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExportMetalObjectsInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExportMetalObjectsInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExportMetalObjectsInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExportMetalObjectsInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_METAL_OBJECTS_INFO_EXT;
    }

    impl Default for ExportMetalObjectsInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMetalObjectsInfoEXT<'a> {}

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalDeviceInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExportMetalDeviceInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub mtl_device: MTLDevice_id,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExportMetalDeviceInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExportMetalDeviceInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("mtl_device", &self.mtl_device)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExportMetalDeviceInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_METAL_DEVICE_INFO_EXT;
    }

    unsafe impl<'a> Extends<ExportMetalObjectsInfoEXT<'a>> for ExportMetalDeviceInfoEXT<'a> {}

    impl Default for ExportMetalDeviceInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                mtl_device: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMetalDeviceInfoEXT<'a> {
        pub fn mtl_device(mut self, mtl_device: MTLDevice_id) -> Self {
            self.mtl_device = mtl_device;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalCommandQueueInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExportMetalCommandQueueInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub queue: Queue,
        pub mtl_command_queue: MTLCommandQueue_id,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExportMetalCommandQueueInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExportMetalCommandQueueInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("queue", &self.queue)
                .field("mtl_command_queue", &self.mtl_command_queue)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExportMetalCommandQueueInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_METAL_COMMAND_QUEUE_INFO_EXT;
    }

    unsafe impl<'a> Extends<ExportMetalObjectsInfoEXT<'a>> for ExportMetalCommandQueueInfoEXT<'a> {}

    impl Default for ExportMetalCommandQueueInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                queue: Default::default(),
                mtl_command_queue: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMetalCommandQueueInfoEXT<'a> {
        pub fn queue(mut self, queue: Queue) -> Self {
            self.queue = queue;
            self
        }

        pub fn mtl_command_queue(mut self, mtl_command_queue: MTLCommandQueue_id) -> Self {
            self.mtl_command_queue = mtl_command_queue;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalBufferInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExportMetalBufferInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub mtl_buffer: MTLBuffer_id,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExportMetalBufferInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExportMetalBufferInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory", &self.memory)
                .field("mtl_buffer", &self.mtl_buffer)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExportMetalBufferInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_METAL_BUFFER_INFO_EXT;
    }

    unsafe impl<'a> Extends<ExportMetalObjectsInfoEXT<'a>> for ExportMetalBufferInfoEXT<'a> {}

    impl Default for ExportMetalBufferInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                memory: Default::default(),
                mtl_buffer: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMetalBufferInfoEXT<'a> {
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }

        pub fn mtl_buffer(mut self, mtl_buffer: MTLBuffer_id) -> Self {
            self.mtl_buffer = mtl_buffer;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportMetalBufferInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImportMetalBufferInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub mtl_buffer: MTLBuffer_id,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImportMetalBufferInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImportMetalBufferInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("mtl_buffer", &self.mtl_buffer)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImportMetalBufferInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_METAL_BUFFER_INFO_EXT;
    }

    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for ImportMetalBufferInfoEXT<'a> {}

    impl Default for ImportMetalBufferInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                mtl_buffer: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportMetalBufferInfoEXT<'a> {
        pub fn mtl_buffer(mut self, mtl_buffer: MTLBuffer_id) -> Self {
            self.mtl_buffer = mtl_buffer;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalTextureInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExportMetalTextureInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image: Image,
        pub image_view: ImageView,
        pub buffer_view: BufferView,
        pub plane: ImageAspectFlagBits,
        pub mtl_texture: MTLTexture_id,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExportMetalTextureInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExportMetalTextureInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image", &self.image)
                .field("image_view", &self.image_view)
                .field("buffer_view", &self.buffer_view)
                .field("plane", &self.plane)
                .field("mtl_texture", &self.mtl_texture)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExportMetalTextureInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_METAL_TEXTURE_INFO_EXT;
    }

    unsafe impl<'a> Extends<ExportMetalObjectsInfoEXT<'a>> for ExportMetalTextureInfoEXT<'a> {}

    impl Default for ExportMetalTextureInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                image: Default::default(),
                image_view: Default::default(),
                buffer_view: Default::default(),
                plane: Default::default(),
                mtl_texture: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMetalTextureInfoEXT<'a> {
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }

        pub fn image_view(mut self, image_view: ImageView) -> Self {
            self.image_view = image_view;
            self
        }

        pub fn buffer_view(mut self, buffer_view: BufferView) -> Self {
            self.buffer_view = buffer_view;
            self
        }

        pub fn plane(mut self, plane: ImageAspectFlagBits) -> Self {
            self.plane = plane;
            self
        }

        pub fn mtl_texture(mut self, mtl_texture: MTLTexture_id) -> Self {
            self.mtl_texture = mtl_texture;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportMetalTextureInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImportMetalTextureInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub plane: ImageAspectFlagBits,
        pub mtl_texture: MTLTexture_id,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImportMetalTextureInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImportMetalTextureInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("plane", &self.plane)
                .field("mtl_texture", &self.mtl_texture)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImportMetalTextureInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_METAL_TEXTURE_INFO_EXT;
    }

    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for ImportMetalTextureInfoEXT<'a> {}

    impl Default for ImportMetalTextureInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                plane: Default::default(),
                mtl_texture: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportMetalTextureInfoEXT<'a> {
        pub fn plane(mut self, plane: ImageAspectFlagBits) -> Self {
            self.plane = plane;
            self
        }

        pub fn mtl_texture(mut self, mtl_texture: MTLTexture_id) -> Self {
            self.mtl_texture = mtl_texture;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalIOSurfaceInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExportMetalIOSurfaceInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image: Image,
        pub io_surface: IOSurfaceRef,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExportMetalIOSurfaceInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExportMetalIOSurfaceInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image", &self.image)
                .field("io_surface", &self.io_surface)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExportMetalIOSurfaceInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_METAL_IO_SURFACE_INFO_EXT;
    }

    unsafe impl<'a> Extends<ExportMetalObjectsInfoEXT<'a>> for ExportMetalIOSurfaceInfoEXT<'a> {}

    impl Default for ExportMetalIOSurfaceInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                image: Default::default(),
                io_surface: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMetalIOSurfaceInfoEXT<'a> {
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }

        pub fn io_surface(mut self, io_surface: IOSurfaceRef) -> Self {
            self.io_surface = io_surface;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportMetalIOSurfaceInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImportMetalIOSurfaceInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub io_surface: IOSurfaceRef,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImportMetalIOSurfaceInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImportMetalIOSurfaceInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("io_surface", &self.io_surface)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImportMetalIOSurfaceInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_METAL_IO_SURFACE_INFO_EXT;
    }

    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for ImportMetalIOSurfaceInfoEXT<'a> {}

    impl Default for ImportMetalIOSurfaceInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                io_surface: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportMetalIOSurfaceInfoEXT<'a> {
        pub fn io_surface(mut self, io_surface: IOSurfaceRef) -> Self {
            self.io_surface = io_surface;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalSharedEventInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExportMetalSharedEventInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub semaphore: Semaphore,
        pub event: Event,
        pub mtl_shared_event: MTLSharedEvent_id,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExportMetalSharedEventInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExportMetalSharedEventInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("semaphore", &self.semaphore)
                .field("event", &self.event)
                .field("mtl_shared_event", &self.mtl_shared_event)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExportMetalSharedEventInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_METAL_SHARED_EVENT_INFO_EXT;
    }

    unsafe impl<'a> Extends<ExportMetalObjectsInfoEXT<'a>> for ExportMetalSharedEventInfoEXT<'a> {}

    impl Default for ExportMetalSharedEventInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                semaphore: Default::default(),
                event: Default::default(),
                mtl_shared_event: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMetalSharedEventInfoEXT<'a> {
        pub fn semaphore(mut self, semaphore: Semaphore) -> Self {
            self.semaphore = semaphore;
            self
        }

        pub fn event(mut self, event: Event) -> Self {
            self.event = event;
            self
        }

        pub fn mtl_shared_event(mut self, mtl_shared_event: MTLSharedEvent_id) -> Self {
            self.mtl_shared_event = mtl_shared_event;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportMetalSharedEventInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImportMetalSharedEventInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub mtl_shared_event: MTLSharedEvent_id,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImportMetalSharedEventInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImportMetalSharedEventInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("mtl_shared_event", &self.mtl_shared_event)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImportMetalSharedEventInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_METAL_SHARED_EVENT_INFO_EXT;
    }

    unsafe impl<'a> Extends<SemaphoreCreateInfo<'a>> for ImportMetalSharedEventInfoEXT<'a> {}
    unsafe impl<'a> Extends<EventCreateInfo<'a>> for ImportMetalSharedEventInfoEXT<'a> {}

    impl Default for ImportMetalSharedEventInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                mtl_shared_event: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportMetalSharedEventInfoEXT<'a> {
        pub fn mtl_shared_event(mut self, mtl_shared_event: MTLSharedEvent_id) -> Self {
            self.mtl_shared_event = mtl_shared_event;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalObjectTypeFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ExportMetalObjectTypeFlagsEXT(Flags);
    vk_bitflags_wrapped!(ExportMetalObjectTypeFlagsEXT, Flags);

    impl ExportMetalObjectTypeFlagsEXT {
        pub const METAL_DEVICE_EXT: Self =
            Self(ExportMetalObjectTypeFlagBitsEXT::METAL_DEVICE_EXT.0);
        pub const METAL_COMMAND_QUEUE_EXT: Self =
            Self(ExportMetalObjectTypeFlagBitsEXT::METAL_COMMAND_QUEUE_EXT.0);
        pub const METAL_BUFFER_EXT: Self =
            Self(ExportMetalObjectTypeFlagBitsEXT::METAL_BUFFER_EXT.0);
        pub const METAL_TEXTURE_EXT: Self =
            Self(ExportMetalObjectTypeFlagBitsEXT::METAL_TEXTURE_EXT.0);
        pub const METAL_IOSURFACE_EXT: Self =
            Self(ExportMetalObjectTypeFlagBitsEXT::METAL_IOSURFACE_EXT.0);
        pub const METAL_SHARED_EVENT_EXT: Self =
            Self(ExportMetalObjectTypeFlagBitsEXT::METAL_SHARED_EVENT_EXT.0);
    }

    impl fmt::Debug for ExportMetalObjectTypeFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    ExportMetalObjectTypeFlagsEXT::METAL_DEVICE_EXT.0,
                    "METAL_DEVICE_EXT",
                ),
                (
                    ExportMetalObjectTypeFlagsEXT::METAL_COMMAND_QUEUE_EXT.0,
                    "METAL_COMMAND_QUEUE_EXT",
                ),
                (
                    ExportMetalObjectTypeFlagsEXT::METAL_BUFFER_EXT.0,
                    "METAL_BUFFER_EXT",
                ),
                (
                    ExportMetalObjectTypeFlagsEXT::METAL_TEXTURE_EXT.0,
                    "METAL_TEXTURE_EXT",
                ),
                (
                    ExportMetalObjectTypeFlagsEXT::METAL_IOSURFACE_EXT.0,
                    "METAL_IOSURFACE_EXT",
                ),
                (
                    ExportMetalObjectTypeFlagsEXT::METAL_SHARED_EVENT_EXT.0,
                    "METAL_SHARED_EVENT_EXT",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalObjectTypeFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ExportMetalObjectTypeFlagBitsEXT(u32);

    impl ExportMetalObjectTypeFlagBitsEXT {
        pub const METAL_DEVICE_EXT: Self = Self(1 << 0);
        pub const METAL_COMMAND_QUEUE_EXT: Self = Self(1 << 1);
        pub const METAL_BUFFER_EXT: Self = Self(1 << 2);
        pub const METAL_TEXTURE_EXT: Self = Self(1 << 3);
        pub const METAL_IOSURFACE_EXT: Self = Self(1 << 4);
        pub const METAL_SHARED_EVENT_EXT: Self = Self(1 << 5);
    }

    impl fmt::Debug for ExportMetalObjectTypeFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::METAL_DEVICE_EXT => Some("METAL_DEVICE_EXT"),
                Self::METAL_COMMAND_QUEUE_EXT => Some("METAL_COMMAND_QUEUE_EXT"),
                Self::METAL_BUFFER_EXT => Some("METAL_BUFFER_EXT"),
                Self::METAL_TEXTURE_EXT => Some("METAL_TEXTURE_EXT"),
                Self::METAL_IOSURFACE_EXT => Some("METAL_IOSURFACE_EXT"),
                Self::METAL_SHARED_EVENT_EXT => Some("METAL_SHARED_EVENT_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkExportMetalObjectsEXT.html>
    pub type PFN_vkExportMetalObjectsEXT = unsafe extern "system" fn(
        device: Device,
        p_metal_objects_info: *mut ExportMetalObjectsInfoEXT<'_>,
    );
}

pub struct DeviceFn {
    export_metal_objects_ext: PFN_vkExportMetalObjectsEXT,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                export_metal_objects_ext: transmute(
                    load(c"vkExportMetalObjectsEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkExportMetalObjectsEXT.html>
    pub unsafe fn export_metal_objects_ext(
        &self,
        device: Device,
        metal_objects_info: &mut ExportMetalObjectsInfoEXT<'_>,
    ) {
        unsafe { (self.export_metal_objects_ext)(device, metal_objects_info) }
    }
}

//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_metal_objects.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_metal_objects";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    pub type MTLDevice_id = *const c_void;
    pub type MTLCommandQueue_id = *const c_void;
    pub type MTLBuffer_id = *const c_void;
    pub type MTLTexture_id = *const c_void;
    pub type MTLSharedEvent_id = *const c_void;
    pub type IOSurfaceRef = *const c_void;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalObjectCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    unsafe impl Extends<InstanceCreateInfo<'_>> for ExportMetalObjectCreateInfoEXT<'_> {}
    unsafe impl Extends<MemoryAllocateInfo<'_>> for ExportMetalObjectCreateInfoEXT<'_> {}
    unsafe impl Extends<ImageCreateInfo<'_>> for ExportMetalObjectCreateInfoEXT<'_> {}
    unsafe impl Extends<ImageViewCreateInfo<'_>> for ExportMetalObjectCreateInfoEXT<'_> {}
    unsafe impl Extends<BufferViewCreateInfo<'_>> for ExportMetalObjectCreateInfoEXT<'_> {}
    unsafe impl Extends<SemaphoreCreateInfo<'_>> for ExportMetalObjectCreateInfoEXT<'_> {}
    unsafe impl Extends<EventCreateInfo<'_>> for ExportMetalObjectCreateInfoEXT<'_> {}

    impl Default for ExportMetalObjectCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                export_object_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMetalObjectCreateInfoEXT<'a> {
        #[inline]
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
    #[must_use]
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
                p_next: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMetalObjectsInfoEXT<'a> {}

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalDeviceInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    unsafe impl Extends<ExportMetalObjectsInfoEXT<'_>> for ExportMetalDeviceInfoEXT<'_> {}

    impl Default for ExportMetalDeviceInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                mtl_device: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMetalDeviceInfoEXT<'a> {
        #[inline]
        pub fn mtl_device(mut self, mtl_device: MTLDevice_id) -> Self {
            self.mtl_device = mtl_device;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalCommandQueueInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    unsafe impl Extends<ExportMetalObjectsInfoEXT<'_>> for ExportMetalCommandQueueInfoEXT<'_> {}

    impl Default for ExportMetalCommandQueueInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                queue: Default::default(),
                mtl_command_queue: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMetalCommandQueueInfoEXT<'a> {
        #[inline]
        pub fn queue(mut self, queue: Queue) -> Self {
            self.queue = queue;
            self
        }

        #[inline]
        pub fn mtl_command_queue(mut self, mtl_command_queue: MTLCommandQueue_id) -> Self {
            self.mtl_command_queue = mtl_command_queue;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalBufferInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    unsafe impl Extends<ExportMetalObjectsInfoEXT<'_>> for ExportMetalBufferInfoEXT<'_> {}

    impl Default for ExportMetalBufferInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                memory: Default::default(),
                mtl_buffer: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMetalBufferInfoEXT<'a> {
        #[inline]
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }

        #[inline]
        pub fn mtl_buffer(mut self, mtl_buffer: MTLBuffer_id) -> Self {
            self.mtl_buffer = mtl_buffer;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportMetalBufferInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    unsafe impl Extends<MemoryAllocateInfo<'_>> for ImportMetalBufferInfoEXT<'_> {}

    impl Default for ImportMetalBufferInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                mtl_buffer: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportMetalBufferInfoEXT<'a> {
        #[inline]
        pub fn mtl_buffer(mut self, mtl_buffer: MTLBuffer_id) -> Self {
            self.mtl_buffer = mtl_buffer;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalTextureInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    unsafe impl Extends<ExportMetalObjectsInfoEXT<'_>> for ExportMetalTextureInfoEXT<'_> {}

    impl Default for ExportMetalTextureInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
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
        #[inline]
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }

        #[inline]
        pub fn image_view(mut self, image_view: ImageView) -> Self {
            self.image_view = image_view;
            self
        }

        #[inline]
        pub fn buffer_view(mut self, buffer_view: BufferView) -> Self {
            self.buffer_view = buffer_view;
            self
        }

        #[inline]
        pub fn plane(mut self, plane: ImageAspectFlagBits) -> Self {
            self.plane = plane;
            self
        }

        #[inline]
        pub fn mtl_texture(mut self, mtl_texture: MTLTexture_id) -> Self {
            self.mtl_texture = mtl_texture;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportMetalTextureInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    unsafe impl Extends<ImageCreateInfo<'_>> for ImportMetalTextureInfoEXT<'_> {}

    impl Default for ImportMetalTextureInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                plane: Default::default(),
                mtl_texture: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportMetalTextureInfoEXT<'a> {
        #[inline]
        pub fn plane(mut self, plane: ImageAspectFlagBits) -> Self {
            self.plane = plane;
            self
        }

        #[inline]
        pub fn mtl_texture(mut self, mtl_texture: MTLTexture_id) -> Self {
            self.mtl_texture = mtl_texture;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalIOSurfaceInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    unsafe impl Extends<ExportMetalObjectsInfoEXT<'_>> for ExportMetalIOSurfaceInfoEXT<'_> {}

    impl Default for ExportMetalIOSurfaceInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                image: Default::default(),
                io_surface: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMetalIOSurfaceInfoEXT<'a> {
        #[inline]
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }

        #[inline]
        pub fn io_surface(mut self, io_surface: IOSurfaceRef) -> Self {
            self.io_surface = io_surface;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportMetalIOSurfaceInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    unsafe impl Extends<ImageCreateInfo<'_>> for ImportMetalIOSurfaceInfoEXT<'_> {}

    impl Default for ImportMetalIOSurfaceInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                io_surface: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportMetalIOSurfaceInfoEXT<'a> {
        #[inline]
        pub fn io_surface(mut self, io_surface: IOSurfaceRef) -> Self {
            self.io_surface = io_surface;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalSharedEventInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    unsafe impl Extends<ExportMetalObjectsInfoEXT<'_>> for ExportMetalSharedEventInfoEXT<'_> {}

    impl Default for ExportMetalSharedEventInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                semaphore: Default::default(),
                event: Default::default(),
                mtl_shared_event: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMetalSharedEventInfoEXT<'a> {
        #[inline]
        pub fn semaphore(mut self, semaphore: Semaphore) -> Self {
            self.semaphore = semaphore;
            self
        }

        #[inline]
        pub fn event(mut self, event: Event) -> Self {
            self.event = event;
            self
        }

        #[inline]
        pub fn mtl_shared_event(mut self, mtl_shared_event: MTLSharedEvent_id) -> Self {
            self.mtl_shared_event = mtl_shared_event;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportMetalSharedEventInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    unsafe impl Extends<SemaphoreCreateInfo<'_>> for ImportMetalSharedEventInfoEXT<'_> {}
    unsafe impl Extends<EventCreateInfo<'_>> for ImportMetalSharedEventInfoEXT<'_> {}

    impl Default for ImportMetalSharedEventInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                mtl_shared_event: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportMetalSharedEventInfoEXT<'a> {
        #[inline]
        pub fn mtl_shared_event(mut self, mtl_shared_event: MTLSharedEvent_id) -> Self {
            self.mtl_shared_event = mtl_shared_event;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMetalObjectTypeFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ExportMetalObjectTypeFlagsEXT(Flags);
    vk_bitflags_wrapped!(
        ExportMetalObjectTypeFlagsEXT,
        Flags,
        ExportMetalObjectTypeFlagBitsEXT
    );

    impl fmt::Debug for ExportMetalObjectTypeFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    ExportMetalObjectTypeFlagBitsEXT::METAL_DEVICE_EXT.0,
                    "METAL_DEVICE_EXT",
                ),
                (
                    ExportMetalObjectTypeFlagBitsEXT::METAL_COMMAND_QUEUE_EXT.0,
                    "METAL_COMMAND_QUEUE_EXT",
                ),
                (
                    ExportMetalObjectTypeFlagBitsEXT::METAL_BUFFER_EXT.0,
                    "METAL_BUFFER_EXT",
                ),
                (
                    ExportMetalObjectTypeFlagBitsEXT::METAL_TEXTURE_EXT.0,
                    "METAL_TEXTURE_EXT",
                ),
                (
                    ExportMetalObjectTypeFlagBitsEXT::METAL_IOSURFACE_EXT.0,
                    "METAL_IOSURFACE_EXT",
                ),
                (
                    ExportMetalObjectTypeFlagBitsEXT::METAL_SHARED_EVENT_EXT.0,
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

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkExportMetalObjectCreateInfoEXT = ExportMetalObjectCreateInfoEXT<'static>;
    pub type VkExportMetalObjectsInfoEXT = ExportMetalObjectsInfoEXT<'static>;
    pub type VkExportMetalDeviceInfoEXT = ExportMetalDeviceInfoEXT<'static>;
    pub type VkExportMetalCommandQueueInfoEXT = ExportMetalCommandQueueInfoEXT<'static>;
    pub type VkExportMetalBufferInfoEXT = ExportMetalBufferInfoEXT<'static>;
    pub type VkImportMetalBufferInfoEXT = ImportMetalBufferInfoEXT<'static>;
    pub type VkExportMetalTextureInfoEXT = ExportMetalTextureInfoEXT<'static>;
    pub type VkImportMetalTextureInfoEXT = ImportMetalTextureInfoEXT<'static>;
    pub type VkExportMetalIOSurfaceInfoEXT = ExportMetalIOSurfaceInfoEXT<'static>;
    pub type VkImportMetalIOSurfaceInfoEXT = ImportMetalIOSurfaceInfoEXT<'static>;
    pub type VkExportMetalSharedEventInfoEXT = ExportMetalSharedEventInfoEXT<'static>;
    pub type VkImportMetalSharedEventInfoEXT = ImportMetalSharedEventInfoEXT<'static>;
    pub type VkExportMetalObjectTypeFlagsEXT = ExportMetalObjectTypeFlagsEXT;
    pub type VkExportMetalObjectTypeFlagBitsEXT = ExportMetalObjectTypeFlagBitsEXT;
    impl ExportMetalObjectCreateInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExportMetalObjectCreateInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExportMetalObjectsInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExportMetalObjectsInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExportMetalDeviceInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExportMetalDeviceInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExportMetalCommandQueueInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExportMetalCommandQueueInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExportMetalBufferInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExportMetalBufferInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImportMetalBufferInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImportMetalBufferInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExportMetalTextureInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExportMetalTextureInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImportMetalTextureInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImportMetalTextureInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExportMetalIOSurfaceInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExportMetalIOSurfaceInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImportMetalIOSurfaceInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImportMetalIOSurfaceInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ExportMetalSharedEventInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkExportMetalSharedEventInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImportMetalSharedEventInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImportMetalSharedEventInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    export_metal_objects_ext: PFN_vkExportMetalObjectsEXT,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
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
    #[inline]
    pub unsafe fn export_metal_objects_ext(
        &self,
        device: Device,
        metal_objects_info: &mut ExportMetalObjectsInfoEXT<'_>,
    ) {
        unsafe { (self.export_metal_objects_ext)(device, metal_objects_info) }
    }
}

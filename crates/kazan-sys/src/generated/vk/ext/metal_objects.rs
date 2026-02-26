#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type MTLDevice_id = *const c_void;
pub type MTLCommandQueue_id = *const c_void;
pub type MTLBuffer_id = *const c_void;
pub type MTLTexture_id = *const c_void;
pub type MTLSharedEvent_id = *const c_void;
pub type IOSurfaceRef = *const c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMetalObjectCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub export_object_type: ExportMetalObjectTypeFlagBitsEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExportMetalObjectCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXPORT_METAL_OBJECT_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            export_object_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMetalObjectsInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExportMetalObjectsInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXPORT_METAL_OBJECTS_INFO_EXT,
            p_next: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMetalDeviceInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mtl_device: MTLDevice_id,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExportMetalDeviceInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXPORT_METAL_DEVICE_INFO_EXT,
            p_next: core::ptr::null(),
            mtl_device: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMetalCommandQueueInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue: Queue,
    pub mtl_command_queue: MTLCommandQueue_id,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExportMetalCommandQueueInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXPORT_METAL_COMMAND_QUEUE_INFO_EXT,
            p_next: core::ptr::null(),
            queue: Default::default(),
            mtl_command_queue: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMetalBufferInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub mtl_buffer: MTLBuffer_id,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExportMetalBufferInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXPORT_METAL_BUFFER_INFO_EXT,
            p_next: core::ptr::null(),
            memory: Default::default(),
            mtl_buffer: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMetalBufferInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mtl_buffer: MTLBuffer_id,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportMetalBufferInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_METAL_BUFFER_INFO_EXT,
            p_next: core::ptr::null(),
            mtl_buffer: Default::default(),
            _marker: PhantomData,
        }
    }
}
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
impl Default for ExportMetalTextureInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXPORT_METAL_TEXTURE_INFO_EXT,
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMetalTextureInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane: ImageAspectFlagBits,
    pub mtl_texture: MTLTexture_id,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportMetalTextureInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_METAL_TEXTURE_INFO_EXT,
            p_next: core::ptr::null(),
            plane: Default::default(),
            mtl_texture: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMetalIOSurfaceInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub io_surface: IOSurfaceRef,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExportMetalIOSurfaceInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXPORT_METAL_IO_SURFACE_INFO_EXT,
            p_next: core::ptr::null(),
            image: Default::default(),
            io_surface: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMetalIOSurfaceInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub io_surface: IOSurfaceRef,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportMetalIOSurfaceInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_METAL_IO_SURFACE_INFO_EXT,
            p_next: core::ptr::null(),
            io_surface: Default::default(),
            _marker: PhantomData,
        }
    }
}
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
impl Default for ExportMetalSharedEventInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXPORT_METAL_SHARED_EVENT_INFO_EXT,
            p_next: core::ptr::null(),
            semaphore: Default::default(),
            event: Default::default(),
            mtl_shared_event: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMetalSharedEventInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mtl_shared_event: MTLSharedEvent_id,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImportMetalSharedEventInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMPORT_METAL_SHARED_EVENT_INFO_EXT,
            p_next: core::ptr::null(),
            mtl_shared_event: Default::default(),
            _marker: PhantomData,
        }
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ExportMetalObjectTypeFlagsEXT: Flags {
        const METAL_DEVICE_EXT = ExportMetalObjectTypeFlagBitsEXT::METAL_DEVICE_EXT.0;
        const METAL_COMMAND_QUEUE_EXT = ExportMetalObjectTypeFlagBitsEXT::METAL_COMMAND_QUEUE_EXT.0;
        const METAL_BUFFER_EXT = ExportMetalObjectTypeFlagBitsEXT::METAL_BUFFER_EXT.0;
        const METAL_TEXTURE_EXT = ExportMetalObjectTypeFlagBitsEXT::METAL_TEXTURE_EXT.0;
        const METAL_IOSURFACE_EXT = ExportMetalObjectTypeFlagBitsEXT::METAL_IOSURFACE_EXT.0;
        const METAL_SHARED_EVENT_EXT = ExportMetalObjectTypeFlagBitsEXT::METAL_SHARED_EVENT_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExportMetalObjectTypeFlagBitsEXT(u32);
impl ExportMetalObjectTypeFlagBitsEXT {
    pub const METAL_DEVICE_EXT: Self = Self(1 << 0);
    pub const METAL_COMMAND_QUEUE_EXT: Self = Self(1 << 1);
    pub const METAL_BUFFER_EXT: Self = Self(1 << 2);
    pub const METAL_TEXTURE_EXT: Self = Self(1 << 3);
    pub const METAL_IOSURFACE_EXT: Self = Self(1 << 4);
    pub const METAL_SHARED_EVENT_EXT: Self = Self(1 << 5);
}
pub type PFN_vkExportMetalObjectsEXT = unsafe extern "system" fn(
    device: Device,
    p_metal_objects_info: *mut ExportMetalObjectsInfoEXT<'_>,
);

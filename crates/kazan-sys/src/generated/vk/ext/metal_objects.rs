#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type MTLDevice_id = *const c_void;
pub type MTLCommandQueue_id = *const c_void;
pub type MTLBuffer_id = *const c_void;
pub type MTLTexture_id = *const c_void;
pub type MTLSharedEvent_id = *const c_void;
pub type IOSurfaceRef = *const c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMetalObjectCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub export_object_type: ExportMetalObjectTypeFlagsEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMetalObjectsInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMetalDeviceInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mtl_device: MTLDevice_id,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMetalCommandQueueInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue: Queue,
    pub mtl_command_queue: MTLCommandQueue_id,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMetalBufferInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub mtl_buffer: MTLBuffer_id,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMetalBufferInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mtl_buffer: MTLBuffer_id,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMetalTextureInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub image_view: ImageView,
    pub buffer_view: BufferView,
    pub plane: ImageAspectFlags,
    pub mtl_texture: MTLTexture_id,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMetalTextureInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane: ImageAspectFlags,
    pub mtl_texture: MTLTexture_id,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMetalIOSurfaceInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub io_surface: IOSurfaceRef,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMetalIOSurfaceInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub io_surface: IOSurfaceRef,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMetalSharedEventInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub event: Event,
    pub mtl_shared_event: MTLSharedEvent_id,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMetalSharedEventInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mtl_shared_event: MTLSharedEvent_id,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ExportMetalObjectTypeFlagsEXT: Flags {
        const METAL_DEVICE_EXT = 1 << 0;
        const METAL_COMMAND_QUEUE_EXT = 1 << 1;
        const METAL_BUFFER_EXT = 1 << 2;
        const METAL_TEXTURE_EXT = 1 << 3;
        const METAL_IOSURFACE_EXT = 1 << 4;
        const METAL_SHARED_EVENT_EXT = 1 << 5;
    }
}
pub type PFN_vkExportMetalObjectsEXT =
    unsafe extern "system" fn(device: Device, p_metal_objects_info: *mut ExportMetalObjectsInfoEXT);

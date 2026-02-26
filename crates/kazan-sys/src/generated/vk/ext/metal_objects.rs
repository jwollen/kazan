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
pub struct ExportMetalObjectCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub export_object_type: ExportMetalObjectTypeFlagBitsEXT,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ExportMetalObjectsInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ExportMetalDeviceInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mtl_device: MTLDevice_id,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ExportMetalCommandQueueInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue: Queue,
    pub mtl_command_queue: MTLCommandQueue_id,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ExportMetalBufferInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub mtl_buffer: MTLBuffer_id,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ImportMetalBufferInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mtl_buffer: MTLBuffer_id,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
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
#[repr(C)]
pub struct ImportMetalTextureInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane: ImageAspectFlagBits,
    pub mtl_texture: MTLTexture_id,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ExportMetalIOSurfaceInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub io_surface: IOSurfaceRef,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ImportMetalIOSurfaceInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub io_surface: IOSurfaceRef,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ExportMetalSharedEventInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub event: Event,
    pub mtl_shared_event: MTLSharedEvent_id,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ImportMetalSharedEventInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mtl_shared_event: MTLSharedEvent_id,
    pub _marker: PhantomData<&'a ()>,
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

#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    create_buffer_collection_fuchsia: PFN_vkCreateBufferCollectionFUCHSIA,
    set_buffer_collection_image_constraints_fuchsia:
        PFN_vkSetBufferCollectionImageConstraintsFUCHSIA,
    set_buffer_collection_buffer_constraints_fuchsia:
        PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA,
    destroy_buffer_collection_fuchsia: PFN_vkDestroyBufferCollectionFUCHSIA,
    get_buffer_collection_properties_fuchsia: PFN_vkGetBufferCollectionPropertiesFUCHSIA,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_buffer_collection_fuchsia: transmute(
                    load(c"vkCreateBufferCollectionFUCHSIA").ok_or(LoadingError)?,
                ),
                set_buffer_collection_image_constraints_fuchsia: transmute(
                    load(c"vkSetBufferCollectionImageConstraintsFUCHSIA").ok_or(LoadingError)?,
                ),
                set_buffer_collection_buffer_constraints_fuchsia: transmute(
                    load(c"vkSetBufferCollectionBufferConstraintsFUCHSIA").ok_or(LoadingError)?,
                ),
                destroy_buffer_collection_fuchsia: transmute(
                    load(c"vkDestroyBufferCollectionFUCHSIA").ok_or(LoadingError)?,
                ),
                get_buffer_collection_properties_fuchsia: transmute(
                    load(c"vkGetBufferCollectionPropertiesFUCHSIA").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_buffer_collection_fuchsia(
        &self,
        device: Device,
        create_info: &BufferCollectionCreateInfoFUCHSIA,
        allocator: Option<&AllocationCallbacks>,
        collection: &mut BufferCollectionFUCHSIA,
    ) -> crate::Result<()> {
        unsafe {
            result((self.create_buffer_collection_fuchsia)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                collection,
            ))
        }
    }
    pub unsafe fn set_buffer_collection_image_constraints_fuchsia(
        &self,
        device: Device,
        collection: BufferCollectionFUCHSIA,
        image_constraints_info: &ImageConstraintsInfoFUCHSIA,
    ) -> crate::Result<()> {
        unsafe {
            result((self.set_buffer_collection_image_constraints_fuchsia)(
                device,
                collection,
                image_constraints_info,
            ))
        }
    }
    pub unsafe fn set_buffer_collection_buffer_constraints_fuchsia(
        &self,
        device: Device,
        collection: BufferCollectionFUCHSIA,
        buffer_constraints_info: &BufferConstraintsInfoFUCHSIA,
    ) -> crate::Result<()> {
        unsafe {
            result((self.set_buffer_collection_buffer_constraints_fuchsia)(
                device,
                collection,
                buffer_constraints_info,
            ))
        }
    }
    pub unsafe fn destroy_buffer_collection_fuchsia(
        &self,
        device: Device,
        collection: BufferCollectionFUCHSIA,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_buffer_collection_fuchsia)(device, collection, allocator.to_raw_ptr())
        }
    }
    pub unsafe fn get_buffer_collection_properties_fuchsia(
        &self,
        device: Device,
        collection: BufferCollectionFUCHSIA,
        properties: &mut BufferCollectionPropertiesFUCHSIA,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_buffer_collection_properties_fuchsia)(
                device, collection, properties,
            ))
        }
    }
}

#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    create_buffer_collection_fuchsia: PFN_vkCreateBufferCollectionFUCHSIA,
    set_buffer_collection_buffer_constraints_fuchsia:
        PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA,
    set_buffer_collection_image_constraints_fuchsia:
        PFN_vkSetBufferCollectionImageConstraintsFUCHSIA,
    destroy_buffer_collection_fuchsia: PFN_vkDestroyBufferCollectionFUCHSIA,
    get_buffer_collection_properties_fuchsia: PFN_vkGetBufferCollectionPropertiesFUCHSIA,
}
impl DeviceFn {
    pub unsafe fn create_buffer_collection_fuchsia(
        &self,
        device: Device,
        create_info: &BufferCollectionCreateInfoFUCHSIA,
        allocator: Option<&AllocationCallbacks>,
        collection: &mut BufferCollectionFUCHSIA,
    ) -> Result {
        unsafe {
            (self.create_buffer_collection_fuchsia)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                collection,
            )
        }
    }
    pub unsafe fn set_buffer_collection_buffer_constraints_fuchsia(
        &self,
        device: Device,
        collection: BufferCollectionFUCHSIA,
        buffer_constraints_info: &BufferConstraintsInfoFUCHSIA,
    ) -> Result {
        unsafe {
            (self.set_buffer_collection_buffer_constraints_fuchsia)(
                device,
                collection,
                buffer_constraints_info,
            )
        }
    }
    pub unsafe fn set_buffer_collection_image_constraints_fuchsia(
        &self,
        device: Device,
        collection: BufferCollectionFUCHSIA,
        image_constraints_info: &ImageConstraintsInfoFUCHSIA,
    ) -> Result {
        unsafe {
            (self.set_buffer_collection_image_constraints_fuchsia)(
                device,
                collection,
                image_constraints_info,
            )
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
    ) -> Result {
        unsafe { (self.get_buffer_collection_properties_fuchsia)(device, collection, properties) }
    }
}

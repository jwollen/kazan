#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    create_private_data_slot: PFN_vkCreatePrivateDataSlot,
    destroy_private_data_slot: PFN_vkDestroyPrivateDataSlot,
    set_private_data: PFN_vkSetPrivateData,
    get_private_data: PFN_vkGetPrivateData,
}
impl DeviceFn {
    pub unsafe fn create_private_data_slot(
        &self,
        device: Device,
        create_info: &PrivateDataSlotCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        private_data_slot: &mut PrivateDataSlot,
    ) -> Result {
        unsafe {
            (self.create_private_data_slot)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                private_data_slot,
            )
        }
    }
    pub unsafe fn destroy_private_data_slot(
        &self,
        device: Device,
        private_data_slot: PrivateDataSlot,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_private_data_slot)(device, private_data_slot, allocator.to_raw_ptr())
        }
    }
    pub unsafe fn set_private_data(
        &self,
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> Result {
        unsafe {
            (self.set_private_data)(device, object_type, object_handle, private_data_slot, data)
        }
    }
    pub unsafe fn get_private_data(
        &self,
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: &mut u64,
    ) {
        unsafe {
            (self.get_private_data)(device, object_type, object_handle, private_data_slot, data)
        }
    }
}

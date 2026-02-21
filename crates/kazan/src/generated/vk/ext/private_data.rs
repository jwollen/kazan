#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    create_private_data_slot_ext: PFN_vkCreatePrivateDataSlot,
    destroy_private_data_slot_ext: PFN_vkDestroyPrivateDataSlot,
    set_private_data_ext: PFN_vkSetPrivateData,
    get_private_data_ext: PFN_vkGetPrivateData,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_private_data_slot_ext: transmute(
                    load(c"vkCreatePrivateDataSlotEXT").ok_or(LoadingError)?,
                ),
                destroy_private_data_slot_ext: transmute(
                    load(c"vkDestroyPrivateDataSlotEXT").ok_or(LoadingError)?,
                ),
                set_private_data_ext: transmute(load(c"vkSetPrivateDataEXT").ok_or(LoadingError)?),
                get_private_data_ext: transmute(load(c"vkGetPrivateDataEXT").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_private_data_slot_ext(
        &self,
        device: Device,
        create_info: &PrivateDataSlotCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        private_data_slot: &mut PrivateDataSlot,
    ) -> Result {
        unsafe {
            (self.create_private_data_slot_ext)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                private_data_slot,
            )
        }
    }
    pub unsafe fn destroy_private_data_slot_ext(
        &self,
        device: Device,
        private_data_slot: PrivateDataSlot,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_private_data_slot_ext)(device, private_data_slot, allocator.to_raw_ptr())
        }
    }
    pub unsafe fn set_private_data_ext(
        &self,
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> Result {
        unsafe {
            (self.set_private_data_ext)(device, object_type, object_handle, private_data_slot, data)
        }
    }
    pub unsafe fn get_private_data_ext(
        &self,
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: &mut u64,
    ) {
        unsafe {
            (self.get_private_data_ext)(device, object_type, object_handle, private_data_slot, data)
        }
    }
}

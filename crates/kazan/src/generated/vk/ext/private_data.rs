#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_private_data";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPrivateDataSlotEXT.html>
    pub type PrivateDataSlotEXT = PrivateDataSlot;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDevicePrivateDataCreateInfoEXT.html>
    pub type DevicePrivateDataCreateInfoEXT<'a> = DevicePrivateDataCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPrivateDataSlotCreateInfoEXT.html>
    pub type PrivateDataSlotCreateInfoEXT<'a> = PrivateDataSlotCreateInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePrivateDataFeaturesEXT.html>
    pub type PhysicalDevicePrivateDataFeaturesEXT<'a> = PhysicalDevicePrivateDataFeatures<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPrivateDataSlotCreateFlagsEXT.html>
    pub type PrivateDataSlotCreateFlagsEXT = PrivateDataSlotCreateFlags;
    pub type PFN_vkCreatePrivateDataSlotEXT = PFN_vkCreatePrivateDataSlot;
    pub type PFN_vkDestroyPrivateDataSlotEXT = PFN_vkDestroyPrivateDataSlot;
    pub type PFN_vkSetPrivateDataEXT = PFN_vkSetPrivateData;
    pub type PFN_vkGetPrivateDataEXT = PFN_vkGetPrivateData;
}

pub struct DeviceFn {
    create_private_data_slot_ext: PFN_vkCreatePrivateDataSlot,
    destroy_private_data_slot_ext: PFN_vkDestroyPrivateDataSlot,
    set_private_data_ext: PFN_vkSetPrivateData,
    get_private_data_ext: PFN_vkGetPrivateData,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_private_data_slot_ext: transmute(
                    load(c"vkCreatePrivateDataSlotEXT").ok_or(MissingEntryPointError)?,
                ),
                destroy_private_data_slot_ext: transmute(
                    load(c"vkDestroyPrivateDataSlotEXT").ok_or(MissingEntryPointError)?,
                ),
                set_private_data_ext: transmute(
                    load(c"vkSetPrivateDataEXT").ok_or(MissingEntryPointError)?,
                ),
                get_private_data_ext: transmute(
                    load(c"vkGetPrivateDataEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreatePrivateDataSlotEXT.html>
    #[inline]
    pub unsafe fn create_private_data_slot_ext(
        &self,
        device: Device,
        create_info: &PrivateDataSlotCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<PrivateDataSlot> {
        unsafe {
            let mut private_data_slot = core::mem::MaybeUninit::uninit();
            let result = (self.create_private_data_slot_ext)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                private_data_slot.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(private_data_slot.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyPrivateDataSlotEXT.html>
    #[inline]
    pub unsafe fn destroy_private_data_slot_ext(
        &self,
        device: Device,
        private_data_slot: PrivateDataSlot,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_private_data_slot_ext)(device, private_data_slot, allocator.to_raw_ptr())
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetPrivateDataEXT.html>
    #[inline]
    pub unsafe fn set_private_data_ext(
        &self,
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.set_private_data_ext)(
                device,
                object_type,
                object_handle,
                private_data_slot,
                data,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPrivateDataEXT.html>
    #[inline]
    pub unsafe fn get_private_data_ext(
        &self,
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
    ) -> u64 {
        unsafe {
            let mut data = core::mem::MaybeUninit::uninit();
            (self.get_private_data_ext)(
                device,
                object_type,
                object_handle,
                private_data_slot,
                data.as_mut_ptr(),
            );
            data.assume_init()
        }
    }
}

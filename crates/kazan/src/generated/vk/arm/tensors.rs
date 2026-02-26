#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    get_physical_device_external_tensor_properties_arm:
        PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_external_tensor_properties_arm: transmute(
                    load(c"vkGetPhysicalDeviceExternalTensorPropertiesARM").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_external_tensor_properties_arm(
        &self,
        physical_device: PhysicalDevice,
        external_tensor_info: &PhysicalDeviceExternalTensorInfoARM<'_>,
    ) -> ExternalTensorPropertiesARM<'_> {
        unsafe {
            let mut external_tensor_properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_external_tensor_properties_arm)(
                physical_device,
                external_tensor_info,
                external_tensor_properties.as_mut_ptr(),
            );
            external_tensor_properties.assume_init()
        }
    }
}
pub struct DeviceFn {
    create_tensor_arm: PFN_vkCreateTensorARM,
    destroy_tensor_arm: PFN_vkDestroyTensorARM,
    create_tensor_view_arm: PFN_vkCreateTensorViewARM,
    destroy_tensor_view_arm: PFN_vkDestroyTensorViewARM,
    get_tensor_memory_requirements_arm: PFN_vkGetTensorMemoryRequirementsARM,
    bind_tensor_memory_arm: PFN_vkBindTensorMemoryARM,
    get_device_tensor_memory_requirements_arm: PFN_vkGetDeviceTensorMemoryRequirementsARM,
    cmd_copy_tensor_arm: PFN_vkCmdCopyTensorARM,
    get_tensor_opaque_capture_descriptor_data_arm:
        Option<PFN_vkGetTensorOpaqueCaptureDescriptorDataARM>,
    get_tensor_view_opaque_capture_descriptor_data_arm:
        Option<PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_tensor_arm: transmute(load(c"vkCreateTensorARM").ok_or(LoadingError)?),
                destroy_tensor_arm: transmute(load(c"vkDestroyTensorARM").ok_or(LoadingError)?),
                create_tensor_view_arm: transmute(
                    load(c"vkCreateTensorViewARM").ok_or(LoadingError)?,
                ),
                destroy_tensor_view_arm: transmute(
                    load(c"vkDestroyTensorViewARM").ok_or(LoadingError)?,
                ),
                get_tensor_memory_requirements_arm: transmute(
                    load(c"vkGetTensorMemoryRequirementsARM").ok_or(LoadingError)?,
                ),
                bind_tensor_memory_arm: transmute(
                    load(c"vkBindTensorMemoryARM").ok_or(LoadingError)?,
                ),
                get_device_tensor_memory_requirements_arm: transmute(
                    load(c"vkGetDeviceTensorMemoryRequirementsARM").ok_or(LoadingError)?,
                ),
                cmd_copy_tensor_arm: transmute(load(c"vkCmdCopyTensorARM").ok_or(LoadingError)?),
                get_tensor_opaque_capture_descriptor_data_arm: transmute(load(
                    c"vkGetTensorOpaqueCaptureDescriptorDataARM",
                )),
                get_tensor_view_opaque_capture_descriptor_data_arm: transmute(load(
                    c"vkGetTensorViewOpaqueCaptureDescriptorDataARM",
                )),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_tensor_arm(
        &self,
        device: Device,
        create_info: &TensorCreateInfoARM<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<TensorARM> {
        unsafe {
            let mut tensor = core::mem::MaybeUninit::uninit();
            let result = (self.create_tensor_arm)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                tensor.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(tensor.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_tensor_arm(
        &self,
        device: Device,
        tensor: TensorARM,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_tensor_arm)(device, tensor, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_tensor_view_arm(
        &self,
        device: Device,
        create_info: &TensorViewCreateInfoARM<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<TensorViewARM> {
        unsafe {
            let mut view = core::mem::MaybeUninit::uninit();
            let result = (self.create_tensor_view_arm)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                view.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(view.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_tensor_view_arm(
        &self,
        device: Device,
        tensor_view: TensorViewARM,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_tensor_view_arm)(device, tensor_view, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_tensor_memory_requirements_arm(
        &self,
        device: Device,
        info: &TensorMemoryRequirementsInfoARM<'_>,
    ) -> MemoryRequirements2<'_> {
        unsafe {
            let mut memory_requirements = core::mem::MaybeUninit::uninit();
            (self.get_tensor_memory_requirements_arm)(
                device,
                info,
                memory_requirements.as_mut_ptr(),
            );
            memory_requirements.assume_init()
        }
    }
    pub unsafe fn bind_tensor_memory_arm(
        &self,
        device: Device,
        bind_infos: &[BindTensorMemoryInfoARM<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.bind_tensor_memory_arm)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_device_tensor_memory_requirements_arm(
        &self,
        device: Device,
        info: &DeviceTensorMemoryRequirementsARM<'_>,
    ) -> MemoryRequirements2<'_> {
        unsafe {
            let mut memory_requirements = core::mem::MaybeUninit::uninit();
            (self.get_device_tensor_memory_requirements_arm)(
                device,
                info,
                memory_requirements.as_mut_ptr(),
            );
            memory_requirements.assume_init()
        }
    }
    pub unsafe fn cmd_copy_tensor_arm(
        &self,
        command_buffer: CommandBuffer,
        copy_tensor_info: &CopyTensorInfoARM<'_>,
    ) {
        unsafe { (self.cmd_copy_tensor_arm)(command_buffer, copy_tensor_info) }
    }
    pub unsafe fn get_tensor_opaque_capture_descriptor_data_arm(
        &self,
        device: Device,
        info: &TensorCaptureDescriptorDataInfoARM<'_>,
        data: &mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            let result =
                (self.get_tensor_opaque_capture_descriptor_data_arm.unwrap())(device, info, data);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_tensor_view_opaque_capture_descriptor_data_arm(
        &self,
        device: Device,
        info: &TensorViewCaptureDescriptorDataInfoARM<'_>,
        data: &mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self
                .get_tensor_view_opaque_capture_descriptor_data_arm
                .unwrap())(device, info, data);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}

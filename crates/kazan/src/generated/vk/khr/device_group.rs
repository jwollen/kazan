#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryAllocateFlagsInfoKHR.html>
    pub type MemoryAllocateFlagsInfoKHR<'a> = MemoryAllocateFlagsInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindBufferMemoryDeviceGroupInfoKHR.html>
    pub type BindBufferMemoryDeviceGroupInfoKHR<'a> = BindBufferMemoryDeviceGroupInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindImageMemoryDeviceGroupInfoKHR.html>
    pub type BindImageMemoryDeviceGroupInfoKHR<'a> = BindImageMemoryDeviceGroupInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceGroupRenderPassBeginInfoKHR.html>
    pub type DeviceGroupRenderPassBeginInfoKHR<'a> = DeviceGroupRenderPassBeginInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceGroupCommandBufferBeginInfoKHR.html>
    pub type DeviceGroupCommandBufferBeginInfoKHR<'a> = DeviceGroupCommandBufferBeginInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceGroupSubmitInfoKHR.html>
    pub type DeviceGroupSubmitInfoKHR<'a> = DeviceGroupSubmitInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceGroupBindSparseInfoKHR.html>
    pub type DeviceGroupBindSparseInfoKHR<'a> = DeviceGroupBindSparseInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPeerMemoryFeatureFlagsKHR.html>
    pub type PeerMemoryFeatureFlagsKHR = PeerMemoryFeatureFlags;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryAllocateFlagsKHR.html>
    pub type MemoryAllocateFlagsKHR = MemoryAllocateFlags;
    pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR = PFN_vkGetDeviceGroupPeerMemoryFeatures;
    pub type PFN_vkCmdSetDeviceMaskKHR = PFN_vkCmdSetDeviceMask;
    pub type PFN_vkCmdDispatchBaseKHR = PFN_vkCmdDispatchBase;
}
pub struct InstanceFn {
    get_physical_device_present_rectangles_khr: Option<PFN_vkGetPhysicalDevicePresentRectanglesKHR>,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_present_rectangles_khr: transmute(load(
                    c"vkGetPhysicalDevicePresentRectanglesKHR",
                )),
            })
        }
    }
}
impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html>
    pub unsafe fn get_physical_device_present_rectangles_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        mut rects: impl ExtendUninit<Rect2D>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |rect_count, rects| {
                let result = (self.get_physical_device_present_rectangles_khr.unwrap())(
                    physical_device,
                    surface,
                    rect_count,
                    rects as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let rects_buf = rects.reserve(capacity);
            len = rects_buf.len().try_into().unwrap();
            let result = call(&mut len, rects_buf.as_mut_ptr() as *mut _)?;
            rects.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}
pub struct DeviceFn {
    get_device_group_peer_memory_features_khr: PFN_vkGetDeviceGroupPeerMemoryFeatures,
    cmd_set_device_mask_khr: PFN_vkCmdSetDeviceMask,
    cmd_dispatch_base_khr: PFN_vkCmdDispatchBase,
    get_device_group_present_capabilities_khr: Option<PFN_vkGetDeviceGroupPresentCapabilitiesKHR>,
    get_device_group_surface_present_modes_khr: Option<PFN_vkGetDeviceGroupSurfacePresentModesKHR>,
    acquire_next_image2_khr: Option<PFN_vkAcquireNextImage2KHR>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_device_group_peer_memory_features_khr: transmute(
                    load(c"vkGetDeviceGroupPeerMemoryFeaturesKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_device_mask_khr: transmute(
                    load(c"vkCmdSetDeviceMaskKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_dispatch_base_khr: transmute(
                    load(c"vkCmdDispatchBaseKHR").ok_or(MissingEntryPointError)?,
                ),
                get_device_group_present_capabilities_khr: transmute(load(
                    c"vkGetDeviceGroupPresentCapabilitiesKHR",
                )),
                get_device_group_surface_present_modes_khr: transmute(load(
                    c"vkGetDeviceGroupSurfacePresentModesKHR",
                )),
                acquire_next_image2_khr: transmute(load(c"vkAcquireNextImage2KHR")),
            })
        }
    }
}
impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceGroupPeerMemoryFeaturesKHR.html>
    pub unsafe fn get_device_group_peer_memory_features_khr(
        &self,
        device: Device,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> PeerMemoryFeatureFlags {
        unsafe {
            let mut peer_memory_features = core::mem::MaybeUninit::uninit();
            (self.get_device_group_peer_memory_features_khr)(
                device,
                heap_index,
                local_device_index,
                remote_device_index,
                peer_memory_features.as_mut_ptr(),
            );
            peer_memory_features.assume_init()
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDeviceMaskKHR.html>
    pub unsafe fn cmd_set_device_mask_khr(&self, command_buffer: CommandBuffer, device_mask: u32) {
        unsafe { (self.cmd_set_device_mask_khr)(command_buffer, device_mask) }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDispatchBaseKHR.html>
    pub unsafe fn cmd_dispatch_base_khr(
        &self,
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe {
            (self.cmd_dispatch_base_khr)(
                command_buffer,
                base_group_x,
                base_group_y,
                base_group_z,
                group_count_x,
                group_count_y,
                group_count_z,
            )
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html>
    pub unsafe fn get_device_group_present_capabilities_khr(
        &self,
        device: Device,
        device_group_present_capabilities: &mut DeviceGroupPresentCapabilitiesKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_device_group_present_capabilities_khr.unwrap())(
                device,
                device_group_present_capabilities,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html>
    pub unsafe fn get_device_group_surface_present_modes_khr(
        &self,
        device: Device,
        surface: SurfaceKHR,
    ) -> crate::Result<DeviceGroupPresentModeFlagsKHR> {
        unsafe {
            let mut modes = core::mem::MaybeUninit::uninit();
            let result = (self.get_device_group_surface_present_modes_khr.unwrap())(
                device,
                surface,
                modes.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(modes.assume_init()),
                err => Err(err),
            }
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAcquireNextImage2KHR.html>
    pub unsafe fn acquire_next_image2_khr(
        &self,
        device: Device,
        acquire_info: &AcquireNextImageInfoKHR<'_>,
    ) -> crate::Result<(u32, bool)> {
        unsafe {
            let mut image_index = core::mem::MaybeUninit::uninit();
            let result = (self.acquire_next_image2_khr.unwrap())(
                device,
                acquire_info,
                image_index.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok((image_index.assume_init(), false)),
                VkResult::SUBOPTIMAL_KHR => Ok((image_index.assume_init(), true)),
                err => Err(err),
            }
        }
    }
}

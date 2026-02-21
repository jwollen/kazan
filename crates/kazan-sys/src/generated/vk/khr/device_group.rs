#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type MemoryAllocateFlagsInfoKHR = MemoryAllocateFlagsInfo;
pub type BindBufferMemoryDeviceGroupInfoKHR = BindBufferMemoryDeviceGroupInfo;
pub type BindImageMemoryDeviceGroupInfoKHR = BindImageMemoryDeviceGroupInfo;
pub type DeviceGroupRenderPassBeginInfoKHR = DeviceGroupRenderPassBeginInfo;
pub type DeviceGroupCommandBufferBeginInfoKHR = DeviceGroupCommandBufferBeginInfo;
pub type DeviceGroupSubmitInfoKHR = DeviceGroupSubmitInfo;
pub type DeviceGroupBindSparseInfoKHR = DeviceGroupBindSparseInfo;
pub type PeerMemoryFeatureFlagsKHR = PeerMemoryFeatureFlags;
pub type MemoryAllocateFlagsKHR = MemoryAllocateFlags;
pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR = PFN_vkGetDeviceGroupPeerMemoryFeatures;
pub type PFN_vkCmdSetDeviceMaskKHR = PFN_vkCmdSetDeviceMask;
pub type PFN_vkCmdDispatchBaseKHR = PFN_vkCmdDispatchBase;

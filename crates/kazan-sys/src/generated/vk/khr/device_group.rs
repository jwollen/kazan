#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type MemoryAllocateFlagsInfoKHR<'a> = MemoryAllocateFlagsInfo<'a>;
pub type BindBufferMemoryDeviceGroupInfoKHR<'a> = BindBufferMemoryDeviceGroupInfo<'a>;
pub type BindImageMemoryDeviceGroupInfoKHR<'a> = BindImageMemoryDeviceGroupInfo<'a>;
pub type DeviceGroupRenderPassBeginInfoKHR<'a> = DeviceGroupRenderPassBeginInfo<'a>;
pub type DeviceGroupCommandBufferBeginInfoKHR<'a> = DeviceGroupCommandBufferBeginInfo<'a>;
pub type DeviceGroupSubmitInfoKHR<'a> = DeviceGroupSubmitInfo<'a>;
pub type DeviceGroupBindSparseInfoKHR<'a> = DeviceGroupBindSparseInfo<'a>;
pub type PeerMemoryFeatureFlagsKHR = PeerMemoryFeatureFlags;
pub type MemoryAllocateFlagsKHR = MemoryAllocateFlags;
pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR = PFN_vkGetDeviceGroupPeerMemoryFeatures;
pub type PFN_vkCmdSetDeviceMaskKHR = PFN_vkCmdSetDeviceMask;
pub type PFN_vkCmdDispatchBaseKHR = PFN_vkCmdDispatchBase;

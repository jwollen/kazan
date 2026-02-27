#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPresentInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_rect: Rect2D,
    pub dst_rect: Rect2D,
    pub persistent: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DisplayPresentInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DISPLAY_PRESENT_INFO_KHR,
            p_next: core::ptr::null(),
            src_rect: Default::default(),
            dst_rect: Default::default(),
            persistent: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DisplayPresentInfoKHR<'a> {
    pub fn src_rect(mut self, src_rect: Rect2D) -> Self {
        self.src_rect = src_rect;
        self
    }
    pub fn dst_rect(mut self, dst_rect: Rect2D) -> Self {
        self.dst_rect = dst_rect;
        self
    }
    pub fn persistent(mut self, persistent: Bool32) -> Self {
        self.persistent = persistent;
        self
    }
}
pub type PFN_vkCreateSharedSwapchainsKHR = unsafe extern "system" fn(
    device: Device,
    swapchain_count: u32,
    p_create_infos: *const SwapchainCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_swapchains: *mut SwapchainKHR,
) -> Result;

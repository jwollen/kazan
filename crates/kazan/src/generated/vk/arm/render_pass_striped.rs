#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceRenderPassStripedFeaturesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub render_pass_striped: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceRenderPassStripedFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM,
                p_next: core::ptr::null_mut(),
                render_pass_striped: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceRenderPassStripedFeaturesARM<'a> {
        pub fn render_pass_striped(mut self, render_pass_striped: Bool32) -> Self {
            self.render_pass_striped = render_pass_striped;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceRenderPassStripedPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub render_pass_stripe_granularity: Extent2D,
        pub max_render_pass_stripes: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceRenderPassStripedPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM,
                p_next: core::ptr::null_mut(),
                render_pass_stripe_granularity: Default::default(),
                max_render_pass_stripes: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceRenderPassStripedPropertiesARM<'a> {
        pub fn render_pass_stripe_granularity(
            mut self,
            render_pass_stripe_granularity: Extent2D,
        ) -> Self {
            self.render_pass_stripe_granularity = render_pass_stripe_granularity;
            self
        }
        pub fn max_render_pass_stripes(mut self, max_render_pass_stripes: u32) -> Self {
            self.max_render_pass_stripes = max_render_pass_stripes;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderPassStripeInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub stripe_area: Rect2D,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for RenderPassStripeInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::RENDER_PASS_STRIPE_INFO_ARM,
                p_next: core::ptr::null(),
                stripe_area: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderPassStripeInfoARM<'a> {
        pub fn stripe_area(mut self, stripe_area: Rect2D) -> Self {
            self.stripe_area = stripe_area;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderPassStripeBeginInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub stripe_info_count: u32,
        pub p_stripe_infos: *const RenderPassStripeInfoARM<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for RenderPassStripeBeginInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::RENDER_PASS_STRIPE_BEGIN_INFO_ARM,
                p_next: core::ptr::null(),
                stripe_info_count: Default::default(),
                p_stripe_infos: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderPassStripeBeginInfoARM<'a> {
        pub fn stripe_infos(mut self, stripe_infos: &'a [RenderPassStripeInfoARM<'a>]) -> Self {
            self.stripe_info_count = stripe_infos.len().try_into().unwrap();
            self.p_stripe_infos = stripe_infos.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderPassStripeSubmitInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub stripe_semaphore_info_count: u32,
        pub p_stripe_semaphore_infos: *const SemaphoreSubmitInfo<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for RenderPassStripeSubmitInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::RENDER_PASS_STRIPE_SUBMIT_INFO_ARM,
                p_next: core::ptr::null(),
                stripe_semaphore_info_count: Default::default(),
                p_stripe_semaphore_infos: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderPassStripeSubmitInfoARM<'a> {
        pub fn stripe_semaphore_infos(
            mut self,
            stripe_semaphore_infos: &'a [SemaphoreSubmitInfo<'a>],
        ) -> Self {
            self.stripe_semaphore_info_count = stripe_semaphore_infos.len().try_into().unwrap();
            self.p_stripe_semaphore_infos = stripe_semaphore_infos.as_ptr();
            self
        }
    }
}

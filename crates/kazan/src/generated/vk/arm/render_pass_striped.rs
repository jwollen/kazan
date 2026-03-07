#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_ARM_render_pass_striped";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRenderPassStripedFeaturesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceRenderPassStripedFeaturesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub render_pass_striped: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceRenderPassStripedFeaturesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceRenderPassStripedFeaturesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("render_pass_striped", &self.render_pass_striped)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRenderPassStripedFeaturesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceRenderPassStripedFeaturesARM<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceRenderPassStripedFeaturesARM<'a> {}

    impl Default for PhysicalDeviceRenderPassStripedFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                render_pass_striped: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRenderPassStripedFeaturesARM<'a> {
        pub fn render_pass_striped(mut self, render_pass_striped: bool) -> Self {
            self.render_pass_striped = render_pass_striped.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRenderPassStripedPropertiesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceRenderPassStripedPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub render_pass_stripe_granularity: Extent2D,
        pub max_render_pass_stripes: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceRenderPassStripedPropertiesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceRenderPassStripedPropertiesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "render_pass_stripe_granularity",
                    &self.render_pass_stripe_granularity,
                )
                .field("max_render_pass_stripes", &self.max_render_pass_stripes)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRenderPassStripedPropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceRenderPassStripedPropertiesARM<'a>
    {
    }

    impl Default for PhysicalDeviceRenderPassStripedPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassStripeInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderPassStripeInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub stripe_area: Rect2D,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for RenderPassStripeInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderPassStripeInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("stripe_area", &self.stripe_area)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderPassStripeInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDER_PASS_STRIPE_INFO_ARM;
    }

    impl Default for RenderPassStripeInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassStripeBeginInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderPassStripeBeginInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub stripe_info_count: u32,
        pub p_stripe_infos: *const RenderPassStripeInfoARM<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for RenderPassStripeBeginInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderPassStripeBeginInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("stripe_info_count", &self.stripe_info_count)
                .field("p_stripe_infos", &self.p_stripe_infos)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderPassStripeBeginInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDER_PASS_STRIPE_BEGIN_INFO_ARM;
    }

    unsafe impl<'a> Extends<RenderingInfo<'a>> for RenderPassStripeBeginInfoARM<'a> {}
    unsafe impl<'a> Extends<RenderPassBeginInfo<'a>> for RenderPassStripeBeginInfoARM<'a> {}

    impl Default for RenderPassStripeBeginInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassStripeSubmitInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RenderPassStripeSubmitInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub stripe_semaphore_info_count: u32,
        pub p_stripe_semaphore_infos: *const SemaphoreSubmitInfo<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for RenderPassStripeSubmitInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderPassStripeSubmitInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "stripe_semaphore_info_count",
                    &self.stripe_semaphore_info_count,
                )
                .field("p_stripe_semaphore_infos", &self.p_stripe_semaphore_infos)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderPassStripeSubmitInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDER_PASS_STRIPE_SUBMIT_INFO_ARM;
    }

    unsafe impl<'a> Extends<CommandBufferSubmitInfo<'a>> for RenderPassStripeSubmitInfoARM<'a> {}

    impl Default for RenderPassStripeSubmitInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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

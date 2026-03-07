#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_shading_rate_image";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkShadingRatePaletteNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ShadingRatePaletteNV<'a> {
        pub shading_rate_palette_entry_count: u32,
        pub p_shading_rate_palette_entries: *const ShadingRatePaletteEntryNV,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for ShadingRatePaletteNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ShadingRatePaletteNV")
                .field(
                    "shading_rate_palette_entry_count",
                    &self.shading_rate_palette_entry_count,
                )
                .field(
                    "p_shading_rate_palette_entries",
                    &self.p_shading_rate_palette_entries,
                )
                .finish()
        }
    }

    impl Default for ShadingRatePaletteNV<'_> {
        fn default() -> Self {
            Self {
                shading_rate_palette_entry_count: Default::default(),
                p_shading_rate_palette_entries: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ShadingRatePaletteNV<'a> {
        pub fn shading_rate_palette_entries(
            mut self,
            shading_rate_palette_entries: &'a [ShadingRatePaletteEntryNV],
        ) -> Self {
            self.shading_rate_palette_entry_count =
                shading_rate_palette_entries.len().try_into().unwrap();
            self.p_shading_rate_palette_entries = shading_rate_palette_entries.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineViewportShadingRateImageStateCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineViewportShadingRateImageStateCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub shading_rate_image_enable: Bool32,
        pub viewport_count: u32,
        pub p_shading_rate_palettes: *const ShadingRatePaletteNV<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PipelineViewportShadingRateImageStateCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineViewportShadingRateImageStateCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shading_rate_image_enable", &self.shading_rate_image_enable)
                .field("viewport_count", &self.viewport_count)
                .field("p_shading_rate_palettes", &self.p_shading_rate_palettes)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineViewportShadingRateImageStateCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV;
    }

    unsafe impl<'a> Extends<PipelineViewportStateCreateInfo<'a>>
        for PipelineViewportShadingRateImageStateCreateInfoNV<'a>
    {
    }

    impl Default for PipelineViewportShadingRateImageStateCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                shading_rate_image_enable: Default::default(),
                viewport_count: Default::default(),
                p_shading_rate_palettes: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineViewportShadingRateImageStateCreateInfoNV<'a> {
        pub fn shading_rate_image_enable(mut self, shading_rate_image_enable: bool) -> Self {
            self.shading_rate_image_enable = shading_rate_image_enable.into();
            self
        }

        pub fn shading_rate_palettes(
            mut self,
            shading_rate_palettes: &'a [ShadingRatePaletteNV<'a>],
        ) -> Self {
            self.viewport_count = shading_rate_palettes.len().try_into().unwrap();
            self.p_shading_rate_palettes = shading_rate_palettes.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShadingRateImageFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShadingRateImageFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shading_rate_image: Bool32,
        pub shading_rate_coarse_sample_order: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceShadingRateImageFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShadingRateImageFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shading_rate_image", &self.shading_rate_image)
                .field(
                    "shading_rate_coarse_sample_order",
                    &self.shading_rate_coarse_sample_order,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShadingRateImageFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShadingRateImageFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShadingRateImageFeaturesNV<'a> {}

    impl Default for PhysicalDeviceShadingRateImageFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shading_rate_image: Default::default(),
                shading_rate_coarse_sample_order: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShadingRateImageFeaturesNV<'a> {
        pub fn shading_rate_image(mut self, shading_rate_image: bool) -> Self {
            self.shading_rate_image = shading_rate_image.into();
            self
        }

        pub fn shading_rate_coarse_sample_order(
            mut self,
            shading_rate_coarse_sample_order: bool,
        ) -> Self {
            self.shading_rate_coarse_sample_order = shading_rate_coarse_sample_order.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShadingRateImagePropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShadingRateImagePropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shading_rate_texel_size: Extent2D,
        pub shading_rate_palette_size: u32,
        pub shading_rate_max_coarse_samples: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceShadingRateImagePropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShadingRateImagePropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shading_rate_texel_size", &self.shading_rate_texel_size)
                .field("shading_rate_palette_size", &self.shading_rate_palette_size)
                .field(
                    "shading_rate_max_coarse_samples",
                    &self.shading_rate_max_coarse_samples,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShadingRateImagePropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceShadingRateImagePropertiesNV<'a>
    {
    }

    impl Default for PhysicalDeviceShadingRateImagePropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shading_rate_texel_size: Default::default(),
                shading_rate_palette_size: Default::default(),
                shading_rate_max_coarse_samples: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShadingRateImagePropertiesNV<'a> {
        pub fn shading_rate_texel_size(mut self, shading_rate_texel_size: Extent2D) -> Self {
            self.shading_rate_texel_size = shading_rate_texel_size;
            self
        }

        pub fn shading_rate_palette_size(mut self, shading_rate_palette_size: u32) -> Self {
            self.shading_rate_palette_size = shading_rate_palette_size;
            self
        }

        pub fn shading_rate_max_coarse_samples(
            mut self,
            shading_rate_max_coarse_samples: u32,
        ) -> Self {
            self.shading_rate_max_coarse_samples = shading_rate_max_coarse_samples;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCoarseSampleLocationNV.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct CoarseSampleLocationNV {
        pub pixel_x: u32,
        pub pixel_y: u32,
        pub sample: u32,
    }

    impl CoarseSampleLocationNV {
        pub fn pixel_x(mut self, pixel_x: u32) -> Self {
            self.pixel_x = pixel_x;
            self
        }

        pub fn pixel_y(mut self, pixel_y: u32) -> Self {
            self.pixel_y = pixel_y;
            self
        }

        pub fn sample(mut self, sample: u32) -> Self {
            self.sample = sample;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCoarseSampleOrderCustomNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CoarseSampleOrderCustomNV<'a> {
        pub shading_rate: ShadingRatePaletteEntryNV,
        pub sample_count: u32,
        pub sample_location_count: u32,
        pub p_sample_locations: *const CoarseSampleLocationNV,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for CoarseSampleOrderCustomNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CoarseSampleOrderCustomNV")
                .field("shading_rate", &self.shading_rate)
                .field("sample_count", &self.sample_count)
                .field("sample_location_count", &self.sample_location_count)
                .field("p_sample_locations", &self.p_sample_locations)
                .finish()
        }
    }

    impl Default for CoarseSampleOrderCustomNV<'_> {
        fn default() -> Self {
            Self {
                shading_rate: Default::default(),
                sample_count: Default::default(),
                sample_location_count: Default::default(),
                p_sample_locations: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CoarseSampleOrderCustomNV<'a> {
        pub fn shading_rate(mut self, shading_rate: ShadingRatePaletteEntryNV) -> Self {
            self.shading_rate = shading_rate;
            self
        }

        pub fn sample_count(mut self, sample_count: u32) -> Self {
            self.sample_count = sample_count;
            self
        }

        pub fn sample_locations(mut self, sample_locations: &'a [CoarseSampleLocationNV]) -> Self {
            self.sample_location_count = sample_locations.len().try_into().unwrap();
            self.p_sample_locations = sample_locations.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineViewportCoarseSampleOrderStateCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub sample_order_type: CoarseSampleOrderTypeNV,
        pub custom_sample_order_count: u32,
        pub p_custom_sample_orders: *const CoarseSampleOrderCustomNV<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PipelineViewportCoarseSampleOrderStateCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineViewportCoarseSampleOrderStateCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("sample_order_type", &self.sample_order_type)
                .field("custom_sample_order_count", &self.custom_sample_order_count)
                .field("p_custom_sample_orders", &self.p_custom_sample_orders)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineViewportCoarseSampleOrderStateCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV;
    }

    unsafe impl<'a> Extends<PipelineViewportStateCreateInfo<'a>>
        for PipelineViewportCoarseSampleOrderStateCreateInfoNV<'a>
    {
    }

    impl Default for PipelineViewportCoarseSampleOrderStateCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                sample_order_type: Default::default(),
                custom_sample_order_count: Default::default(),
                p_custom_sample_orders: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineViewportCoarseSampleOrderStateCreateInfoNV<'a> {
        pub fn sample_order_type(mut self, sample_order_type: CoarseSampleOrderTypeNV) -> Self {
            self.sample_order_type = sample_order_type;
            self
        }

        pub fn custom_sample_orders(
            mut self,
            custom_sample_orders: &'a [CoarseSampleOrderCustomNV<'a>],
        ) -> Self {
            self.custom_sample_order_count = custom_sample_orders.len().try_into().unwrap();
            self.p_custom_sample_orders = custom_sample_orders.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkShadingRatePaletteEntryNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ShadingRatePaletteEntryNV(i32);

    impl ShadingRatePaletteEntryNV {
        pub const NO_INVOCATIONS_NV: Self = Self(0);
        pub const _16_INVOCATIONS_PER_PIXEL_NV: Self = Self(1);
        pub const _8_INVOCATIONS_PER_PIXEL_NV: Self = Self(2);
        pub const _4_INVOCATIONS_PER_PIXEL_NV: Self = Self(3);
        pub const _2_INVOCATIONS_PER_PIXEL_NV: Self = Self(4);
        pub const _1_INVOCATION_PER_PIXEL_NV: Self = Self(5);
        pub const _1_INVOCATION_PER_2X1_PIXELS_NV: Self = Self(6);
        pub const _1_INVOCATION_PER_1X2_PIXELS_NV: Self = Self(7);
        pub const _1_INVOCATION_PER_2X2_PIXELS_NV: Self = Self(8);
        pub const _1_INVOCATION_PER_4X2_PIXELS_NV: Self = Self(9);
        pub const _1_INVOCATION_PER_2X4_PIXELS_NV: Self = Self(10);
        pub const _1_INVOCATION_PER_4X4_PIXELS_NV: Self = Self(11);
    }

    impl fmt::Debug for ShadingRatePaletteEntryNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::NO_INVOCATIONS_NV => Some("NO_INVOCATIONS_NV"),
                Self::_16_INVOCATIONS_PER_PIXEL_NV => Some("_16_INVOCATIONS_PER_PIXEL_NV"),
                Self::_8_INVOCATIONS_PER_PIXEL_NV => Some("_8_INVOCATIONS_PER_PIXEL_NV"),
                Self::_4_INVOCATIONS_PER_PIXEL_NV => Some("_4_INVOCATIONS_PER_PIXEL_NV"),
                Self::_2_INVOCATIONS_PER_PIXEL_NV => Some("_2_INVOCATIONS_PER_PIXEL_NV"),
                Self::_1_INVOCATION_PER_PIXEL_NV => Some("_1_INVOCATION_PER_PIXEL_NV"),
                Self::_1_INVOCATION_PER_2X1_PIXELS_NV => Some("_1_INVOCATION_PER_2X1_PIXELS_NV"),
                Self::_1_INVOCATION_PER_1X2_PIXELS_NV => Some("_1_INVOCATION_PER_1X2_PIXELS_NV"),
                Self::_1_INVOCATION_PER_2X2_PIXELS_NV => Some("_1_INVOCATION_PER_2X2_PIXELS_NV"),
                Self::_1_INVOCATION_PER_4X2_PIXELS_NV => Some("_1_INVOCATION_PER_4X2_PIXELS_NV"),
                Self::_1_INVOCATION_PER_2X4_PIXELS_NV => Some("_1_INVOCATION_PER_2X4_PIXELS_NV"),
                Self::_1_INVOCATION_PER_4X4_PIXELS_NV => Some("_1_INVOCATION_PER_4X4_PIXELS_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCoarseSampleOrderTypeNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CoarseSampleOrderTypeNV(i32);

    impl CoarseSampleOrderTypeNV {
        pub const DEFAULT_NV: Self = Self(0);
        pub const CUSTOM_NV: Self = Self(1);
        pub const PIXEL_MAJOR_NV: Self = Self(2);
        pub const SAMPLE_MAJOR_NV: Self = Self(3);
    }

    impl fmt::Debug for CoarseSampleOrderTypeNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEFAULT_NV => Some("DEFAULT_NV"),
                Self::CUSTOM_NV => Some("CUSTOM_NV"),
                Self::PIXEL_MAJOR_NV => Some("PIXEL_MAJOR_NV"),
                Self::SAMPLE_MAJOR_NV => Some("SAMPLE_MAJOR_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindShadingRateImageNV.html>
    pub type PFN_vkCmdBindShadingRateImageNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        image_view: ImageView,
        image_layout: ImageLayout,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetViewportShadingRatePaletteNV.html>
    pub type PFN_vkCmdSetViewportShadingRatePaletteNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_shading_rate_palettes: *const ShadingRatePaletteNV<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoarseSampleOrderNV.html>
    pub type PFN_vkCmdSetCoarseSampleOrderNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        sample_order_type: CoarseSampleOrderTypeNV,
        custom_sample_order_count: u32,
        p_custom_sample_orders: *const CoarseSampleOrderCustomNV<'_>,
    );
}

pub struct DeviceFn {
    cmd_bind_shading_rate_image_nv: PFN_vkCmdBindShadingRateImageNV,
    cmd_set_viewport_shading_rate_palette_nv: PFN_vkCmdSetViewportShadingRatePaletteNV,
    cmd_set_coarse_sample_order_nv: PFN_vkCmdSetCoarseSampleOrderNV,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_bind_shading_rate_image_nv: transmute(
                    load(c"vkCmdBindShadingRateImageNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_viewport_shading_rate_palette_nv: transmute(
                    load(c"vkCmdSetViewportShadingRatePaletteNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_coarse_sample_order_nv: transmute(
                    load(c"vkCmdSetCoarseSampleOrderNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindShadingRateImageNV.html>
    pub unsafe fn cmd_bind_shading_rate_image_nv(
        &self,
        command_buffer: CommandBuffer,
        image_view: ImageView,
        image_layout: ImageLayout,
    ) {
        unsafe { (self.cmd_bind_shading_rate_image_nv)(command_buffer, image_view, image_layout) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetViewportShadingRatePaletteNV.html>
    pub unsafe fn cmd_set_viewport_shading_rate_palette_nv(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        shading_rate_palettes: &[ShadingRatePaletteNV<'_>],
    ) {
        unsafe {
            (self.cmd_set_viewport_shading_rate_palette_nv)(
                command_buffer,
                first_viewport,
                shading_rate_palettes.len().try_into().unwrap(),
                shading_rate_palettes.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoarseSampleOrderNV.html>
    pub unsafe fn cmd_set_coarse_sample_order_nv(
        &self,
        command_buffer: CommandBuffer,
        sample_order_type: CoarseSampleOrderTypeNV,
        custom_sample_orders: &[CoarseSampleOrderCustomNV<'_>],
    ) {
        unsafe {
            (self.cmd_set_coarse_sample_order_nv)(
                command_buffer,
                sample_order_type,
                custom_sample_orders.len().try_into().unwrap(),
                custom_sample_orders.as_ptr() as _,
            )
        }
    }
}

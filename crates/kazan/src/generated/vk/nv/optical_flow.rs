#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_optical_flow";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    handle_nondispatchable!(
        OpticalFlowSessionNV,
        OPTICAL_FLOW_SESSION_NV,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpticalFlowSessionNV.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceOpticalFlowFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceOpticalFlowFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub optical_flow: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceOpticalFlowFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceOpticalFlowFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("optical_flow", &self.optical_flow)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceOpticalFlowFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceOpticalFlowFeaturesNV<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceOpticalFlowFeaturesNV<'a> {}

    impl Default for PhysicalDeviceOpticalFlowFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                optical_flow: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceOpticalFlowFeaturesNV<'a> {
        pub fn optical_flow(mut self, optical_flow: bool) -> Self {
            self.optical_flow = optical_flow.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceOpticalFlowPropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceOpticalFlowPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub supported_output_grid_sizes: OpticalFlowGridSizeFlagsNV,
        pub supported_hint_grid_sizes: OpticalFlowGridSizeFlagsNV,
        pub hint_supported: Bool32,
        pub cost_supported: Bool32,
        pub bidirectional_flow_supported: Bool32,
        pub global_flow_supported: Bool32,
        pub min_width: u32,
        pub min_height: u32,
        pub max_width: u32,
        pub max_height: u32,
        pub max_num_regions_of_interest: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceOpticalFlowPropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceOpticalFlowPropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "supported_output_grid_sizes",
                    &self.supported_output_grid_sizes,
                )
                .field("supported_hint_grid_sizes", &self.supported_hint_grid_sizes)
                .field("hint_supported", &self.hint_supported)
                .field("cost_supported", &self.cost_supported)
                .field(
                    "bidirectional_flow_supported",
                    &self.bidirectional_flow_supported,
                )
                .field("global_flow_supported", &self.global_flow_supported)
                .field("min_width", &self.min_width)
                .field("min_height", &self.min_height)
                .field("max_width", &self.max_width)
                .field("max_height", &self.max_height)
                .field(
                    "max_num_regions_of_interest",
                    &self.max_num_regions_of_interest,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceOpticalFlowPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceOpticalFlowPropertiesNV<'a>
    {
    }

    impl Default for PhysicalDeviceOpticalFlowPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                supported_output_grid_sizes: Default::default(),
                supported_hint_grid_sizes: Default::default(),
                hint_supported: Default::default(),
                cost_supported: Default::default(),
                bidirectional_flow_supported: Default::default(),
                global_flow_supported: Default::default(),
                min_width: Default::default(),
                min_height: Default::default(),
                max_width: Default::default(),
                max_height: Default::default(),
                max_num_regions_of_interest: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceOpticalFlowPropertiesNV<'a> {
        pub fn supported_output_grid_sizes(
            mut self,
            supported_output_grid_sizes: OpticalFlowGridSizeFlagsNV,
        ) -> Self {
            self.supported_output_grid_sizes = supported_output_grid_sizes;
            self
        }

        pub fn supported_hint_grid_sizes(
            mut self,
            supported_hint_grid_sizes: OpticalFlowGridSizeFlagsNV,
        ) -> Self {
            self.supported_hint_grid_sizes = supported_hint_grid_sizes;
            self
        }

        pub fn hint_supported(mut self, hint_supported: bool) -> Self {
            self.hint_supported = hint_supported.into();
            self
        }

        pub fn cost_supported(mut self, cost_supported: bool) -> Self {
            self.cost_supported = cost_supported.into();
            self
        }

        pub fn bidirectional_flow_supported(mut self, bidirectional_flow_supported: bool) -> Self {
            self.bidirectional_flow_supported = bidirectional_flow_supported.into();
            self
        }

        pub fn global_flow_supported(mut self, global_flow_supported: bool) -> Self {
            self.global_flow_supported = global_flow_supported.into();
            self
        }

        pub fn min_width(mut self, min_width: u32) -> Self {
            self.min_width = min_width;
            self
        }

        pub fn min_height(mut self, min_height: u32) -> Self {
            self.min_height = min_height;
            self
        }

        pub fn max_width(mut self, max_width: u32) -> Self {
            self.max_width = max_width;
            self
        }

        pub fn max_height(mut self, max_height: u32) -> Self {
            self.max_height = max_height;
            self
        }

        pub fn max_num_regions_of_interest(mut self, max_num_regions_of_interest: u32) -> Self {
            self.max_num_regions_of_interest = max_num_regions_of_interest;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpticalFlowImageFormatInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct OpticalFlowImageFormatInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub usage: OpticalFlowUsageFlagsNV,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for OpticalFlowImageFormatInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OpticalFlowImageFormatInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("usage", &self.usage)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for OpticalFlowImageFormatInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceImageFormatInfo2<'a>> for OpticalFlowImageFormatInfoNV<'a> {}
    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for OpticalFlowImageFormatInfoNV<'a> {}

    impl Default for OpticalFlowImageFormatInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> OpticalFlowImageFormatInfoNV<'a> {
        pub fn usage(mut self, usage: OpticalFlowUsageFlagsNV) -> Self {
            self.usage = usage;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpticalFlowImageFormatPropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct OpticalFlowImageFormatPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub format: Format,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for OpticalFlowImageFormatPropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OpticalFlowImageFormatPropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("format", &self.format)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for OpticalFlowImageFormatPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV;
    }

    impl Default for OpticalFlowImageFormatPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                format: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> OpticalFlowImageFormatPropertiesNV<'a> {
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpticalFlowSessionCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct OpticalFlowSessionCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub width: u32,
        pub height: u32,
        pub image_format: Format,
        pub flow_vector_format: Format,
        pub cost_format: Format,
        pub output_grid_size: OpticalFlowGridSizeFlagsNV,
        pub hint_grid_size: OpticalFlowGridSizeFlagsNV,
        pub performance_level: OpticalFlowPerformanceLevelNV,
        pub flags: OpticalFlowSessionCreateFlagsNV,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for OpticalFlowSessionCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OpticalFlowSessionCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("width", &self.width)
                .field("height", &self.height)
                .field("image_format", &self.image_format)
                .field("flow_vector_format", &self.flow_vector_format)
                .field("cost_format", &self.cost_format)
                .field("output_grid_size", &self.output_grid_size)
                .field("hint_grid_size", &self.hint_grid_size)
                .field("performance_level", &self.performance_level)
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for OpticalFlowSessionCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::OPTICAL_FLOW_SESSION_CREATE_INFO_NV;
    }

    impl Default for OpticalFlowSessionCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                width: Default::default(),
                height: Default::default(),
                image_format: Default::default(),
                flow_vector_format: Default::default(),
                cost_format: Default::default(),
                output_grid_size: Default::default(),
                hint_grid_size: Default::default(),
                performance_level: Default::default(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> OpticalFlowSessionCreateInfoNV<'a> {
        pub fn width(mut self, width: u32) -> Self {
            self.width = width;
            self
        }

        pub fn height(mut self, height: u32) -> Self {
            self.height = height;
            self
        }

        pub fn image_format(mut self, image_format: Format) -> Self {
            self.image_format = image_format;
            self
        }

        pub fn flow_vector_format(mut self, flow_vector_format: Format) -> Self {
            self.flow_vector_format = flow_vector_format;
            self
        }

        pub fn cost_format(mut self, cost_format: Format) -> Self {
            self.cost_format = cost_format;
            self
        }

        pub fn output_grid_size(mut self, output_grid_size: OpticalFlowGridSizeFlagsNV) -> Self {
            self.output_grid_size = output_grid_size;
            self
        }

        pub fn hint_grid_size(mut self, hint_grid_size: OpticalFlowGridSizeFlagsNV) -> Self {
            self.hint_grid_size = hint_grid_size;
            self
        }

        pub fn performance_level(
            mut self,
            performance_level: OpticalFlowPerformanceLevelNV,
        ) -> Self {
            self.performance_level = performance_level;
            self
        }

        pub fn flags(mut self, flags: OpticalFlowSessionCreateFlagsNV) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpticalFlowSessionCreatePrivateDataInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct OpticalFlowSessionCreatePrivateDataInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub id: u32,
        pub size: u32,
        pub p_private_data: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for OpticalFlowSessionCreatePrivateDataInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OpticalFlowSessionCreatePrivateDataInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("id", &self.id)
                .field("size", &self.size)
                .field("p_private_data", &self.p_private_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for OpticalFlowSessionCreatePrivateDataInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV;
    }

    unsafe impl<'a> Extends<OpticalFlowSessionCreateInfoNV<'a>>
        for OpticalFlowSessionCreatePrivateDataInfoNV<'a>
    {
    }

    impl Default for OpticalFlowSessionCreatePrivateDataInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                id: Default::default(),
                size: Default::default(),
                p_private_data: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> OpticalFlowSessionCreatePrivateDataInfoNV<'a> {
        pub fn id(mut self, id: u32) -> Self {
            self.id = id;
            self
        }

        pub fn size(mut self, size: u32) -> Self {
            self.size = size;
            self
        }

        pub fn private_data(mut self, private_data: *const c_void) -> Self {
            self.p_private_data = private_data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpticalFlowExecuteInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct OpticalFlowExecuteInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: OpticalFlowExecuteFlagsNV,
        pub region_count: u32,
        pub p_regions: *const Rect2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for OpticalFlowExecuteInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OpticalFlowExecuteInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("region_count", &self.region_count)
                .field("p_regions", &self.p_regions)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for OpticalFlowExecuteInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::OPTICAL_FLOW_EXECUTE_INFO_NV;
    }

    impl Default for OpticalFlowExecuteInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                flags: Default::default(),
                region_count: Default::default(),
                p_regions: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> OpticalFlowExecuteInfoNV<'a> {
        pub fn flags(mut self, flags: OpticalFlowExecuteFlagsNV) -> Self {
            self.flags = flags;
            self
        }

        pub fn regions(mut self, regions: &'a [Rect2D]) -> Self {
            self.region_count = regions.len().try_into().unwrap();
            self.p_regions = regions.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpticalFlowPerformanceLevelNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OpticalFlowPerformanceLevelNV(i32);

    impl OpticalFlowPerformanceLevelNV {
        pub const UNKNOWN_NV: Self = Self(0);
        pub const SLOW_NV: Self = Self(1);
        pub const MEDIUM_NV: Self = Self(2);
        pub const FAST_NV: Self = Self(3);
    }

    impl fmt::Debug for OpticalFlowPerformanceLevelNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::UNKNOWN_NV => Some("UNKNOWN_NV"),
                Self::SLOW_NV => Some("SLOW_NV"),
                Self::MEDIUM_NV => Some("MEDIUM_NV"),
                Self::FAST_NV => Some("FAST_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpticalFlowSessionBindingPointNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OpticalFlowSessionBindingPointNV(i32);

    impl OpticalFlowSessionBindingPointNV {
        pub const UNKNOWN_NV: Self = Self(0);
        pub const INPUT_NV: Self = Self(1);
        pub const REFERENCE_NV: Self = Self(2);
        pub const HINT_NV: Self = Self(3);
        pub const FLOW_VECTOR_NV: Self = Self(4);
        pub const BACKWARD_FLOW_VECTOR_NV: Self = Self(5);
        pub const COST_NV: Self = Self(6);
        pub const BACKWARD_COST_NV: Self = Self(7);
        pub const GLOBAL_FLOW_NV: Self = Self(8);
    }

    impl fmt::Debug for OpticalFlowSessionBindingPointNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::UNKNOWN_NV => Some("UNKNOWN_NV"),
                Self::INPUT_NV => Some("INPUT_NV"),
                Self::REFERENCE_NV => Some("REFERENCE_NV"),
                Self::HINT_NV => Some("HINT_NV"),
                Self::FLOW_VECTOR_NV => Some("FLOW_VECTOR_NV"),
                Self::BACKWARD_FLOW_VECTOR_NV => Some("BACKWARD_FLOW_VECTOR_NV"),
                Self::COST_NV => Some("COST_NV"),
                Self::BACKWARD_COST_NV => Some("BACKWARD_COST_NV"),
                Self::GLOBAL_FLOW_NV => Some("GLOBAL_FLOW_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpticalFlowGridSizeFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct OpticalFlowGridSizeFlagsNV(Flags);
    vk_bitflags_wrapped!(OpticalFlowGridSizeFlagsNV, Flags);

    impl OpticalFlowGridSizeFlagsNV {
        pub const _1X1_NV: Self = Self(OpticalFlowGridSizeFlagBitsNV::_1X1_NV.0);
        pub const _2X2_NV: Self = Self(OpticalFlowGridSizeFlagBitsNV::_2X2_NV.0);
        pub const _4X4_NV: Self = Self(OpticalFlowGridSizeFlagBitsNV::_4X4_NV.0);
        pub const _8X8_NV: Self = Self(OpticalFlowGridSizeFlagBitsNV::_8X8_NV.0);
        pub const UNKNOWN: Self = Self(0);
    }

    impl fmt::Debug for OpticalFlowGridSizeFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (OpticalFlowGridSizeFlagsNV::_1X1_NV.0, "_1X1_NV"),
                (OpticalFlowGridSizeFlagsNV::_2X2_NV.0, "_2X2_NV"),
                (OpticalFlowGridSizeFlagsNV::_4X4_NV.0, "_4X4_NV"),
                (OpticalFlowGridSizeFlagsNV::_8X8_NV.0, "_8X8_NV"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpticalFlowGridSizeFlagBitsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct OpticalFlowGridSizeFlagBitsNV(u32);

    impl OpticalFlowGridSizeFlagBitsNV {
        pub const _1X1_NV: Self = Self(1 << 0);
        pub const _2X2_NV: Self = Self(1 << 1);
        pub const _4X4_NV: Self = Self(1 << 2);
        pub const _8X8_NV: Self = Self(1 << 3);
    }

    impl fmt::Debug for OpticalFlowGridSizeFlagBitsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_1X1_NV => Some("_1X1_NV"),
                Self::_2X2_NV => Some("_2X2_NV"),
                Self::_4X4_NV => Some("_4X4_NV"),
                Self::_8X8_NV => Some("_8X8_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpticalFlowUsageFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct OpticalFlowUsageFlagsNV(Flags);
    vk_bitflags_wrapped!(OpticalFlowUsageFlagsNV, Flags);

    impl OpticalFlowUsageFlagsNV {
        pub const INPUT_NV: Self = Self(OpticalFlowUsageFlagBitsNV::INPUT_NV.0);
        pub const OUTPUT_NV: Self = Self(OpticalFlowUsageFlagBitsNV::OUTPUT_NV.0);
        pub const HINT_NV: Self = Self(OpticalFlowUsageFlagBitsNV::HINT_NV.0);
        pub const COST_NV: Self = Self(OpticalFlowUsageFlagBitsNV::COST_NV.0);
        pub const GLOBAL_FLOW_NV: Self = Self(OpticalFlowUsageFlagBitsNV::GLOBAL_FLOW_NV.0);
        pub const UNKNOWN: Self = Self(0);
    }

    impl fmt::Debug for OpticalFlowUsageFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (OpticalFlowUsageFlagsNV::INPUT_NV.0, "INPUT_NV"),
                (OpticalFlowUsageFlagsNV::OUTPUT_NV.0, "OUTPUT_NV"),
                (OpticalFlowUsageFlagsNV::HINT_NV.0, "HINT_NV"),
                (OpticalFlowUsageFlagsNV::COST_NV.0, "COST_NV"),
                (OpticalFlowUsageFlagsNV::GLOBAL_FLOW_NV.0, "GLOBAL_FLOW_NV"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpticalFlowUsageFlagBitsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct OpticalFlowUsageFlagBitsNV(u32);

    impl OpticalFlowUsageFlagBitsNV {
        pub const INPUT_NV: Self = Self(1 << 0);
        pub const OUTPUT_NV: Self = Self(1 << 1);
        pub const HINT_NV: Self = Self(1 << 2);
        pub const COST_NV: Self = Self(1 << 3);
        pub const GLOBAL_FLOW_NV: Self = Self(1 << 4);
    }

    impl fmt::Debug for OpticalFlowUsageFlagBitsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::INPUT_NV => Some("INPUT_NV"),
                Self::OUTPUT_NV => Some("OUTPUT_NV"),
                Self::HINT_NV => Some("HINT_NV"),
                Self::COST_NV => Some("COST_NV"),
                Self::GLOBAL_FLOW_NV => Some("GLOBAL_FLOW_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpticalFlowSessionCreateFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct OpticalFlowSessionCreateFlagsNV(Flags);
    vk_bitflags_wrapped!(OpticalFlowSessionCreateFlagsNV, Flags);

    impl OpticalFlowSessionCreateFlagsNV {
        pub const ENABLE_HINT_NV: Self = Self(OpticalFlowSessionCreateFlagBitsNV::ENABLE_HINT_NV.0);
        pub const ENABLE_COST_NV: Self = Self(OpticalFlowSessionCreateFlagBitsNV::ENABLE_COST_NV.0);
        pub const ENABLE_GLOBAL_FLOW_NV: Self =
            Self(OpticalFlowSessionCreateFlagBitsNV::ENABLE_GLOBAL_FLOW_NV.0);
        pub const ALLOW_REGIONS_NV: Self =
            Self(OpticalFlowSessionCreateFlagBitsNV::ALLOW_REGIONS_NV.0);
        pub const BOTH_DIRECTIONS_NV: Self =
            Self(OpticalFlowSessionCreateFlagBitsNV::BOTH_DIRECTIONS_NV.0);
    }

    impl fmt::Debug for OpticalFlowSessionCreateFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    OpticalFlowSessionCreateFlagsNV::ENABLE_HINT_NV.0,
                    "ENABLE_HINT_NV",
                ),
                (
                    OpticalFlowSessionCreateFlagsNV::ENABLE_COST_NV.0,
                    "ENABLE_COST_NV",
                ),
                (
                    OpticalFlowSessionCreateFlagsNV::ENABLE_GLOBAL_FLOW_NV.0,
                    "ENABLE_GLOBAL_FLOW_NV",
                ),
                (
                    OpticalFlowSessionCreateFlagsNV::ALLOW_REGIONS_NV.0,
                    "ALLOW_REGIONS_NV",
                ),
                (
                    OpticalFlowSessionCreateFlagsNV::BOTH_DIRECTIONS_NV.0,
                    "BOTH_DIRECTIONS_NV",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpticalFlowSessionCreateFlagBitsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct OpticalFlowSessionCreateFlagBitsNV(u32);

    impl OpticalFlowSessionCreateFlagBitsNV {
        pub const ENABLE_HINT_NV: Self = Self(1 << 0);
        pub const ENABLE_COST_NV: Self = Self(1 << 1);
        pub const ENABLE_GLOBAL_FLOW_NV: Self = Self(1 << 2);
        pub const ALLOW_REGIONS_NV: Self = Self(1 << 3);
        pub const BOTH_DIRECTIONS_NV: Self = Self(1 << 4);
    }

    impl fmt::Debug for OpticalFlowSessionCreateFlagBitsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ENABLE_HINT_NV => Some("ENABLE_HINT_NV"),
                Self::ENABLE_COST_NV => Some("ENABLE_COST_NV"),
                Self::ENABLE_GLOBAL_FLOW_NV => Some("ENABLE_GLOBAL_FLOW_NV"),
                Self::ALLOW_REGIONS_NV => Some("ALLOW_REGIONS_NV"),
                Self::BOTH_DIRECTIONS_NV => Some("BOTH_DIRECTIONS_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpticalFlowExecuteFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct OpticalFlowExecuteFlagsNV(Flags);
    vk_bitflags_wrapped!(OpticalFlowExecuteFlagsNV, Flags);

    impl OpticalFlowExecuteFlagsNV {
        pub const DISABLE_TEMPORAL_HINTS_NV: Self =
            Self(OpticalFlowExecuteFlagBitsNV::DISABLE_TEMPORAL_HINTS_NV.0);
    }

    impl fmt::Debug for OpticalFlowExecuteFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[(
                OpticalFlowExecuteFlagsNV::DISABLE_TEMPORAL_HINTS_NV.0,
                "DISABLE_TEMPORAL_HINTS_NV",
            )];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpticalFlowExecuteFlagBitsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct OpticalFlowExecuteFlagBitsNV(u32);

    impl OpticalFlowExecuteFlagBitsNV {
        pub const DISABLE_TEMPORAL_HINTS_NV: Self = Self(1 << 0);
    }

    impl fmt::Debug for OpticalFlowExecuteFlagBitsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DISABLE_TEMPORAL_HINTS_NV => Some("DISABLE_TEMPORAL_HINTS_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html>
    pub type PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_optical_flow_image_format_info: *const OpticalFlowImageFormatInfoNV<'_>,
            p_format_count: *mut u32,
            p_image_format_properties: *mut OpticalFlowImageFormatPropertiesNV<'_>,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateOpticalFlowSessionNV.html>
    pub type PFN_vkCreateOpticalFlowSessionNV = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const OpticalFlowSessionCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_session: *mut OpticalFlowSessionNV,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyOpticalFlowSessionNV.html>
    pub type PFN_vkDestroyOpticalFlowSessionNV = unsafe extern "system" fn(
        device: Device,
        session: OpticalFlowSessionNV,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBindOpticalFlowSessionImageNV.html>
    pub type PFN_vkBindOpticalFlowSessionImageNV = unsafe extern "system" fn(
        device: Device,
        session: OpticalFlowSessionNV,
        binding_point: OpticalFlowSessionBindingPointNV,
        view: ImageView,
        layout: ImageLayout,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdOpticalFlowExecuteNV.html>
    pub type PFN_vkCmdOpticalFlowExecuteNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        session: OpticalFlowSessionNV,
        p_execute_info: *const OpticalFlowExecuteInfoNV<'_>,
    );
}

pub struct InstanceFn {
    get_physical_device_optical_flow_image_formats_nv:
        PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_optical_flow_image_formats_nv: transmute(
                    load(c"vkGetPhysicalDeviceOpticalFlowImageFormatsNV")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html>
    pub unsafe fn get_physical_device_optical_flow_image_formats_nv<'a>(
        &self,
        physical_device: PhysicalDevice,
        optical_flow_image_format_info: &OpticalFlowImageFormatInfoNV<'a>,
        mut image_format_properties: impl ExtendUninit<OpticalFlowImageFormatPropertiesNV<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |format_count, image_format_properties| {
                let result = (self.get_physical_device_optical_flow_image_formats_nv)(
                    physical_device,
                    optical_flow_image_format_info,
                    format_count,
                    image_format_properties as _,
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
            let image_format_properties_buf = image_format_properties.reserve(capacity);
            len = image_format_properties_buf.len().try_into().unwrap();
            let result = call(&mut len, image_format_properties_buf.as_mut_ptr() as *mut _)?;
            image_format_properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}

pub struct DeviceFn {
    create_optical_flow_session_nv: PFN_vkCreateOpticalFlowSessionNV,
    destroy_optical_flow_session_nv: PFN_vkDestroyOpticalFlowSessionNV,
    bind_optical_flow_session_image_nv: PFN_vkBindOpticalFlowSessionImageNV,
    cmd_optical_flow_execute_nv: PFN_vkCmdOpticalFlowExecuteNV,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_optical_flow_session_nv: transmute(
                    load(c"vkCreateOpticalFlowSessionNV").ok_or(MissingEntryPointError)?,
                ),
                destroy_optical_flow_session_nv: transmute(
                    load(c"vkDestroyOpticalFlowSessionNV").ok_or(MissingEntryPointError)?,
                ),
                bind_optical_flow_session_image_nv: transmute(
                    load(c"vkBindOpticalFlowSessionImageNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_optical_flow_execute_nv: transmute(
                    load(c"vkCmdOpticalFlowExecuteNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateOpticalFlowSessionNV.html>
    pub unsafe fn create_optical_flow_session_nv(
        &self,
        device: Device,
        create_info: &OpticalFlowSessionCreateInfoNV<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<OpticalFlowSessionNV> {
        unsafe {
            let mut session = core::mem::MaybeUninit::uninit();
            let result = (self.create_optical_flow_session_nv)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                session.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(session.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyOpticalFlowSessionNV.html>
    pub unsafe fn destroy_optical_flow_session_nv(
        &self,
        device: Device,
        session: OpticalFlowSessionNV,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_optical_flow_session_nv)(device, session, allocator.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkBindOpticalFlowSessionImageNV.html>
    pub unsafe fn bind_optical_flow_session_image_nv(
        &self,
        device: Device,
        session: OpticalFlowSessionNV,
        binding_point: OpticalFlowSessionBindingPointNV,
        view: ImageView,
        layout: ImageLayout,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.bind_optical_flow_session_image_nv)(
                device,
                session,
                binding_point,
                view,
                layout,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdOpticalFlowExecuteNV.html>
    pub unsafe fn cmd_optical_flow_execute_nv(
        &self,
        command_buffer: CommandBuffer,
        session: OpticalFlowSessionNV,
        execute_info: &OpticalFlowExecuteInfoNV<'_>,
    ) {
        unsafe { (self.cmd_optical_flow_execute_nv)(command_buffer, session, execute_info) }
    }
}

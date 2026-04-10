//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_ARM_data_graph_optical_flow.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_ARM_data_graph_optical_flow";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphPipelineResourceInfoImageLayoutARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DataGraphPipelineResourceInfoImageLayoutARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub layout: ImageLayout,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DataGraphPipelineResourceInfoImageLayoutARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DataGraphPipelineResourceInfoImageLayoutARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("layout", &self.layout)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineResourceInfoImageLayoutARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_RESOURCE_INFO_IMAGE_LAYOUT_ARM;
    }

    unsafe impl Extends<DataGraphPipelineResourceInfoARM<'_>>
        for DataGraphPipelineResourceInfoImageLayoutARM<'_>
    {
    }

    impl Default for DataGraphPipelineResourceInfoImageLayoutARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DataGraphPipelineResourceInfoImageLayoutARM<'a> {
        #[inline]
        pub fn layout(mut self, layout: ImageLayout) -> Self {
            self.layout = layout;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphPipelineSingleNodeConnectionARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DataGraphPipelineSingleNodeConnectionARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub set: u32,
        pub binding: u32,
        pub connection: DataGraphPipelineNodeConnectionTypeARM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DataGraphPipelineSingleNodeConnectionARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DataGraphPipelineSingleNodeConnectionARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("set", &self.set)
                .field("binding", &self.binding)
                .field("connection", &self.connection)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineSingleNodeConnectionARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_SINGLE_NODE_CONNECTION_ARM;
    }

    impl Default for DataGraphPipelineSingleNodeConnectionARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                set: Default::default(),
                binding: Default::default(),
                connection: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DataGraphPipelineSingleNodeConnectionARM<'a> {
        #[inline]
        pub fn set(mut self, set: u32) -> Self {
            self.set = set;
            self
        }

        #[inline]
        pub fn binding(mut self, binding: u32) -> Self {
            self.binding = binding;
            self
        }

        #[inline]
        pub fn connection(mut self, connection: DataGraphPipelineNodeConnectionTypeARM) -> Self {
            self.connection = connection;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDataGraphOpticalFlowFeaturesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub data_graph_optical_flow: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDataGraphOpticalFlowFeaturesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDataGraphOpticalFlowFeaturesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("data_graph_optical_flow", &self.data_graph_optical_flow)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDataGraphOpticalFlowFeaturesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DATA_GRAPH_OPTICAL_FLOW_FEATURES_ARM;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceDataGraphOpticalFlowFeaturesARM<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceDataGraphOpticalFlowFeaturesARM<'_> {}

    impl Default for PhysicalDeviceDataGraphOpticalFlowFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                data_graph_optical_flow: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDataGraphOpticalFlowFeaturesARM<'a> {
        #[inline]
        pub fn data_graph_optical_flow(mut self, data_graph_optical_flow: bool) -> Self {
            self.data_graph_optical_flow = data_graph_optical_flow.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueFamilyDataGraphOpticalFlowPropertiesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct QueueFamilyDataGraphOpticalFlowPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub supported_output_grid_sizes: DataGraphOpticalFlowGridSizeFlagsARM,
        pub supported_hint_grid_sizes: DataGraphOpticalFlowGridSizeFlagsARM,
        pub hint_supported: Bool32,
        pub cost_supported: Bool32,
        pub min_width: u32,
        pub min_height: u32,
        pub max_width: u32,
        pub max_height: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for QueueFamilyDataGraphOpticalFlowPropertiesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("QueueFamilyDataGraphOpticalFlowPropertiesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "supported_output_grid_sizes",
                    &self.supported_output_grid_sizes,
                )
                .field("supported_hint_grid_sizes", &self.supported_hint_grid_sizes)
                .field("hint_supported", &self.hint_supported)
                .field("cost_supported", &self.cost_supported)
                .field("min_width", &self.min_width)
                .field("min_height", &self.min_height)
                .field("max_width", &self.max_width)
                .field("max_height", &self.max_height)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for QueueFamilyDataGraphOpticalFlowPropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::QUEUE_FAMILY_DATA_GRAPH_OPTICAL_FLOW_PROPERTIES_ARM;
    }

    impl Default for QueueFamilyDataGraphOpticalFlowPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                supported_output_grid_sizes: Default::default(),
                supported_hint_grid_sizes: Default::default(),
                hint_supported: Default::default(),
                cost_supported: Default::default(),
                min_width: Default::default(),
                min_height: Default::default(),
                max_width: Default::default(),
                max_height: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> QueueFamilyDataGraphOpticalFlowPropertiesARM<'a> {
        #[inline]
        pub fn supported_output_grid_sizes(
            mut self,
            supported_output_grid_sizes: DataGraphOpticalFlowGridSizeFlagsARM,
        ) -> Self {
            self.supported_output_grid_sizes = supported_output_grid_sizes;
            self
        }

        #[inline]
        pub fn supported_hint_grid_sizes(
            mut self,
            supported_hint_grid_sizes: DataGraphOpticalFlowGridSizeFlagsARM,
        ) -> Self {
            self.supported_hint_grid_sizes = supported_hint_grid_sizes;
            self
        }

        #[inline]
        pub fn hint_supported(mut self, hint_supported: bool) -> Self {
            self.hint_supported = hint_supported.into();
            self
        }

        #[inline]
        pub fn cost_supported(mut self, cost_supported: bool) -> Self {
            self.cost_supported = cost_supported.into();
            self
        }

        #[inline]
        pub fn min_width(mut self, min_width: u32) -> Self {
            self.min_width = min_width;
            self
        }

        #[inline]
        pub fn min_height(mut self, min_height: u32) -> Self {
            self.min_height = min_height;
            self
        }

        #[inline]
        pub fn max_width(mut self, max_width: u32) -> Self {
            self.max_width = max_width;
            self
        }

        #[inline]
        pub fn max_height(mut self, max_height: u32) -> Self {
            self.max_height = max_height;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphOpticalFlowImageFormatInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DataGraphOpticalFlowImageFormatInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub usage: DataGraphOpticalFlowImageUsageFlagsARM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DataGraphOpticalFlowImageFormatInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DataGraphOpticalFlowImageFormatInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("usage", &self.usage)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DataGraphOpticalFlowImageFormatInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_INFO_ARM;
    }

    unsafe impl Extends<PhysicalDeviceImageFormatInfo2<'_>>
        for DataGraphOpticalFlowImageFormatInfoARM<'_>
    {
    }
    unsafe impl Extends<ImageCreateInfo<'_>> for DataGraphOpticalFlowImageFormatInfoARM<'_> {}

    impl Default for DataGraphOpticalFlowImageFormatInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DataGraphOpticalFlowImageFormatInfoARM<'a> {
        #[inline]
        pub fn usage(mut self, usage: DataGraphOpticalFlowImageUsageFlagsARM) -> Self {
            self.usage = usage;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphOpticalFlowImageFormatPropertiesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DataGraphOpticalFlowImageFormatPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub format: Format,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DataGraphOpticalFlowImageFormatPropertiesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DataGraphOpticalFlowImageFormatPropertiesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("format", &self.format)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DataGraphOpticalFlowImageFormatPropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_ARM;
    }

    impl Default for DataGraphOpticalFlowImageFormatPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                format: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DataGraphOpticalFlowImageFormatPropertiesARM<'a> {
        #[inline]
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphPipelineSingleNodeCreateInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DataGraphPipelineSingleNodeCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub node_type: DataGraphPipelineNodeTypeARM,
        pub connection_count: u32,
        pub p_connections: *const DataGraphPipelineSingleNodeConnectionARM<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DataGraphPipelineSingleNodeCreateInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DataGraphPipelineSingleNodeCreateInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("node_type", &self.node_type)
                .field("connection_count", &self.connection_count)
                .field("p_connections", &self.p_connections)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineSingleNodeCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_SINGLE_NODE_CREATE_INFO_ARM;
    }

    unsafe impl Extends<DataGraphPipelineCreateInfoARM<'_>>
        for DataGraphPipelineSingleNodeCreateInfoARM<'_>
    {
    }

    impl Default for DataGraphPipelineSingleNodeCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                node_type: Default::default(),
                connection_count: Default::default(),
                p_connections: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DataGraphPipelineSingleNodeCreateInfoARM<'a> {
        #[inline]
        pub fn node_type(mut self, node_type: DataGraphPipelineNodeTypeARM) -> Self {
            self.node_type = node_type;
            self
        }

        #[inline]
        pub fn connections(
            mut self,
            connections: &'a [DataGraphPipelineSingleNodeConnectionARM<'_>],
        ) -> Self {
            self.connection_count = connections.len().try_into().unwrap();
            self.p_connections = connections.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphPipelineOpticalFlowCreateInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DataGraphPipelineOpticalFlowCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub width: u32,
        pub height: u32,
        pub image_format: Format,
        pub flow_vector_format: Format,
        pub cost_format: Format,
        pub output_grid_size: DataGraphOpticalFlowGridSizeFlagsARM,
        pub hint_grid_size: DataGraphOpticalFlowGridSizeFlagsARM,
        pub performance_level: DataGraphOpticalFlowPerformanceLevelARM,
        pub flags: DataGraphOpticalFlowCreateFlagsARM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DataGraphPipelineOpticalFlowCreateInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DataGraphPipelineOpticalFlowCreateInfoARM")
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

    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineOpticalFlowCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_OPTICAL_FLOW_CREATE_INFO_ARM;
    }

    unsafe impl Extends<DataGraphPipelineCreateInfoARM<'_>>
        for DataGraphPipelineOpticalFlowCreateInfoARM<'_>
    {
    }

    impl Default for DataGraphPipelineOpticalFlowCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
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

    impl<'a> DataGraphPipelineOpticalFlowCreateInfoARM<'a> {
        #[inline]
        pub fn width(mut self, width: u32) -> Self {
            self.width = width;
            self
        }

        #[inline]
        pub fn height(mut self, height: u32) -> Self {
            self.height = height;
            self
        }

        #[inline]
        pub fn image_format(mut self, image_format: Format) -> Self {
            self.image_format = image_format;
            self
        }

        #[inline]
        pub fn flow_vector_format(mut self, flow_vector_format: Format) -> Self {
            self.flow_vector_format = flow_vector_format;
            self
        }

        #[inline]
        pub fn cost_format(mut self, cost_format: Format) -> Self {
            self.cost_format = cost_format;
            self
        }

        #[inline]
        pub fn output_grid_size(
            mut self,
            output_grid_size: DataGraphOpticalFlowGridSizeFlagsARM,
        ) -> Self {
            self.output_grid_size = output_grid_size;
            self
        }

        #[inline]
        pub fn hint_grid_size(
            mut self,
            hint_grid_size: DataGraphOpticalFlowGridSizeFlagsARM,
        ) -> Self {
            self.hint_grid_size = hint_grid_size;
            self
        }

        #[inline]
        pub fn performance_level(
            mut self,
            performance_level: DataGraphOpticalFlowPerformanceLevelARM,
        ) -> Self {
            self.performance_level = performance_level;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: DataGraphOpticalFlowCreateFlagsARM) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphPipelineOpticalFlowDispatchInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DataGraphPipelineOpticalFlowDispatchInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: DataGraphOpticalFlowExecuteFlagsARM,
        pub mean_flow_l1_norm_hint: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DataGraphPipelineOpticalFlowDispatchInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DataGraphPipelineOpticalFlowDispatchInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("mean_flow_l1_norm_hint", &self.mean_flow_l1_norm_hint)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineOpticalFlowDispatchInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_OPTICAL_FLOW_DISPATCH_INFO_ARM;
    }

    unsafe impl Extends<DataGraphPipelineDispatchInfoARM<'_>>
        for DataGraphPipelineOpticalFlowDispatchInfoARM<'_>
    {
    }

    impl Default for DataGraphPipelineOpticalFlowDispatchInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                flags: Default::default(),
                mean_flow_l1_norm_hint: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DataGraphPipelineOpticalFlowDispatchInfoARM<'a> {
        #[inline]
        pub fn flags(mut self, flags: DataGraphOpticalFlowExecuteFlagsARM) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn mean_flow_l1_norm_hint(mut self, mean_flow_l1_norm_hint: u32) -> Self {
            self.mean_flow_l1_norm_hint = mean_flow_l1_norm_hint;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphOpticalFlowPerformanceLevelARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataGraphOpticalFlowPerformanceLevelARM(i32);

    impl DataGraphOpticalFlowPerformanceLevelARM {
        pub const UNKNOWN_ARM: Self = Self(0);
        pub const SLOW_ARM: Self = Self(1);
        pub const MEDIUM_ARM: Self = Self(2);
        pub const FAST_ARM: Self = Self(3);
    }

    impl fmt::Debug for DataGraphOpticalFlowPerformanceLevelARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::UNKNOWN_ARM => Some("UNKNOWN_ARM"),
                Self::SLOW_ARM => Some("SLOW_ARM"),
                Self::MEDIUM_ARM => Some("MEDIUM_ARM"),
                Self::FAST_ARM => Some("FAST_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphPipelineNodeConnectionTypeARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataGraphPipelineNodeConnectionTypeARM(i32);

    impl DataGraphPipelineNodeConnectionTypeARM {
        // VK_ARM_data_graph_optical_flow
        pub const OPTICAL_FLOW_INPUT_ARM: Self = Self(1000631000);
        pub const OPTICAL_FLOW_REFERENCE_ARM: Self = Self(1000631001);
        pub const OPTICAL_FLOW_HINT_ARM: Self = Self(1000631002);
        pub const OPTICAL_FLOW_FLOW_VECTOR_ARM: Self = Self(1000631003);
        pub const OPTICAL_FLOW_COST_ARM: Self = Self(1000631004);
    }

    impl fmt::Debug for DataGraphPipelineNodeConnectionTypeARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::OPTICAL_FLOW_INPUT_ARM => Some("OPTICAL_FLOW_INPUT_ARM"),
                Self::OPTICAL_FLOW_REFERENCE_ARM => Some("OPTICAL_FLOW_REFERENCE_ARM"),
                Self::OPTICAL_FLOW_HINT_ARM => Some("OPTICAL_FLOW_HINT_ARM"),
                Self::OPTICAL_FLOW_FLOW_VECTOR_ARM => Some("OPTICAL_FLOW_FLOW_VECTOR_ARM"),
                Self::OPTICAL_FLOW_COST_ARM => Some("OPTICAL_FLOW_COST_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphPipelineNodeTypeARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataGraphPipelineNodeTypeARM(i32);

    impl DataGraphPipelineNodeTypeARM {
        // VK_ARM_data_graph_optical_flow
        pub const OPTICAL_FLOW_ARM: Self = Self(1000631000);
    }

    impl fmt::Debug for DataGraphPipelineNodeTypeARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::OPTICAL_FLOW_ARM => Some("OPTICAL_FLOW_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphOpticalFlowGridSizeFlagsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DataGraphOpticalFlowGridSizeFlagsARM(Flags);
    vk_bitflags_wrapped!(
        DataGraphOpticalFlowGridSizeFlagsARM,
        Flags,
        DataGraphOpticalFlowGridSizeFlagBitsARM
    );

    impl DataGraphOpticalFlowGridSizeFlagsARM {
        pub const UNKNOWN: Self = Self(0);
    }

    impl fmt::Debug for DataGraphOpticalFlowGridSizeFlagsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    DataGraphOpticalFlowGridSizeFlagBitsARM::_1X1_ARM.0,
                    "_1X1_ARM",
                ),
                (
                    DataGraphOpticalFlowGridSizeFlagBitsARM::_2X2_ARM.0,
                    "_2X2_ARM",
                ),
                (
                    DataGraphOpticalFlowGridSizeFlagBitsARM::_4X4_ARM.0,
                    "_4X4_ARM",
                ),
                (
                    DataGraphOpticalFlowGridSizeFlagBitsARM::_8X8_ARM.0,
                    "_8X8_ARM",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphOpticalFlowGridSizeFlagBitsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DataGraphOpticalFlowGridSizeFlagBitsARM(u32);

    impl DataGraphOpticalFlowGridSizeFlagBitsARM {
        pub const _1X1_ARM: Self = Self(1 << 0);
        pub const _2X2_ARM: Self = Self(1 << 1);
        pub const _4X4_ARM: Self = Self(1 << 2);
        pub const _8X8_ARM: Self = Self(1 << 3);
    }

    impl fmt::Debug for DataGraphOpticalFlowGridSizeFlagBitsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_1X1_ARM => Some("_1X1_ARM"),
                Self::_2X2_ARM => Some("_2X2_ARM"),
                Self::_4X4_ARM => Some("_4X4_ARM"),
                Self::_8X8_ARM => Some("_8X8_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphOpticalFlowImageUsageFlagsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DataGraphOpticalFlowImageUsageFlagsARM(Flags);
    vk_bitflags_wrapped!(
        DataGraphOpticalFlowImageUsageFlagsARM,
        Flags,
        DataGraphOpticalFlowImageUsageFlagBitsARM
    );

    impl DataGraphOpticalFlowImageUsageFlagsARM {
        pub const UNKNOWN: Self = Self(0);
    }

    impl fmt::Debug for DataGraphOpticalFlowImageUsageFlagsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    DataGraphOpticalFlowImageUsageFlagBitsARM::INPUT_ARM.0,
                    "INPUT_ARM",
                ),
                (
                    DataGraphOpticalFlowImageUsageFlagBitsARM::OUTPUT_ARM.0,
                    "OUTPUT_ARM",
                ),
                (
                    DataGraphOpticalFlowImageUsageFlagBitsARM::HINT_ARM.0,
                    "HINT_ARM",
                ),
                (
                    DataGraphOpticalFlowImageUsageFlagBitsARM::COST_ARM.0,
                    "COST_ARM",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphOpticalFlowImageUsageFlagBitsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DataGraphOpticalFlowImageUsageFlagBitsARM(u32);

    impl DataGraphOpticalFlowImageUsageFlagBitsARM {
        pub const INPUT_ARM: Self = Self(1 << 0);
        pub const OUTPUT_ARM: Self = Self(1 << 1);
        pub const HINT_ARM: Self = Self(1 << 2);
        pub const COST_ARM: Self = Self(1 << 3);
    }

    impl fmt::Debug for DataGraphOpticalFlowImageUsageFlagBitsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::INPUT_ARM => Some("INPUT_ARM"),
                Self::OUTPUT_ARM => Some("OUTPUT_ARM"),
                Self::HINT_ARM => Some("HINT_ARM"),
                Self::COST_ARM => Some("COST_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphOpticalFlowCreateFlagsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DataGraphOpticalFlowCreateFlagsARM(Flags);
    vk_bitflags_wrapped!(
        DataGraphOpticalFlowCreateFlagsARM,
        Flags,
        DataGraphOpticalFlowCreateFlagBitsARM
    );

    impl fmt::Debug for DataGraphOpticalFlowCreateFlagsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    DataGraphOpticalFlowCreateFlagBitsARM::ENABLE_HINT_ARM.0,
                    "ENABLE_HINT_ARM",
                ),
                (
                    DataGraphOpticalFlowCreateFlagBitsARM::ENABLE_COST_ARM.0,
                    "ENABLE_COST_ARM",
                ),
                (
                    DataGraphOpticalFlowCreateFlagBitsARM::RESERVED_30_ARM.0,
                    "RESERVED_30_ARM",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphOpticalFlowCreateFlagBitsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DataGraphOpticalFlowCreateFlagBitsARM(u32);

    impl DataGraphOpticalFlowCreateFlagBitsARM {
        pub const ENABLE_HINT_ARM: Self = Self(1 << 0);
        pub const ENABLE_COST_ARM: Self = Self(1 << 1);
        pub const RESERVED_30_ARM: Self = Self(1 << 30);
    }

    impl fmt::Debug for DataGraphOpticalFlowCreateFlagBitsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ENABLE_HINT_ARM => Some("ENABLE_HINT_ARM"),
                Self::ENABLE_COST_ARM => Some("ENABLE_COST_ARM"),
                Self::RESERVED_30_ARM => Some("RESERVED_30_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphOpticalFlowExecuteFlagsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct DataGraphOpticalFlowExecuteFlagsARM(Flags);
    vk_bitflags_wrapped!(
        DataGraphOpticalFlowExecuteFlagsARM,
        Flags,
        DataGraphOpticalFlowExecuteFlagBitsARM
    );

    impl fmt::Debug for DataGraphOpticalFlowExecuteFlagsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    DataGraphOpticalFlowExecuteFlagBitsARM::DISABLE_TEMPORAL_HINTS_ARM.0,
                    "DISABLE_TEMPORAL_HINTS_ARM",
                ),
                (
                    DataGraphOpticalFlowExecuteFlagBitsARM::INPUT_UNCHANGED_ARM.0,
                    "INPUT_UNCHANGED_ARM",
                ),
                (
                    DataGraphOpticalFlowExecuteFlagBitsARM::REFERENCE_UNCHANGED_ARM.0,
                    "REFERENCE_UNCHANGED_ARM",
                ),
                (
                    DataGraphOpticalFlowExecuteFlagBitsARM::INPUT_IS_PREVIOUS_REFERENCE_ARM.0,
                    "INPUT_IS_PREVIOUS_REFERENCE_ARM",
                ),
                (
                    DataGraphOpticalFlowExecuteFlagBitsARM::REFERENCE_IS_PREVIOUS_INPUT_ARM.0,
                    "REFERENCE_IS_PREVIOUS_INPUT_ARM",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphOpticalFlowExecuteFlagBitsARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct DataGraphOpticalFlowExecuteFlagBitsARM(u32);

    impl DataGraphOpticalFlowExecuteFlagBitsARM {
        pub const DISABLE_TEMPORAL_HINTS_ARM: Self = Self(1 << 0);
        pub const INPUT_UNCHANGED_ARM: Self = Self(1 << 1);
        pub const REFERENCE_UNCHANGED_ARM: Self = Self(1 << 2);
        pub const INPUT_IS_PREVIOUS_REFERENCE_ARM: Self = Self(1 << 3);
        pub const REFERENCE_IS_PREVIOUS_INPUT_ARM: Self = Self(1 << 4);
    }

    impl fmt::Debug for DataGraphOpticalFlowExecuteFlagBitsARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DISABLE_TEMPORAL_HINTS_ARM => Some("DISABLE_TEMPORAL_HINTS_ARM"),
                Self::INPUT_UNCHANGED_ARM => Some("INPUT_UNCHANGED_ARM"),
                Self::REFERENCE_UNCHANGED_ARM => Some("REFERENCE_UNCHANGED_ARM"),
                Self::INPUT_IS_PREVIOUS_REFERENCE_ARM => Some("INPUT_IS_PREVIOUS_REFERENCE_ARM"),
                Self::REFERENCE_IS_PREVIOUS_INPUT_ARM => Some("REFERENCE_IS_PREVIOUS_INPUT_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM.html>
    pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
            p_queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM<'_>,
            p_optical_flow_image_format_info: *const DataGraphOpticalFlowImageFormatInfoARM<'_>,
            p_format_count: *mut u32,
            p_image_format_properties: *mut DataGraphOpticalFlowImageFormatPropertiesARM<'_>,
        ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkDataGraphPipelineResourceInfoImageLayoutARM =
        DataGraphPipelineResourceInfoImageLayoutARM<'static>;
    pub type VkDataGraphPipelineSingleNodeConnectionARM =
        DataGraphPipelineSingleNodeConnectionARM<'static>;
    pub type VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM =
        PhysicalDeviceDataGraphOpticalFlowFeaturesARM<'static>;
    pub type VkQueueFamilyDataGraphOpticalFlowPropertiesARM =
        QueueFamilyDataGraphOpticalFlowPropertiesARM<'static>;
    pub type VkDataGraphOpticalFlowImageFormatInfoARM =
        DataGraphOpticalFlowImageFormatInfoARM<'static>;
    pub type VkDataGraphOpticalFlowImageFormatPropertiesARM =
        DataGraphOpticalFlowImageFormatPropertiesARM<'static>;
    pub type VkDataGraphPipelineSingleNodeCreateInfoARM =
        DataGraphPipelineSingleNodeCreateInfoARM<'static>;
    pub type VkDataGraphPipelineOpticalFlowCreateInfoARM =
        DataGraphPipelineOpticalFlowCreateInfoARM<'static>;
    pub type VkDataGraphPipelineOpticalFlowDispatchInfoARM =
        DataGraphPipelineOpticalFlowDispatchInfoARM<'static>;
    pub type VkDataGraphOpticalFlowPerformanceLevelARM = DataGraphOpticalFlowPerformanceLevelARM;
    pub type VkDataGraphPipelineNodeConnectionTypeARM = DataGraphPipelineNodeConnectionTypeARM;
    pub type VkDataGraphPipelineNodeTypeARM = DataGraphPipelineNodeTypeARM;
    pub type VkDataGraphOpticalFlowGridSizeFlagsARM = DataGraphOpticalFlowGridSizeFlagsARM;
    pub type VkDataGraphOpticalFlowGridSizeFlagBitsARM = DataGraphOpticalFlowGridSizeFlagBitsARM;
    pub type VkDataGraphOpticalFlowImageUsageFlagsARM = DataGraphOpticalFlowImageUsageFlagsARM;
    pub type VkDataGraphOpticalFlowImageUsageFlagBitsARM =
        DataGraphOpticalFlowImageUsageFlagBitsARM;
    pub type VkDataGraphOpticalFlowCreateFlagsARM = DataGraphOpticalFlowCreateFlagsARM;
    pub type VkDataGraphOpticalFlowCreateFlagBitsARM = DataGraphOpticalFlowCreateFlagBitsARM;
    pub type VkDataGraphOpticalFlowExecuteFlagsARM = DataGraphOpticalFlowExecuteFlagsARM;
    pub type VkDataGraphOpticalFlowExecuteFlagBitsARM = DataGraphOpticalFlowExecuteFlagBitsARM;
    impl DataGraphPipelineResourceInfoImageLayoutARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkDataGraphPipelineResourceInfoImageLayoutARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DataGraphPipelineSingleNodeConnectionARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDataGraphPipelineSingleNodeConnectionARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceDataGraphOpticalFlowFeaturesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl QueueFamilyDataGraphOpticalFlowPropertiesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkQueueFamilyDataGraphOpticalFlowPropertiesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DataGraphOpticalFlowImageFormatInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDataGraphOpticalFlowImageFormatInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DataGraphOpticalFlowImageFormatPropertiesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkDataGraphOpticalFlowImageFormatPropertiesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DataGraphPipelineSingleNodeCreateInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDataGraphPipelineSingleNodeCreateInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DataGraphPipelineOpticalFlowCreateInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDataGraphPipelineOpticalFlowCreateInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DataGraphPipelineOpticalFlowDispatchInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkDataGraphPipelineOpticalFlowDispatchInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    get_physical_device_queue_family_data_graph_optical_flow_image_formats:
        PFN_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM,
    get_physical_device_queue_family_data_graph_engine_operation_properties:
        PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_queue_family_data_graph_optical_flow_image_formats: transmute(
                    load(c"vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_physical_device_queue_family_data_graph_engine_operation_properties: transmute(
                    load(c"vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM.html>
    #[inline]
    pub unsafe fn get_physical_device_queue_family_data_graph_optical_flow_image_formats<'a>(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM<'a>,
        optical_flow_image_format_info: &DataGraphOpticalFlowImageFormatInfoARM<'a>,
        mut image_format_properties: impl EnumerateInto<
            DataGraphOpticalFlowImageFormatPropertiesARM<'a>,
        >,
    ) -> crate::Result<()> {
        unsafe {
            let call = |format_count, image_format_properties| {
                let result = (self
                    .get_physical_device_queue_family_data_graph_optical_flow_image_formats)(
                    physical_device,
                    queue_family_index,
                    queue_family_data_graph_properties,
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM.html>
    #[inline]
    pub unsafe fn get_physical_device_queue_family_data_graph_engine_operation_properties(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM<'_>,
        properties: &mut BaseOutStructure<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self
                .get_physical_device_queue_family_data_graph_engine_operation_properties)(
                physical_device,
                queue_family_index,
                queue_family_data_graph_properties,
                properties,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}

//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_ARM_data_graph_neural_accelerator_statistics.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_ARM_data_graph_neural_accelerator_statistics";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub data_graph_neural_accelerator_statistics: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "data_graph_neural_accelerator_statistics",
                    &self.data_graph_neural_accelerator_statistics,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_FEATURES_ARM;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'_>
    {
    }

    impl Default for PhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                data_graph_neural_accelerator_statistics: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'a> {
        #[inline]
        pub fn data_graph_neural_accelerator_statistics(
            mut self,
            data_graph_neural_accelerator_statistics: bool,
        ) -> Self {
            self.data_graph_neural_accelerator_statistics =
                data_graph_neural_accelerator_statistics.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphPipelineNeuralStatisticsCreateInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DataGraphPipelineNeuralStatisticsCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub allow_neural_statistics: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DataGraphPipelineNeuralStatisticsCreateInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DataGraphPipelineNeuralStatisticsCreateInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("allow_neural_statistics", &self.allow_neural_statistics)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineNeuralStatisticsCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_NEURAL_STATISTICS_CREATE_INFO_ARM;
    }

    unsafe impl Extends<DataGraphPipelineCreateInfoARM<'_>>
        for DataGraphPipelineNeuralStatisticsCreateInfoARM<'_>
    {
    }

    impl Default for DataGraphPipelineNeuralStatisticsCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                allow_neural_statistics: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DataGraphPipelineNeuralStatisticsCreateInfoARM<'a> {
        #[inline]
        pub fn allow_neural_statistics(mut self, allow_neural_statistics: bool) -> Self {
            self.allow_neural_statistics = allow_neural_statistics.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DataGraphPipelineSessionNeuralStatisticsCreateInfoARM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub mode: NeuralAcceleratorStatisticsModeARM,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DataGraphPipelineSessionNeuralStatisticsCreateInfoARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DataGraphPipelineSessionNeuralStatisticsCreateInfoARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("mode", &self.mode)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DataGraphPipelineSessionNeuralStatisticsCreateInfoARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DATA_GRAPH_PIPELINE_SESSION_NEURAL_STATISTICS_CREATE_INFO_ARM;
    }

    unsafe impl Extends<DataGraphPipelineSessionCreateInfoARM<'_>>
        for DataGraphPipelineSessionNeuralStatisticsCreateInfoARM<'_>
    {
    }

    impl Default for DataGraphPipelineSessionNeuralStatisticsCreateInfoARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DataGraphPipelineSessionNeuralStatisticsCreateInfoARM<'a> {
        #[inline]
        pub fn mode(mut self, mode: NeuralAcceleratorStatisticsModeARM) -> Self {
            self.mode = mode;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkNeuralAcceleratorStatisticsModeARM.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct NeuralAcceleratorStatisticsModeARM(i32);

    impl NeuralAcceleratorStatisticsModeARM {
        pub const DISABLED_ARM: Self = Self(0);
        pub const STATISTICS0_ARM: Self = Self(1);
        pub const STATISTICS1_ARM: Self = Self(2);
    }

    impl fmt::Debug for NeuralAcceleratorStatisticsModeARM {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DISABLED_ARM => Some("DISABLED_ARM"),
                Self::STATISTICS0_ARM => Some("STATISTICS0_ARM"),
                Self::STATISTICS1_ARM => Some("STATISTICS1_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM =
        PhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'static>;
    pub type VkDataGraphPipelineNeuralStatisticsCreateInfoARM =
        DataGraphPipelineNeuralStatisticsCreateInfoARM<'static>;
    pub type VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM =
        DataGraphPipelineSessionNeuralStatisticsCreateInfoARM<'static>;
    pub type VkNeuralAcceleratorStatisticsModeARM = NeuralAcceleratorStatisticsModeARM;
    impl PhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DataGraphPipelineNeuralStatisticsCreateInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkDataGraphPipelineNeuralStatisticsCreateInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DataGraphPipelineSessionNeuralStatisticsCreateInfoARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM {
            unsafe { core::mem::transmute(self) }
        }
    }
}

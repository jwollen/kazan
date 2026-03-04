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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_executable_info: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'a>
    {
    }
    impl Default for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                pipeline_executable_info: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'a> {
        pub fn pipeline_executable_info(mut self, pipeline_executable_info: Bool32) -> Self {
            self.pipeline_executable_info = pipeline_executable_info;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub pipeline: Pipeline,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_INFO_KHR;
    }
    impl Default for PipelineInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                pipeline: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineInfoKHR<'a> {
        pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
            self.pipeline = pipeline;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineExecutablePropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub stages: ShaderStageFlags,
        pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
        pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
        pub subgroup_size: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineExecutablePropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_EXECUTABLE_PROPERTIES_KHR;
    }
    impl Default for PipelineExecutablePropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                stages: Default::default(),
                name: [Default::default(); _],
                description: [Default::default(); _],
                subgroup_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineExecutablePropertiesKHR<'a> {
        pub fn stages(mut self, stages: ShaderStageFlags) -> Self {
            self.stages = stages;
            self
        }
        pub fn subgroup_size(mut self, subgroup_size: u32) -> Self {
            self.subgroup_size = subgroup_size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineExecutableInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub pipeline: Pipeline,
        pub executable_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineExecutableInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_EXECUTABLE_INFO_KHR;
    }
    impl Default for PipelineExecutableInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                pipeline: Default::default(),
                executable_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineExecutableInfoKHR<'a> {
        pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
            self.pipeline = pipeline;
            self
        }
        pub fn executable_index(mut self, executable_index: u32) -> Self {
            self.executable_index = executable_index;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineExecutableStatisticKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
        pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
        pub format: PipelineExecutableStatisticFormatKHR,
        pub value: PipelineExecutableStatisticValueKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineExecutableStatisticKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_EXECUTABLE_STATISTIC_KHR;
    }
    impl Default for PipelineExecutableStatisticKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                name: [Default::default(); _],
                description: [Default::default(); _],
                format: Default::default(),
                value: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineExecutableStatisticKHR<'a> {
        pub fn format(mut self, format: PipelineExecutableStatisticFormatKHR) -> Self {
            self.format = format;
            self
        }
        pub fn value(mut self, value: PipelineExecutableStatisticValueKHR) -> Self {
            self.value = value;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineExecutableInternalRepresentationKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
        pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
        pub is_text: Bool32,
        pub data_size: usize,
        pub p_data: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineExecutableInternalRepresentationKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR;
    }
    impl Default for PipelineExecutableInternalRepresentationKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                name: [Default::default(); _],
                description: [Default::default(); _],
                is_text: Default::default(),
                data_size: Default::default(),
                p_data: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineExecutableInternalRepresentationKHR<'a> {
        pub fn is_text(mut self, is_text: Bool32) -> Self {
            self.is_text = is_text;
            self
        }
        pub fn data(mut self, data: &'a mut [u8]) -> Self {
            self.data_size = data.len().try_into().unwrap();
            self.p_data = data.as_mut_ptr() as _;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union PipelineExecutableStatisticValueKHR {
        pub b32: Bool32,
        pub i64: i64,
        pub u64: u64,
        pub f64: f64,
    }
    impl Default for PipelineExecutableStatisticValueKHR {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineExecutableStatisticFormatKHR(i32);
    impl PipelineExecutableStatisticFormatKHR {
        pub const BOOL32_KHR: Self = Self(0);
        pub const INT64_KHR: Self = Self(1);
        pub const UINT64_KHR: Self = Self(2);
        pub const FLOAT64_KHR: Self = Self(3);
    }
    impl fmt::Debug for PipelineExecutableStatisticFormatKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::BOOL32_KHR => Some("BOOL32_KHR"),
                Self::INT64_KHR => Some("INT64_KHR"),
                Self::UINT64_KHR => Some("UINT64_KHR"),
                Self::FLOAT64_KHR => Some("FLOAT64_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
    pub type PFN_vkGetPipelineExecutablePropertiesKHR = unsafe extern "system" fn(
        device: Device,
        p_pipeline_info: *const PipelineInfoKHR<'_>,
        p_executable_count: *mut u32,
        p_properties: *mut PipelineExecutablePropertiesKHR<'_>,
    )
        -> vk::Result;
    pub type PFN_vkGetPipelineExecutableStatisticsKHR = unsafe extern "system" fn(
        device: Device,
        p_executable_info: *const PipelineExecutableInfoKHR<'_>,
        p_statistic_count: *mut u32,
        p_statistics: *mut PipelineExecutableStatisticKHR<'_>,
    )
        -> vk::Result;
    pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR =
        unsafe extern "system" fn(
            device: Device,
            p_executable_info: *const PipelineExecutableInfoKHR<'_>,
            p_internal_representation_count: *mut u32,
            p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR<'_>,
        ) -> vk::Result;
}
pub struct DeviceFn {
    get_pipeline_executable_properties_khr: PFN_vkGetPipelineExecutablePropertiesKHR,
    get_pipeline_executable_statistics_khr: PFN_vkGetPipelineExecutableStatisticsKHR,
    get_pipeline_executable_internal_representations_khr:
        PFN_vkGetPipelineExecutableInternalRepresentationsKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_pipeline_executable_properties_khr: transmute(
                    load(c"vkGetPipelineExecutablePropertiesKHR").ok_or(MissingEntryPointError)?,
                ),
                get_pipeline_executable_statistics_khr: transmute(
                    load(c"vkGetPipelineExecutableStatisticsKHR").ok_or(MissingEntryPointError)?,
                ),
                get_pipeline_executable_internal_representations_khr: transmute(
                    load(c"vkGetPipelineExecutableInternalRepresentationsKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_pipeline_executable_properties_khr<'a>(
        &self,
        device: Device,
        pipeline_info: &PipelineInfoKHR<'_>,
        properties: impl ExtendUninit<PipelineExecutablePropertiesKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |executable_count, properties| {
                let result = (self.get_pipeline_executable_properties_khr)(
                    device,
                    pipeline_info,
                    executable_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn get_pipeline_executable_statistics_khr<'a>(
        &self,
        device: Device,
        executable_info: &PipelineExecutableInfoKHR<'_>,
        statistics: impl ExtendUninit<PipelineExecutableStatisticKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(statistics, |statistic_count, statistics| {
                let result = (self.get_pipeline_executable_statistics_khr)(
                    device,
                    executable_info,
                    statistic_count,
                    statistics as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn get_pipeline_executable_internal_representations_khr<'a>(
        &self,
        device: Device,
        executable_info: &PipelineExecutableInfoKHR<'_>,
        internal_representations: impl ExtendUninit<PipelineExecutableInternalRepresentationKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(
                internal_representations,
                |internal_representation_count, internal_representations| {
                    let result = (self.get_pipeline_executable_internal_representations_khr)(
                        device,
                        executable_info,
                        internal_representation_count,
                        internal_representations as _,
                    );

                    match result {
                        VkResult::SUCCESS => Ok(()),
                        VkResult::INCOMPLETE => Ok(()),
                        err => Err(err),
                    }
                },
            )
        }
    }
}

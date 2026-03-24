//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_pipeline_executable_properties.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_pipeline_executable_properties";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_executable_info: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePipelineExecutablePropertiesFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("pipeline_executable_info", &self.pipeline_executable_info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'_>
    {
    }

    impl Default for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                pipeline_executable_info: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'a> {
        #[inline]
        pub fn pipeline_executable_info(mut self, pipeline_executable_info: bool) -> Self {
            self.pipeline_executable_info = pipeline_executable_info.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub pipeline: Pipeline,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("pipeline", &self.pipeline)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_INFO_KHR;
    }

    impl Default for PipelineInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                pipeline: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineInfoKHR<'a> {
        #[inline]
        pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
            self.pipeline = pipeline;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineExecutablePropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineExecutablePropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub stages: ShaderStageFlags,
        pub name: ArrayCStr<{ MAX_DESCRIPTION_SIZE as usize }>,
        pub description: ArrayCStr<{ MAX_DESCRIPTION_SIZE as usize }>,
        pub subgroup_size: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineExecutablePropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineExecutablePropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("stages", &self.stages)
                .field("name", &self.name)
                .field("description", &self.description)
                .field("subgroup_size", &self.subgroup_size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineExecutablePropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_EXECUTABLE_PROPERTIES_KHR;
    }

    impl Default for PipelineExecutablePropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                stages: Default::default(),
                name: Default::default(),
                description: Default::default(),
                subgroup_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineExecutablePropertiesKHR<'a> {
        #[inline]
        pub fn stages(mut self, stages: ShaderStageFlags) -> Self {
            self.stages = stages;
            self
        }

        #[inline]
        pub fn name(
            mut self,
            name: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            self.name.write_c_str(name)?;
            Ok(self)
        }

        #[inline]
        pub fn description(
            mut self,
            description: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            self.description.write_c_str(description)?;
            Ok(self)
        }

        #[inline]
        pub fn subgroup_size(mut self, subgroup_size: u32) -> Self {
            self.subgroup_size = subgroup_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineExecutableInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineExecutableInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub pipeline: Pipeline,
        pub executable_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineExecutableInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineExecutableInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("pipeline", &self.pipeline)
                .field("executable_index", &self.executable_index)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineExecutableInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_EXECUTABLE_INFO_KHR;
    }

    impl Default for PipelineExecutableInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                pipeline: Default::default(),
                executable_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineExecutableInfoKHR<'a> {
        #[inline]
        pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
            self.pipeline = pipeline;
            self
        }

        #[inline]
        pub fn executable_index(mut self, executable_index: u32) -> Self {
            self.executable_index = executable_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineExecutableStatisticKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineExecutableStatisticKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub name: ArrayCStr<{ MAX_DESCRIPTION_SIZE as usize }>,
        pub description: ArrayCStr<{ MAX_DESCRIPTION_SIZE as usize }>,
        pub format: PipelineExecutableStatisticFormatKHR,
        pub value: PipelineExecutableStatisticValueKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineExecutableStatisticKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineExecutableStatisticKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("name", &self.name)
                .field("description", &self.description)
                .field("format", &self.format)
                .field("value", &self.value)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineExecutableStatisticKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_EXECUTABLE_STATISTIC_KHR;
    }

    impl Default for PipelineExecutableStatisticKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                name: Default::default(),
                description: Default::default(),
                format: Default::default(),
                value: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineExecutableStatisticKHR<'a> {
        #[inline]
        pub fn name(
            mut self,
            name: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            self.name.write_c_str(name)?;
            Ok(self)
        }

        #[inline]
        pub fn description(
            mut self,
            description: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            self.description.write_c_str(description)?;
            Ok(self)
        }

        #[inline]
        pub fn format(mut self, format: PipelineExecutableStatisticFormatKHR) -> Self {
            self.format = format;
            self
        }

        #[inline]
        pub fn value(mut self, value: PipelineExecutableStatisticValueKHR) -> Self {
            self.value = value;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineExecutableInternalRepresentationKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineExecutableInternalRepresentationKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub name: ArrayCStr<{ MAX_DESCRIPTION_SIZE as usize }>,
        pub description: ArrayCStr<{ MAX_DESCRIPTION_SIZE as usize }>,
        pub is_text: Bool32,
        pub data_size: usize,
        pub p_data: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineExecutableInternalRepresentationKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineExecutableInternalRepresentationKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("name", &self.name)
                .field("description", &self.description)
                .field("is_text", &self.is_text)
                .field("data_size", &self.data_size)
                .field("p_data", &self.p_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineExecutableInternalRepresentationKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR;
    }

    impl Default for PipelineExecutableInternalRepresentationKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                name: Default::default(),
                description: Default::default(),
                is_text: Default::default(),
                data_size: Default::default(),
                p_data: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineExecutableInternalRepresentationKHR<'a> {
        #[inline]
        pub fn name(
            mut self,
            name: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            self.name.write_c_str(name)?;
            Ok(self)
        }

        #[inline]
        pub fn description(
            mut self,
            description: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            self.description.write_c_str(description)?;
            Ok(self)
        }

        #[inline]
        pub fn is_text(mut self, is_text: bool) -> Self {
            self.is_text = is_text.into();
            self
        }

        #[inline]
        pub fn data(mut self, data: &'a mut [u8]) -> Self {
            self.data_size = data.len().try_into().unwrap();
            self.p_data = data.as_mut_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineExecutableStatisticValueKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union PipelineExecutableStatisticValueKHR {
        pub b32: Bool32,
        pub i64: i64,
        pub u64: u64,
        pub f64: f64,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineExecutableStatisticValueKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineExecutableStatisticValueKHR")
                .finish()
        }
    }

    impl Default for PipelineExecutableStatisticValueKHR {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineExecutableStatisticFormatKHR.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPipelineExecutablePropertiesKHR.html>
    pub type PFN_vkGetPipelineExecutablePropertiesKHR = unsafe extern "system" fn(
        device: Device,
        p_pipeline_info: *const PipelineInfoKHR<'_>,
        p_executable_count: *mut u32,
        p_properties: *mut PipelineExecutablePropertiesKHR<'_>,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPipelineExecutableStatisticsKHR.html>
    pub type PFN_vkGetPipelineExecutableStatisticsKHR = unsafe extern "system" fn(
        device: Device,
        p_executable_info: *const PipelineExecutableInfoKHR<'_>,
        p_statistic_count: *mut u32,
        p_statistics: *mut PipelineExecutableStatisticKHR<'_>,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html>
    pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR =
        unsafe extern "system" fn(
            device: Device,
            p_executable_info: *const PipelineExecutableInfoKHR<'_>,
            p_internal_representation_count: *mut u32,
            p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR<'_>,
        ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR =
        PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'static>;
    pub type VkPipelineInfoKHR = PipelineInfoKHR<'static>;
    pub type VkPipelineExecutablePropertiesKHR = PipelineExecutablePropertiesKHR<'static>;
    pub type VkPipelineExecutableInfoKHR = PipelineExecutableInfoKHR<'static>;
    pub type VkPipelineExecutableStatisticKHR = PipelineExecutableStatisticKHR<'static>;
    pub type VkPipelineExecutableInternalRepresentationKHR =
        PipelineExecutableInternalRepresentationKHR<'static>;
    pub type VkPipelineExecutableStatisticValueKHR = PipelineExecutableStatisticValueKHR;
    pub type VkPipelineExecutableStatisticFormatKHR = PipelineExecutableStatisticFormatKHR;
    impl PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineExecutablePropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineExecutablePropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineExecutableInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineExecutableInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineExecutableStatisticKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineExecutableStatisticKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineExecutableInternalRepresentationKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPipelineExecutableInternalRepresentationKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_pipeline_executable_properties: PFN_vkGetPipelineExecutablePropertiesKHR,
    get_pipeline_executable_statistics: PFN_vkGetPipelineExecutableStatisticsKHR,
    get_pipeline_executable_internal_representations:
        PFN_vkGetPipelineExecutableInternalRepresentationsKHR,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_pipeline_executable_properties: transmute(
                    load(c"vkGetPipelineExecutablePropertiesKHR").ok_or(MissingEntryPointError)?,
                ),
                get_pipeline_executable_statistics: transmute(
                    load(c"vkGetPipelineExecutableStatisticsKHR").ok_or(MissingEntryPointError)?,
                ),
                get_pipeline_executable_internal_representations: transmute(
                    load(c"vkGetPipelineExecutableInternalRepresentationsKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPipelineExecutablePropertiesKHR.html>
    #[inline]
    pub unsafe fn get_pipeline_executable_properties<'a>(
        &self,
        device: Device,
        pipeline_info: &PipelineInfoKHR<'a>,
        mut properties: impl EnumerateInto<PipelineExecutablePropertiesKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |executable_count, properties| {
                let result = (self.get_pipeline_executable_properties)(
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
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let properties_buf = properties.reserve(capacity);
            len = properties_buf.len().try_into().unwrap();
            let result = call(&mut len, properties_buf.as_mut_ptr() as *mut _)?;
            properties.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPipelineExecutableStatisticsKHR.html>
    #[inline]
    pub unsafe fn get_pipeline_executable_statistics<'a>(
        &self,
        device: Device,
        executable_info: &PipelineExecutableInfoKHR<'a>,
        mut statistics: impl EnumerateInto<PipelineExecutableStatisticKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |statistic_count, statistics| {
                let result = (self.get_pipeline_executable_statistics)(
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
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let statistics_buf = statistics.reserve(capacity);
            len = statistics_buf.len().try_into().unwrap();
            let result = call(&mut len, statistics_buf.as_mut_ptr() as *mut _)?;
            statistics.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html>
    #[inline]
    pub unsafe fn get_pipeline_executable_internal_representations<'a>(
        &self,
        device: Device,
        executable_info: &PipelineExecutableInfoKHR<'a>,
        mut internal_representations: impl EnumerateInto<
            PipelineExecutableInternalRepresentationKHR<'a>,
        >,
    ) -> crate::Result<()> {
        unsafe {
            let call = |internal_representation_count, internal_representations| {
                let result = (self.get_pipeline_executable_internal_representations)(
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
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let internal_representations_buf = internal_representations.reserve(capacity);
            len = internal_representations_buf.len().try_into().unwrap();
            let result = call(
                &mut len,
                internal_representations_buf.as_mut_ptr() as *mut _,
            )?;
            internal_representations.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}

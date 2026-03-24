//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_pipeline_binary.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_pipeline_binary";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    pub const MAX_PIPELINE_BINARY_KEY_SIZE_KHR: u32 = 32;

    handle_nondispatchable!(
        PipelineBinaryKHR,
        PIPELINE_BINARY_KHR,
        doc =
            "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineBinaryKHR.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineBinaryCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineBinaryCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_keys_and_data_info: *const PipelineBinaryKeysAndDataKHR<'a>,
        pub pipeline: Pipeline,
        pub p_pipeline_create_info: *const PipelineCreateInfoKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineBinaryCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineBinaryCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_keys_and_data_info", &self.p_keys_and_data_info)
                .field("pipeline", &self.pipeline)
                .field("p_pipeline_create_info", &self.p_pipeline_create_info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineBinaryCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_BINARY_CREATE_INFO_KHR;
    }

    impl Default for PipelineBinaryCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_keys_and_data_info: ptr::null(),
                pipeline: Default::default(),
                p_pipeline_create_info: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineBinaryCreateInfoKHR<'a> {
        #[inline]
        pub fn keys_and_data_info(
            mut self,
            keys_and_data_info: &'a PipelineBinaryKeysAndDataKHR<'a>,
        ) -> Self {
            self.p_keys_and_data_info = keys_and_data_info;
            self
        }

        #[inline]
        pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
            self.pipeline = pipeline;
            self
        }

        #[inline]
        pub fn pipeline_create_info(
            mut self,
            pipeline_create_info: &'a PipelineCreateInfoKHR<'a>,
        ) -> Self {
            self.p_pipeline_create_info = pipeline_create_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineBinaryHandlesInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineBinaryHandlesInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub pipeline_binary_count: u32,
        pub p_pipeline_binaries: *mut PipelineBinaryKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineBinaryHandlesInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineBinaryHandlesInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("pipeline_binary_count", &self.pipeline_binary_count)
                .field("p_pipeline_binaries", &self.p_pipeline_binaries)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineBinaryHandlesInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_BINARY_HANDLES_INFO_KHR;
    }

    impl Default for PipelineBinaryHandlesInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                pipeline_binary_count: Default::default(),
                p_pipeline_binaries: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineBinaryHandlesInfoKHR<'a> {
        #[inline]
        pub fn pipeline_binaries(mut self, pipeline_binaries: &'a mut [PipelineBinaryKHR]) -> Self {
            self.pipeline_binary_count = pipeline_binaries.len().try_into().unwrap();
            self.p_pipeline_binaries = pipeline_binaries.as_mut_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineBinaryDataKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineBinaryDataKHR<'a> {
        pub data_size: usize,
        pub p_data: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineBinaryDataKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineBinaryDataKHR")
                .field("data_size", &self.data_size)
                .field("p_data", &self.p_data)
                .finish()
        }
    }

    impl Default for PipelineBinaryDataKHR<'_> {
        fn default() -> Self {
            Self {
                data_size: Default::default(),
                p_data: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineBinaryDataKHR<'a> {
        #[inline]
        pub fn data(mut self, data: &'a mut [u8]) -> Self {
            self.data_size = data.len().try_into().unwrap();
            self.p_data = data.as_mut_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineBinaryKeysAndDataKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineBinaryKeysAndDataKHR<'a> {
        pub binary_count: u32,
        pub p_pipeline_binary_keys: *const PipelineBinaryKeyKHR<'a>,
        pub p_pipeline_binary_data: *const PipelineBinaryDataKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineBinaryKeysAndDataKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineBinaryKeysAndDataKHR")
                .field("binary_count", &self.binary_count)
                .field("p_pipeline_binary_keys", &self.p_pipeline_binary_keys)
                .field("p_pipeline_binary_data", &self.p_pipeline_binary_data)
                .finish()
        }
    }

    impl Default for PipelineBinaryKeysAndDataKHR<'_> {
        fn default() -> Self {
            Self {
                binary_count: Default::default(),
                p_pipeline_binary_keys: ptr::null(),
                p_pipeline_binary_data: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineBinaryKeysAndDataKHR<'a> {
        #[inline]
        pub fn binaries(
            mut self,
            pipeline_binary_keys: &'a [PipelineBinaryKeyKHR<'_>],
            pipeline_binary_data: &'a [PipelineBinaryDataKHR<'_>],
        ) -> Self {
            self.binary_count = pipeline_binary_keys.len().try_into().unwrap();
            assert_eq!(pipeline_binary_data.len(), self.binary_count as usize);
            self.p_pipeline_binary_keys = pipeline_binary_keys.as_ptr() as _;
            self.p_pipeline_binary_data = pipeline_binary_data.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineBinaryKeyKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineBinaryKeyKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub key_size: u32,
        pub key: [u8; MAX_PIPELINE_BINARY_KEY_SIZE_KHR as usize],
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineBinaryKeyKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineBinaryKeyKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("key_size", &self.key_size)
                .field("key", &self.key)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineBinaryKeyKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_BINARY_KEY_KHR;
    }

    impl Default for PipelineBinaryKeyKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                key_size: Default::default(),
                key: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineBinaryKeyKHR<'a> {
        #[inline]
        pub fn key_size(mut self, key_size: u32) -> Self {
            self.key_size = key_size;
            self
        }

        #[inline]
        pub fn key(mut self, key: [u8; MAX_PIPELINE_BINARY_KEY_SIZE_KHR as usize]) -> Self {
            self.key = key;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineBinaryInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineBinaryInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub binary_count: u32,
        pub p_pipeline_binaries: *const PipelineBinaryKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineBinaryInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineBinaryInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("binary_count", &self.binary_count)
                .field("p_pipeline_binaries", &self.p_pipeline_binaries)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineBinaryInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_BINARY_INFO_KHR;
    }

    unsafe impl Extends<GraphicsPipelineCreateInfo<'_>> for PipelineBinaryInfoKHR<'_> {}
    unsafe impl Extends<ComputePipelineCreateInfo<'_>> for PipelineBinaryInfoKHR<'_> {}
    unsafe impl Extends<RayTracingPipelineCreateInfoKHR<'_>> for PipelineBinaryInfoKHR<'_> {}

    impl Default for PipelineBinaryInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                binary_count: Default::default(),
                p_pipeline_binaries: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineBinaryInfoKHR<'a> {
        #[inline]
        pub fn pipeline_binaries(mut self, pipeline_binaries: &'a [PipelineBinaryKHR]) -> Self {
            self.binary_count = pipeline_binaries.len().try_into().unwrap();
            self.p_pipeline_binaries = pipeline_binaries.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkReleaseCapturedPipelineDataInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ReleaseCapturedPipelineDataInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline: Pipeline,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ReleaseCapturedPipelineDataInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ReleaseCapturedPipelineDataInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("pipeline", &self.pipeline)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ReleaseCapturedPipelineDataInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR;
    }

    impl Default for ReleaseCapturedPipelineDataInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                pipeline: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ReleaseCapturedPipelineDataInfoKHR<'a> {
        #[inline]
        pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
            self.pipeline = pipeline;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineBinaryDataInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineBinaryDataInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_binary: PipelineBinaryKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineBinaryDataInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineBinaryDataInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("pipeline_binary", &self.pipeline_binary)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineBinaryDataInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_BINARY_DATA_INFO_KHR;
    }

    impl Default for PipelineBinaryDataInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                pipeline_binary: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineBinaryDataInfoKHR<'a> {
        #[inline]
        pub fn pipeline_binary(mut self, pipeline_binary: PipelineBinaryKHR) -> Self {
            self.pipeline_binary = pipeline_binary;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_CREATE_INFO_KHR;
    }

    impl Default for PipelineCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineCreateInfoKHR<'a> {}

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePipelineBinaryFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePipelineBinaryFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_binaries: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePipelineBinaryFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePipelineBinaryFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("pipeline_binaries", &self.pipeline_binaries)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePipelineBinaryFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDevicePipelineBinaryFeaturesKHR<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDevicePipelineBinaryFeaturesKHR<'_> {}

    impl Default for PhysicalDevicePipelineBinaryFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                pipeline_binaries: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePipelineBinaryFeaturesKHR<'a> {
        #[inline]
        pub fn pipeline_binaries(mut self, pipeline_binaries: bool) -> Self {
            self.pipeline_binaries = pipeline_binaries.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDevicePipelineBinaryInternalCacheControlKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DevicePipelineBinaryInternalCacheControlKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub disable_internal_cache: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DevicePipelineBinaryInternalCacheControlKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DevicePipelineBinaryInternalCacheControlKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("disable_internal_cache", &self.disable_internal_cache)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DevicePipelineBinaryInternalCacheControlKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR;
    }

    unsafe impl Extends<DeviceCreateInfo<'_>> for DevicePipelineBinaryInternalCacheControlKHR<'_> {}

    impl Default for DevicePipelineBinaryInternalCacheControlKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                disable_internal_cache: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DevicePipelineBinaryInternalCacheControlKHR<'a> {
        #[inline]
        pub fn disable_internal_cache(mut self, disable_internal_cache: bool) -> Self {
            self.disable_internal_cache = disable_internal_cache.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePipelineBinaryPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePipelineBinaryPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_binary_internal_cache: Bool32,
        pub pipeline_binary_internal_cache_control: Bool32,
        pub pipeline_binary_prefers_internal_cache: Bool32,
        pub pipeline_binary_precompiled_internal_cache: Bool32,
        pub pipeline_binary_compressed_data: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePipelineBinaryPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePipelineBinaryPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "pipeline_binary_internal_cache",
                    &self.pipeline_binary_internal_cache,
                )
                .field(
                    "pipeline_binary_internal_cache_control",
                    &self.pipeline_binary_internal_cache_control,
                )
                .field(
                    "pipeline_binary_prefers_internal_cache",
                    &self.pipeline_binary_prefers_internal_cache,
                )
                .field(
                    "pipeline_binary_precompiled_internal_cache",
                    &self.pipeline_binary_precompiled_internal_cache,
                )
                .field(
                    "pipeline_binary_compressed_data",
                    &self.pipeline_binary_compressed_data,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePipelineBinaryPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDevicePipelineBinaryPropertiesKHR<'_>
    {
    }

    impl Default for PhysicalDevicePipelineBinaryPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                pipeline_binary_internal_cache: Default::default(),
                pipeline_binary_internal_cache_control: Default::default(),
                pipeline_binary_prefers_internal_cache: Default::default(),
                pipeline_binary_precompiled_internal_cache: Default::default(),
                pipeline_binary_compressed_data: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePipelineBinaryPropertiesKHR<'a> {
        #[inline]
        pub fn pipeline_binary_internal_cache(
            mut self,
            pipeline_binary_internal_cache: bool,
        ) -> Self {
            self.pipeline_binary_internal_cache = pipeline_binary_internal_cache.into();
            self
        }

        #[inline]
        pub fn pipeline_binary_internal_cache_control(
            mut self,
            pipeline_binary_internal_cache_control: bool,
        ) -> Self {
            self.pipeline_binary_internal_cache_control =
                pipeline_binary_internal_cache_control.into();
            self
        }

        #[inline]
        pub fn pipeline_binary_prefers_internal_cache(
            mut self,
            pipeline_binary_prefers_internal_cache: bool,
        ) -> Self {
            self.pipeline_binary_prefers_internal_cache =
                pipeline_binary_prefers_internal_cache.into();
            self
        }

        #[inline]
        pub fn pipeline_binary_precompiled_internal_cache(
            mut self,
            pipeline_binary_precompiled_internal_cache: bool,
        ) -> Self {
            self.pipeline_binary_precompiled_internal_cache =
                pipeline_binary_precompiled_internal_cache.into();
            self
        }

        #[inline]
        pub fn pipeline_binary_compressed_data(
            mut self,
            pipeline_binary_compressed_data: bool,
        ) -> Self {
            self.pipeline_binary_compressed_data = pipeline_binary_compressed_data.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreatePipelineBinariesKHR.html>
    pub type PFN_vkCreatePipelineBinariesKHR = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const PipelineBinaryCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_binaries: *mut PipelineBinaryHandlesInfoKHR<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyPipelineBinaryKHR.html>
    pub type PFN_vkDestroyPipelineBinaryKHR = unsafe extern "system" fn(
        device: Device,
        pipeline_binary: PipelineBinaryKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPipelineKeyKHR.html>
    pub type PFN_vkGetPipelineKeyKHR = unsafe extern "system" fn(
        device: Device,
        p_pipeline_create_info: *const PipelineCreateInfoKHR<'_>,
        p_pipeline_key: *mut PipelineBinaryKeyKHR<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPipelineBinaryDataKHR.html>
    pub type PFN_vkGetPipelineBinaryDataKHR = unsafe extern "system" fn(
        device: Device,
        p_info: *const PipelineBinaryDataInfoKHR<'_>,
        p_pipeline_binary_key: *mut PipelineBinaryKeyKHR<'_>,
        p_pipeline_binary_data_size: *mut usize,
        p_pipeline_binary_data: *mut c_void,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkReleaseCapturedPipelineDataKHR.html>
    pub type PFN_vkReleaseCapturedPipelineDataKHR = unsafe extern "system" fn(
        device: Device,
        p_info: *const ReleaseCapturedPipelineDataInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPipelineBinaryKHR = PipelineBinaryKHR;
    pub type VkPipelineBinaryCreateInfoKHR = PipelineBinaryCreateInfoKHR<'static>;
    pub type VkPipelineBinaryHandlesInfoKHR = PipelineBinaryHandlesInfoKHR<'static>;
    pub type VkPipelineBinaryDataKHR = PipelineBinaryDataKHR<'static>;
    pub type VkPipelineBinaryKeysAndDataKHR = PipelineBinaryKeysAndDataKHR<'static>;
    pub type VkPipelineBinaryKeyKHR = PipelineBinaryKeyKHR<'static>;
    pub type VkPipelineBinaryInfoKHR = PipelineBinaryInfoKHR<'static>;
    pub type VkReleaseCapturedPipelineDataInfoKHR = ReleaseCapturedPipelineDataInfoKHR<'static>;
    pub type VkPipelineBinaryDataInfoKHR = PipelineBinaryDataInfoKHR<'static>;
    pub type VkPipelineCreateInfoKHR = PipelineCreateInfoKHR<'static>;
    pub type VkPhysicalDevicePipelineBinaryFeaturesKHR =
        PhysicalDevicePipelineBinaryFeaturesKHR<'static>;
    pub type VkDevicePipelineBinaryInternalCacheControlKHR =
        DevicePipelineBinaryInternalCacheControlKHR<'static>;
    pub type VkPhysicalDevicePipelineBinaryPropertiesKHR =
        PhysicalDevicePipelineBinaryPropertiesKHR<'static>;
    impl PipelineBinaryCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineBinaryCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineBinaryHandlesInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineBinaryHandlesInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineBinaryDataKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineBinaryDataKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineBinaryKeysAndDataKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineBinaryKeysAndDataKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineBinaryKeyKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineBinaryKeyKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineBinaryInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineBinaryInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ReleaseCapturedPipelineDataInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkReleaseCapturedPipelineDataInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineBinaryDataInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineBinaryDataInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDevicePipelineBinaryFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDevicePipelineBinaryFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DevicePipelineBinaryInternalCacheControlKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkDevicePipelineBinaryInternalCacheControlKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDevicePipelineBinaryPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDevicePipelineBinaryPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    create_pipeline_binaries: PFN_vkCreatePipelineBinariesKHR,
    destroy_pipeline_binary: PFN_vkDestroyPipelineBinaryKHR,
    get_pipeline_key: PFN_vkGetPipelineKeyKHR,
    get_pipeline_binary_data: PFN_vkGetPipelineBinaryDataKHR,
    release_captured_pipeline_data: PFN_vkReleaseCapturedPipelineDataKHR,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_pipeline_binaries: transmute(
                    load(c"vkCreatePipelineBinariesKHR").ok_or(MissingEntryPointError)?,
                ),
                destroy_pipeline_binary: transmute(
                    load(c"vkDestroyPipelineBinaryKHR").ok_or(MissingEntryPointError)?,
                ),
                get_pipeline_key: transmute(
                    load(c"vkGetPipelineKeyKHR").ok_or(MissingEntryPointError)?,
                ),
                get_pipeline_binary_data: transmute(
                    load(c"vkGetPipelineBinaryDataKHR").ok_or(MissingEntryPointError)?,
                ),
                release_captured_pipeline_data: transmute(
                    load(c"vkReleaseCapturedPipelineDataKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreatePipelineBinariesKHR.html>
    #[inline]
    pub unsafe fn create_pipeline_binaries(
        &self,
        device: Device,
        create_info: &PipelineBinaryCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
        binaries: &mut PipelineBinaryHandlesInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.create_pipeline_binaries)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                binaries,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyPipelineBinaryKHR.html>
    #[inline]
    pub unsafe fn destroy_pipeline_binary(
        &self,
        device: Device,
        pipeline_binary: PipelineBinaryKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_pipeline_binary)(device, pipeline_binary, allocator.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPipelineKeyKHR.html>
    #[inline]
    pub unsafe fn get_pipeline_key(
        &self,
        device: Device,
        pipeline_create_info: Option<&PipelineCreateInfoKHR<'_>>,
        pipeline_key: &mut PipelineBinaryKeyKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result =
                (self.get_pipeline_key)(device, pipeline_create_info.to_raw_ptr(), pipeline_key);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPipelineBinaryDataKHR.html>
    #[inline]
    pub unsafe fn get_pipeline_binary_data<'a>(
        &self,
        device: Device,
        info: &PipelineBinaryDataInfoKHR<'a>,
        pipeline_binary_key: &mut PipelineBinaryKeyKHR<'a>,
        mut pipeline_binary_data: impl EnumerateInto<u8>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |pipeline_binary_data_size,
                        pipeline_binary_data,
                        pipeline_binary_key: &mut PipelineBinaryKeyKHR<'a>| {
                let result = (self.get_pipeline_binary_data)(
                    device,
                    info,
                    pipeline_binary_key,
                    pipeline_binary_data_size,
                    pipeline_binary_data as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut(), pipeline_binary_key)?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let pipeline_binary_data_buf = pipeline_binary_data.reserve(capacity);
            len = pipeline_binary_data_buf.len().try_into().unwrap();
            let result = call(
                &mut len,
                pipeline_binary_data_buf.as_mut_ptr() as *mut _,
                pipeline_binary_key,
            )?;
            pipeline_binary_data.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkReleaseCapturedPipelineDataKHR.html>
    #[inline]
    pub unsafe fn release_captured_pipeline_data(
        &self,
        device: Device,
        info: &ReleaseCapturedPipelineDataInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<()> {
        unsafe {
            let result =
                (self.release_captured_pipeline_data)(device, info, allocator.to_raw_ptr());

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}

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
    pub const MAX_PIPELINE_BINARY_KEY_SIZE_KHR: u32 = 32;
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct PipelineBinaryKHR(u64);
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineBinaryCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_keys_and_data_info: *const PipelineBinaryKeysAndDataKHR<'a>,
        pub pipeline: Pipeline,
        pub p_pipeline_create_info: *const PipelineCreateInfoKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PipelineBinaryCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PIPELINE_BINARY_CREATE_INFO_KHR,
                p_next: core::ptr::null(),
                p_keys_and_data_info: core::ptr::null(),
                pipeline: Default::default(),
                p_pipeline_create_info: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineBinaryCreateInfoKHR<'a> {
        pub fn keys_and_data_info(
            mut self,
            keys_and_data_info: &'a PipelineBinaryKeysAndDataKHR<'a>,
        ) -> Self {
            self.p_keys_and_data_info = keys_and_data_info;
            self
        }
        pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
            self.pipeline = pipeline;
            self
        }
        pub fn pipeline_create_info(
            mut self,
            pipeline_create_info: &'a PipelineCreateInfoKHR<'a>,
        ) -> Self {
            self.p_pipeline_create_info = pipeline_create_info;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineBinaryHandlesInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub pipeline_binary_count: u32,
        pub p_pipeline_binaries: *mut PipelineBinaryKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PipelineBinaryHandlesInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PIPELINE_BINARY_HANDLES_INFO_KHR,
                p_next: core::ptr::null(),
                pipeline_binary_count: Default::default(),
                p_pipeline_binaries: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineBinaryHandlesInfoKHR<'a> {
        pub fn pipeline_binaries(mut self, pipeline_binaries: &'a mut [PipelineBinaryKHR]) -> Self {
            self.pipeline_binary_count = pipeline_binaries.len().try_into().unwrap();
            self.p_pipeline_binaries = pipeline_binaries.as_mut_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineBinaryDataKHR<'a> {
        pub data_size: usize,
        pub p_data: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PipelineBinaryDataKHR<'_> {
        fn default() -> Self {
            Self {
                data_size: Default::default(),
                p_data: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineBinaryDataKHR<'a> {
        pub fn data(mut self, data: &'a mut [u8]) -> Self {
            self.data_size = data.len().try_into().unwrap();
            self.p_data = data.as_mut_ptr() as _;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineBinaryKeysAndDataKHR<'a> {
        pub binary_count: u32,
        pub p_pipeline_binary_keys: *const PipelineBinaryKeyKHR<'a>,
        pub p_pipeline_binary_data: *const PipelineBinaryDataKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PipelineBinaryKeysAndDataKHR<'_> {
        fn default() -> Self {
            Self {
                binary_count: Default::default(),
                p_pipeline_binary_keys: core::ptr::null(),
                p_pipeline_binary_data: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineBinaryKeysAndDataKHR<'a> {
        pub fn pipeline_binary_keys(
            mut self,
            pipeline_binary_keys: &'a [PipelineBinaryKeyKHR<'a>],
        ) -> Self {
            self.binary_count = pipeline_binary_keys.len().try_into().unwrap();
            self.p_pipeline_binary_keys = pipeline_binary_keys.as_ptr();
            self
        }
        pub fn pipeline_binary_data(
            mut self,
            pipeline_binary_data: &'a [PipelineBinaryDataKHR<'a>],
        ) -> Self {
            self.binary_count = pipeline_binary_data.len().try_into().unwrap();
            self.p_pipeline_binary_data = pipeline_binary_data.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineBinaryKeyKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub key_size: u32,
        pub key: [u8; MAX_PIPELINE_BINARY_KEY_SIZE_KHR as usize],
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PipelineBinaryKeyKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PIPELINE_BINARY_KEY_KHR,
                p_next: core::ptr::null_mut(),
                key_size: Default::default(),
                key: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineBinaryKeyKHR<'a> {
        pub fn key_size(mut self, key_size: u32) -> Self {
            self.key_size = key_size;
            self
        }
        pub fn key(mut self, key: [u8; MAX_PIPELINE_BINARY_KEY_SIZE_KHR as usize]) -> Self {
            self.key = key;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineBinaryInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub binary_count: u32,
        pub p_pipeline_binaries: *const PipelineBinaryKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PipelineBinaryInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PIPELINE_BINARY_INFO_KHR,
                p_next: core::ptr::null(),
                binary_count: Default::default(),
                p_pipeline_binaries: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineBinaryInfoKHR<'a> {
        pub fn pipeline_binaries(mut self, pipeline_binaries: &'a [PipelineBinaryKHR]) -> Self {
            self.binary_count = pipeline_binaries.len().try_into().unwrap();
            self.p_pipeline_binaries = pipeline_binaries.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ReleaseCapturedPipelineDataInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline: Pipeline,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for ReleaseCapturedPipelineDataInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR,
                p_next: core::ptr::null_mut(),
                pipeline: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ReleaseCapturedPipelineDataInfoKHR<'a> {
        pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
            self.pipeline = pipeline;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineBinaryDataInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_binary: PipelineBinaryKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PipelineBinaryDataInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PIPELINE_BINARY_DATA_INFO_KHR,
                p_next: core::ptr::null_mut(),
                pipeline_binary: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineBinaryDataInfoKHR<'a> {
        pub fn pipeline_binary(mut self, pipeline_binary: PipelineBinaryKHR) -> Self {
            self.pipeline_binary = pipeline_binary;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PipelineCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PIPELINE_CREATE_INFO_KHR,
                p_next: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineCreateInfoKHR<'a> {}
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePipelineBinaryFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_binaries: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDevicePipelineBinaryFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR,
                p_next: core::ptr::null_mut(),
                pipeline_binaries: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePipelineBinaryFeaturesKHR<'a> {
        pub fn pipeline_binaries(mut self, pipeline_binaries: Bool32) -> Self {
            self.pipeline_binaries = pipeline_binaries;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DevicePipelineBinaryInternalCacheControlKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub disable_internal_cache: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for DevicePipelineBinaryInternalCacheControlKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR,
                p_next: core::ptr::null(),
                disable_internal_cache: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DevicePipelineBinaryInternalCacheControlKHR<'a> {
        pub fn disable_internal_cache(mut self, disable_internal_cache: Bool32) -> Self {
            self.disable_internal_cache = disable_internal_cache;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    impl Default for PhysicalDevicePipelineBinaryPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR,
                p_next: core::ptr::null_mut(),
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
        pub fn pipeline_binary_internal_cache(
            mut self,
            pipeline_binary_internal_cache: Bool32,
        ) -> Self {
            self.pipeline_binary_internal_cache = pipeline_binary_internal_cache;
            self
        }
        pub fn pipeline_binary_internal_cache_control(
            mut self,
            pipeline_binary_internal_cache_control: Bool32,
        ) -> Self {
            self.pipeline_binary_internal_cache_control = pipeline_binary_internal_cache_control;
            self
        }
        pub fn pipeline_binary_prefers_internal_cache(
            mut self,
            pipeline_binary_prefers_internal_cache: Bool32,
        ) -> Self {
            self.pipeline_binary_prefers_internal_cache = pipeline_binary_prefers_internal_cache;
            self
        }
        pub fn pipeline_binary_precompiled_internal_cache(
            mut self,
            pipeline_binary_precompiled_internal_cache: Bool32,
        ) -> Self {
            self.pipeline_binary_precompiled_internal_cache =
                pipeline_binary_precompiled_internal_cache;
            self
        }
        pub fn pipeline_binary_compressed_data(
            mut self,
            pipeline_binary_compressed_data: Bool32,
        ) -> Self {
            self.pipeline_binary_compressed_data = pipeline_binary_compressed_data;
            self
        }
    }
    pub type PFN_vkCreatePipelineBinariesKHR = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const PipelineBinaryCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_binaries: *mut PipelineBinaryHandlesInfoKHR<'_>,
    ) -> vk::Result;
    pub type PFN_vkDestroyPipelineBinaryKHR = unsafe extern "system" fn(
        device: Device,
        pipeline_binary: PipelineBinaryKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    pub type PFN_vkGetPipelineKeyKHR = unsafe extern "system" fn(
        device: Device,
        p_pipeline_create_info: *const PipelineCreateInfoKHR<'_>,
        p_pipeline_key: *mut PipelineBinaryKeyKHR<'_>,
    ) -> vk::Result;
    pub type PFN_vkGetPipelineBinaryDataKHR = unsafe extern "system" fn(
        device: Device,
        p_info: *const PipelineBinaryDataInfoKHR<'_>,
        p_pipeline_binary_key: *mut PipelineBinaryKeyKHR<'_>,
        p_pipeline_binary_data_size: *mut usize,
        p_pipeline_binary_data: *mut c_void,
    ) -> vk::Result;
    pub type PFN_vkReleaseCapturedPipelineDataKHR = unsafe extern "system" fn(
        device: Device,
        p_info: *const ReleaseCapturedPipelineDataInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
    ) -> vk::Result;
}
pub struct DeviceFn {
    create_pipeline_binaries_khr: PFN_vkCreatePipelineBinariesKHR,
    destroy_pipeline_binary_khr: PFN_vkDestroyPipelineBinaryKHR,
    get_pipeline_key_khr: PFN_vkGetPipelineKeyKHR,
    get_pipeline_binary_data_khr: PFN_vkGetPipelineBinaryDataKHR,
    release_captured_pipeline_data_khr: PFN_vkReleaseCapturedPipelineDataKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_pipeline_binaries_khr: transmute(
                    load(c"vkCreatePipelineBinariesKHR").ok_or(LoadingError)?,
                ),
                destroy_pipeline_binary_khr: transmute(
                    load(c"vkDestroyPipelineBinaryKHR").ok_or(LoadingError)?,
                ),
                get_pipeline_key_khr: transmute(load(c"vkGetPipelineKeyKHR").ok_or(LoadingError)?),
                get_pipeline_binary_data_khr: transmute(
                    load(c"vkGetPipelineBinaryDataKHR").ok_or(LoadingError)?,
                ),
                release_captured_pipeline_data_khr: transmute(
                    load(c"vkReleaseCapturedPipelineDataKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_pipeline_binaries_khr(
        &self,
        device: Device,
        create_info: &PipelineBinaryCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
        binaries: &mut PipelineBinaryHandlesInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.create_pipeline_binaries_khr)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                binaries,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::INCOMPLETE => Ok(()),
                VkResult::PIPELINE_BINARY_MISSING_KHR => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_pipeline_binary_khr(
        &self,
        device: Device,
        pipeline_binary: PipelineBinaryKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_pipeline_binary_khr)(device, pipeline_binary, allocator.to_raw_ptr())
        }
    }
    pub unsafe fn get_pipeline_key_khr(
        &self,
        device: Device,
        pipeline_create_info: Option<&PipelineCreateInfoKHR<'_>>,
    ) -> crate::Result<PipelineBinaryKeyKHR<'_>> {
        unsafe {
            let mut pipeline_key = core::mem::MaybeUninit::uninit();
            let result = (self.get_pipeline_key_khr)(
                device,
                pipeline_create_info.to_raw_ptr(),
                pipeline_key.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(pipeline_key.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_pipeline_binary_data_khr<'a>(
        &self,
        device: Device,
        info: &PipelineBinaryDataInfoKHR<'_>,
        pipeline_binary_data: impl ExtendUninit<u8>,
    ) -> crate::Result<PipelineBinaryKeyKHR<'_>> {
        unsafe {
            try_extend_uninit(
                pipeline_binary_data,
                |pipeline_binary_data_size, pipeline_binary_data| {
                    let mut pipeline_binary_key = core::mem::MaybeUninit::uninit();
                    let result = (self.get_pipeline_binary_data_khr)(
                        device,
                        info,
                        pipeline_binary_key.as_mut_ptr(),
                        pipeline_binary_data_size,
                        pipeline_binary_data as _,
                    );

                    match result {
                        VkResult::SUCCESS => Ok(pipeline_binary_key.assume_init()),
                        err => Err(err),
                    }
                },
            )
        }
    }
    pub unsafe fn release_captured_pipeline_data_khr(
        &self,
        device: Device,
        info: &ReleaseCapturedPipelineDataInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<()> {
        unsafe {
            let result =
                (self.release_captured_pipeline_data_khr)(device, info, allocator.to_raw_ptr());

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}

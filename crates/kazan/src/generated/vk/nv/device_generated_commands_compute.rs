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
    pub struct ComputePipelineIndirectBufferInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub device_address: DeviceAddress,
        pub size: DeviceSize,
        pub pipeline_device_address_capture_replay: DeviceAddress,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ComputePipelineIndirectBufferInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV;
    }
    unsafe impl<'a> Extends<ComputePipelineCreateInfo<'a>> for ComputePipelineIndirectBufferInfoNV<'a> {}
    impl Default for ComputePipelineIndirectBufferInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                device_address: Default::default(),
                size: Default::default(),
                pipeline_device_address_capture_replay: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ComputePipelineIndirectBufferInfoNV<'a> {
        pub fn device_address(mut self, device_address: DeviceAddress) -> Self {
            self.device_address = device_address;
            self
        }
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
        pub fn pipeline_device_address_capture_replay(
            mut self,
            pipeline_device_address_capture_replay: DeviceAddress,
        ) -> Self {
            self.pipeline_device_address_capture_replay = pipeline_device_address_capture_replay;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub device_generated_compute: Bool32,
        pub device_generated_compute_pipelines: Bool32,
        pub device_generated_compute_capture_replay: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'a>
    {
    }
    impl Default for PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                device_generated_compute: Default::default(),
                device_generated_compute_pipelines: Default::default(),
                device_generated_compute_capture_replay: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV<'a> {
        pub fn device_generated_compute(mut self, device_generated_compute: Bool32) -> Self {
            self.device_generated_compute = device_generated_compute;
            self
        }
        pub fn device_generated_compute_pipelines(
            mut self,
            device_generated_compute_pipelines: Bool32,
        ) -> Self {
            self.device_generated_compute_pipelines = device_generated_compute_pipelines;
            self
        }
        pub fn device_generated_compute_capture_replay(
            mut self,
            device_generated_compute_capture_replay: Bool32,
        ) -> Self {
            self.device_generated_compute_capture_replay = device_generated_compute_capture_replay;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineIndirectDeviceAddressInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub pipeline_bind_point: PipelineBindPoint,
        pub pipeline: Pipeline,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineIndirectDeviceAddressInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV;
    }
    impl Default for PipelineIndirectDeviceAddressInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                pipeline_bind_point: Default::default(),
                pipeline: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineIndirectDeviceAddressInfoNV<'a> {
        pub fn pipeline_bind_point(mut self, pipeline_bind_point: PipelineBindPoint) -> Self {
            self.pipeline_bind_point = pipeline_bind_point;
            self
        }
        pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
            self.pipeline = pipeline;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct BindPipelineIndirectCommandNV {
        pub pipeline_address: DeviceAddress,
    }
    impl BindPipelineIndirectCommandNV {
        pub fn pipeline_address(mut self, pipeline_address: DeviceAddress) -> Self {
            self.pipeline_address = pipeline_address;
            self
        }
    }
    pub type PFN_vkCmdUpdatePipelineIndirectBufferNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    );
    pub type PFN_vkGetPipelineIndirectMemoryRequirementsNV = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ComputePipelineCreateInfo<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    );
    pub type PFN_vkGetPipelineIndirectDeviceAddressNV = unsafe extern "system" fn(
        device: Device,
        p_info: *const PipelineIndirectDeviceAddressInfoNV<'_>,
    )
        -> DeviceAddress;
}
pub struct DeviceFn {
    get_pipeline_indirect_memory_requirements_nv: PFN_vkGetPipelineIndirectMemoryRequirementsNV,
    cmd_update_pipeline_indirect_buffer_nv: PFN_vkCmdUpdatePipelineIndirectBufferNV,
    get_pipeline_indirect_device_address_nv: PFN_vkGetPipelineIndirectDeviceAddressNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_pipeline_indirect_memory_requirements_nv: transmute(
                    load(c"vkGetPipelineIndirectMemoryRequirementsNV")
                        .ok_or(MissingEntryPointError)?,
                ),
                cmd_update_pipeline_indirect_buffer_nv: transmute(
                    load(c"vkCmdUpdatePipelineIndirectBufferNV").ok_or(MissingEntryPointError)?,
                ),
                get_pipeline_indirect_device_address_nv: transmute(
                    load(c"vkGetPipelineIndirectDeviceAddressNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_pipeline_indirect_memory_requirements_nv(
        &self,
        device: Device,
        create_info: &ComputePipelineCreateInfo<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe {
            (self.get_pipeline_indirect_memory_requirements_nv)(
                device,
                create_info,
                memory_requirements,
            )
        }
    }
    pub unsafe fn cmd_update_pipeline_indirect_buffer_nv(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) {
        unsafe {
            (self.cmd_update_pipeline_indirect_buffer_nv)(
                command_buffer,
                pipeline_bind_point,
                pipeline,
            )
        }
    }
    pub unsafe fn get_pipeline_indirect_device_address_nv(
        &self,
        device: Device,
        info: &PipelineIndirectDeviceAddressInfoNV<'_>,
    ) -> DeviceAddress {
        unsafe { (self.get_pipeline_indirect_device_address_nv)(device, info) }
    }
}

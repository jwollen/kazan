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
    pub const SHADER_INDEX_UNUSED_AMDX: u32 = !0;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderEnqueuePropertiesAMDX<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_execution_graph_depth: u32,
        pub max_execution_graph_shader_output_nodes: u32,
        pub max_execution_graph_shader_payload_size: u32,
        pub max_execution_graph_shader_payload_count: u32,
        pub execution_graph_dispatch_address_alignment: u32,
        pub max_execution_graph_workgroup_count: [u32; 3],
        pub max_execution_graph_workgroups: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderEnqueuePropertiesAMDX<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceShaderEnqueuePropertiesAMDX<'a>
    {
    }
    impl Default for PhysicalDeviceShaderEnqueuePropertiesAMDX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_execution_graph_depth: Default::default(),
                max_execution_graph_shader_output_nodes: Default::default(),
                max_execution_graph_shader_payload_size: Default::default(),
                max_execution_graph_shader_payload_count: Default::default(),
                execution_graph_dispatch_address_alignment: Default::default(),
                max_execution_graph_workgroup_count: [Default::default(); _],
                max_execution_graph_workgroups: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderEnqueuePropertiesAMDX<'a> {
        pub fn max_execution_graph_depth(mut self, max_execution_graph_depth: u32) -> Self {
            self.max_execution_graph_depth = max_execution_graph_depth;
            self
        }
        pub fn max_execution_graph_shader_output_nodes(
            mut self,
            max_execution_graph_shader_output_nodes: u32,
        ) -> Self {
            self.max_execution_graph_shader_output_nodes = max_execution_graph_shader_output_nodes;
            self
        }
        pub fn max_execution_graph_shader_payload_size(
            mut self,
            max_execution_graph_shader_payload_size: u32,
        ) -> Self {
            self.max_execution_graph_shader_payload_size = max_execution_graph_shader_payload_size;
            self
        }
        pub fn max_execution_graph_shader_payload_count(
            mut self,
            max_execution_graph_shader_payload_count: u32,
        ) -> Self {
            self.max_execution_graph_shader_payload_count =
                max_execution_graph_shader_payload_count;
            self
        }
        pub fn execution_graph_dispatch_address_alignment(
            mut self,
            execution_graph_dispatch_address_alignment: u32,
        ) -> Self {
            self.execution_graph_dispatch_address_alignment =
                execution_graph_dispatch_address_alignment;
            self
        }
        pub fn max_execution_graph_workgroup_count(
            mut self,
            max_execution_graph_workgroup_count: [u32; 3],
        ) -> Self {
            self.max_execution_graph_workgroup_count = max_execution_graph_workgroup_count;
            self
        }
        pub fn max_execution_graph_workgroups(
            mut self,
            max_execution_graph_workgroups: u32,
        ) -> Self {
            self.max_execution_graph_workgroups = max_execution_graph_workgroups;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderEnqueueFeaturesAMDX<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_enqueue: Bool32,
        pub shader_mesh_enqueue: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderEnqueueFeaturesAMDX<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderEnqueueFeaturesAMDX<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShaderEnqueueFeaturesAMDX<'a> {}
    impl Default for PhysicalDeviceShaderEnqueueFeaturesAMDX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_enqueue: Default::default(),
                shader_mesh_enqueue: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderEnqueueFeaturesAMDX<'a> {
        pub fn shader_enqueue(mut self, shader_enqueue: Bool32) -> Self {
            self.shader_enqueue = shader_enqueue;
            self
        }
        pub fn shader_mesh_enqueue(mut self, shader_mesh_enqueue: Bool32) -> Self {
            self.shader_mesh_enqueue = shader_mesh_enqueue;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExecutionGraphPipelineCreateInfoAMDX<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineCreateFlags,
        pub stage_count: u32,
        pub p_stages: *const PipelineShaderStageCreateInfo<'a>,
        pub p_library_info: *const PipelineLibraryCreateInfoKHR<'a>,
        pub layout: PipelineLayout,
        pub base_pipeline_handle: Pipeline,
        pub base_pipeline_index: i32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ExecutionGraphPipelineCreateInfoAMDX<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX;
    }
    impl Default for ExecutionGraphPipelineCreateInfoAMDX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                stage_count: Default::default(),
                p_stages: core::ptr::null(),
                p_library_info: core::ptr::null(),
                layout: Default::default(),
                base_pipeline_handle: Default::default(),
                base_pipeline_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ExecutionGraphPipelineCreateInfoAMDX<'a> {
        pub fn flags(mut self, flags: PipelineCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn stages(mut self, stages: &'a [PipelineShaderStageCreateInfo<'a>]) -> Self {
            self.stage_count = stages.len().try_into().unwrap();
            self.p_stages = stages.as_ptr();
            self
        }
        pub fn library_info(mut self, library_info: &'a PipelineLibraryCreateInfoKHR<'a>) -> Self {
            self.p_library_info = library_info;
            self
        }
        pub fn layout(mut self, layout: PipelineLayout) -> Self {
            self.layout = layout;
            self
        }
        pub fn base_pipeline_handle(mut self, base_pipeline_handle: Pipeline) -> Self {
            self.base_pipeline_handle = base_pipeline_handle;
            self
        }
        pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
            self.base_pipeline_index = base_pipeline_index;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineShaderStageNodeCreateInfoAMDX<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_name: *const c_char,
        pub index: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PipelineShaderStageNodeCreateInfoAMDX<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX;
    }
    unsafe impl<'a> Extends<PipelineShaderStageCreateInfo<'a>>
        for PipelineShaderStageNodeCreateInfoAMDX<'a>
    {
    }
    impl Default for PipelineShaderStageNodeCreateInfoAMDX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_name: core::ptr::null(),
                index: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineShaderStageNodeCreateInfoAMDX<'a> {
        pub fn name(mut self, name: &'a CStr) -> Self {
            self.p_name = name.as_ptr();
            self
        }
        pub fn index(mut self, index: u32) -> Self {
            self.index = index;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExecutionGraphPipelineScratchSizeAMDX<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub min_size: DeviceSize,
        pub max_size: DeviceSize,
        pub size_granularity: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ExecutionGraphPipelineScratchSizeAMDX<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX;
    }
    impl Default for ExecutionGraphPipelineScratchSizeAMDX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                min_size: Default::default(),
                max_size: Default::default(),
                size_granularity: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ExecutionGraphPipelineScratchSizeAMDX<'a> {
        pub fn min_size(mut self, min_size: DeviceSize) -> Self {
            self.min_size = min_size;
            self
        }
        pub fn max_size(mut self, max_size: DeviceSize) -> Self {
            self.max_size = max_size;
            self
        }
        pub fn size_granularity(mut self, size_granularity: DeviceSize) -> Self {
            self.size_granularity = size_granularity;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DispatchGraphInfoAMDX<'a> {
        pub node_index: u32,
        pub payload_count: u32,
        pub payloads: DeviceOrHostAddressConstAMDX<'a>,
        pub payload_stride: u64,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for DispatchGraphInfoAMDX<'_> {
        fn default() -> Self {
            Self {
                node_index: Default::default(),
                payload_count: Default::default(),
                payloads: Default::default(),
                payload_stride: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DispatchGraphInfoAMDX<'a> {
        pub fn node_index(mut self, node_index: u32) -> Self {
            self.node_index = node_index;
            self
        }
        pub fn payload_count(mut self, payload_count: u32) -> Self {
            self.payload_count = payload_count;
            self
        }
        pub fn payloads(mut self, payloads: DeviceOrHostAddressConstAMDX<'a>) -> Self {
            self.payloads = payloads;
            self
        }
        pub fn payload_stride(mut self, payload_stride: u64) -> Self {
            self.payload_stride = payload_stride;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DispatchGraphCountInfoAMDX<'a> {
        pub count: u32,
        pub infos: DeviceOrHostAddressConstAMDX<'a>,
        pub stride: u64,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for DispatchGraphCountInfoAMDX<'_> {
        fn default() -> Self {
            Self {
                count: Default::default(),
                infos: Default::default(),
                stride: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> DispatchGraphCountInfoAMDX<'a> {
        pub fn count(mut self, count: u32) -> Self {
            self.count = count;
            self
        }
        pub fn infos(mut self, infos: DeviceOrHostAddressConstAMDX<'a>) -> Self {
            self.infos = infos;
            self
        }
        pub fn stride(mut self, stride: u64) -> Self {
            self.stride = stride;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union DeviceOrHostAddressConstAMDX<'a> {
        pub device_address: DeviceAddress,
        pub host_address: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for DeviceOrHostAddressConstAMDX<'_> {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }
    pub type PFN_vkGetExecutionGraphPipelineScratchSizeAMDX =
        unsafe extern "system" fn(
            device: Device,
            execution_graph: Pipeline,
            p_size_info: *mut ExecutionGraphPipelineScratchSizeAMDX<'_>,
        ) -> vk::Result;
    pub type PFN_vkGetExecutionGraphPipelineNodeIndexAMDX = unsafe extern "system" fn(
        device: Device,
        execution_graph: Pipeline,
        p_node_info: *const PipelineShaderStageNodeCreateInfoAMDX<'_>,
        p_node_index: *mut u32,
    )
        -> vk::Result;
    pub type PFN_vkCreateExecutionGraphPipelinesAMDX = unsafe extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const ExecutionGraphPipelineCreateInfoAMDX<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> vk::Result;
    pub type PFN_vkCmdInitializeGraphScratchMemoryAMDX = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        execution_graph: Pipeline,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
    );
    pub type PFN_vkCmdDispatchGraphAMDX = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        p_count_info: *const DispatchGraphCountInfoAMDX<'_>,
    );
    pub type PFN_vkCmdDispatchGraphIndirectAMDX = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        p_count_info: *const DispatchGraphCountInfoAMDX<'_>,
    );
    pub type PFN_vkCmdDispatchGraphIndirectCountAMDX = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: DeviceAddress,
    );
}
pub struct DeviceFn {
    create_execution_graph_pipelines_amdx: PFN_vkCreateExecutionGraphPipelinesAMDX,
    get_execution_graph_pipeline_scratch_size_amdx: PFN_vkGetExecutionGraphPipelineScratchSizeAMDX,
    get_execution_graph_pipeline_node_index_amdx: PFN_vkGetExecutionGraphPipelineNodeIndexAMDX,
    cmd_initialize_graph_scratch_memory_amdx: PFN_vkCmdInitializeGraphScratchMemoryAMDX,
    cmd_dispatch_graph_amdx: PFN_vkCmdDispatchGraphAMDX,
    cmd_dispatch_graph_indirect_amdx: PFN_vkCmdDispatchGraphIndirectAMDX,
    cmd_dispatch_graph_indirect_count_amdx: PFN_vkCmdDispatchGraphIndirectCountAMDX,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_execution_graph_pipelines_amdx: transmute(
                    load(c"vkCreateExecutionGraphPipelinesAMDX").ok_or(MissingEntryPointError)?,
                ),
                get_execution_graph_pipeline_scratch_size_amdx: transmute(
                    load(c"vkGetExecutionGraphPipelineScratchSizeAMDX")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_execution_graph_pipeline_node_index_amdx: transmute(
                    load(c"vkGetExecutionGraphPipelineNodeIndexAMDX")
                        .ok_or(MissingEntryPointError)?,
                ),
                cmd_initialize_graph_scratch_memory_amdx: transmute(
                    load(c"vkCmdInitializeGraphScratchMemoryAMDX").ok_or(MissingEntryPointError)?,
                ),
                cmd_dispatch_graph_amdx: transmute(
                    load(c"vkCmdDispatchGraphAMDX").ok_or(MissingEntryPointError)?,
                ),
                cmd_dispatch_graph_indirect_amdx: transmute(
                    load(c"vkCmdDispatchGraphIndirectAMDX").ok_or(MissingEntryPointError)?,
                ),
                cmd_dispatch_graph_indirect_count_amdx: transmute(
                    load(c"vkCmdDispatchGraphIndirectCountAMDX").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_execution_graph_pipelines_amdx(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_infos: &[ExecutionGraphPipelineCreateInfoAMDX<'_>],
        allocator: Option<&AllocationCallbacks<'_>>,
        pipelines: &mut [Pipeline],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.create_execution_graph_pipelines_amdx)(
                device,
                pipeline_cache,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                pipelines.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::PIPELINE_COMPILE_REQUIRED_EXT => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_execution_graph_pipeline_scratch_size_amdx(
        &self,
        device: Device,
        execution_graph: Pipeline,
        size_info: &mut ExecutionGraphPipelineScratchSizeAMDX<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_execution_graph_pipeline_scratch_size_amdx)(
                device,
                execution_graph,
                size_info,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_execution_graph_pipeline_node_index_amdx(
        &self,
        device: Device,
        execution_graph: Pipeline,
        node_info: &PipelineShaderStageNodeCreateInfoAMDX<'_>,
    ) -> crate::Result<u32> {
        unsafe {
            let mut node_index = core::mem::MaybeUninit::uninit();
            let result = (self.get_execution_graph_pipeline_node_index_amdx)(
                device,
                execution_graph,
                node_info,
                node_index.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(node_index.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn cmd_initialize_graph_scratch_memory_amdx(
        &self,
        command_buffer: CommandBuffer,
        execution_graph: Pipeline,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
    ) {
        unsafe {
            (self.cmd_initialize_graph_scratch_memory_amdx)(
                command_buffer,
                execution_graph,
                scratch,
                scratch_size,
            )
        }
    }
    pub unsafe fn cmd_dispatch_graph_amdx(
        &self,
        command_buffer: CommandBuffer,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: &DispatchGraphCountInfoAMDX<'_>,
    ) {
        unsafe { (self.cmd_dispatch_graph_amdx)(command_buffer, scratch, scratch_size, count_info) }
    }
    pub unsafe fn cmd_dispatch_graph_indirect_amdx(
        &self,
        command_buffer: CommandBuffer,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: &DispatchGraphCountInfoAMDX<'_>,
    ) {
        unsafe {
            (self.cmd_dispatch_graph_indirect_amdx)(
                command_buffer,
                scratch,
                scratch_size,
                count_info,
            )
        }
    }
    pub unsafe fn cmd_dispatch_graph_indirect_count_amdx(
        &self,
        command_buffer: CommandBuffer,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: DeviceAddress,
    ) {
        unsafe {
            (self.cmd_dispatch_graph_indirect_count_amdx)(
                command_buffer,
                scratch,
                scratch_size,
                count_info,
            )
        }
    }
}

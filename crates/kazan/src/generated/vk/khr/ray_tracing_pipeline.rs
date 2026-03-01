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
    pub const SHADER_UNUSED_KHR: u32 = !0;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RayTracingShaderGroupCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub ty: RayTracingShaderGroupTypeKHR,
        pub general_shader: u32,
        pub closest_hit_shader: u32,
        pub any_hit_shader: u32,
        pub intersection_shader: u32,
        pub p_shader_group_capture_replay_handle: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for RayTracingShaderGroupCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR;
    }
    impl Default for RayTracingShaderGroupCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                ty: Default::default(),
                general_shader: Default::default(),
                closest_hit_shader: Default::default(),
                any_hit_shader: Default::default(),
                intersection_shader: Default::default(),
                p_shader_group_capture_replay_handle: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RayTracingShaderGroupCreateInfoKHR<'a> {
        pub fn ty(mut self, ty: RayTracingShaderGroupTypeKHR) -> Self {
            self.ty = ty;
            self
        }
        pub fn general_shader(mut self, general_shader: u32) -> Self {
            self.general_shader = general_shader;
            self
        }
        pub fn closest_hit_shader(mut self, closest_hit_shader: u32) -> Self {
            self.closest_hit_shader = closest_hit_shader;
            self
        }
        pub fn any_hit_shader(mut self, any_hit_shader: u32) -> Self {
            self.any_hit_shader = any_hit_shader;
            self
        }
        pub fn intersection_shader(mut self, intersection_shader: u32) -> Self {
            self.intersection_shader = intersection_shader;
            self
        }
        pub fn shader_group_capture_replay_handle(
            mut self,
            shader_group_capture_replay_handle: &'a c_void,
        ) -> Self {
            self.p_shader_group_capture_replay_handle = shader_group_capture_replay_handle;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RayTracingPipelineCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineCreateFlags,
        pub stage_count: u32,
        pub p_stages: *const PipelineShaderStageCreateInfo<'a>,
        pub group_count: u32,
        pub p_groups: *const RayTracingShaderGroupCreateInfoKHR<'a>,
        pub max_pipeline_ray_recursion_depth: u32,
        pub p_library_info: *const PipelineLibraryCreateInfoKHR<'a>,
        pub p_library_interface: *const RayTracingPipelineInterfaceCreateInfoKHR<'a>,
        pub p_dynamic_state: *const PipelineDynamicStateCreateInfo<'a>,
        pub layout: PipelineLayout,
        pub base_pipeline_handle: Pipeline,
        pub base_pipeline_index: i32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for RayTracingPipelineCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RAY_TRACING_PIPELINE_CREATE_INFO_KHR;
    }
    impl Default for RayTracingPipelineCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                stage_count: Default::default(),
                p_stages: core::ptr::null(),
                group_count: Default::default(),
                p_groups: core::ptr::null(),
                max_pipeline_ray_recursion_depth: Default::default(),
                p_library_info: core::ptr::null(),
                p_library_interface: core::ptr::null(),
                p_dynamic_state: core::ptr::null(),
                layout: Default::default(),
                base_pipeline_handle: Default::default(),
                base_pipeline_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RayTracingPipelineCreateInfoKHR<'a> {
        pub fn flags(mut self, flags: PipelineCreateFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn stages(mut self, stages: &'a [PipelineShaderStageCreateInfo<'a>]) -> Self {
            self.stage_count = stages.len().try_into().unwrap();
            self.p_stages = stages.as_ptr();
            self
        }
        pub fn groups(mut self, groups: &'a [RayTracingShaderGroupCreateInfoKHR<'a>]) -> Self {
            self.group_count = groups.len().try_into().unwrap();
            self.p_groups = groups.as_ptr();
            self
        }
        pub fn max_pipeline_ray_recursion_depth(
            mut self,
            max_pipeline_ray_recursion_depth: u32,
        ) -> Self {
            self.max_pipeline_ray_recursion_depth = max_pipeline_ray_recursion_depth;
            self
        }
        pub fn library_info(mut self, library_info: &'a PipelineLibraryCreateInfoKHR<'a>) -> Self {
            self.p_library_info = library_info;
            self
        }
        pub fn library_interface(
            mut self,
            library_interface: &'a RayTracingPipelineInterfaceCreateInfoKHR<'a>,
        ) -> Self {
            self.p_library_interface = library_interface;
            self
        }
        pub fn dynamic_state(
            mut self,
            dynamic_state: &'a PipelineDynamicStateCreateInfo<'a>,
        ) -> Self {
            self.p_dynamic_state = dynamic_state;
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
    pub struct PhysicalDeviceRayTracingPipelineFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ray_tracing_pipeline: Bool32,
        pub ray_tracing_pipeline_shader_group_handle_capture_replay: Bool32,
        pub ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: Bool32,
        pub ray_tracing_pipeline_trace_rays_indirect: Bool32,
        pub ray_traversal_primitive_culling: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRayTracingPipelineFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceRayTracingPipelineFeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceRayTracingPipelineFeaturesKHR<'a> {}
    impl Default for PhysicalDeviceRayTracingPipelineFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                ray_tracing_pipeline: Default::default(),
                ray_tracing_pipeline_shader_group_handle_capture_replay: Default::default(),
                ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: Default::default(),
                ray_tracing_pipeline_trace_rays_indirect: Default::default(),
                ray_traversal_primitive_culling: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceRayTracingPipelineFeaturesKHR<'a> {
        pub fn ray_tracing_pipeline(mut self, ray_tracing_pipeline: Bool32) -> Self {
            self.ray_tracing_pipeline = ray_tracing_pipeline;
            self
        }
        pub fn ray_tracing_pipeline_shader_group_handle_capture_replay(
            mut self,
            ray_tracing_pipeline_shader_group_handle_capture_replay: Bool32,
        ) -> Self {
            self.ray_tracing_pipeline_shader_group_handle_capture_replay =
                ray_tracing_pipeline_shader_group_handle_capture_replay;
            self
        }
        pub fn ray_tracing_pipeline_shader_group_handle_capture_replay_mixed(
            mut self,
            ray_tracing_pipeline_shader_group_handle_capture_replay_mixed: Bool32,
        ) -> Self {
            self.ray_tracing_pipeline_shader_group_handle_capture_replay_mixed =
                ray_tracing_pipeline_shader_group_handle_capture_replay_mixed;
            self
        }
        pub fn ray_tracing_pipeline_trace_rays_indirect(
            mut self,
            ray_tracing_pipeline_trace_rays_indirect: Bool32,
        ) -> Self {
            self.ray_tracing_pipeline_trace_rays_indirect =
                ray_tracing_pipeline_trace_rays_indirect;
            self
        }
        pub fn ray_traversal_primitive_culling(
            mut self,
            ray_traversal_primitive_culling: Bool32,
        ) -> Self {
            self.ray_traversal_primitive_culling = ray_traversal_primitive_culling;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceRayTracingPipelinePropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_group_handle_size: u32,
        pub max_ray_recursion_depth: u32,
        pub max_shader_group_stride: u32,
        pub shader_group_base_alignment: u32,
        pub shader_group_handle_capture_replay_size: u32,
        pub max_ray_dispatch_invocation_count: u32,
        pub shader_group_handle_alignment: u32,
        pub max_ray_hit_attribute_size: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRayTracingPipelinePropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceRayTracingPipelinePropertiesKHR<'a>
    {
    }
    impl Default for PhysicalDeviceRayTracingPipelinePropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_group_handle_size: Default::default(),
                max_ray_recursion_depth: Default::default(),
                max_shader_group_stride: Default::default(),
                shader_group_base_alignment: Default::default(),
                shader_group_handle_capture_replay_size: Default::default(),
                max_ray_dispatch_invocation_count: Default::default(),
                shader_group_handle_alignment: Default::default(),
                max_ray_hit_attribute_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceRayTracingPipelinePropertiesKHR<'a> {
        pub fn shader_group_handle_size(mut self, shader_group_handle_size: u32) -> Self {
            self.shader_group_handle_size = shader_group_handle_size;
            self
        }
        pub fn max_ray_recursion_depth(mut self, max_ray_recursion_depth: u32) -> Self {
            self.max_ray_recursion_depth = max_ray_recursion_depth;
            self
        }
        pub fn max_shader_group_stride(mut self, max_shader_group_stride: u32) -> Self {
            self.max_shader_group_stride = max_shader_group_stride;
            self
        }
        pub fn shader_group_base_alignment(mut self, shader_group_base_alignment: u32) -> Self {
            self.shader_group_base_alignment = shader_group_base_alignment;
            self
        }
        pub fn shader_group_handle_capture_replay_size(
            mut self,
            shader_group_handle_capture_replay_size: u32,
        ) -> Self {
            self.shader_group_handle_capture_replay_size = shader_group_handle_capture_replay_size;
            self
        }
        pub fn max_ray_dispatch_invocation_count(
            mut self,
            max_ray_dispatch_invocation_count: u32,
        ) -> Self {
            self.max_ray_dispatch_invocation_count = max_ray_dispatch_invocation_count;
            self
        }
        pub fn shader_group_handle_alignment(mut self, shader_group_handle_alignment: u32) -> Self {
            self.shader_group_handle_alignment = shader_group_handle_alignment;
            self
        }
        pub fn max_ray_hit_attribute_size(mut self, max_ray_hit_attribute_size: u32) -> Self {
            self.max_ray_hit_attribute_size = max_ray_hit_attribute_size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct StridedDeviceAddressRegionKHR {
        pub device_address: DeviceAddress,
        pub stride: DeviceSize,
        pub size: DeviceSize,
    }
    impl StridedDeviceAddressRegionKHR {
        pub fn device_address(mut self, device_address: DeviceAddress) -> Self {
            self.device_address = device_address;
            self
        }
        pub fn stride(mut self, stride: DeviceSize) -> Self {
            self.stride = stride;
            self
        }
        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct TraceRaysIndirectCommandKHR {
        pub width: u32,
        pub height: u32,
        pub depth: u32,
    }
    impl TraceRaysIndirectCommandKHR {
        pub fn width(mut self, width: u32) -> Self {
            self.width = width;
            self
        }
        pub fn height(mut self, height: u32) -> Self {
            self.height = height;
            self
        }
        pub fn depth(mut self, depth: u32) -> Self {
            self.depth = depth;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct RayTracingPipelineInterfaceCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub max_pipeline_ray_payload_size: u32,
        pub max_pipeline_ray_hit_attribute_size: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for RayTracingPipelineInterfaceCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR;
    }
    impl Default for RayTracingPipelineInterfaceCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                max_pipeline_ray_payload_size: Default::default(),
                max_pipeline_ray_hit_attribute_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RayTracingPipelineInterfaceCreateInfoKHR<'a> {
        pub fn max_pipeline_ray_payload_size(mut self, max_pipeline_ray_payload_size: u32) -> Self {
            self.max_pipeline_ray_payload_size = max_pipeline_ray_payload_size;
            self
        }
        pub fn max_pipeline_ray_hit_attribute_size(
            mut self,
            max_pipeline_ray_hit_attribute_size: u32,
        ) -> Self {
            self.max_pipeline_ray_hit_attribute_size = max_pipeline_ray_hit_attribute_size;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RayTracingShaderGroupTypeKHR(i32);
    impl RayTracingShaderGroupTypeKHR {
        pub const GENERAL_KHR: Self = Self(0);
        pub const TRIANGLES_HIT_GROUP_KHR: Self = Self(1);
        pub const PROCEDURAL_HIT_GROUP_KHR: Self = Self(2);
        pub const GENERAL_NV: Self = Self::GENERAL_KHR;
        pub const PROCEDURAL_HIT_GROUP_NV: Self = Self::PROCEDURAL_HIT_GROUP_KHR;
        pub const TRIANGLES_HIT_GROUP_NV: Self = Self::TRIANGLES_HIT_GROUP_KHR;
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ShaderGroupShaderKHR(i32);
    impl ShaderGroupShaderKHR {
        pub const GENERAL_KHR: Self = Self(0);
        pub const CLOSEST_HIT_KHR: Self = Self(1);
        pub const ANY_HIT_KHR: Self = Self(2);
        pub const INTERSECTION_KHR: Self = Self(3);
    }
    pub type PFN_vkCmdTraceRaysKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    );
    pub type PFN_vkGetRayTracingShaderGroupHandlesKHR = unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    )
        -> vk::Result;
    pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR =
        unsafe extern "system" fn(
            device: Device,
            pipeline: Pipeline,
            first_group: u32,
            group_count: u32,
            data_size: usize,
            p_data: *mut c_void,
        ) -> vk::Result;
    pub type PFN_vkCreateRayTracingPipelinesKHR = unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const RayTracingPipelineCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> vk::Result;
    pub type PFN_vkCmdTraceRaysIndirectKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        indirect_device_address: DeviceAddress,
    );
    pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR = unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        group: u32,
        group_shader: ShaderGroupShaderKHR,
    )
        -> DeviceSize;
    pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR =
        unsafe extern "system" fn(command_buffer: CommandBuffer, pipeline_stack_size: u32);
}
pub struct DeviceFn {
    cmd_trace_rays_khr: PFN_vkCmdTraceRaysKHR,
    create_ray_tracing_pipelines_khr: PFN_vkCreateRayTracingPipelinesKHR,
    get_ray_tracing_shader_group_handles_khr: PFN_vkGetRayTracingShaderGroupHandlesKHR,
    get_ray_tracing_capture_replay_shader_group_handles_khr:
        PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR,
    cmd_trace_rays_indirect_khr: PFN_vkCmdTraceRaysIndirectKHR,
    get_ray_tracing_shader_group_stack_size_khr: PFN_vkGetRayTracingShaderGroupStackSizeKHR,
    cmd_set_ray_tracing_pipeline_stack_size_khr: PFN_vkCmdSetRayTracingPipelineStackSizeKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_trace_rays_khr: transmute(load(c"vkCmdTraceRaysKHR").ok_or(LoadingError)?),
                create_ray_tracing_pipelines_khr: transmute(
                    load(c"vkCreateRayTracingPipelinesKHR").ok_or(LoadingError)?,
                ),
                get_ray_tracing_shader_group_handles_khr: transmute(
                    load(c"vkGetRayTracingShaderGroupHandlesKHR").ok_or(LoadingError)?,
                ),
                get_ray_tracing_capture_replay_shader_group_handles_khr: transmute(
                    load(c"vkGetRayTracingCaptureReplayShaderGroupHandlesKHR")
                        .ok_or(LoadingError)?,
                ),
                cmd_trace_rays_indirect_khr: transmute(
                    load(c"vkCmdTraceRaysIndirectKHR").ok_or(LoadingError)?,
                ),
                get_ray_tracing_shader_group_stack_size_khr: transmute(
                    load(c"vkGetRayTracingShaderGroupStackSizeKHR").ok_or(LoadingError)?,
                ),
                cmd_set_ray_tracing_pipeline_stack_size_khr: transmute(
                    load(c"vkCmdSetRayTracingPipelineStackSizeKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_trace_rays_khr(
        &self,
        command_buffer: CommandBuffer,
        raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        unsafe {
            (self.cmd_trace_rays_khr)(
                command_buffer,
                raygen_shader_binding_table,
                miss_shader_binding_table,
                hit_shader_binding_table,
                callable_shader_binding_table,
                width,
                height,
                depth,
            )
        }
    }
    pub unsafe fn create_ray_tracing_pipelines_khr(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_infos: &[RayTracingPipelineCreateInfoKHR<'_>],
        allocator: Option<&AllocationCallbacks<'_>>,
        pipelines: &mut [Pipeline],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.create_ray_tracing_pipelines_khr)(
                device,
                deferred_operation,
                pipeline_cache,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                pipelines.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::OPERATION_DEFERRED_KHR => Ok(()),
                VkResult::OPERATION_NOT_DEFERRED_KHR => Ok(()),
                VkResult::PIPELINE_COMPILE_REQUIRED_EXT => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_ray_tracing_shader_group_handles_khr(
        &self,
        device: Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data: &mut [u8],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_ray_tracing_shader_group_handles_khr)(
                device,
                pipeline,
                first_group,
                group_count,
                data.len().try_into().unwrap(),
                data.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_ray_tracing_capture_replay_shader_group_handles_khr(
        &self,
        device: Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data: &mut [u8],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_ray_tracing_capture_replay_shader_group_handles_khr)(
                device,
                pipeline,
                first_group,
                group_count,
                data.len().try_into().unwrap(),
                data.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn cmd_trace_rays_indirect_khr(
        &self,
        command_buffer: CommandBuffer,
        raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        indirect_device_address: DeviceAddress,
    ) {
        unsafe {
            (self.cmd_trace_rays_indirect_khr)(
                command_buffer,
                raygen_shader_binding_table,
                miss_shader_binding_table,
                hit_shader_binding_table,
                callable_shader_binding_table,
                indirect_device_address,
            )
        }
    }
    pub unsafe fn get_ray_tracing_shader_group_stack_size_khr(
        &self,
        device: Device,
        pipeline: Pipeline,
        group: u32,
        group_shader: ShaderGroupShaderKHR,
    ) -> DeviceSize {
        unsafe {
            (self.get_ray_tracing_shader_group_stack_size_khr)(
                device,
                pipeline,
                group,
                group_shader,
            )
        }
    }
    pub unsafe fn cmd_set_ray_tracing_pipeline_stack_size_khr(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stack_size: u32,
    ) {
        unsafe {
            (self.cmd_set_ray_tracing_pipeline_stack_size_khr)(command_buffer, pipeline_stack_size)
        }
    }
}

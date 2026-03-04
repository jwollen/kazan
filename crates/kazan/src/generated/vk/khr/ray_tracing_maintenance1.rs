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
    #[derive(Copy, Clone, Default)]
    pub struct TraceRaysIndirectCommand2KHR {
        pub raygen_shader_record_address: DeviceAddress,
        pub raygen_shader_record_size: DeviceSize,
        pub miss_shader_binding_table_address: DeviceAddress,
        pub miss_shader_binding_table_size: DeviceSize,
        pub miss_shader_binding_table_stride: DeviceSize,
        pub hit_shader_binding_table_address: DeviceAddress,
        pub hit_shader_binding_table_size: DeviceSize,
        pub hit_shader_binding_table_stride: DeviceSize,
        pub callable_shader_binding_table_address: DeviceAddress,
        pub callable_shader_binding_table_size: DeviceSize,
        pub callable_shader_binding_table_stride: DeviceSize,
        pub width: u32,
        pub height: u32,
        pub depth: u32,
    }
    impl TraceRaysIndirectCommand2KHR {
        pub fn raygen_shader_record_address(
            mut self,
            raygen_shader_record_address: DeviceAddress,
        ) -> Self {
            self.raygen_shader_record_address = raygen_shader_record_address;
            self
        }
        pub fn raygen_shader_record_size(mut self, raygen_shader_record_size: DeviceSize) -> Self {
            self.raygen_shader_record_size = raygen_shader_record_size;
            self
        }
        pub fn miss_shader_binding_table_address(
            mut self,
            miss_shader_binding_table_address: DeviceAddress,
        ) -> Self {
            self.miss_shader_binding_table_address = miss_shader_binding_table_address;
            self
        }
        pub fn miss_shader_binding_table_size(
            mut self,
            miss_shader_binding_table_size: DeviceSize,
        ) -> Self {
            self.miss_shader_binding_table_size = miss_shader_binding_table_size;
            self
        }
        pub fn miss_shader_binding_table_stride(
            mut self,
            miss_shader_binding_table_stride: DeviceSize,
        ) -> Self {
            self.miss_shader_binding_table_stride = miss_shader_binding_table_stride;
            self
        }
        pub fn hit_shader_binding_table_address(
            mut self,
            hit_shader_binding_table_address: DeviceAddress,
        ) -> Self {
            self.hit_shader_binding_table_address = hit_shader_binding_table_address;
            self
        }
        pub fn hit_shader_binding_table_size(
            mut self,
            hit_shader_binding_table_size: DeviceSize,
        ) -> Self {
            self.hit_shader_binding_table_size = hit_shader_binding_table_size;
            self
        }
        pub fn hit_shader_binding_table_stride(
            mut self,
            hit_shader_binding_table_stride: DeviceSize,
        ) -> Self {
            self.hit_shader_binding_table_stride = hit_shader_binding_table_stride;
            self
        }
        pub fn callable_shader_binding_table_address(
            mut self,
            callable_shader_binding_table_address: DeviceAddress,
        ) -> Self {
            self.callable_shader_binding_table_address = callable_shader_binding_table_address;
            self
        }
        pub fn callable_shader_binding_table_size(
            mut self,
            callable_shader_binding_table_size: DeviceSize,
        ) -> Self {
            self.callable_shader_binding_table_size = callable_shader_binding_table_size;
            self
        }
        pub fn callable_shader_binding_table_stride(
            mut self,
            callable_shader_binding_table_stride: DeviceSize,
        ) -> Self {
            self.callable_shader_binding_table_stride = callable_shader_binding_table_stride;
            self
        }
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
    pub struct PhysicalDeviceRayTracingMaintenance1FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ray_tracing_maintenance1: Bool32,
        pub ray_tracing_pipeline_trace_rays_indirect2: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRayTracingMaintenance1FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceRayTracingMaintenance1FeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceRayTracingMaintenance1FeaturesKHR<'a>
    {
    }
    impl Default for PhysicalDeviceRayTracingMaintenance1FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                ray_tracing_maintenance1: Default::default(),
                ray_tracing_pipeline_trace_rays_indirect2: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceRayTracingMaintenance1FeaturesKHR<'a> {
        pub fn ray_tracing_maintenance1(mut self, ray_tracing_maintenance1: Bool32) -> Self {
            self.ray_tracing_maintenance1 = ray_tracing_maintenance1;
            self
        }
        pub fn ray_tracing_pipeline_trace_rays_indirect2(
            mut self,
            ray_tracing_pipeline_trace_rays_indirect2: Bool32,
        ) -> Self {
            self.ray_tracing_pipeline_trace_rays_indirect2 =
                ray_tracing_pipeline_trace_rays_indirect2;
            self
        }
    }
    pub type PFN_vkCmdTraceRaysIndirect2KHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        indirect_device_address: DeviceAddress,
    );
}
pub struct DeviceFn {
    cmd_trace_rays_indirect2_khr: Option<PFN_vkCmdTraceRaysIndirect2KHR>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_trace_rays_indirect2_khr: transmute(load(c"vkCmdTraceRaysIndirect2KHR")),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_trace_rays_indirect2_khr(
        &self,
        command_buffer: CommandBuffer,
        indirect_device_address: DeviceAddress,
    ) {
        unsafe {
            (self.cmd_trace_rays_indirect2_khr.unwrap())(command_buffer, indirect_device_address)
        }
    }
}

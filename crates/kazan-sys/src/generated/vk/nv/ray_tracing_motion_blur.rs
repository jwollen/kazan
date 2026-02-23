#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_motion_blur: Bool32,
    pub ray_tracing_motion_blur_pipeline_trace_rays_indirect: Bool32,
}
#[repr(C)]
pub struct AccelerationStructureGeometryMotionTrianglesDataNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_data: DeviceOrHostAddressConstKHR,
}
#[repr(C)]
pub struct AccelerationStructureMotionInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_instances: u32,
    pub flags: AccelerationStructureMotionInfoFlagsNV,
}
#[repr(C)]
pub struct SRTDataNV {
    pub sx: f32,
    pub a: f32,
    pub b: f32,
    pub pvx: f32,
    pub sy: f32,
    pub c: f32,
    pub pvy: f32,
    pub sz: f32,
    pub pvz: f32,
    pub qx: f32,
    pub qy: f32,
    pub qz: f32,
    pub qw: f32,
    pub tx: f32,
    pub ty: f32,
    pub tz: f32,
}
#[repr(C)]
pub struct AccelerationStructureSRTMotionInstanceNV {
    pub transform_t0: SRTDataNV,
    pub transform_t1: SRTDataNV,
    pub instance_custom_index: u32,
    pub mask: u32,
    pub instance_shader_binding_table_record_offset: u32,
    pub flags: GeometryInstanceFlagsKHR,
    pub acceleration_structure_reference: u64,
}
#[repr(C)]
pub struct AccelerationStructureMatrixMotionInstanceNV {
    pub transform_t0: TransformMatrixKHR,
    pub transform_t1: TransformMatrixKHR,
    pub instance_custom_index: u32,
    pub mask: u32,
    pub instance_shader_binding_table_record_offset: u32,
    pub flags: GeometryInstanceFlagsKHR,
    pub acceleration_structure_reference: u64,
}
#[repr(C)]
pub struct AccelerationStructureMotionInstanceNV {
    pub ty: AccelerationStructureMotionInstanceTypeNV,
    pub flags: AccelerationStructureMotionInstanceFlagsNV,
    pub data: AccelerationStructureMotionInstanceDataNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union AccelerationStructureMotionInstanceDataNV {
    pub static_instance: AccelerationStructureInstanceKHR,
    pub matrix_motion_instance: AccelerationStructureMatrixMotionInstanceNV,
    pub srt_motion_instance: AccelerationStructureSRTMotionInstanceNV,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccelerationStructureMotionInstanceTypeNV(i32);
impl AccelerationStructureMotionInstanceTypeNV {
    pub const STATIC_NV: Self = Self(0);
    pub const MATRIX_MOTION_NV: Self = Self(1);
    pub const SRT_MOTION_NV: Self = Self(2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct AccelerationStructureMotionInfoFlagsNV: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct AccelerationStructureMotionInstanceFlagsNV: Flags {
    }
}

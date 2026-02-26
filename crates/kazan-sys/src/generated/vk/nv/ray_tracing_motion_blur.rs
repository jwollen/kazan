#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRayTracingMotionBlurFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_motion_blur: Bool32,
    pub ray_tracing_motion_blur_pipeline_trace_rays_indirect: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceRayTracingMotionBlurFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            ray_tracing_motion_blur: Default::default(),
            ray_tracing_motion_blur_pipeline_trace_rays_indirect: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureGeometryMotionTrianglesDataNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_data: DeviceOrHostAddressConstKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AccelerationStructureGeometryMotionTrianglesDataNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV,
            p_next: core::ptr::null(),
            vertex_data: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureMotionInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_instances: u32,
    pub flags: AccelerationStructureMotionInfoFlagsNV,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AccelerationStructureMotionInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ACCELERATION_STRUCTURE_MOTION_INFO_NV,
            p_next: core::ptr::null(),
            max_instances: Default::default(),
            flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
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
#[derive(Copy, Clone, Default)]
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
#[derive(Copy, Clone)]
pub struct AccelerationStructureMatrixMotionInstanceNV {
    pub transform_t0: TransformMatrixKHR,
    pub transform_t1: TransformMatrixKHR,
    pub instance_custom_index: u32,
    pub mask: u32,
    pub instance_shader_binding_table_record_offset: u32,
    pub flags: GeometryInstanceFlagsKHR,
    pub acceleration_structure_reference: u64,
}
impl Default for AccelerationStructureMatrixMotionInstanceNV {
    fn default() -> Self {
        Self {
            transform_t0: Default::default(),
            transform_t1: Default::default(),
            instance_custom_index: Default::default(),
            mask: Default::default(),
            instance_shader_binding_table_record_offset: Default::default(),
            flags: Default::default(),
            acceleration_structure_reference: Default::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureMotionInstanceNV {
    pub ty: AccelerationStructureMotionInstanceTypeNV,
    pub flags: AccelerationStructureMotionInstanceFlagsNV,
    pub data: AccelerationStructureMotionInstanceDataNV,
}
impl Default for AccelerationStructureMotionInstanceNV {
    fn default() -> Self {
        Self {
            ty: Default::default(),
            flags: Default::default(),
            data: Default::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union AccelerationStructureMotionInstanceDataNV {
    pub static_instance: AccelerationStructureInstanceKHR,
    pub matrix_motion_instance: AccelerationStructureMatrixMotionInstanceNV,
    pub srt_motion_instance: AccelerationStructureSRTMotionInstanceNV,
}
impl Default for AccelerationStructureMotionInstanceDataNV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccelerationStructureMotionInfoFlagsNV: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccelerationStructureMotionInstanceFlagsNV: Flags {
    }
}

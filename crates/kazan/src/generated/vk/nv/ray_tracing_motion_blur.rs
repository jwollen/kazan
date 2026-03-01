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
    pub struct PhysicalDeviceRayTracingMotionBlurFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ray_tracing_motion_blur: Bool32,
        pub ray_tracing_motion_blur_pipeline_trace_rays_indirect: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRayTracingMotionBlurFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceRayTracingMotionBlurFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceRayTracingMotionBlurFeaturesNV<'a> {}
    impl Default for PhysicalDeviceRayTracingMotionBlurFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                ray_tracing_motion_blur: Default::default(),
                ray_tracing_motion_blur_pipeline_trace_rays_indirect: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceRayTracingMotionBlurFeaturesNV<'a> {
        pub fn ray_tracing_motion_blur(mut self, ray_tracing_motion_blur: Bool32) -> Self {
            self.ray_tracing_motion_blur = ray_tracing_motion_blur;
            self
        }
        pub fn ray_tracing_motion_blur_pipeline_trace_rays_indirect(
            mut self,
            ray_tracing_motion_blur_pipeline_trace_rays_indirect: Bool32,
        ) -> Self {
            self.ray_tracing_motion_blur_pipeline_trace_rays_indirect =
                ray_tracing_motion_blur_pipeline_trace_rays_indirect;
            self
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
    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureGeometryMotionTrianglesDataNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV;
    }
    unsafe impl<'a> Extends<AccelerationStructureGeometryTrianglesDataKHR<'a>>
        for AccelerationStructureGeometryMotionTrianglesDataNV<'a>
    {
    }
    impl Default for AccelerationStructureGeometryMotionTrianglesDataNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                vertex_data: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> AccelerationStructureGeometryMotionTrianglesDataNV<'a> {
        pub fn vertex_data(mut self, vertex_data: DeviceOrHostAddressConstKHR<'a>) -> Self {
            self.vertex_data = vertex_data;
            self
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
    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureMotionInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ACCELERATION_STRUCTURE_MOTION_INFO_NV;
    }
    unsafe impl<'a> Extends<AccelerationStructureCreateInfoKHR<'a>>
        for AccelerationStructureMotionInfoNV<'a>
    {
    }
    impl Default for AccelerationStructureMotionInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                max_instances: Default::default(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> AccelerationStructureMotionInfoNV<'a> {
        pub fn max_instances(mut self, max_instances: u32) -> Self {
            self.max_instances = max_instances;
            self
        }
        pub fn flags(mut self, flags: AccelerationStructureMotionInfoFlagsNV) -> Self {
            self.flags = flags;
            self
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
    impl SRTDataNV {
        pub fn sx(mut self, sx: f32) -> Self {
            self.sx = sx;
            self
        }
        pub fn a(mut self, a: f32) -> Self {
            self.a = a;
            self
        }
        pub fn b(mut self, b: f32) -> Self {
            self.b = b;
            self
        }
        pub fn pvx(mut self, pvx: f32) -> Self {
            self.pvx = pvx;
            self
        }
        pub fn sy(mut self, sy: f32) -> Self {
            self.sy = sy;
            self
        }
        pub fn c(mut self, c: f32) -> Self {
            self.c = c;
            self
        }
        pub fn pvy(mut self, pvy: f32) -> Self {
            self.pvy = pvy;
            self
        }
        pub fn sz(mut self, sz: f32) -> Self {
            self.sz = sz;
            self
        }
        pub fn pvz(mut self, pvz: f32) -> Self {
            self.pvz = pvz;
            self
        }
        pub fn qx(mut self, qx: f32) -> Self {
            self.qx = qx;
            self
        }
        pub fn qy(mut self, qy: f32) -> Self {
            self.qy = qy;
            self
        }
        pub fn qz(mut self, qz: f32) -> Self {
            self.qz = qz;
            self
        }
        pub fn qw(mut self, qw: f32) -> Self {
            self.qw = qw;
            self
        }
        pub fn tx(mut self, tx: f32) -> Self {
            self.tx = tx;
            self
        }
        pub fn ty(mut self, ty: f32) -> Self {
            self.ty = ty;
            self
        }
        pub fn tz(mut self, tz: f32) -> Self {
            self.tz = tz;
            self
        }
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
    impl AccelerationStructureSRTMotionInstanceNV {
        pub fn transform_t0(mut self, transform_t0: SRTDataNV) -> Self {
            self.transform_t0 = transform_t0;
            self
        }
        pub fn transform_t1(mut self, transform_t1: SRTDataNV) -> Self {
            self.transform_t1 = transform_t1;
            self
        }
        pub fn instance_custom_index(mut self, instance_custom_index: u32) -> Self {
            self.instance_custom_index = instance_custom_index;
            self
        }
        pub fn mask(mut self, mask: u32) -> Self {
            self.mask = mask;
            self
        }
        pub fn instance_shader_binding_table_record_offset(
            mut self,
            instance_shader_binding_table_record_offset: u32,
        ) -> Self {
            self.instance_shader_binding_table_record_offset =
                instance_shader_binding_table_record_offset;
            self
        }
        pub fn flags(mut self, flags: GeometryInstanceFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn acceleration_structure_reference(
            mut self,
            acceleration_structure_reference: u64,
        ) -> Self {
            self.acceleration_structure_reference = acceleration_structure_reference;
            self
        }
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
    impl AccelerationStructureMatrixMotionInstanceNV {
        pub fn transform_t0(mut self, transform_t0: TransformMatrixKHR) -> Self {
            self.transform_t0 = transform_t0;
            self
        }
        pub fn transform_t1(mut self, transform_t1: TransformMatrixKHR) -> Self {
            self.transform_t1 = transform_t1;
            self
        }
        pub fn instance_custom_index(mut self, instance_custom_index: u32) -> Self {
            self.instance_custom_index = instance_custom_index;
            self
        }
        pub fn mask(mut self, mask: u32) -> Self {
            self.mask = mask;
            self
        }
        pub fn instance_shader_binding_table_record_offset(
            mut self,
            instance_shader_binding_table_record_offset: u32,
        ) -> Self {
            self.instance_shader_binding_table_record_offset =
                instance_shader_binding_table_record_offset;
            self
        }
        pub fn flags(mut self, flags: GeometryInstanceFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn acceleration_structure_reference(
            mut self,
            acceleration_structure_reference: u64,
        ) -> Self {
            self.acceleration_structure_reference = acceleration_structure_reference;
            self
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
    impl AccelerationStructureMotionInstanceNV {
        pub fn ty(mut self, ty: AccelerationStructureMotionInstanceTypeNV) -> Self {
            self.ty = ty;
            self
        }
        pub fn flags(mut self, flags: AccelerationStructureMotionInstanceFlagsNV) -> Self {
            self.flags = flags;
            self
        }
        pub fn data(mut self, data: AccelerationStructureMotionInstanceDataNV) -> Self {
            self.data = data;
            self
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
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct AccelerationStructureMotionInfoFlagsNV(Flags);
    vk_bitflags_wrapped!(AccelerationStructureMotionInfoFlagsNV, Flags);
    impl AccelerationStructureMotionInfoFlagsNV {}
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct AccelerationStructureMotionInstanceFlagsNV(Flags);
    vk_bitflags_wrapped!(AccelerationStructureMotionInstanceFlagsNV, Flags);
    impl AccelerationStructureMotionInstanceFlagsNV {}
}

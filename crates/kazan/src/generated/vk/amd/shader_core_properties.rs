//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_AMD_shader_core_properties.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_AMD_shader_core_properties";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderCorePropertiesAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderCorePropertiesAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_engine_count: u32,
        pub shader_arrays_per_engine_count: u32,
        pub compute_units_per_shader_array: u32,
        pub simd_per_compute_unit: u32,
        pub wavefronts_per_simd: u32,
        pub wavefront_size: u32,
        pub sgprs_per_simd: u32,
        pub min_sgpr_allocation: u32,
        pub max_sgpr_allocation: u32,
        pub sgpr_allocation_granularity: u32,
        pub vgprs_per_simd: u32,
        pub min_vgpr_allocation: u32,
        pub max_vgpr_allocation: u32,
        pub vgpr_allocation_granularity: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderCorePropertiesAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderCorePropertiesAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_engine_count", &self.shader_engine_count)
                .field(
                    "shader_arrays_per_engine_count",
                    &self.shader_arrays_per_engine_count,
                )
                .field(
                    "compute_units_per_shader_array",
                    &self.compute_units_per_shader_array,
                )
                .field("simd_per_compute_unit", &self.simd_per_compute_unit)
                .field("wavefronts_per_simd", &self.wavefronts_per_simd)
                .field("wavefront_size", &self.wavefront_size)
                .field("sgprs_per_simd", &self.sgprs_per_simd)
                .field("min_sgpr_allocation", &self.min_sgpr_allocation)
                .field("max_sgpr_allocation", &self.max_sgpr_allocation)
                .field(
                    "sgpr_allocation_granularity",
                    &self.sgpr_allocation_granularity,
                )
                .field("vgprs_per_simd", &self.vgprs_per_simd)
                .field("min_vgpr_allocation", &self.min_vgpr_allocation)
                .field("max_vgpr_allocation", &self.max_vgpr_allocation)
                .field(
                    "vgpr_allocation_granularity",
                    &self.vgpr_allocation_granularity,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderCorePropertiesAMD<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceShaderCorePropertiesAMD<'a>
    {
    }

    impl Default for PhysicalDeviceShaderCorePropertiesAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_engine_count: Default::default(),
                shader_arrays_per_engine_count: Default::default(),
                compute_units_per_shader_array: Default::default(),
                simd_per_compute_unit: Default::default(),
                wavefronts_per_simd: Default::default(),
                wavefront_size: Default::default(),
                sgprs_per_simd: Default::default(),
                min_sgpr_allocation: Default::default(),
                max_sgpr_allocation: Default::default(),
                sgpr_allocation_granularity: Default::default(),
                vgprs_per_simd: Default::default(),
                min_vgpr_allocation: Default::default(),
                max_vgpr_allocation: Default::default(),
                vgpr_allocation_granularity: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderCorePropertiesAMD<'a> {
        #[inline]
        pub fn shader_engine_count(mut self, shader_engine_count: u32) -> Self {
            self.shader_engine_count = shader_engine_count;
            self
        }

        #[inline]
        pub fn shader_arrays_per_engine_count(
            mut self,
            shader_arrays_per_engine_count: u32,
        ) -> Self {
            self.shader_arrays_per_engine_count = shader_arrays_per_engine_count;
            self
        }

        #[inline]
        pub fn compute_units_per_shader_array(
            mut self,
            compute_units_per_shader_array: u32,
        ) -> Self {
            self.compute_units_per_shader_array = compute_units_per_shader_array;
            self
        }

        #[inline]
        pub fn simd_per_compute_unit(mut self, simd_per_compute_unit: u32) -> Self {
            self.simd_per_compute_unit = simd_per_compute_unit;
            self
        }

        #[inline]
        pub fn wavefronts_per_simd(mut self, wavefronts_per_simd: u32) -> Self {
            self.wavefronts_per_simd = wavefronts_per_simd;
            self
        }

        #[inline]
        pub fn wavefront_size(mut self, wavefront_size: u32) -> Self {
            self.wavefront_size = wavefront_size;
            self
        }

        #[inline]
        pub fn sgprs_per_simd(mut self, sgprs_per_simd: u32) -> Self {
            self.sgprs_per_simd = sgprs_per_simd;
            self
        }

        #[inline]
        pub fn min_sgpr_allocation(mut self, min_sgpr_allocation: u32) -> Self {
            self.min_sgpr_allocation = min_sgpr_allocation;
            self
        }

        #[inline]
        pub fn max_sgpr_allocation(mut self, max_sgpr_allocation: u32) -> Self {
            self.max_sgpr_allocation = max_sgpr_allocation;
            self
        }

        #[inline]
        pub fn sgpr_allocation_granularity(mut self, sgpr_allocation_granularity: u32) -> Self {
            self.sgpr_allocation_granularity = sgpr_allocation_granularity;
            self
        }

        #[inline]
        pub fn vgprs_per_simd(mut self, vgprs_per_simd: u32) -> Self {
            self.vgprs_per_simd = vgprs_per_simd;
            self
        }

        #[inline]
        pub fn min_vgpr_allocation(mut self, min_vgpr_allocation: u32) -> Self {
            self.min_vgpr_allocation = min_vgpr_allocation;
            self
        }

        #[inline]
        pub fn max_vgpr_allocation(mut self, max_vgpr_allocation: u32) -> Self {
            self.max_vgpr_allocation = max_vgpr_allocation;
            self
        }

        #[inline]
        pub fn vgpr_allocation_granularity(mut self, vgpr_allocation_granularity: u32) -> Self {
            self.vgpr_allocation_granularity = vgpr_allocation_granularity;
            self
        }
    }
}

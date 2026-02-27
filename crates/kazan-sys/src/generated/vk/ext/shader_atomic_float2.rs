#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_buffer_float16_atomics: Bool32,
    pub shader_buffer_float16_atomic_add: Bool32,
    pub shader_buffer_float16_atomic_min_max: Bool32,
    pub shader_buffer_float32_atomic_min_max: Bool32,
    pub shader_buffer_float64_atomic_min_max: Bool32,
    pub shader_shared_float16_atomics: Bool32,
    pub shader_shared_float16_atomic_add: Bool32,
    pub shader_shared_float16_atomic_min_max: Bool32,
    pub shader_shared_float32_atomic_min_max: Bool32,
    pub shader_shared_float64_atomic_min_max: Bool32,
    pub shader_image_float32_atomic_min_max: Bool32,
    pub sparse_image_float32_atomic_min_max: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            shader_buffer_float16_atomics: Default::default(),
            shader_buffer_float16_atomic_add: Default::default(),
            shader_buffer_float16_atomic_min_max: Default::default(),
            shader_buffer_float32_atomic_min_max: Default::default(),
            shader_buffer_float64_atomic_min_max: Default::default(),
            shader_shared_float16_atomics: Default::default(),
            shader_shared_float16_atomic_add: Default::default(),
            shader_shared_float16_atomic_min_max: Default::default(),
            shader_shared_float32_atomic_min_max: Default::default(),
            shader_shared_float64_atomic_min_max: Default::default(),
            shader_image_float32_atomic_min_max: Default::default(),
            sparse_image_float32_atomic_min_max: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'a> {
    pub fn shader_buffer_float16_atomics(mut self, shader_buffer_float16_atomics: Bool32) -> Self {
        self.shader_buffer_float16_atomics = shader_buffer_float16_atomics;
        self
    }
    pub fn shader_buffer_float16_atomic_add(
        mut self,
        shader_buffer_float16_atomic_add: Bool32,
    ) -> Self {
        self.shader_buffer_float16_atomic_add = shader_buffer_float16_atomic_add;
        self
    }
    pub fn shader_buffer_float16_atomic_min_max(
        mut self,
        shader_buffer_float16_atomic_min_max: Bool32,
    ) -> Self {
        self.shader_buffer_float16_atomic_min_max = shader_buffer_float16_atomic_min_max;
        self
    }
    pub fn shader_buffer_float32_atomic_min_max(
        mut self,
        shader_buffer_float32_atomic_min_max: Bool32,
    ) -> Self {
        self.shader_buffer_float32_atomic_min_max = shader_buffer_float32_atomic_min_max;
        self
    }
    pub fn shader_buffer_float64_atomic_min_max(
        mut self,
        shader_buffer_float64_atomic_min_max: Bool32,
    ) -> Self {
        self.shader_buffer_float64_atomic_min_max = shader_buffer_float64_atomic_min_max;
        self
    }
    pub fn shader_shared_float16_atomics(mut self, shader_shared_float16_atomics: Bool32) -> Self {
        self.shader_shared_float16_atomics = shader_shared_float16_atomics;
        self
    }
    pub fn shader_shared_float16_atomic_add(
        mut self,
        shader_shared_float16_atomic_add: Bool32,
    ) -> Self {
        self.shader_shared_float16_atomic_add = shader_shared_float16_atomic_add;
        self
    }
    pub fn shader_shared_float16_atomic_min_max(
        mut self,
        shader_shared_float16_atomic_min_max: Bool32,
    ) -> Self {
        self.shader_shared_float16_atomic_min_max = shader_shared_float16_atomic_min_max;
        self
    }
    pub fn shader_shared_float32_atomic_min_max(
        mut self,
        shader_shared_float32_atomic_min_max: Bool32,
    ) -> Self {
        self.shader_shared_float32_atomic_min_max = shader_shared_float32_atomic_min_max;
        self
    }
    pub fn shader_shared_float64_atomic_min_max(
        mut self,
        shader_shared_float64_atomic_min_max: Bool32,
    ) -> Self {
        self.shader_shared_float64_atomic_min_max = shader_shared_float64_atomic_min_max;
        self
    }
    pub fn shader_image_float32_atomic_min_max(
        mut self,
        shader_image_float32_atomic_min_max: Bool32,
    ) -> Self {
        self.shader_image_float32_atomic_min_max = shader_image_float32_atomic_min_max;
        self
    }
    pub fn sparse_image_float32_atomic_min_max(
        mut self,
        sparse_image_float32_atomic_min_max: Bool32,
    ) -> Self {
        self.sparse_image_float32_atomic_min_max = sparse_image_float32_atomic_min_max;
        self
    }
}

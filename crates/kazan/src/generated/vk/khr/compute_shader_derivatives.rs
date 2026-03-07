#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_compute_shader_derivatives";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceComputeShaderDerivativesFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceComputeShaderDerivativesFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub compute_derivative_group_quads: Bool32,
        pub compute_derivative_group_linear: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceComputeShaderDerivativesFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceComputeShaderDerivativesFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "compute_derivative_group_quads",
                    &self.compute_derivative_group_quads,
                )
                .field(
                    "compute_derivative_group_linear",
                    &self.compute_derivative_group_linear,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceComputeShaderDerivativesFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceComputeShaderDerivativesFeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceComputeShaderDerivativesFeaturesKHR<'a>
    {
    }

    impl Default for PhysicalDeviceComputeShaderDerivativesFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                compute_derivative_group_quads: Default::default(),
                compute_derivative_group_linear: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceComputeShaderDerivativesFeaturesKHR<'a> {
        pub fn compute_derivative_group_quads(
            mut self,
            compute_derivative_group_quads: bool,
        ) -> Self {
            self.compute_derivative_group_quads = compute_derivative_group_quads.into();
            self
        }

        pub fn compute_derivative_group_linear(
            mut self,
            compute_derivative_group_linear: bool,
        ) -> Self {
            self.compute_derivative_group_linear = compute_derivative_group_linear.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceComputeShaderDerivativesPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceComputeShaderDerivativesPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub mesh_and_task_shader_derivatives: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceComputeShaderDerivativesPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceComputeShaderDerivativesPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "mesh_and_task_shader_derivatives",
                    &self.mesh_and_task_shader_derivatives,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceComputeShaderDerivativesPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_PROPERTIES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceComputeShaderDerivativesPropertiesKHR<'a>
    {
    }

    impl Default for PhysicalDeviceComputeShaderDerivativesPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                mesh_and_task_shader_derivatives: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceComputeShaderDerivativesPropertiesKHR<'a> {
        pub fn mesh_and_task_shader_derivatives(
            mut self,
            mesh_and_task_shader_derivatives: bool,
        ) -> Self {
            self.mesh_and_task_shader_derivatives = mesh_and_task_shader_derivatives.into();
            self
        }
    }
}

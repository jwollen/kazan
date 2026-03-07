#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_primitives_generated_query";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub primitives_generated_query: Bool32,
        pub primitives_generated_query_with_rasterizer_discard: Bool32,
        pub primitives_generated_query_with_non_zero_streams: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "primitives_generated_query",
                    &self.primitives_generated_query,
                )
                .field(
                    "primitives_generated_query_with_rasterizer_discard",
                    &self.primitives_generated_query_with_rasterizer_discard,
                )
                .field(
                    "primitives_generated_query_with_non_zero_streams",
                    &self.primitives_generated_query_with_non_zero_streams,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                primitives_generated_query: Default::default(),
                primitives_generated_query_with_rasterizer_discard: Default::default(),
                primitives_generated_query_with_non_zero_streams: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT<'a> {
        pub fn primitives_generated_query(mut self, primitives_generated_query: bool) -> Self {
            self.primitives_generated_query = primitives_generated_query.into();
            self
        }

        pub fn primitives_generated_query_with_rasterizer_discard(
            mut self,
            primitives_generated_query_with_rasterizer_discard: bool,
        ) -> Self {
            self.primitives_generated_query_with_rasterizer_discard =
                primitives_generated_query_with_rasterizer_discard.into();
            self
        }

        pub fn primitives_generated_query_with_non_zero_streams(
            mut self,
            primitives_generated_query_with_non_zero_streams: bool,
        ) -> Self {
            self.primitives_generated_query_with_non_zero_streams =
                primitives_generated_query_with_non_zero_streams.into();
            self
        }
    }
}

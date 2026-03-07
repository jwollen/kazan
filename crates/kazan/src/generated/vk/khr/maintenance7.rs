#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_maintenance7";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance7FeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMaintenance7FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub maintenance7: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMaintenance7FeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMaintenance7FeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("maintenance7", &self.maintenance7)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance7FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceMaintenance7FeaturesKHR<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceMaintenance7FeaturesKHR<'a> {}

    impl Default for PhysicalDeviceMaintenance7FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                maintenance7: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMaintenance7FeaturesKHR<'a> {
        pub fn maintenance7(mut self, maintenance7: bool) -> Self {
            self.maintenance7 = maintenance7.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance7PropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMaintenance7PropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub robust_fragment_shading_rate_attachment_access: Bool32,
        pub separate_depth_stencil_attachment_access: Bool32,
        pub max_descriptor_set_total_uniform_buffers_dynamic: u32,
        pub max_descriptor_set_total_storage_buffers_dynamic: u32,
        pub max_descriptor_set_total_buffers_dynamic: u32,
        pub max_descriptor_set_update_after_bind_total_uniform_buffers_dynamic: u32,
        pub max_descriptor_set_update_after_bind_total_storage_buffers_dynamic: u32,
        pub max_descriptor_set_update_after_bind_total_buffers_dynamic: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMaintenance7PropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMaintenance7PropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "robust_fragment_shading_rate_attachment_access",
                    &self.robust_fragment_shading_rate_attachment_access,
                )
                .field(
                    "separate_depth_stencil_attachment_access",
                    &self.separate_depth_stencil_attachment_access,
                )
                .field(
                    "max_descriptor_set_total_uniform_buffers_dynamic",
                    &self.max_descriptor_set_total_uniform_buffers_dynamic,
                )
                .field(
                    "max_descriptor_set_total_storage_buffers_dynamic",
                    &self.max_descriptor_set_total_storage_buffers_dynamic,
                )
                .field(
                    "max_descriptor_set_total_buffers_dynamic",
                    &self.max_descriptor_set_total_buffers_dynamic,
                )
                .field(
                    "max_descriptor_set_update_after_bind_total_uniform_buffers_dynamic",
                    &self.max_descriptor_set_update_after_bind_total_uniform_buffers_dynamic,
                )
                .field(
                    "max_descriptor_set_update_after_bind_total_storage_buffers_dynamic",
                    &self.max_descriptor_set_update_after_bind_total_storage_buffers_dynamic,
                )
                .field(
                    "max_descriptor_set_update_after_bind_total_buffers_dynamic",
                    &self.max_descriptor_set_update_after_bind_total_buffers_dynamic,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance7PropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceMaintenance7PropertiesKHR<'a>
    {
    }

    impl Default for PhysicalDeviceMaintenance7PropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                robust_fragment_shading_rate_attachment_access: Default::default(),
                separate_depth_stencil_attachment_access: Default::default(),
                max_descriptor_set_total_uniform_buffers_dynamic: Default::default(),
                max_descriptor_set_total_storage_buffers_dynamic: Default::default(),
                max_descriptor_set_total_buffers_dynamic: Default::default(),
                max_descriptor_set_update_after_bind_total_uniform_buffers_dynamic:
                    Default::default(),
                max_descriptor_set_update_after_bind_total_storage_buffers_dynamic:
                    Default::default(),
                max_descriptor_set_update_after_bind_total_buffers_dynamic: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMaintenance7PropertiesKHR<'a> {
        pub fn robust_fragment_shading_rate_attachment_access(
            mut self,
            robust_fragment_shading_rate_attachment_access: bool,
        ) -> Self {
            self.robust_fragment_shading_rate_attachment_access =
                robust_fragment_shading_rate_attachment_access.into();
            self
        }

        pub fn separate_depth_stencil_attachment_access(
            mut self,
            separate_depth_stencil_attachment_access: bool,
        ) -> Self {
            self.separate_depth_stencil_attachment_access =
                separate_depth_stencil_attachment_access.into();
            self
        }

        pub fn max_descriptor_set_total_uniform_buffers_dynamic(
            mut self,
            max_descriptor_set_total_uniform_buffers_dynamic: u32,
        ) -> Self {
            self.max_descriptor_set_total_uniform_buffers_dynamic =
                max_descriptor_set_total_uniform_buffers_dynamic;
            self
        }

        pub fn max_descriptor_set_total_storage_buffers_dynamic(
            mut self,
            max_descriptor_set_total_storage_buffers_dynamic: u32,
        ) -> Self {
            self.max_descriptor_set_total_storage_buffers_dynamic =
                max_descriptor_set_total_storage_buffers_dynamic;
            self
        }

        pub fn max_descriptor_set_total_buffers_dynamic(
            mut self,
            max_descriptor_set_total_buffers_dynamic: u32,
        ) -> Self {
            self.max_descriptor_set_total_buffers_dynamic =
                max_descriptor_set_total_buffers_dynamic;
            self
        }

        pub fn max_descriptor_set_update_after_bind_total_uniform_buffers_dynamic(
            mut self,
            max_descriptor_set_update_after_bind_total_uniform_buffers_dynamic: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_total_uniform_buffers_dynamic =
                max_descriptor_set_update_after_bind_total_uniform_buffers_dynamic;
            self
        }

        pub fn max_descriptor_set_update_after_bind_total_storage_buffers_dynamic(
            mut self,
            max_descriptor_set_update_after_bind_total_storage_buffers_dynamic: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_total_storage_buffers_dynamic =
                max_descriptor_set_update_after_bind_total_storage_buffers_dynamic;
            self
        }

        pub fn max_descriptor_set_update_after_bind_total_buffers_dynamic(
            mut self,
            max_descriptor_set_update_after_bind_total_buffers_dynamic: u32,
        ) -> Self {
            self.max_descriptor_set_update_after_bind_total_buffers_dynamic =
                max_descriptor_set_update_after_bind_total_buffers_dynamic;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceLayeredApiPropertiesListKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceLayeredApiPropertiesListKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub layered_api_count: u32,
        pub p_layered_apis: *mut PhysicalDeviceLayeredApiPropertiesKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceLayeredApiPropertiesListKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceLayeredApiPropertiesListKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("layered_api_count", &self.layered_api_count)
                .field("p_layered_apis", &self.p_layered_apis)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceLayeredApiPropertiesListKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceLayeredApiPropertiesListKHR<'a>
    {
    }

    impl Default for PhysicalDeviceLayeredApiPropertiesListKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                layered_api_count: Default::default(),
                p_layered_apis: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceLayeredApiPropertiesListKHR<'a> {
        pub fn layered_apis(
            mut self,
            layered_apis: &'a mut [PhysicalDeviceLayeredApiPropertiesKHR<'a>],
        ) -> Self {
            self.layered_api_count = layered_apis.len().try_into().unwrap();
            self.p_layered_apis = layered_apis.as_mut_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceLayeredApiPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceLayeredApiPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub vendor_id: u32,
        pub device_id: u32,
        pub layered_api: PhysicalDeviceLayeredApiKHR,
        pub device_name: [c_char; MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceLayeredApiPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceLayeredApiPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("vendor_id", &self.vendor_id)
                .field("device_id", &self.device_id)
                .field("layered_api", &self.layered_api)
                .field(
                    "device_name",
                    &wrap_c_str_slice_until_nul(&self.device_name),
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceLayeredApiPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR;
    }

    impl Default for PhysicalDeviceLayeredApiPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                vendor_id: Default::default(),
                device_id: Default::default(),
                layered_api: Default::default(),
                device_name: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceLayeredApiPropertiesKHR<'a> {
        pub fn vendor_id(mut self, vendor_id: u32) -> Self {
            self.vendor_id = vendor_id;
            self
        }

        pub fn device_id(mut self, device_id: u32) -> Self {
            self.device_id = device_id;
            self
        }

        pub fn layered_api(mut self, layered_api: PhysicalDeviceLayeredApiKHR) -> Self {
            self.layered_api = layered_api;
            self
        }

        pub fn device_name(
            mut self,
            device_name: [c_char; MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
        ) -> Self {
            self.device_name = device_name;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceLayeredApiVulkanPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceLayeredApiVulkanPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub properties: PhysicalDeviceProperties2<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceLayeredApiVulkanPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceLayeredApiVulkanPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("properties", &self.properties)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceLayeredApiVulkanPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceLayeredApiPropertiesKHR<'a>>
        for PhysicalDeviceLayeredApiVulkanPropertiesKHR<'a>
    {
    }

    impl Default for PhysicalDeviceLayeredApiVulkanPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                properties: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceLayeredApiVulkanPropertiesKHR<'a> {
        pub fn properties(mut self, properties: PhysicalDeviceProperties2<'a>) -> Self {
            self.properties = properties;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceLayeredApiKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PhysicalDeviceLayeredApiKHR(i32);

    impl PhysicalDeviceLayeredApiKHR {
        pub const VULKAN_KHR: Self = Self(0);
        pub const D3D12_KHR: Self = Self(1);
        pub const METAL_KHR: Self = Self(2);
        pub const OPENGL_KHR: Self = Self(3);
        pub const OPENGLES_KHR: Self = Self(4);
    }

    impl fmt::Debug for PhysicalDeviceLayeredApiKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::VULKAN_KHR => Some("VULKAN_KHR"),
                Self::D3D12_KHR => Some("D3D12_KHR"),
                Self::METAL_KHR => Some("METAL_KHR"),
                Self::OPENGL_KHR => Some("OPENGL_KHR"),
                Self::OPENGLES_KHR => Some("OPENGLES_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
}

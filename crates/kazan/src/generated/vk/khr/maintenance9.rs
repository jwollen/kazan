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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance9FeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMaintenance9FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub maintenance9: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance9FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceMaintenance9FeaturesKHR<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceMaintenance9FeaturesKHR<'a> {}
    impl Default for PhysicalDeviceMaintenance9FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                maintenance9: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMaintenance9FeaturesKHR<'a> {
        pub fn maintenance9(mut self, maintenance9: bool) -> Self {
            self.maintenance9 = maintenance9.into();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance9PropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceMaintenance9PropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image2_d_view_of3_d_sparse: Bool32,
        pub default_vertex_attribute_value: DefaultVertexAttributeValueKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance9PropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceMaintenance9PropertiesKHR<'a>
    {
    }
    impl Default for PhysicalDeviceMaintenance9PropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                image2_d_view_of3_d_sparse: Default::default(),
                default_vertex_attribute_value: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceMaintenance9PropertiesKHR<'a> {
        pub fn image2_d_view_of3_d_sparse(mut self, image2_d_view_of3_d_sparse: bool) -> Self {
            self.image2_d_view_of3_d_sparse = image2_d_view_of3_d_sparse.into();
            self
        }
        pub fn default_vertex_attribute_value(
            mut self,
            default_vertex_attribute_value: DefaultVertexAttributeValueKHR,
        ) -> Self {
            self.default_vertex_attribute_value = default_vertex_attribute_value;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueFamilyOwnershipTransferPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct QueueFamilyOwnershipTransferPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub optimal_image_transfer_to_queue_families: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for QueueFamilyOwnershipTransferPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR;
    }
    unsafe impl<'a> Extends<QueueFamilyProperties2<'a>>
        for QueueFamilyOwnershipTransferPropertiesKHR<'a>
    {
    }
    impl Default for QueueFamilyOwnershipTransferPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                optimal_image_transfer_to_queue_families: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> QueueFamilyOwnershipTransferPropertiesKHR<'a> {
        pub fn optimal_image_transfer_to_queue_families(
            mut self,
            optimal_image_transfer_to_queue_families: u32,
        ) -> Self {
            self.optimal_image_transfer_to_queue_families =
                optimal_image_transfer_to_queue_families;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDefaultVertexAttributeValueKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DefaultVertexAttributeValueKHR(i32);
    impl DefaultVertexAttributeValueKHR {
        pub const ZERO_ZERO_ZERO_ZERO_KHR: Self = Self(0);
        pub const ZERO_ZERO_ZERO_ONE_KHR: Self = Self(1);
    }
    impl fmt::Debug for DefaultVertexAttributeValueKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ZERO_ZERO_ZERO_ZERO_KHR => Some("ZERO_ZERO_ZERO_ZERO_KHR"),
                Self::ZERO_ZERO_ZERO_ONE_KHR => Some("ZERO_ZERO_ZERO_ONE_KHR"),
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

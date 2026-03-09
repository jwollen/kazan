//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_VALVE_descriptor_set_host_mapping.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_VALVE_descriptor_set_host_mapping";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub descriptor_set_host_mapping: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "descriptor_set_host_mapping",
                    &self.descriptor_set_host_mapping,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'a>
    {
    }

    impl Default for PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                descriptor_set_host_mapping: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'a> {
        #[inline]
        pub fn descriptor_set_host_mapping(mut self, descriptor_set_host_mapping: bool) -> Self {
            self.descriptor_set_host_mapping = descriptor_set_host_mapping.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorSetBindingReferenceVALVE.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DescriptorSetBindingReferenceVALVE<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub descriptor_set_layout: DescriptorSetLayout,
        pub binding: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorSetBindingReferenceVALVE<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorSetBindingReferenceVALVE")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("descriptor_set_layout", &self.descriptor_set_layout)
                .field("binding", &self.binding)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DescriptorSetBindingReferenceVALVE<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DESCRIPTOR_SET_BINDING_REFERENCE_VALVE;
    }

    impl Default for DescriptorSetBindingReferenceVALVE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                descriptor_set_layout: Default::default(),
                binding: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DescriptorSetBindingReferenceVALVE<'a> {
        #[inline]
        pub fn descriptor_set_layout(mut self, descriptor_set_layout: DescriptorSetLayout) -> Self {
            self.descriptor_set_layout = descriptor_set_layout;
            self
        }

        #[inline]
        pub fn binding(mut self, binding: u32) -> Self {
            self.binding = binding;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorSetLayoutHostMappingInfoVALVE.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DescriptorSetLayoutHostMappingInfoVALVE<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub descriptor_offset: usize,
        pub descriptor_size: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorSetLayoutHostMappingInfoVALVE<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorSetLayoutHostMappingInfoVALVE")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("descriptor_offset", &self.descriptor_offset)
                .field("descriptor_size", &self.descriptor_size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DescriptorSetLayoutHostMappingInfoVALVE<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE;
    }

    impl Default for DescriptorSetLayoutHostMappingInfoVALVE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                descriptor_offset: Default::default(),
                descriptor_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DescriptorSetLayoutHostMappingInfoVALVE<'a> {
        #[inline]
        pub fn descriptor_offset(mut self, descriptor_offset: usize) -> Self {
            self.descriptor_offset = descriptor_offset;
            self
        }

        #[inline]
        pub fn descriptor_size(mut self, descriptor_size: u32) -> Self {
            self.descriptor_size = descriptor_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html>
    pub type PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE = unsafe extern "system" fn(
        device: Device,
        p_binding_reference: *const DescriptorSetBindingReferenceVALVE<'_>,
        p_host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDescriptorSetHostMappingVALVE.html>
    pub type PFN_vkGetDescriptorSetHostMappingVALVE = unsafe extern "system" fn(
        device: Device,
        descriptor_set: DescriptorSet,
        pp_data: *mut *mut c_void,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE =
        PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'static>;
    pub type VkDescriptorSetBindingReferenceVALVE = DescriptorSetBindingReferenceVALVE<'static>;
    pub type VkDescriptorSetLayoutHostMappingInfoVALVE =
        DescriptorSetLayoutHostMappingInfoVALVE<'static>;
    impl PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DescriptorSetBindingReferenceVALVE<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDescriptorSetBindingReferenceVALVE {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DescriptorSetLayoutHostMappingInfoVALVE<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDescriptorSetLayoutHostMappingInfoVALVE {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_descriptor_set_layout_host_mapping_info_valve:
        PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE,
    get_descriptor_set_host_mapping_valve: PFN_vkGetDescriptorSetHostMappingVALVE,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_descriptor_set_layout_host_mapping_info_valve: transmute(
                    load(c"vkGetDescriptorSetLayoutHostMappingInfoVALVE")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_descriptor_set_host_mapping_valve: transmute(
                    load(c"vkGetDescriptorSetHostMappingVALVE").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html>
    #[inline]
    pub unsafe fn get_descriptor_set_layout_host_mapping_info_valve(
        &self,
        device: Device,
        binding_reference: &DescriptorSetBindingReferenceVALVE<'_>,
        host_mapping: &mut DescriptorSetLayoutHostMappingInfoVALVE<'_>,
    ) {
        unsafe {
            (self.get_descriptor_set_layout_host_mapping_info_valve)(
                device,
                binding_reference,
                host_mapping,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDescriptorSetHostMappingVALVE.html>
    #[inline]
    pub unsafe fn get_descriptor_set_host_mapping_valve(
        &self,
        device: Device,
        descriptor_set: DescriptorSet,
    ) -> *mut c_void {
        unsafe {
            let mut data = core::mem::MaybeUninit::uninit();
            (self.get_descriptor_set_host_mapping_valve)(device, descriptor_set, data.as_mut_ptr());
            data.assume_init()
        }
    }
}

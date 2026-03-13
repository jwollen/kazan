//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_descriptor_buffer.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_descriptor_buffer";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDescriptorBufferFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDescriptorBufferFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub descriptor_buffer: Bool32,
        pub descriptor_buffer_capture_replay: Bool32,
        pub descriptor_buffer_image_layout_ignored: Bool32,
        pub descriptor_buffer_push_descriptors: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDescriptorBufferFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDescriptorBufferFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("descriptor_buffer", &self.descriptor_buffer)
                .field(
                    "descriptor_buffer_capture_replay",
                    &self.descriptor_buffer_capture_replay,
                )
                .field(
                    "descriptor_buffer_image_layout_ignored",
                    &self.descriptor_buffer_image_layout_ignored,
                )
                .field(
                    "descriptor_buffer_push_descriptors",
                    &self.descriptor_buffer_push_descriptors,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDescriptorBufferFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceDescriptorBufferFeaturesEXT<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceDescriptorBufferFeaturesEXT<'_> {}

    impl Default for PhysicalDeviceDescriptorBufferFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                descriptor_buffer: Default::default(),
                descriptor_buffer_capture_replay: Default::default(),
                descriptor_buffer_image_layout_ignored: Default::default(),
                descriptor_buffer_push_descriptors: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDescriptorBufferFeaturesEXT<'a> {
        #[inline]
        pub fn descriptor_buffer(mut self, descriptor_buffer: bool) -> Self {
            self.descriptor_buffer = descriptor_buffer.into();
            self
        }

        #[inline]
        pub fn descriptor_buffer_capture_replay(
            mut self,
            descriptor_buffer_capture_replay: bool,
        ) -> Self {
            self.descriptor_buffer_capture_replay = descriptor_buffer_capture_replay.into();
            self
        }

        #[inline]
        pub fn descriptor_buffer_image_layout_ignored(
            mut self,
            descriptor_buffer_image_layout_ignored: bool,
        ) -> Self {
            self.descriptor_buffer_image_layout_ignored =
                descriptor_buffer_image_layout_ignored.into();
            self
        }

        #[inline]
        pub fn descriptor_buffer_push_descriptors(
            mut self,
            descriptor_buffer_push_descriptors: bool,
        ) -> Self {
            self.descriptor_buffer_push_descriptors = descriptor_buffer_push_descriptors.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDescriptorBufferPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDescriptorBufferPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub combined_image_sampler_descriptor_single_array: Bool32,
        pub bufferless_push_descriptors: Bool32,
        pub allow_sampler_image_view_post_submit_creation: Bool32,
        pub descriptor_buffer_offset_alignment: DeviceSize,
        pub max_descriptor_buffer_bindings: u32,
        pub max_resource_descriptor_buffer_bindings: u32,
        pub max_sampler_descriptor_buffer_bindings: u32,
        pub max_embedded_immutable_sampler_bindings: u32,
        pub max_embedded_immutable_samplers: u32,
        pub buffer_capture_replay_descriptor_data_size: usize,
        pub image_capture_replay_descriptor_data_size: usize,
        pub image_view_capture_replay_descriptor_data_size: usize,
        pub sampler_capture_replay_descriptor_data_size: usize,
        pub acceleration_structure_capture_replay_descriptor_data_size: usize,
        pub sampler_descriptor_size: usize,
        pub combined_image_sampler_descriptor_size: usize,
        pub sampled_image_descriptor_size: usize,
        pub storage_image_descriptor_size: usize,
        pub uniform_texel_buffer_descriptor_size: usize,
        pub robust_uniform_texel_buffer_descriptor_size: usize,
        pub storage_texel_buffer_descriptor_size: usize,
        pub robust_storage_texel_buffer_descriptor_size: usize,
        pub uniform_buffer_descriptor_size: usize,
        pub robust_uniform_buffer_descriptor_size: usize,
        pub storage_buffer_descriptor_size: usize,
        pub robust_storage_buffer_descriptor_size: usize,
        pub input_attachment_descriptor_size: usize,
        pub acceleration_structure_descriptor_size: usize,
        pub max_sampler_descriptor_buffer_range: DeviceSize,
        pub max_resource_descriptor_buffer_range: DeviceSize,
        pub sampler_descriptor_buffer_address_space_size: DeviceSize,
        pub resource_descriptor_buffer_address_space_size: DeviceSize,
        pub descriptor_buffer_address_space_size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDescriptorBufferPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDescriptorBufferPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "combined_image_sampler_descriptor_single_array",
                    &self.combined_image_sampler_descriptor_single_array,
                )
                .field(
                    "bufferless_push_descriptors",
                    &self.bufferless_push_descriptors,
                )
                .field(
                    "allow_sampler_image_view_post_submit_creation",
                    &self.allow_sampler_image_view_post_submit_creation,
                )
                .field(
                    "descriptor_buffer_offset_alignment",
                    &self.descriptor_buffer_offset_alignment,
                )
                .field(
                    "max_descriptor_buffer_bindings",
                    &self.max_descriptor_buffer_bindings,
                )
                .field(
                    "max_resource_descriptor_buffer_bindings",
                    &self.max_resource_descriptor_buffer_bindings,
                )
                .field(
                    "max_sampler_descriptor_buffer_bindings",
                    &self.max_sampler_descriptor_buffer_bindings,
                )
                .field(
                    "max_embedded_immutable_sampler_bindings",
                    &self.max_embedded_immutable_sampler_bindings,
                )
                .field(
                    "max_embedded_immutable_samplers",
                    &self.max_embedded_immutable_samplers,
                )
                .field(
                    "buffer_capture_replay_descriptor_data_size",
                    &self.buffer_capture_replay_descriptor_data_size,
                )
                .field(
                    "image_capture_replay_descriptor_data_size",
                    &self.image_capture_replay_descriptor_data_size,
                )
                .field(
                    "image_view_capture_replay_descriptor_data_size",
                    &self.image_view_capture_replay_descriptor_data_size,
                )
                .field(
                    "sampler_capture_replay_descriptor_data_size",
                    &self.sampler_capture_replay_descriptor_data_size,
                )
                .field(
                    "acceleration_structure_capture_replay_descriptor_data_size",
                    &self.acceleration_structure_capture_replay_descriptor_data_size,
                )
                .field("sampler_descriptor_size", &self.sampler_descriptor_size)
                .field(
                    "combined_image_sampler_descriptor_size",
                    &self.combined_image_sampler_descriptor_size,
                )
                .field(
                    "sampled_image_descriptor_size",
                    &self.sampled_image_descriptor_size,
                )
                .field(
                    "storage_image_descriptor_size",
                    &self.storage_image_descriptor_size,
                )
                .field(
                    "uniform_texel_buffer_descriptor_size",
                    &self.uniform_texel_buffer_descriptor_size,
                )
                .field(
                    "robust_uniform_texel_buffer_descriptor_size",
                    &self.robust_uniform_texel_buffer_descriptor_size,
                )
                .field(
                    "storage_texel_buffer_descriptor_size",
                    &self.storage_texel_buffer_descriptor_size,
                )
                .field(
                    "robust_storage_texel_buffer_descriptor_size",
                    &self.robust_storage_texel_buffer_descriptor_size,
                )
                .field(
                    "uniform_buffer_descriptor_size",
                    &self.uniform_buffer_descriptor_size,
                )
                .field(
                    "robust_uniform_buffer_descriptor_size",
                    &self.robust_uniform_buffer_descriptor_size,
                )
                .field(
                    "storage_buffer_descriptor_size",
                    &self.storage_buffer_descriptor_size,
                )
                .field(
                    "robust_storage_buffer_descriptor_size",
                    &self.robust_storage_buffer_descriptor_size,
                )
                .field(
                    "input_attachment_descriptor_size",
                    &self.input_attachment_descriptor_size,
                )
                .field(
                    "acceleration_structure_descriptor_size",
                    &self.acceleration_structure_descriptor_size,
                )
                .field(
                    "max_sampler_descriptor_buffer_range",
                    &self.max_sampler_descriptor_buffer_range,
                )
                .field(
                    "max_resource_descriptor_buffer_range",
                    &self.max_resource_descriptor_buffer_range,
                )
                .field(
                    "sampler_descriptor_buffer_address_space_size",
                    &self.sampler_descriptor_buffer_address_space_size,
                )
                .field(
                    "resource_descriptor_buffer_address_space_size",
                    &self.resource_descriptor_buffer_address_space_size,
                )
                .field(
                    "descriptor_buffer_address_space_size",
                    &self.descriptor_buffer_address_space_size,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDescriptorBufferPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceDescriptorBufferPropertiesEXT<'_>
    {
    }

    impl Default for PhysicalDeviceDescriptorBufferPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                combined_image_sampler_descriptor_single_array: Default::default(),
                bufferless_push_descriptors: Default::default(),
                allow_sampler_image_view_post_submit_creation: Default::default(),
                descriptor_buffer_offset_alignment: Default::default(),
                max_descriptor_buffer_bindings: Default::default(),
                max_resource_descriptor_buffer_bindings: Default::default(),
                max_sampler_descriptor_buffer_bindings: Default::default(),
                max_embedded_immutable_sampler_bindings: Default::default(),
                max_embedded_immutable_samplers: Default::default(),
                buffer_capture_replay_descriptor_data_size: Default::default(),
                image_capture_replay_descriptor_data_size: Default::default(),
                image_view_capture_replay_descriptor_data_size: Default::default(),
                sampler_capture_replay_descriptor_data_size: Default::default(),
                acceleration_structure_capture_replay_descriptor_data_size: Default::default(),
                sampler_descriptor_size: Default::default(),
                combined_image_sampler_descriptor_size: Default::default(),
                sampled_image_descriptor_size: Default::default(),
                storage_image_descriptor_size: Default::default(),
                uniform_texel_buffer_descriptor_size: Default::default(),
                robust_uniform_texel_buffer_descriptor_size: Default::default(),
                storage_texel_buffer_descriptor_size: Default::default(),
                robust_storage_texel_buffer_descriptor_size: Default::default(),
                uniform_buffer_descriptor_size: Default::default(),
                robust_uniform_buffer_descriptor_size: Default::default(),
                storage_buffer_descriptor_size: Default::default(),
                robust_storage_buffer_descriptor_size: Default::default(),
                input_attachment_descriptor_size: Default::default(),
                acceleration_structure_descriptor_size: Default::default(),
                max_sampler_descriptor_buffer_range: Default::default(),
                max_resource_descriptor_buffer_range: Default::default(),
                sampler_descriptor_buffer_address_space_size: Default::default(),
                resource_descriptor_buffer_address_space_size: Default::default(),
                descriptor_buffer_address_space_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDescriptorBufferPropertiesEXT<'a> {
        #[inline]
        pub fn combined_image_sampler_descriptor_single_array(
            mut self,
            combined_image_sampler_descriptor_single_array: bool,
        ) -> Self {
            self.combined_image_sampler_descriptor_single_array =
                combined_image_sampler_descriptor_single_array.into();
            self
        }

        #[inline]
        pub fn bufferless_push_descriptors(mut self, bufferless_push_descriptors: bool) -> Self {
            self.bufferless_push_descriptors = bufferless_push_descriptors.into();
            self
        }

        #[inline]
        pub fn allow_sampler_image_view_post_submit_creation(
            mut self,
            allow_sampler_image_view_post_submit_creation: bool,
        ) -> Self {
            self.allow_sampler_image_view_post_submit_creation =
                allow_sampler_image_view_post_submit_creation.into();
            self
        }

        #[inline]
        pub fn descriptor_buffer_offset_alignment(
            mut self,
            descriptor_buffer_offset_alignment: DeviceSize,
        ) -> Self {
            self.descriptor_buffer_offset_alignment = descriptor_buffer_offset_alignment;
            self
        }

        #[inline]
        pub fn max_descriptor_buffer_bindings(
            mut self,
            max_descriptor_buffer_bindings: u32,
        ) -> Self {
            self.max_descriptor_buffer_bindings = max_descriptor_buffer_bindings;
            self
        }

        #[inline]
        pub fn max_resource_descriptor_buffer_bindings(
            mut self,
            max_resource_descriptor_buffer_bindings: u32,
        ) -> Self {
            self.max_resource_descriptor_buffer_bindings = max_resource_descriptor_buffer_bindings;
            self
        }

        #[inline]
        pub fn max_sampler_descriptor_buffer_bindings(
            mut self,
            max_sampler_descriptor_buffer_bindings: u32,
        ) -> Self {
            self.max_sampler_descriptor_buffer_bindings = max_sampler_descriptor_buffer_bindings;
            self
        }

        #[inline]
        pub fn max_embedded_immutable_sampler_bindings(
            mut self,
            max_embedded_immutable_sampler_bindings: u32,
        ) -> Self {
            self.max_embedded_immutable_sampler_bindings = max_embedded_immutable_sampler_bindings;
            self
        }

        #[inline]
        pub fn max_embedded_immutable_samplers(
            mut self,
            max_embedded_immutable_samplers: u32,
        ) -> Self {
            self.max_embedded_immutable_samplers = max_embedded_immutable_samplers;
            self
        }

        #[inline]
        pub fn buffer_capture_replay_descriptor_data_size(
            mut self,
            buffer_capture_replay_descriptor_data_size: usize,
        ) -> Self {
            self.buffer_capture_replay_descriptor_data_size =
                buffer_capture_replay_descriptor_data_size;
            self
        }

        #[inline]
        pub fn image_capture_replay_descriptor_data_size(
            mut self,
            image_capture_replay_descriptor_data_size: usize,
        ) -> Self {
            self.image_capture_replay_descriptor_data_size =
                image_capture_replay_descriptor_data_size;
            self
        }

        #[inline]
        pub fn image_view_capture_replay_descriptor_data_size(
            mut self,
            image_view_capture_replay_descriptor_data_size: usize,
        ) -> Self {
            self.image_view_capture_replay_descriptor_data_size =
                image_view_capture_replay_descriptor_data_size;
            self
        }

        #[inline]
        pub fn sampler_capture_replay_descriptor_data_size(
            mut self,
            sampler_capture_replay_descriptor_data_size: usize,
        ) -> Self {
            self.sampler_capture_replay_descriptor_data_size =
                sampler_capture_replay_descriptor_data_size;
            self
        }

        #[inline]
        pub fn acceleration_structure_capture_replay_descriptor_data_size(
            mut self,
            acceleration_structure_capture_replay_descriptor_data_size: usize,
        ) -> Self {
            self.acceleration_structure_capture_replay_descriptor_data_size =
                acceleration_structure_capture_replay_descriptor_data_size;
            self
        }

        #[inline]
        pub fn sampler_descriptor_size(mut self, sampler_descriptor_size: usize) -> Self {
            self.sampler_descriptor_size = sampler_descriptor_size;
            self
        }

        #[inline]
        pub fn combined_image_sampler_descriptor_size(
            mut self,
            combined_image_sampler_descriptor_size: usize,
        ) -> Self {
            self.combined_image_sampler_descriptor_size = combined_image_sampler_descriptor_size;
            self
        }

        #[inline]
        pub fn sampled_image_descriptor_size(
            mut self,
            sampled_image_descriptor_size: usize,
        ) -> Self {
            self.sampled_image_descriptor_size = sampled_image_descriptor_size;
            self
        }

        #[inline]
        pub fn storage_image_descriptor_size(
            mut self,
            storage_image_descriptor_size: usize,
        ) -> Self {
            self.storage_image_descriptor_size = storage_image_descriptor_size;
            self
        }

        #[inline]
        pub fn uniform_texel_buffer_descriptor_size(
            mut self,
            uniform_texel_buffer_descriptor_size: usize,
        ) -> Self {
            self.uniform_texel_buffer_descriptor_size = uniform_texel_buffer_descriptor_size;
            self
        }

        #[inline]
        pub fn robust_uniform_texel_buffer_descriptor_size(
            mut self,
            robust_uniform_texel_buffer_descriptor_size: usize,
        ) -> Self {
            self.robust_uniform_texel_buffer_descriptor_size =
                robust_uniform_texel_buffer_descriptor_size;
            self
        }

        #[inline]
        pub fn storage_texel_buffer_descriptor_size(
            mut self,
            storage_texel_buffer_descriptor_size: usize,
        ) -> Self {
            self.storage_texel_buffer_descriptor_size = storage_texel_buffer_descriptor_size;
            self
        }

        #[inline]
        pub fn robust_storage_texel_buffer_descriptor_size(
            mut self,
            robust_storage_texel_buffer_descriptor_size: usize,
        ) -> Self {
            self.robust_storage_texel_buffer_descriptor_size =
                robust_storage_texel_buffer_descriptor_size;
            self
        }

        #[inline]
        pub fn uniform_buffer_descriptor_size(
            mut self,
            uniform_buffer_descriptor_size: usize,
        ) -> Self {
            self.uniform_buffer_descriptor_size = uniform_buffer_descriptor_size;
            self
        }

        #[inline]
        pub fn robust_uniform_buffer_descriptor_size(
            mut self,
            robust_uniform_buffer_descriptor_size: usize,
        ) -> Self {
            self.robust_uniform_buffer_descriptor_size = robust_uniform_buffer_descriptor_size;
            self
        }

        #[inline]
        pub fn storage_buffer_descriptor_size(
            mut self,
            storage_buffer_descriptor_size: usize,
        ) -> Self {
            self.storage_buffer_descriptor_size = storage_buffer_descriptor_size;
            self
        }

        #[inline]
        pub fn robust_storage_buffer_descriptor_size(
            mut self,
            robust_storage_buffer_descriptor_size: usize,
        ) -> Self {
            self.robust_storage_buffer_descriptor_size = robust_storage_buffer_descriptor_size;
            self
        }

        #[inline]
        pub fn input_attachment_descriptor_size(
            mut self,
            input_attachment_descriptor_size: usize,
        ) -> Self {
            self.input_attachment_descriptor_size = input_attachment_descriptor_size;
            self
        }

        #[inline]
        pub fn acceleration_structure_descriptor_size(
            mut self,
            acceleration_structure_descriptor_size: usize,
        ) -> Self {
            self.acceleration_structure_descriptor_size = acceleration_structure_descriptor_size;
            self
        }

        #[inline]
        pub fn max_sampler_descriptor_buffer_range(
            mut self,
            max_sampler_descriptor_buffer_range: DeviceSize,
        ) -> Self {
            self.max_sampler_descriptor_buffer_range = max_sampler_descriptor_buffer_range;
            self
        }

        #[inline]
        pub fn max_resource_descriptor_buffer_range(
            mut self,
            max_resource_descriptor_buffer_range: DeviceSize,
        ) -> Self {
            self.max_resource_descriptor_buffer_range = max_resource_descriptor_buffer_range;
            self
        }

        #[inline]
        pub fn sampler_descriptor_buffer_address_space_size(
            mut self,
            sampler_descriptor_buffer_address_space_size: DeviceSize,
        ) -> Self {
            self.sampler_descriptor_buffer_address_space_size =
                sampler_descriptor_buffer_address_space_size;
            self
        }

        #[inline]
        pub fn resource_descriptor_buffer_address_space_size(
            mut self,
            resource_descriptor_buffer_address_space_size: DeviceSize,
        ) -> Self {
            self.resource_descriptor_buffer_address_space_size =
                resource_descriptor_buffer_address_space_size;
            self
        }

        #[inline]
        pub fn descriptor_buffer_address_space_size(
            mut self,
            descriptor_buffer_address_space_size: DeviceSize,
        ) -> Self {
            self.descriptor_buffer_address_space_size = descriptor_buffer_address_space_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub combined_image_sampler_density_map_descriptor_size: usize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "combined_image_sampler_density_map_descriptor_size",
                    &self.combined_image_sampler_density_map_descriptor_size,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT<'_>
    {
    }

    impl Default for PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                combined_image_sampler_density_map_descriptor_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT<'a> {
        #[inline]
        pub fn combined_image_sampler_density_map_descriptor_size(
            mut self,
            combined_image_sampler_density_map_descriptor_size: usize,
        ) -> Self {
            self.combined_image_sampler_density_map_descriptor_size =
                combined_image_sampler_density_map_descriptor_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorAddressInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DescriptorAddressInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub address: DeviceAddress,
        pub range: DeviceSize,
        pub format: Format,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorAddressInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorAddressInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("address", &self.address)
                .field("range", &self.range)
                .field("format", &self.format)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DescriptorAddressInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DESCRIPTOR_ADDRESS_INFO_EXT;
    }

    impl Default for DescriptorAddressInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                address: Default::default(),
                range: Default::default(),
                format: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DescriptorAddressInfoEXT<'a> {
        #[inline]
        pub fn address(mut self, address: DeviceAddress) -> Self {
            self.address = address;
            self
        }

        #[inline]
        pub fn range(mut self, range: DeviceSize) -> Self {
            self.range = range;
            self
        }

        #[inline]
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorBufferBindingInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DescriptorBufferBindingInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub address: DeviceAddress,
        pub usage: BufferUsageFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorBufferBindingInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorBufferBindingInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("address", &self.address)
                .field("usage", &self.usage)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DescriptorBufferBindingInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DESCRIPTOR_BUFFER_BINDING_INFO_EXT;
    }

    impl Default for DescriptorBufferBindingInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                address: Default::default(),
                usage: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DescriptorBufferBindingInfoEXT<'a> {
        #[inline]
        pub fn address(mut self, address: DeviceAddress) -> Self {
            self.address = address;
            self
        }

        #[inline]
        pub fn usage(mut self, usage: BufferUsageFlags) -> Self {
            self.usage = usage;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorBufferBindingPushDescriptorBufferHandleEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DescriptorBufferBindingPushDescriptorBufferHandleEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub buffer: Buffer,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorBufferBindingPushDescriptorBufferHandleEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorBufferBindingPushDescriptorBufferHandleEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("buffer", &self.buffer)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DescriptorBufferBindingPushDescriptorBufferHandleEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT;
    }

    unsafe impl Extends<DescriptorBufferBindingInfoEXT<'_>>
        for DescriptorBufferBindingPushDescriptorBufferHandleEXT<'_>
    {
    }

    impl Default for DescriptorBufferBindingPushDescriptorBufferHandleEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                buffer: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DescriptorBufferBindingPushDescriptorBufferHandleEXT<'a> {
        #[inline]
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorGetInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DescriptorGetInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub ty: DescriptorType,
        pub data: DescriptorDataEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorGetInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorGetInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("ty", &self.ty)
                .field("data", &self.data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DescriptorGetInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DESCRIPTOR_GET_INFO_EXT;
    }

    impl Default for DescriptorGetInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                ty: Default::default(),
                data: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DescriptorGetInfoEXT<'a> {
        #[inline]
        pub fn ty(mut self, ty: DescriptorType) -> Self {
            self.ty = ty;
            self
        }

        #[inline]
        pub fn data(mut self, data: DescriptorDataEXT<'a>) -> Self {
            self.data = data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBufferCaptureDescriptorDataInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct BufferCaptureDescriptorDataInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub buffer: Buffer,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BufferCaptureDescriptorDataInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BufferCaptureDescriptorDataInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("buffer", &self.buffer)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BufferCaptureDescriptorDataInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT;
    }

    impl Default for BufferCaptureDescriptorDataInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                buffer: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BufferCaptureDescriptorDataInfoEXT<'a> {
        #[inline]
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageCaptureDescriptorDataInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageCaptureDescriptorDataInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image: Image,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageCaptureDescriptorDataInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageCaptureDescriptorDataInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image", &self.image)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageCaptureDescriptorDataInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT;
    }

    impl Default for ImageCaptureDescriptorDataInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                image: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageCaptureDescriptorDataInfoEXT<'a> {
        #[inline]
        pub fn image(mut self, image: Image) -> Self {
            self.image = image;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageViewCaptureDescriptorDataInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageViewCaptureDescriptorDataInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub image_view: ImageView,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageViewCaptureDescriptorDataInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageViewCaptureDescriptorDataInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image_view", &self.image_view)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageViewCaptureDescriptorDataInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT;
    }

    impl Default for ImageViewCaptureDescriptorDataInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                image_view: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageViewCaptureDescriptorDataInfoEXT<'a> {
        #[inline]
        pub fn image_view(mut self, image_view: ImageView) -> Self {
            self.image_view = image_view;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerCaptureDescriptorDataInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SamplerCaptureDescriptorDataInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub sampler: Sampler,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SamplerCaptureDescriptorDataInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SamplerCaptureDescriptorDataInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("sampler", &self.sampler)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SamplerCaptureDescriptorDataInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT;
    }

    impl Default for SamplerCaptureDescriptorDataInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                sampler: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SamplerCaptureDescriptorDataInfoEXT<'a> {
        #[inline]
        pub fn sampler(mut self, sampler: Sampler) -> Self {
            self.sampler = sampler;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAccelerationStructureCaptureDescriptorDataInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AccelerationStructureCaptureDescriptorDataInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub acceleration_structure: AccelerationStructureKHR,
        pub acceleration_structure_nv: AccelerationStructureNV,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AccelerationStructureCaptureDescriptorDataInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AccelerationStructureCaptureDescriptorDataInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("acceleration_structure", &self.acceleration_structure)
                .field("acceleration_structure_nv", &self.acceleration_structure_nv)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AccelerationStructureCaptureDescriptorDataInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT;
    }

    impl Default for AccelerationStructureCaptureDescriptorDataInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                acceleration_structure: Default::default(),
                acceleration_structure_nv: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AccelerationStructureCaptureDescriptorDataInfoEXT<'a> {
        #[inline]
        pub fn acceleration_structure(
            mut self,
            acceleration_structure: AccelerationStructureKHR,
        ) -> Self {
            self.acceleration_structure = acceleration_structure;
            self
        }

        #[inline]
        pub fn acceleration_structure_nv(
            mut self,
            acceleration_structure_nv: AccelerationStructureNV,
        ) -> Self {
            self.acceleration_structure_nv = acceleration_structure_nv;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpaqueCaptureDescriptorDataCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct OpaqueCaptureDescriptorDataCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub opaque_capture_descriptor_data: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for OpaqueCaptureDescriptorDataCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OpaqueCaptureDescriptorDataCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "opaque_capture_descriptor_data",
                    &self.opaque_capture_descriptor_data,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for OpaqueCaptureDescriptorDataCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT;
    }

    unsafe impl Extends<BufferCreateInfo<'_>> for OpaqueCaptureDescriptorDataCreateInfoEXT<'_> {}
    unsafe impl Extends<ImageCreateInfo<'_>> for OpaqueCaptureDescriptorDataCreateInfoEXT<'_> {}
    unsafe impl Extends<ImageViewCreateInfo<'_>> for OpaqueCaptureDescriptorDataCreateInfoEXT<'_> {}
    unsafe impl Extends<SamplerCreateInfo<'_>> for OpaqueCaptureDescriptorDataCreateInfoEXT<'_> {}
    unsafe impl Extends<AccelerationStructureCreateInfoKHR<'_>>
        for OpaqueCaptureDescriptorDataCreateInfoEXT<'_>
    {
    }
    unsafe impl Extends<AccelerationStructureCreateInfoNV<'_>>
        for OpaqueCaptureDescriptorDataCreateInfoEXT<'_>
    {
    }
    unsafe impl Extends<TensorCreateInfoARM<'_>> for OpaqueCaptureDescriptorDataCreateInfoEXT<'_> {}
    unsafe impl Extends<TensorViewCreateInfoARM<'_>> for OpaqueCaptureDescriptorDataCreateInfoEXT<'_> {}

    impl Default for OpaqueCaptureDescriptorDataCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                opaque_capture_descriptor_data: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> OpaqueCaptureDescriptorDataCreateInfoEXT<'a> {
        #[inline]
        pub fn opaque_capture_descriptor_data(
            mut self,
            opaque_capture_descriptor_data: *const c_void,
        ) -> Self {
            self.opaque_capture_descriptor_data = opaque_capture_descriptor_data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorDataEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union DescriptorDataEXT<'a> {
        pub p_sampler: *const Sampler,
        pub p_combined_image_sampler: *const DescriptorImageInfo,
        pub p_input_attachment_image: *const DescriptorImageInfo,
        pub p_sampled_image: *const DescriptorImageInfo,
        pub p_storage_image: *const DescriptorImageInfo,
        pub p_uniform_texel_buffer: *const DescriptorAddressInfoEXT<'a>,
        pub p_storage_texel_buffer: *const DescriptorAddressInfoEXT<'a>,
        pub p_uniform_buffer: *const DescriptorAddressInfoEXT<'a>,
        pub p_storage_buffer: *const DescriptorAddressInfoEXT<'a>,
        pub acceleration_structure: DeviceAddress,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorDataEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorDataEXT").finish()
        }
    }

    impl Default for DescriptorDataEXT<'_> {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDescriptorSetLayoutSizeEXT.html>
    pub type PFN_vkGetDescriptorSetLayoutSizeEXT = unsafe extern "system" fn(
        device: Device,
        layout: DescriptorSetLayout,
        p_layout_size_in_bytes: *mut DeviceSize,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDescriptorSetLayoutBindingOffsetEXT.html>
    pub type PFN_vkGetDescriptorSetLayoutBindingOffsetEXT = unsafe extern "system" fn(
        device: Device,
        layout: DescriptorSetLayout,
        binding: u32,
        p_offset: *mut DeviceSize,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDescriptorEXT.html>
    pub type PFN_vkGetDescriptorEXT = unsafe extern "system" fn(
        device: Device,
        p_descriptor_info: *const DescriptorGetInfoEXT<'_>,
        data_size: usize,
        p_descriptor: *mut c_void,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindDescriptorBuffersEXT.html>
    pub type PFN_vkCmdBindDescriptorBuffersEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer_count: u32,
        p_binding_infos: *const DescriptorBufferBindingInfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDescriptorBufferOffsetsEXT.html>
    pub type PFN_vkCmdSetDescriptorBufferOffsetsEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        set_count: u32,
        p_buffer_indices: *const u32,
        p_offsets: *const DeviceSize,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html>
    pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetBufferOpaqueCaptureDescriptorDataEXT.html>
    pub type PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT =
        unsafe extern "system" fn(
            device: Device,
            p_info: *const BufferCaptureDescriptorDataInfoEXT<'_>,
            p_data: *mut c_void,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageOpaqueCaptureDescriptorDataEXT.html>
    pub type PFN_vkGetImageOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
        device: Device,
        p_info: *const ImageCaptureDescriptorDataInfoEXT<'_>,
        p_data: *mut c_void,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html>
    pub type PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT =
        unsafe extern "system" fn(
            device: Device,
            p_info: *const ImageViewCaptureDescriptorDataInfoEXT<'_>,
            p_data: *mut c_void,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html>
    pub type PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT =
        unsafe extern "system" fn(
            device: Device,
            p_info: *const SamplerCaptureDescriptorDataInfoEXT<'_>,
            p_data: *mut c_void,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html>
    pub type PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT =
        unsafe extern "system" fn(
            device: Device,
            p_info: *const AccelerationStructureCaptureDescriptorDataInfoEXT<'_>,
            p_data: *mut c_void,
        ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceDescriptorBufferFeaturesEXT =
        PhysicalDeviceDescriptorBufferFeaturesEXT<'static>;
    pub type VkPhysicalDeviceDescriptorBufferPropertiesEXT =
        PhysicalDeviceDescriptorBufferPropertiesEXT<'static>;
    pub type VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT =
        PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT<'static>;
    pub type VkDescriptorAddressInfoEXT = DescriptorAddressInfoEXT<'static>;
    pub type VkDescriptorBufferBindingInfoEXT = DescriptorBufferBindingInfoEXT<'static>;
    pub type VkDescriptorBufferBindingPushDescriptorBufferHandleEXT =
        DescriptorBufferBindingPushDescriptorBufferHandleEXT<'static>;
    pub type VkDescriptorGetInfoEXT = DescriptorGetInfoEXT<'static>;
    pub type VkBufferCaptureDescriptorDataInfoEXT = BufferCaptureDescriptorDataInfoEXT<'static>;
    pub type VkImageCaptureDescriptorDataInfoEXT = ImageCaptureDescriptorDataInfoEXT<'static>;
    pub type VkImageViewCaptureDescriptorDataInfoEXT =
        ImageViewCaptureDescriptorDataInfoEXT<'static>;
    pub type VkSamplerCaptureDescriptorDataInfoEXT = SamplerCaptureDescriptorDataInfoEXT<'static>;
    pub type VkAccelerationStructureCaptureDescriptorDataInfoEXT =
        AccelerationStructureCaptureDescriptorDataInfoEXT<'static>;
    pub type VkOpaqueCaptureDescriptorDataCreateInfoEXT =
        OpaqueCaptureDescriptorDataCreateInfoEXT<'static>;
    pub type VkDescriptorDataEXT = DescriptorDataEXT<'static>;
    impl PhysicalDeviceDescriptorBufferFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceDescriptorBufferFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceDescriptorBufferPropertiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceDescriptorBufferPropertiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DescriptorAddressInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDescriptorAddressInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DescriptorBufferBindingInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDescriptorBufferBindingInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DescriptorBufferBindingPushDescriptorBufferHandleEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkDescriptorBufferBindingPushDescriptorBufferHandleEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DescriptorGetInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDescriptorGetInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl BufferCaptureDescriptorDataInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkBufferCaptureDescriptorDataInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageCaptureDescriptorDataInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageCaptureDescriptorDataInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageViewCaptureDescriptorDataInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageViewCaptureDescriptorDataInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SamplerCaptureDescriptorDataInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSamplerCaptureDescriptorDataInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AccelerationStructureCaptureDescriptorDataInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkAccelerationStructureCaptureDescriptorDataInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl OpaqueCaptureDescriptorDataCreateInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkOpaqueCaptureDescriptorDataCreateInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DescriptorDataEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDescriptorDataEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_descriptor_set_layout_size: PFN_vkGetDescriptorSetLayoutSizeEXT,
    get_descriptor_set_layout_binding_offset: PFN_vkGetDescriptorSetLayoutBindingOffsetEXT,
    get_descriptor: PFN_vkGetDescriptorEXT,
    cmd_bind_descriptor_buffers: PFN_vkCmdBindDescriptorBuffersEXT,
    cmd_set_descriptor_buffer_offsets: PFN_vkCmdSetDescriptorBufferOffsetsEXT,
    cmd_bind_descriptor_buffer_embedded_samplers: PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT,
    get_buffer_opaque_capture_descriptor_data: PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT,
    get_image_opaque_capture_descriptor_data: PFN_vkGetImageOpaqueCaptureDescriptorDataEXT,
    get_image_view_opaque_capture_descriptor_data: PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT,
    get_sampler_opaque_capture_descriptor_data: PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT,
    get_acceleration_structure_opaque_capture_descriptor_data:
        Option<PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT>,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_descriptor_set_layout_size: transmute(
                    load(c"vkGetDescriptorSetLayoutSizeEXT").ok_or(MissingEntryPointError)?,
                ),
                get_descriptor_set_layout_binding_offset: transmute(
                    load(c"vkGetDescriptorSetLayoutBindingOffsetEXT")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_descriptor: transmute(
                    load(c"vkGetDescriptorEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_bind_descriptor_buffers: transmute(
                    load(c"vkCmdBindDescriptorBuffersEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_descriptor_buffer_offsets: transmute(
                    load(c"vkCmdSetDescriptorBufferOffsetsEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_bind_descriptor_buffer_embedded_samplers: transmute(
                    load(c"vkCmdBindDescriptorBufferEmbeddedSamplersEXT")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_buffer_opaque_capture_descriptor_data: transmute(
                    load(c"vkGetBufferOpaqueCaptureDescriptorDataEXT")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_image_opaque_capture_descriptor_data: transmute(
                    load(c"vkGetImageOpaqueCaptureDescriptorDataEXT")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_image_view_opaque_capture_descriptor_data: transmute(
                    load(c"vkGetImageViewOpaqueCaptureDescriptorDataEXT")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_sampler_opaque_capture_descriptor_data: transmute(
                    load(c"vkGetSamplerOpaqueCaptureDescriptorDataEXT")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_acceleration_structure_opaque_capture_descriptor_data: transmute(load(
                    c"vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT",
                )),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDescriptorSetLayoutSizeEXT.html>
    #[inline]
    pub unsafe fn get_descriptor_set_layout_size(
        &self,
        device: Device,
        layout: DescriptorSetLayout,
    ) -> DeviceSize {
        unsafe {
            let mut layout_size_in_bytes = core::mem::MaybeUninit::uninit();
            (self.get_descriptor_set_layout_size)(
                device,
                layout,
                layout_size_in_bytes.as_mut_ptr(),
            );
            layout_size_in_bytes.assume_init()
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDescriptorSetLayoutBindingOffsetEXT.html>
    #[inline]
    pub unsafe fn get_descriptor_set_layout_binding_offset(
        &self,
        device: Device,
        layout: DescriptorSetLayout,
        binding: u32,
    ) -> DeviceSize {
        unsafe {
            let mut offset = core::mem::MaybeUninit::uninit();
            (self.get_descriptor_set_layout_binding_offset)(
                device,
                layout,
                binding,
                offset.as_mut_ptr(),
            );
            offset.assume_init()
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDescriptorEXT.html>
    #[inline]
    pub unsafe fn get_descriptor(
        &self,
        device: Device,
        descriptor_info: &DescriptorGetInfoEXT<'_>,
        descriptor: &mut [u8],
    ) {
        unsafe {
            (self.get_descriptor)(
                device,
                descriptor_info,
                descriptor.len().try_into().unwrap(),
                descriptor.as_mut_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindDescriptorBuffersEXT.html>
    #[inline]
    pub unsafe fn cmd_bind_descriptor_buffers(
        &self,
        command_buffer: CommandBuffer,
        binding_infos: &[DescriptorBufferBindingInfoEXT<'_>],
    ) {
        unsafe {
            (self.cmd_bind_descriptor_buffers)(
                command_buffer,
                binding_infos.len().try_into().unwrap(),
                binding_infos.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDescriptorBufferOffsetsEXT.html>
    #[inline]
    pub unsafe fn cmd_set_descriptor_buffer_offsets(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        buffer_indices: &[u32],
        offsets: &[DeviceSize],
    ) {
        unsafe {
            (self.cmd_set_descriptor_buffer_offsets)(
                command_buffer,
                pipeline_bind_point,
                layout,
                first_set,
                buffer_indices.len().try_into().unwrap(),
                buffer_indices.as_ptr() as _,
                offsets.as_ptr() as _,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html>
    #[inline]
    pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
    ) {
        unsafe {
            (self.cmd_bind_descriptor_buffer_embedded_samplers)(
                command_buffer,
                pipeline_bind_point,
                layout,
                set,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetBufferOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    pub unsafe fn get_buffer_opaque_capture_descriptor_data(
        &self,
        device: Device,
        info: &BufferCaptureDescriptorDataInfoEXT<'_>,
        data: *mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_buffer_opaque_capture_descriptor_data)(device, info, data);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    pub unsafe fn get_image_opaque_capture_descriptor_data(
        &self,
        device: Device,
        info: &ImageCaptureDescriptorDataInfoEXT<'_>,
        data: *mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_image_opaque_capture_descriptor_data)(device, info, data);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    pub unsafe fn get_image_view_opaque_capture_descriptor_data(
        &self,
        device: Device,
        info: &ImageViewCaptureDescriptorDataInfoEXT<'_>,
        data: *mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_image_view_opaque_capture_descriptor_data)(device, info, data);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    pub unsafe fn get_sampler_opaque_capture_descriptor_data(
        &self,
        device: Device,
        info: &SamplerCaptureDescriptorDataInfoEXT<'_>,
        data: *mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_sampler_opaque_capture_descriptor_data)(device, info, data);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    pub unsafe fn get_acceleration_structure_opaque_capture_descriptor_data(
        &self,
        device: Device,
        info: &AccelerationStructureCaptureDescriptorDataInfoEXT<'_>,
        data: *mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self
                .get_acceleration_structure_opaque_capture_descriptor_data
                .unwrap())(device, info, data);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}

#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_descriptor_heap";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkHostAddressRangeEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct HostAddressRangeEXT<'a> {
        pub address: *mut c_void,
        pub size: usize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for HostAddressRangeEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("HostAddressRangeEXT")
                .field("address", &self.address)
                .field("size", &self.size)
                .finish()
        }
    }

    impl Default for HostAddressRangeEXT<'_> {
        fn default() -> Self {
            Self {
                address: core::ptr::null_mut(),
                size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> HostAddressRangeEXT<'a> {
        pub fn address(mut self, address: &'a mut [u8]) -> Self {
            self.size = address.len().try_into().unwrap();
            self.address = address.as_mut_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkHostAddressRangeConstEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct HostAddressRangeConstEXT<'a> {
        pub address: *const c_void,
        pub size: usize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for HostAddressRangeConstEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("HostAddressRangeConstEXT")
                .field("address", &self.address)
                .field("size", &self.size)
                .finish()
        }
    }

    impl Default for HostAddressRangeConstEXT<'_> {
        fn default() -> Self {
            Self {
                address: core::ptr::null(),
                size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> HostAddressRangeConstEXT<'a> {
        pub fn address(mut self, address: &'a [u8]) -> Self {
            self.size = address.len().try_into().unwrap();
            self.address = address.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDeviceAddressRangeEXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    pub struct DeviceAddressRangeEXT {
        pub address: DeviceAddress,
        pub size: DeviceSize,
    }

    impl DeviceAddressRangeEXT {
        pub fn address(mut self, address: DeviceAddress) -> Self {
            self.address = address;
            self
        }

        pub fn size(mut self, size: DeviceSize) -> Self {
            self.size = size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkTexelBufferDescriptorInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct TexelBufferDescriptorInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub format: Format,
        pub address_range: DeviceAddressRangeEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for TexelBufferDescriptorInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("TexelBufferDescriptorInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("format", &self.format)
                .field("address_range", &self.address_range)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for TexelBufferDescriptorInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::TEXEL_BUFFER_DESCRIPTOR_INFO_EXT;
    }

    impl Default for TexelBufferDescriptorInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                format: Default::default(),
                address_range: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> TexelBufferDescriptorInfoEXT<'a> {
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }

        pub fn address_range(mut self, address_range: DeviceAddressRangeEXT) -> Self {
            self.address_range = address_range;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageDescriptorInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImageDescriptorInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_view: *const ImageViewCreateInfo<'a>,
        pub layout: ImageLayout,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageDescriptorInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageDescriptorInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_view", &self.p_view)
                .field("layout", &self.layout)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageDescriptorInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_DESCRIPTOR_INFO_EXT;
    }

    impl Default for ImageDescriptorInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_view: core::ptr::null(),
                layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageDescriptorInfoEXT<'a> {
        pub fn view(mut self, view: &'a ImageViewCreateInfo<'a>) -> Self {
            self.p_view = view;
            self
        }

        pub fn layout(mut self, layout: ImageLayout) -> Self {
            self.layout = layout;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkResourceDescriptorInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ResourceDescriptorInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub ty: DescriptorType,
        pub data: ResourceDescriptorDataEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ResourceDescriptorInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ResourceDescriptorInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("ty", &self.ty)
                .field("data", &self.data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ResourceDescriptorInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RESOURCE_DESCRIPTOR_INFO_EXT;
    }

    impl Default for ResourceDescriptorInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                ty: Default::default(),
                data: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ResourceDescriptorInfoEXT<'a> {
        pub fn ty(mut self, ty: DescriptorType) -> Self {
            self.ty = ty;
            self
        }

        pub fn data(mut self, data: ResourceDescriptorDataEXT<'a>) -> Self {
            self.data = data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindHeapInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BindHeapInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub heap_range: DeviceAddressRangeEXT,
        pub reserved_range_offset: DeviceSize,
        pub reserved_range_size: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for BindHeapInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("BindHeapInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("heap_range", &self.heap_range)
                .field("reserved_range_offset", &self.reserved_range_offset)
                .field("reserved_range_size", &self.reserved_range_size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for BindHeapInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::BIND_HEAP_INFO_EXT;
    }

    impl Default for BindHeapInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                heap_range: Default::default(),
                reserved_range_offset: Default::default(),
                reserved_range_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> BindHeapInfoEXT<'a> {
        pub fn heap_range(mut self, heap_range: DeviceAddressRangeEXT) -> Self {
            self.heap_range = heap_range;
            self
        }

        pub fn reserved_range_offset(mut self, reserved_range_offset: DeviceSize) -> Self {
            self.reserved_range_offset = reserved_range_offset;
            self
        }

        pub fn reserved_range_size(mut self, reserved_range_size: DeviceSize) -> Self {
            self.reserved_range_size = reserved_range_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPushDataInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PushDataInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub offset: u32,
        pub data: HostAddressRangeConstEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PushDataInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PushDataInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("offset", &self.offset)
                .field("data", &self.data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PushDataInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PUSH_DATA_INFO_EXT;
    }

    impl Default for PushDataInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                offset: Default::default(),
                data: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PushDataInfoEXT<'a> {
        pub fn offset(mut self, offset: u32) -> Self {
            self.offset = offset;
            self
        }

        pub fn data(mut self, data: HostAddressRangeConstEXT<'a>) -> Self {
            self.data = data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorMappingSourceConstantOffsetEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DescriptorMappingSourceConstantOffsetEXT<'a> {
        pub heap_offset: u32,
        pub heap_array_stride: u32,
        pub p_embedded_sampler: *const SamplerCreateInfo<'a>,
        pub sampler_heap_offset: u32,
        pub sampler_heap_array_stride: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorMappingSourceConstantOffsetEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorMappingSourceConstantOffsetEXT")
                .field("heap_offset", &self.heap_offset)
                .field("heap_array_stride", &self.heap_array_stride)
                .field("p_embedded_sampler", &self.p_embedded_sampler)
                .field("sampler_heap_offset", &self.sampler_heap_offset)
                .field("sampler_heap_array_stride", &self.sampler_heap_array_stride)
                .finish()
        }
    }

    impl Default for DescriptorMappingSourceConstantOffsetEXT<'_> {
        fn default() -> Self {
            Self {
                heap_offset: Default::default(),
                heap_array_stride: Default::default(),
                p_embedded_sampler: core::ptr::null(),
                sampler_heap_offset: Default::default(),
                sampler_heap_array_stride: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DescriptorMappingSourceConstantOffsetEXT<'a> {
        pub fn heap_offset(mut self, heap_offset: u32) -> Self {
            self.heap_offset = heap_offset;
            self
        }

        pub fn heap_array_stride(mut self, heap_array_stride: u32) -> Self {
            self.heap_array_stride = heap_array_stride;
            self
        }

        pub fn embedded_sampler(mut self, embedded_sampler: &'a SamplerCreateInfo<'a>) -> Self {
            self.p_embedded_sampler = embedded_sampler;
            self
        }

        pub fn sampler_heap_offset(mut self, sampler_heap_offset: u32) -> Self {
            self.sampler_heap_offset = sampler_heap_offset;
            self
        }

        pub fn sampler_heap_array_stride(mut self, sampler_heap_array_stride: u32) -> Self {
            self.sampler_heap_array_stride = sampler_heap_array_stride;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorMappingSourcePushIndexEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DescriptorMappingSourcePushIndexEXT<'a> {
        pub heap_offset: u32,
        pub push_offset: u32,
        pub heap_index_stride: u32,
        pub heap_array_stride: u32,
        pub p_embedded_sampler: *const SamplerCreateInfo<'a>,
        pub use_combined_image_sampler_index: Bool32,
        pub sampler_heap_offset: u32,
        pub sampler_push_offset: u32,
        pub sampler_heap_index_stride: u32,
        pub sampler_heap_array_stride: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorMappingSourcePushIndexEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorMappingSourcePushIndexEXT")
                .field("heap_offset", &self.heap_offset)
                .field("push_offset", &self.push_offset)
                .field("heap_index_stride", &self.heap_index_stride)
                .field("heap_array_stride", &self.heap_array_stride)
                .field("p_embedded_sampler", &self.p_embedded_sampler)
                .field(
                    "use_combined_image_sampler_index",
                    &self.use_combined_image_sampler_index,
                )
                .field("sampler_heap_offset", &self.sampler_heap_offset)
                .field("sampler_push_offset", &self.sampler_push_offset)
                .field("sampler_heap_index_stride", &self.sampler_heap_index_stride)
                .field("sampler_heap_array_stride", &self.sampler_heap_array_stride)
                .finish()
        }
    }

    impl Default for DescriptorMappingSourcePushIndexEXT<'_> {
        fn default() -> Self {
            Self {
                heap_offset: Default::default(),
                push_offset: Default::default(),
                heap_index_stride: Default::default(),
                heap_array_stride: Default::default(),
                p_embedded_sampler: core::ptr::null(),
                use_combined_image_sampler_index: Default::default(),
                sampler_heap_offset: Default::default(),
                sampler_push_offset: Default::default(),
                sampler_heap_index_stride: Default::default(),
                sampler_heap_array_stride: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DescriptorMappingSourcePushIndexEXT<'a> {
        pub fn heap_offset(mut self, heap_offset: u32) -> Self {
            self.heap_offset = heap_offset;
            self
        }

        pub fn push_offset(mut self, push_offset: u32) -> Self {
            self.push_offset = push_offset;
            self
        }

        pub fn heap_index_stride(mut self, heap_index_stride: u32) -> Self {
            self.heap_index_stride = heap_index_stride;
            self
        }

        pub fn heap_array_stride(mut self, heap_array_stride: u32) -> Self {
            self.heap_array_stride = heap_array_stride;
            self
        }

        pub fn embedded_sampler(mut self, embedded_sampler: &'a SamplerCreateInfo<'a>) -> Self {
            self.p_embedded_sampler = embedded_sampler;
            self
        }

        pub fn use_combined_image_sampler_index(
            mut self,
            use_combined_image_sampler_index: bool,
        ) -> Self {
            self.use_combined_image_sampler_index = use_combined_image_sampler_index.into();
            self
        }

        pub fn sampler_heap_offset(mut self, sampler_heap_offset: u32) -> Self {
            self.sampler_heap_offset = sampler_heap_offset;
            self
        }

        pub fn sampler_push_offset(mut self, sampler_push_offset: u32) -> Self {
            self.sampler_push_offset = sampler_push_offset;
            self
        }

        pub fn sampler_heap_index_stride(mut self, sampler_heap_index_stride: u32) -> Self {
            self.sampler_heap_index_stride = sampler_heap_index_stride;
            self
        }

        pub fn sampler_heap_array_stride(mut self, sampler_heap_array_stride: u32) -> Self {
            self.sampler_heap_array_stride = sampler_heap_array_stride;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorMappingSourceIndirectIndexEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DescriptorMappingSourceIndirectIndexEXT<'a> {
        pub heap_offset: u32,
        pub push_offset: u32,
        pub address_offset: u32,
        pub heap_index_stride: u32,
        pub heap_array_stride: u32,
        pub p_embedded_sampler: *const SamplerCreateInfo<'a>,
        pub use_combined_image_sampler_index: Bool32,
        pub sampler_heap_offset: u32,
        pub sampler_push_offset: u32,
        pub sampler_address_offset: u32,
        pub sampler_heap_index_stride: u32,
        pub sampler_heap_array_stride: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorMappingSourceIndirectIndexEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorMappingSourceIndirectIndexEXT")
                .field("heap_offset", &self.heap_offset)
                .field("push_offset", &self.push_offset)
                .field("address_offset", &self.address_offset)
                .field("heap_index_stride", &self.heap_index_stride)
                .field("heap_array_stride", &self.heap_array_stride)
                .field("p_embedded_sampler", &self.p_embedded_sampler)
                .field(
                    "use_combined_image_sampler_index",
                    &self.use_combined_image_sampler_index,
                )
                .field("sampler_heap_offset", &self.sampler_heap_offset)
                .field("sampler_push_offset", &self.sampler_push_offset)
                .field("sampler_address_offset", &self.sampler_address_offset)
                .field("sampler_heap_index_stride", &self.sampler_heap_index_stride)
                .field("sampler_heap_array_stride", &self.sampler_heap_array_stride)
                .finish()
        }
    }

    impl Default for DescriptorMappingSourceIndirectIndexEXT<'_> {
        fn default() -> Self {
            Self {
                heap_offset: Default::default(),
                push_offset: Default::default(),
                address_offset: Default::default(),
                heap_index_stride: Default::default(),
                heap_array_stride: Default::default(),
                p_embedded_sampler: core::ptr::null(),
                use_combined_image_sampler_index: Default::default(),
                sampler_heap_offset: Default::default(),
                sampler_push_offset: Default::default(),
                sampler_address_offset: Default::default(),
                sampler_heap_index_stride: Default::default(),
                sampler_heap_array_stride: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DescriptorMappingSourceIndirectIndexEXT<'a> {
        pub fn heap_offset(mut self, heap_offset: u32) -> Self {
            self.heap_offset = heap_offset;
            self
        }

        pub fn push_offset(mut self, push_offset: u32) -> Self {
            self.push_offset = push_offset;
            self
        }

        pub fn address_offset(mut self, address_offset: u32) -> Self {
            self.address_offset = address_offset;
            self
        }

        pub fn heap_index_stride(mut self, heap_index_stride: u32) -> Self {
            self.heap_index_stride = heap_index_stride;
            self
        }

        pub fn heap_array_stride(mut self, heap_array_stride: u32) -> Self {
            self.heap_array_stride = heap_array_stride;
            self
        }

        pub fn embedded_sampler(mut self, embedded_sampler: &'a SamplerCreateInfo<'a>) -> Self {
            self.p_embedded_sampler = embedded_sampler;
            self
        }

        pub fn use_combined_image_sampler_index(
            mut self,
            use_combined_image_sampler_index: bool,
        ) -> Self {
            self.use_combined_image_sampler_index = use_combined_image_sampler_index.into();
            self
        }

        pub fn sampler_heap_offset(mut self, sampler_heap_offset: u32) -> Self {
            self.sampler_heap_offset = sampler_heap_offset;
            self
        }

        pub fn sampler_push_offset(mut self, sampler_push_offset: u32) -> Self {
            self.sampler_push_offset = sampler_push_offset;
            self
        }

        pub fn sampler_address_offset(mut self, sampler_address_offset: u32) -> Self {
            self.sampler_address_offset = sampler_address_offset;
            self
        }

        pub fn sampler_heap_index_stride(mut self, sampler_heap_index_stride: u32) -> Self {
            self.sampler_heap_index_stride = sampler_heap_index_stride;
            self
        }

        pub fn sampler_heap_array_stride(mut self, sampler_heap_array_stride: u32) -> Self {
            self.sampler_heap_array_stride = sampler_heap_array_stride;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorMappingSourceIndirectIndexArrayEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DescriptorMappingSourceIndirectIndexArrayEXT<'a> {
        pub heap_offset: u32,
        pub push_offset: u32,
        pub address_offset: u32,
        pub heap_index_stride: u32,
        pub p_embedded_sampler: *const SamplerCreateInfo<'a>,
        pub use_combined_image_sampler_index: Bool32,
        pub sampler_heap_offset: u32,
        pub sampler_push_offset: u32,
        pub sampler_address_offset: u32,
        pub sampler_heap_index_stride: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorMappingSourceIndirectIndexArrayEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorMappingSourceIndirectIndexArrayEXT")
                .field("heap_offset", &self.heap_offset)
                .field("push_offset", &self.push_offset)
                .field("address_offset", &self.address_offset)
                .field("heap_index_stride", &self.heap_index_stride)
                .field("p_embedded_sampler", &self.p_embedded_sampler)
                .field(
                    "use_combined_image_sampler_index",
                    &self.use_combined_image_sampler_index,
                )
                .field("sampler_heap_offset", &self.sampler_heap_offset)
                .field("sampler_push_offset", &self.sampler_push_offset)
                .field("sampler_address_offset", &self.sampler_address_offset)
                .field("sampler_heap_index_stride", &self.sampler_heap_index_stride)
                .finish()
        }
    }

    impl Default for DescriptorMappingSourceIndirectIndexArrayEXT<'_> {
        fn default() -> Self {
            Self {
                heap_offset: Default::default(),
                push_offset: Default::default(),
                address_offset: Default::default(),
                heap_index_stride: Default::default(),
                p_embedded_sampler: core::ptr::null(),
                use_combined_image_sampler_index: Default::default(),
                sampler_heap_offset: Default::default(),
                sampler_push_offset: Default::default(),
                sampler_address_offset: Default::default(),
                sampler_heap_index_stride: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DescriptorMappingSourceIndirectIndexArrayEXT<'a> {
        pub fn heap_offset(mut self, heap_offset: u32) -> Self {
            self.heap_offset = heap_offset;
            self
        }

        pub fn push_offset(mut self, push_offset: u32) -> Self {
            self.push_offset = push_offset;
            self
        }

        pub fn address_offset(mut self, address_offset: u32) -> Self {
            self.address_offset = address_offset;
            self
        }

        pub fn heap_index_stride(mut self, heap_index_stride: u32) -> Self {
            self.heap_index_stride = heap_index_stride;
            self
        }

        pub fn embedded_sampler(mut self, embedded_sampler: &'a SamplerCreateInfo<'a>) -> Self {
            self.p_embedded_sampler = embedded_sampler;
            self
        }

        pub fn use_combined_image_sampler_index(
            mut self,
            use_combined_image_sampler_index: bool,
        ) -> Self {
            self.use_combined_image_sampler_index = use_combined_image_sampler_index.into();
            self
        }

        pub fn sampler_heap_offset(mut self, sampler_heap_offset: u32) -> Self {
            self.sampler_heap_offset = sampler_heap_offset;
            self
        }

        pub fn sampler_push_offset(mut self, sampler_push_offset: u32) -> Self {
            self.sampler_push_offset = sampler_push_offset;
            self
        }

        pub fn sampler_address_offset(mut self, sampler_address_offset: u32) -> Self {
            self.sampler_address_offset = sampler_address_offset;
            self
        }

        pub fn sampler_heap_index_stride(mut self, sampler_heap_index_stride: u32) -> Self {
            self.sampler_heap_index_stride = sampler_heap_index_stride;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorMappingSourceHeapDataEXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    pub struct DescriptorMappingSourceHeapDataEXT {
        pub heap_offset: u32,
        pub push_offset: u32,
    }

    impl DescriptorMappingSourceHeapDataEXT {
        pub fn heap_offset(mut self, heap_offset: u32) -> Self {
            self.heap_offset = heap_offset;
            self
        }

        pub fn push_offset(mut self, push_offset: u32) -> Self {
            self.push_offset = push_offset;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorMappingSourceShaderRecordIndexEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DescriptorMappingSourceShaderRecordIndexEXT<'a> {
        pub heap_offset: u32,
        pub shader_record_offset: u32,
        pub heap_index_stride: u32,
        pub heap_array_stride: u32,
        pub p_embedded_sampler: *const SamplerCreateInfo<'a>,
        pub use_combined_image_sampler_index: Bool32,
        pub sampler_heap_offset: u32,
        pub sampler_shader_record_offset: u32,
        pub sampler_heap_index_stride: u32,
        pub sampler_heap_array_stride: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorMappingSourceShaderRecordIndexEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorMappingSourceShaderRecordIndexEXT")
                .field("heap_offset", &self.heap_offset)
                .field("shader_record_offset", &self.shader_record_offset)
                .field("heap_index_stride", &self.heap_index_stride)
                .field("heap_array_stride", &self.heap_array_stride)
                .field("p_embedded_sampler", &self.p_embedded_sampler)
                .field(
                    "use_combined_image_sampler_index",
                    &self.use_combined_image_sampler_index,
                )
                .field("sampler_heap_offset", &self.sampler_heap_offset)
                .field(
                    "sampler_shader_record_offset",
                    &self.sampler_shader_record_offset,
                )
                .field("sampler_heap_index_stride", &self.sampler_heap_index_stride)
                .field("sampler_heap_array_stride", &self.sampler_heap_array_stride)
                .finish()
        }
    }

    impl Default for DescriptorMappingSourceShaderRecordIndexEXT<'_> {
        fn default() -> Self {
            Self {
                heap_offset: Default::default(),
                shader_record_offset: Default::default(),
                heap_index_stride: Default::default(),
                heap_array_stride: Default::default(),
                p_embedded_sampler: core::ptr::null(),
                use_combined_image_sampler_index: Default::default(),
                sampler_heap_offset: Default::default(),
                sampler_shader_record_offset: Default::default(),
                sampler_heap_index_stride: Default::default(),
                sampler_heap_array_stride: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DescriptorMappingSourceShaderRecordIndexEXT<'a> {
        pub fn heap_offset(mut self, heap_offset: u32) -> Self {
            self.heap_offset = heap_offset;
            self
        }

        pub fn shader_record_offset(mut self, shader_record_offset: u32) -> Self {
            self.shader_record_offset = shader_record_offset;
            self
        }

        pub fn heap_index_stride(mut self, heap_index_stride: u32) -> Self {
            self.heap_index_stride = heap_index_stride;
            self
        }

        pub fn heap_array_stride(mut self, heap_array_stride: u32) -> Self {
            self.heap_array_stride = heap_array_stride;
            self
        }

        pub fn embedded_sampler(mut self, embedded_sampler: &'a SamplerCreateInfo<'a>) -> Self {
            self.p_embedded_sampler = embedded_sampler;
            self
        }

        pub fn use_combined_image_sampler_index(
            mut self,
            use_combined_image_sampler_index: bool,
        ) -> Self {
            self.use_combined_image_sampler_index = use_combined_image_sampler_index.into();
            self
        }

        pub fn sampler_heap_offset(mut self, sampler_heap_offset: u32) -> Self {
            self.sampler_heap_offset = sampler_heap_offset;
            self
        }

        pub fn sampler_shader_record_offset(mut self, sampler_shader_record_offset: u32) -> Self {
            self.sampler_shader_record_offset = sampler_shader_record_offset;
            self
        }

        pub fn sampler_heap_index_stride(mut self, sampler_heap_index_stride: u32) -> Self {
            self.sampler_heap_index_stride = sampler_heap_index_stride;
            self
        }

        pub fn sampler_heap_array_stride(mut self, sampler_heap_array_stride: u32) -> Self {
            self.sampler_heap_array_stride = sampler_heap_array_stride;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorMappingSourceIndirectAddressEXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    pub struct DescriptorMappingSourceIndirectAddressEXT {
        pub push_offset: u32,
        pub address_offset: u32,
    }

    impl DescriptorMappingSourceIndirectAddressEXT {
        pub fn push_offset(mut self, push_offset: u32) -> Self {
            self.push_offset = push_offset;
            self
        }

        pub fn address_offset(mut self, address_offset: u32) -> Self {
            self.address_offset = address_offset;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorSetAndBindingMappingEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct DescriptorSetAndBindingMappingEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub descriptor_set: u32,
        pub first_binding: u32,
        pub binding_count: u32,
        pub resource_mask: SpirvResourceTypeFlagsEXT,
        pub source: DescriptorMappingSourceEXT,
        pub source_data: DescriptorMappingSourceDataEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorSetAndBindingMappingEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorSetAndBindingMappingEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("descriptor_set", &self.descriptor_set)
                .field("first_binding", &self.first_binding)
                .field("binding_count", &self.binding_count)
                .field("resource_mask", &self.resource_mask)
                .field("source", &self.source)
                .field("source_data", &self.source_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DescriptorSetAndBindingMappingEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DESCRIPTOR_SET_AND_BINDING_MAPPING_EXT;
    }

    impl Default for DescriptorSetAndBindingMappingEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                descriptor_set: Default::default(),
                first_binding: Default::default(),
                binding_count: Default::default(),
                resource_mask: Default::default(),
                source: Default::default(),
                source_data: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DescriptorSetAndBindingMappingEXT<'a> {
        pub fn descriptor_set(mut self, descriptor_set: u32) -> Self {
            self.descriptor_set = descriptor_set;
            self
        }

        pub fn first_binding(mut self, first_binding: u32) -> Self {
            self.first_binding = first_binding;
            self
        }

        pub fn binding_count(mut self, binding_count: u32) -> Self {
            self.binding_count = binding_count;
            self
        }

        pub fn resource_mask(mut self, resource_mask: SpirvResourceTypeFlagsEXT) -> Self {
            self.resource_mask = resource_mask;
            self
        }

        pub fn source(mut self, source: DescriptorMappingSourceEXT) -> Self {
            self.source = source;
            self
        }

        pub fn source_data(mut self, source_data: DescriptorMappingSourceDataEXT<'a>) -> Self {
            self.source_data = source_data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkShaderDescriptorSetAndBindingMappingInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ShaderDescriptorSetAndBindingMappingInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub mapping_count: u32,
        pub p_mappings: *const DescriptorSetAndBindingMappingEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ShaderDescriptorSetAndBindingMappingInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ShaderDescriptorSetAndBindingMappingInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("mapping_count", &self.mapping_count)
                .field("p_mappings", &self.p_mappings)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ShaderDescriptorSetAndBindingMappingInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SHADER_DESCRIPTOR_SET_AND_BINDING_MAPPING_INFO_EXT;
    }

    unsafe impl<'a> Extends<PipelineShaderStageCreateInfo<'a>>
        for ShaderDescriptorSetAndBindingMappingInfoEXT<'a>
    {
    }
    unsafe impl<'a> Extends<ShaderCreateInfoEXT<'a>>
        for ShaderDescriptorSetAndBindingMappingInfoEXT<'a>
    {
    }

    impl Default for ShaderDescriptorSetAndBindingMappingInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                mapping_count: Default::default(),
                p_mappings: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ShaderDescriptorSetAndBindingMappingInfoEXT<'a> {
        pub fn mappings(mut self, mappings: &'a [DescriptorSetAndBindingMappingEXT<'a>]) -> Self {
            self.mapping_count = mappings.len().try_into().unwrap();
            self.p_mappings = mappings.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerCustomBorderColorIndexCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SamplerCustomBorderColorIndexCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub index: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SamplerCustomBorderColorIndexCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SamplerCustomBorderColorIndexCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("index", &self.index)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SamplerCustomBorderColorIndexCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SAMPLER_CUSTOM_BORDER_COLOR_INDEX_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<SamplerCreateInfo<'a>> for SamplerCustomBorderColorIndexCreateInfoEXT<'a> {}

    impl Default for SamplerCustomBorderColorIndexCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SamplerCustomBorderColorIndexCreateInfoEXT<'a> {
        pub fn index(mut self, index: u32) -> Self {
            self.index = index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOpaqueCaptureDataCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct OpaqueCaptureDataCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_data: *const HostAddressRangeConstEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for OpaqueCaptureDataCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OpaqueCaptureDataCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_data", &self.p_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for OpaqueCaptureDataCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::OPAQUE_CAPTURE_DATA_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<ImageCreateInfo<'a>> for OpaqueCaptureDataCreateInfoEXT<'a> {}
    unsafe impl<'a> Extends<TensorCreateInfoARM<'a>> for OpaqueCaptureDataCreateInfoEXT<'a> {}

    impl Default for OpaqueCaptureDataCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_data: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> OpaqueCaptureDataCreateInfoEXT<'a> {
        pub fn data(mut self, data: &'a HostAddressRangeConstEXT<'a>) -> Self {
            self.p_data = data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsLayoutPushDataTokenNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct IndirectCommandsLayoutPushDataTokenNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub push_data_offset: u32,
        pub push_data_size: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for IndirectCommandsLayoutPushDataTokenNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("IndirectCommandsLayoutPushDataTokenNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("push_data_offset", &self.push_data_offset)
                .field("push_data_size", &self.push_data_size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for IndirectCommandsLayoutPushDataTokenNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::INDIRECT_COMMANDS_LAYOUT_PUSH_DATA_TOKEN_NV;
    }

    unsafe impl<'a> Extends<IndirectCommandsLayoutTokenNV<'a>>
        for IndirectCommandsLayoutPushDataTokenNV<'a>
    {
    }

    impl Default for IndirectCommandsLayoutPushDataTokenNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                push_data_offset: Default::default(),
                push_data_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> IndirectCommandsLayoutPushDataTokenNV<'a> {
        pub fn push_data_offset(mut self, push_data_offset: u32) -> Self {
            self.push_data_offset = push_data_offset;
            self
        }

        pub fn push_data_size(mut self, push_data_size: u32) -> Self {
            self.push_data_size = push_data_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubsampledImageFormatPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SubsampledImageFormatPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub subsampled_image_descriptor_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SubsampledImageFormatPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SubsampledImageFormatPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "subsampled_image_descriptor_count",
                    &self.subsampled_image_descriptor_count,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SubsampledImageFormatPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SUBSAMPLED_IMAGE_FORMAT_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<ImageFormatProperties2<'a>> for SubsampledImageFormatPropertiesEXT<'a> {}

    impl Default for SubsampledImageFormatPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                subsampled_image_descriptor_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SubsampledImageFormatPropertiesEXT<'a> {
        pub fn subsampled_image_descriptor_count(
            mut self,
            subsampled_image_descriptor_count: u32,
        ) -> Self {
            self.subsampled_image_descriptor_count = subsampled_image_descriptor_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDescriptorHeapFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDescriptorHeapFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub descriptor_heap: Bool32,
        pub descriptor_heap_capture_replay: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDescriptorHeapFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDescriptorHeapFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("descriptor_heap", &self.descriptor_heap)
                .field(
                    "descriptor_heap_capture_replay",
                    &self.descriptor_heap_capture_replay,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDescriptorHeapFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DESCRIPTOR_HEAP_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDescriptorHeapFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceDescriptorHeapFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceDescriptorHeapFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                descriptor_heap: Default::default(),
                descriptor_heap_capture_replay: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDescriptorHeapFeaturesEXT<'a> {
        pub fn descriptor_heap(mut self, descriptor_heap: bool) -> Self {
            self.descriptor_heap = descriptor_heap.into();
            self
        }

        pub fn descriptor_heap_capture_replay(
            mut self,
            descriptor_heap_capture_replay: bool,
        ) -> Self {
            self.descriptor_heap_capture_replay = descriptor_heap_capture_replay.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDescriptorHeapPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDescriptorHeapPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub sampler_heap_alignment: DeviceSize,
        pub resource_heap_alignment: DeviceSize,
        pub max_sampler_heap_size: DeviceSize,
        pub max_resource_heap_size: DeviceSize,
        pub min_sampler_heap_reserved_range: DeviceSize,
        pub min_sampler_heap_reserved_range_with_embedded: DeviceSize,
        pub min_resource_heap_reserved_range: DeviceSize,
        pub sampler_descriptor_size: DeviceSize,
        pub image_descriptor_size: DeviceSize,
        pub buffer_descriptor_size: DeviceSize,
        pub sampler_descriptor_alignment: DeviceSize,
        pub image_descriptor_alignment: DeviceSize,
        pub buffer_descriptor_alignment: DeviceSize,
        pub max_push_data_size: DeviceSize,
        pub image_capture_replay_opaque_data_size: usize,
        pub max_descriptor_heap_embedded_samplers: u32,
        pub sampler_ycbcr_conversion_count: u32,
        pub sparse_descriptor_heaps: Bool32,
        pub protected_descriptor_heaps: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDescriptorHeapPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDescriptorHeapPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("sampler_heap_alignment", &self.sampler_heap_alignment)
                .field("resource_heap_alignment", &self.resource_heap_alignment)
                .field("max_sampler_heap_size", &self.max_sampler_heap_size)
                .field("max_resource_heap_size", &self.max_resource_heap_size)
                .field(
                    "min_sampler_heap_reserved_range",
                    &self.min_sampler_heap_reserved_range,
                )
                .field(
                    "min_sampler_heap_reserved_range_with_embedded",
                    &self.min_sampler_heap_reserved_range_with_embedded,
                )
                .field(
                    "min_resource_heap_reserved_range",
                    &self.min_resource_heap_reserved_range,
                )
                .field("sampler_descriptor_size", &self.sampler_descriptor_size)
                .field("image_descriptor_size", &self.image_descriptor_size)
                .field("buffer_descriptor_size", &self.buffer_descriptor_size)
                .field(
                    "sampler_descriptor_alignment",
                    &self.sampler_descriptor_alignment,
                )
                .field(
                    "image_descriptor_alignment",
                    &self.image_descriptor_alignment,
                )
                .field(
                    "buffer_descriptor_alignment",
                    &self.buffer_descriptor_alignment,
                )
                .field("max_push_data_size", &self.max_push_data_size)
                .field(
                    "image_capture_replay_opaque_data_size",
                    &self.image_capture_replay_opaque_data_size,
                )
                .field(
                    "max_descriptor_heap_embedded_samplers",
                    &self.max_descriptor_heap_embedded_samplers,
                )
                .field(
                    "sampler_ycbcr_conversion_count",
                    &self.sampler_ycbcr_conversion_count,
                )
                .field("sparse_descriptor_heaps", &self.sparse_descriptor_heaps)
                .field(
                    "protected_descriptor_heaps",
                    &self.protected_descriptor_heaps,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDescriptorHeapPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DESCRIPTOR_HEAP_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceDescriptorHeapPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceDescriptorHeapPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                sampler_heap_alignment: Default::default(),
                resource_heap_alignment: Default::default(),
                max_sampler_heap_size: Default::default(),
                max_resource_heap_size: Default::default(),
                min_sampler_heap_reserved_range: Default::default(),
                min_sampler_heap_reserved_range_with_embedded: Default::default(),
                min_resource_heap_reserved_range: Default::default(),
                sampler_descriptor_size: Default::default(),
                image_descriptor_size: Default::default(),
                buffer_descriptor_size: Default::default(),
                sampler_descriptor_alignment: Default::default(),
                image_descriptor_alignment: Default::default(),
                buffer_descriptor_alignment: Default::default(),
                max_push_data_size: Default::default(),
                image_capture_replay_opaque_data_size: Default::default(),
                max_descriptor_heap_embedded_samplers: Default::default(),
                sampler_ycbcr_conversion_count: Default::default(),
                sparse_descriptor_heaps: Default::default(),
                protected_descriptor_heaps: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDescriptorHeapPropertiesEXT<'a> {
        pub fn sampler_heap_alignment(mut self, sampler_heap_alignment: DeviceSize) -> Self {
            self.sampler_heap_alignment = sampler_heap_alignment;
            self
        }

        pub fn resource_heap_alignment(mut self, resource_heap_alignment: DeviceSize) -> Self {
            self.resource_heap_alignment = resource_heap_alignment;
            self
        }

        pub fn max_sampler_heap_size(mut self, max_sampler_heap_size: DeviceSize) -> Self {
            self.max_sampler_heap_size = max_sampler_heap_size;
            self
        }

        pub fn max_resource_heap_size(mut self, max_resource_heap_size: DeviceSize) -> Self {
            self.max_resource_heap_size = max_resource_heap_size;
            self
        }

        pub fn min_sampler_heap_reserved_range(
            mut self,
            min_sampler_heap_reserved_range: DeviceSize,
        ) -> Self {
            self.min_sampler_heap_reserved_range = min_sampler_heap_reserved_range;
            self
        }

        pub fn min_sampler_heap_reserved_range_with_embedded(
            mut self,
            min_sampler_heap_reserved_range_with_embedded: DeviceSize,
        ) -> Self {
            self.min_sampler_heap_reserved_range_with_embedded =
                min_sampler_heap_reserved_range_with_embedded;
            self
        }

        pub fn min_resource_heap_reserved_range(
            mut self,
            min_resource_heap_reserved_range: DeviceSize,
        ) -> Self {
            self.min_resource_heap_reserved_range = min_resource_heap_reserved_range;
            self
        }

        pub fn sampler_descriptor_size(mut self, sampler_descriptor_size: DeviceSize) -> Self {
            self.sampler_descriptor_size = sampler_descriptor_size;
            self
        }

        pub fn image_descriptor_size(mut self, image_descriptor_size: DeviceSize) -> Self {
            self.image_descriptor_size = image_descriptor_size;
            self
        }

        pub fn buffer_descriptor_size(mut self, buffer_descriptor_size: DeviceSize) -> Self {
            self.buffer_descriptor_size = buffer_descriptor_size;
            self
        }

        pub fn sampler_descriptor_alignment(
            mut self,
            sampler_descriptor_alignment: DeviceSize,
        ) -> Self {
            self.sampler_descriptor_alignment = sampler_descriptor_alignment;
            self
        }

        pub fn image_descriptor_alignment(
            mut self,
            image_descriptor_alignment: DeviceSize,
        ) -> Self {
            self.image_descriptor_alignment = image_descriptor_alignment;
            self
        }

        pub fn buffer_descriptor_alignment(
            mut self,
            buffer_descriptor_alignment: DeviceSize,
        ) -> Self {
            self.buffer_descriptor_alignment = buffer_descriptor_alignment;
            self
        }

        pub fn max_push_data_size(mut self, max_push_data_size: DeviceSize) -> Self {
            self.max_push_data_size = max_push_data_size;
            self
        }

        pub fn image_capture_replay_opaque_data_size(
            mut self,
            image_capture_replay_opaque_data_size: usize,
        ) -> Self {
            self.image_capture_replay_opaque_data_size = image_capture_replay_opaque_data_size;
            self
        }

        pub fn max_descriptor_heap_embedded_samplers(
            mut self,
            max_descriptor_heap_embedded_samplers: u32,
        ) -> Self {
            self.max_descriptor_heap_embedded_samplers = max_descriptor_heap_embedded_samplers;
            self
        }

        pub fn sampler_ycbcr_conversion_count(
            mut self,
            sampler_ycbcr_conversion_count: u32,
        ) -> Self {
            self.sampler_ycbcr_conversion_count = sampler_ycbcr_conversion_count;
            self
        }

        pub fn sparse_descriptor_heaps(mut self, sparse_descriptor_heaps: bool) -> Self {
            self.sparse_descriptor_heaps = sparse_descriptor_heaps.into();
            self
        }

        pub fn protected_descriptor_heaps(mut self, protected_descriptor_heaps: bool) -> Self {
            self.protected_descriptor_heaps = protected_descriptor_heaps.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCommandBufferInheritanceDescriptorHeapInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CommandBufferInheritanceDescriptorHeapInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_sampler_heap_bind_info: *const BindHeapInfoEXT<'a>,
        pub p_resource_heap_bind_info: *const BindHeapInfoEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CommandBufferInheritanceDescriptorHeapInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CommandBufferInheritanceDescriptorHeapInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_sampler_heap_bind_info", &self.p_sampler_heap_bind_info)
                .field("p_resource_heap_bind_info", &self.p_resource_heap_bind_info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CommandBufferInheritanceDescriptorHeapInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::COMMAND_BUFFER_INHERITANCE_DESCRIPTOR_HEAP_INFO_EXT;
    }

    unsafe impl<'a> Extends<CommandBufferInheritanceInfo<'a>>
        for CommandBufferInheritanceDescriptorHeapInfoEXT<'a>
    {
    }

    impl Default for CommandBufferInheritanceDescriptorHeapInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_sampler_heap_bind_info: core::ptr::null(),
                p_resource_heap_bind_info: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CommandBufferInheritanceDescriptorHeapInfoEXT<'a> {
        pub fn sampler_heap_bind_info(
            mut self,
            sampler_heap_bind_info: &'a BindHeapInfoEXT<'a>,
        ) -> Self {
            self.p_sampler_heap_bind_info = sampler_heap_bind_info;
            self
        }

        pub fn resource_heap_bind_info(
            mut self,
            resource_heap_bind_info: &'a BindHeapInfoEXT<'a>,
        ) -> Self {
            self.p_resource_heap_bind_info = resource_heap_bind_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDescriptorHeapTensorPropertiesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDescriptorHeapTensorPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub tensor_descriptor_size: DeviceSize,
        pub tensor_descriptor_alignment: DeviceSize,
        pub tensor_capture_replay_opaque_data_size: usize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDescriptorHeapTensorPropertiesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDescriptorHeapTensorPropertiesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("tensor_descriptor_size", &self.tensor_descriptor_size)
                .field(
                    "tensor_descriptor_alignment",
                    &self.tensor_descriptor_alignment,
                )
                .field(
                    "tensor_capture_replay_opaque_data_size",
                    &self.tensor_capture_replay_opaque_data_size,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDescriptorHeapTensorPropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DESCRIPTOR_HEAP_TENSOR_PROPERTIES_ARM;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceDescriptorHeapTensorPropertiesARM<'a>
    {
    }

    impl Default for PhysicalDeviceDescriptorHeapTensorPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                tensor_descriptor_size: Default::default(),
                tensor_descriptor_alignment: Default::default(),
                tensor_capture_replay_opaque_data_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDescriptorHeapTensorPropertiesARM<'a> {
        pub fn tensor_descriptor_size(mut self, tensor_descriptor_size: DeviceSize) -> Self {
            self.tensor_descriptor_size = tensor_descriptor_size;
            self
        }

        pub fn tensor_descriptor_alignment(
            mut self,
            tensor_descriptor_alignment: DeviceSize,
        ) -> Self {
            self.tensor_descriptor_alignment = tensor_descriptor_alignment;
            self
        }

        pub fn tensor_capture_replay_opaque_data_size(
            mut self,
            tensor_capture_replay_opaque_data_size: usize,
        ) -> Self {
            self.tensor_capture_replay_opaque_data_size = tensor_capture_replay_opaque_data_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkResourceDescriptorDataEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union ResourceDescriptorDataEXT<'a> {
        pub p_image: *const ImageDescriptorInfoEXT<'a>,
        pub p_texel_buffer: *const TexelBufferDescriptorInfoEXT<'a>,
        pub p_address_range: *const DeviceAddressRangeEXT,
        pub p_tensor_arm: *const TensorViewCreateInfoARM<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ResourceDescriptorDataEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ResourceDescriptorDataEXT").finish()
        }
    }

    impl Default for ResourceDescriptorDataEXT<'_> {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorMappingSourceDataEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union DescriptorMappingSourceDataEXT<'a> {
        pub constant_offset: DescriptorMappingSourceConstantOffsetEXT<'a>,
        pub push_index: DescriptorMappingSourcePushIndexEXT<'a>,
        pub indirect_index: DescriptorMappingSourceIndirectIndexEXT<'a>,
        pub indirect_index_array: DescriptorMappingSourceIndirectIndexArrayEXT<'a>,
        pub heap_data: DescriptorMappingSourceHeapDataEXT,
        pub push_data_offset: u32,
        pub push_address_offset: u32,
        pub indirect_address: DescriptorMappingSourceIndirectAddressEXT,
        pub shader_record_index: DescriptorMappingSourceShaderRecordIndexEXT<'a>,
        pub shader_record_data_offset: u32,
        pub shader_record_address_offset: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DescriptorMappingSourceDataEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DescriptorMappingSourceDataEXT").finish()
        }
    }

    impl Default for DescriptorMappingSourceDataEXT<'_> {
        fn default() -> Self {
            unsafe { core::mem::zeroed() }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDescriptorMappingSourceEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DescriptorMappingSourceEXT(i32);

    impl DescriptorMappingSourceEXT {
        pub const HEAP_WITH_CONSTANT_OFFSET_EXT: Self = Self(0);
        pub const HEAP_WITH_PUSH_INDEX_EXT: Self = Self(1);
        pub const HEAP_WITH_INDIRECT_INDEX_EXT: Self = Self(2);
        pub const HEAP_WITH_INDIRECT_INDEX_ARRAY_EXT: Self = Self(3);
        pub const RESOURCE_HEAP_DATA_EXT: Self = Self(4);
        pub const PUSH_DATA_EXT: Self = Self(5);
        pub const PUSH_ADDRESS_EXT: Self = Self(6);
        pub const INDIRECT_ADDRESS_EXT: Self = Self(7);
        // VK_EXT_descriptor_heap
        pub const HEAP_WITH_SHADER_RECORD_INDEX_EXT: Self = Self(8);
        pub const SHADER_RECORD_DATA_EXT: Self = Self(9);
        pub const SHADER_RECORD_ADDRESS_EXT: Self = Self(10);
    }

    impl fmt::Debug for DescriptorMappingSourceEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::HEAP_WITH_CONSTANT_OFFSET_EXT => Some("HEAP_WITH_CONSTANT_OFFSET_EXT"),
                Self::HEAP_WITH_PUSH_INDEX_EXT => Some("HEAP_WITH_PUSH_INDEX_EXT"),
                Self::HEAP_WITH_INDIRECT_INDEX_EXT => Some("HEAP_WITH_INDIRECT_INDEX_EXT"),
                Self::HEAP_WITH_INDIRECT_INDEX_ARRAY_EXT => {
                    Some("HEAP_WITH_INDIRECT_INDEX_ARRAY_EXT")
                }
                Self::RESOURCE_HEAP_DATA_EXT => Some("RESOURCE_HEAP_DATA_EXT"),
                Self::PUSH_DATA_EXT => Some("PUSH_DATA_EXT"),
                Self::PUSH_ADDRESS_EXT => Some("PUSH_ADDRESS_EXT"),
                Self::INDIRECT_ADDRESS_EXT => Some("INDIRECT_ADDRESS_EXT"),
                Self::HEAP_WITH_SHADER_RECORD_INDEX_EXT => {
                    Some("HEAP_WITH_SHADER_RECORD_INDEX_EXT")
                }
                Self::SHADER_RECORD_DATA_EXT => Some("SHADER_RECORD_DATA_EXT"),
                Self::SHADER_RECORD_ADDRESS_EXT => Some("SHADER_RECORD_ADDRESS_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSpirvResourceTypeFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SpirvResourceTypeFlagsEXT(Flags);
    vk_bitflags_wrapped!(SpirvResourceTypeFlagsEXT, Flags);

    impl SpirvResourceTypeFlagsEXT {
        pub const SAMPLER_EXT: Self = Self(SpirvResourceTypeFlagBitsEXT::SAMPLER_EXT.0);
        pub const SAMPLED_IMAGE_EXT: Self = Self(SpirvResourceTypeFlagBitsEXT::SAMPLED_IMAGE_EXT.0);
        pub const READ_ONLY_IMAGE_EXT: Self =
            Self(SpirvResourceTypeFlagBitsEXT::READ_ONLY_IMAGE_EXT.0);
        pub const READ_WRITE_IMAGE_EXT: Self =
            Self(SpirvResourceTypeFlagBitsEXT::READ_WRITE_IMAGE_EXT.0);
        pub const COMBINED_SAMPLED_IMAGE_EXT: Self =
            Self(SpirvResourceTypeFlagBitsEXT::COMBINED_SAMPLED_IMAGE_EXT.0);
        pub const UNIFORM_BUFFER_EXT: Self =
            Self(SpirvResourceTypeFlagBitsEXT::UNIFORM_BUFFER_EXT.0);
        pub const READ_ONLY_STORAGE_BUFFER_EXT: Self =
            Self(SpirvResourceTypeFlagBitsEXT::READ_ONLY_STORAGE_BUFFER_EXT.0);
        pub const READ_WRITE_STORAGE_BUFFER_EXT: Self =
            Self(SpirvResourceTypeFlagBitsEXT::READ_WRITE_STORAGE_BUFFER_EXT.0);
        pub const ALL: Self = Self(0x7FFFFFFF);
        // VK_EXT_descriptor_heap
        pub const ACCELERATION_STRUCTURE_EXT: Self =
            Self(SpirvResourceTypeFlagBitsEXT::ACCELERATION_STRUCTURE_EXT.0);
        pub const TENSOR_ARM: Self = Self(SpirvResourceTypeFlagBitsEXT::TENSOR_ARM.0);
    }

    impl fmt::Debug for SpirvResourceTypeFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (SpirvResourceTypeFlagsEXT::SAMPLER_EXT.0, "SAMPLER_EXT"),
                (
                    SpirvResourceTypeFlagsEXT::SAMPLED_IMAGE_EXT.0,
                    "SAMPLED_IMAGE_EXT",
                ),
                (
                    SpirvResourceTypeFlagsEXT::READ_ONLY_IMAGE_EXT.0,
                    "READ_ONLY_IMAGE_EXT",
                ),
                (
                    SpirvResourceTypeFlagsEXT::READ_WRITE_IMAGE_EXT.0,
                    "READ_WRITE_IMAGE_EXT",
                ),
                (
                    SpirvResourceTypeFlagsEXT::COMBINED_SAMPLED_IMAGE_EXT.0,
                    "COMBINED_SAMPLED_IMAGE_EXT",
                ),
                (
                    SpirvResourceTypeFlagsEXT::UNIFORM_BUFFER_EXT.0,
                    "UNIFORM_BUFFER_EXT",
                ),
                (
                    SpirvResourceTypeFlagsEXT::READ_ONLY_STORAGE_BUFFER_EXT.0,
                    "READ_ONLY_STORAGE_BUFFER_EXT",
                ),
                (
                    SpirvResourceTypeFlagsEXT::READ_WRITE_STORAGE_BUFFER_EXT.0,
                    "READ_WRITE_STORAGE_BUFFER_EXT",
                ),
                (
                    SpirvResourceTypeFlagsEXT::ACCELERATION_STRUCTURE_EXT.0,
                    "ACCELERATION_STRUCTURE_EXT",
                ),
                (SpirvResourceTypeFlagsEXT::TENSOR_ARM.0, "TENSOR_ARM"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSpirvResourceTypeFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct SpirvResourceTypeFlagBitsEXT(u32);

    impl SpirvResourceTypeFlagBitsEXT {
        pub const SAMPLER_EXT: Self = Self(1 << 0);
        pub const SAMPLED_IMAGE_EXT: Self = Self(1 << 1);
        pub const READ_ONLY_IMAGE_EXT: Self = Self(1 << 2);
        pub const READ_WRITE_IMAGE_EXT: Self = Self(1 << 3);
        pub const COMBINED_SAMPLED_IMAGE_EXT: Self = Self(1 << 4);
        pub const UNIFORM_BUFFER_EXT: Self = Self(1 << 5);
        pub const READ_ONLY_STORAGE_BUFFER_EXT: Self = Self(1 << 6);
        pub const READ_WRITE_STORAGE_BUFFER_EXT: Self = Self(1 << 7);
        // VK_EXT_descriptor_heap
        pub const ACCELERATION_STRUCTURE_EXT: Self = Self(1 << 8);
        pub const TENSOR_ARM: Self = Self(1 << 9);
    }

    impl fmt::Debug for SpirvResourceTypeFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SAMPLER_EXT => Some("SAMPLER_EXT"),
                Self::SAMPLED_IMAGE_EXT => Some("SAMPLED_IMAGE_EXT"),
                Self::READ_ONLY_IMAGE_EXT => Some("READ_ONLY_IMAGE_EXT"),
                Self::READ_WRITE_IMAGE_EXT => Some("READ_WRITE_IMAGE_EXT"),
                Self::COMBINED_SAMPLED_IMAGE_EXT => Some("COMBINED_SAMPLED_IMAGE_EXT"),
                Self::UNIFORM_BUFFER_EXT => Some("UNIFORM_BUFFER_EXT"),
                Self::READ_ONLY_STORAGE_BUFFER_EXT => Some("READ_ONLY_STORAGE_BUFFER_EXT"),
                Self::READ_WRITE_STORAGE_BUFFER_EXT => Some("READ_WRITE_STORAGE_BUFFER_EXT"),
                Self::ACCELERATION_STRUCTURE_EXT => Some("ACCELERATION_STRUCTURE_EXT"),
                Self::TENSOR_ARM => Some("TENSOR_ARM"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkWriteSamplerDescriptorsEXT.html>
    pub type PFN_vkWriteSamplerDescriptorsEXT = unsafe extern "system" fn(
        device: Device,
        sampler_count: u32,
        p_samplers: *const SamplerCreateInfo<'_>,
        p_descriptors: *const HostAddressRangeEXT<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkWriteResourceDescriptorsEXT.html>
    pub type PFN_vkWriteResourceDescriptorsEXT = unsafe extern "system" fn(
        device: Device,
        resource_count: u32,
        p_resources: *const ResourceDescriptorInfoEXT<'_>,
        p_descriptors: *const HostAddressRangeEXT<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindSamplerHeapEXT.html>
    pub type PFN_vkCmdBindSamplerHeapEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_bind_info: *const BindHeapInfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindResourceHeapEXT.html>
    pub type PFN_vkCmdBindResourceHeapEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_bind_info: *const BindHeapInfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushDataEXT.html>
    pub type PFN_vkCmdPushDataEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_push_data_info: *const PushDataInfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkRegisterCustomBorderColorEXT.html>
    pub type PFN_vkRegisterCustomBorderColorEXT = unsafe extern "system" fn(
        device: Device,
        p_border_color: *const SamplerCustomBorderColorCreateInfoEXT<'_>,
        request_index: Bool32,
        p_index: *mut u32,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUnregisterCustomBorderColorEXT.html>
    pub type PFN_vkUnregisterCustomBorderColorEXT =
        unsafe extern "system" fn(device: Device, index: u32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageOpaqueCaptureDataEXT.html>
    pub type PFN_vkGetImageOpaqueCaptureDataEXT = unsafe extern "system" fn(
        device: Device,
        image_count: u32,
        p_images: *const Image,
        p_datas: *mut HostAddressRangeEXT<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceDescriptorSizeEXT.html>
    pub type PFN_vkGetPhysicalDeviceDescriptorSizeEXT = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        descriptor_type: DescriptorType,
    )
        -> DeviceSize;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetTensorOpaqueCaptureDataARM.html>
    pub type PFN_vkGetTensorOpaqueCaptureDataARM = unsafe extern "system" fn(
        device: Device,
        tensor_count: u32,
        p_tensors: *const TensorARM,
        p_datas: *mut HostAddressRangeEXT<'_>,
    ) -> vk::Result;
}

pub struct InstanceFn {
    get_physical_device_descriptor_size_ext: PFN_vkGetPhysicalDeviceDescriptorSizeEXT,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_descriptor_size_ext: transmute(
                    load(c"vkGetPhysicalDeviceDescriptorSizeEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceDescriptorSizeEXT.html>
    pub unsafe fn get_physical_device_descriptor_size_ext(
        &self,
        physical_device: PhysicalDevice,
        descriptor_type: DescriptorType,
    ) -> DeviceSize {
        unsafe { (self.get_physical_device_descriptor_size_ext)(physical_device, descriptor_type) }
    }
}

pub struct DeviceFn {
    write_sampler_descriptors_ext: PFN_vkWriteSamplerDescriptorsEXT,
    write_resource_descriptors_ext: PFN_vkWriteResourceDescriptorsEXT,
    cmd_bind_sampler_heap_ext: PFN_vkCmdBindSamplerHeapEXT,
    cmd_bind_resource_heap_ext: PFN_vkCmdBindResourceHeapEXT,
    cmd_push_data_ext: PFN_vkCmdPushDataEXT,
    get_image_opaque_capture_data_ext: PFN_vkGetImageOpaqueCaptureDataEXT,
    register_custom_border_color_ext: Option<PFN_vkRegisterCustomBorderColorEXT>,
    unregister_custom_border_color_ext: Option<PFN_vkUnregisterCustomBorderColorEXT>,
    get_tensor_opaque_capture_data_arm: Option<PFN_vkGetTensorOpaqueCaptureDataARM>,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                write_sampler_descriptors_ext: transmute(
                    load(c"vkWriteSamplerDescriptorsEXT").ok_or(MissingEntryPointError)?,
                ),
                write_resource_descriptors_ext: transmute(
                    load(c"vkWriteResourceDescriptorsEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_bind_sampler_heap_ext: transmute(
                    load(c"vkCmdBindSamplerHeapEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_bind_resource_heap_ext: transmute(
                    load(c"vkCmdBindResourceHeapEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_push_data_ext: transmute(
                    load(c"vkCmdPushDataEXT").ok_or(MissingEntryPointError)?,
                ),
                get_image_opaque_capture_data_ext: transmute(
                    load(c"vkGetImageOpaqueCaptureDataEXT").ok_or(MissingEntryPointError)?,
                ),
                register_custom_border_color_ext: transmute(load(
                    c"vkRegisterCustomBorderColorEXT",
                )),
                unregister_custom_border_color_ext: transmute(load(
                    c"vkUnregisterCustomBorderColorEXT",
                )),
                get_tensor_opaque_capture_data_arm: transmute(load(
                    c"vkGetTensorOpaqueCaptureDataARM",
                )),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkWriteSamplerDescriptorsEXT.html>
    pub unsafe fn write_sampler_descriptors_ext(
        &self,
        device: Device,
        samplers: &[SamplerCreateInfo<'_>],
        descriptors: &[HostAddressRangeEXT<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.write_sampler_descriptors_ext)(
                device,
                samplers.len().try_into().unwrap(),
                samplers.as_ptr() as _,
                descriptors.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkWriteResourceDescriptorsEXT.html>
    pub unsafe fn write_resource_descriptors_ext(
        &self,
        device: Device,
        resources: &[ResourceDescriptorInfoEXT<'_>],
        descriptors: &[HostAddressRangeEXT<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.write_resource_descriptors_ext)(
                device,
                resources.len().try_into().unwrap(),
                resources.as_ptr() as _,
                descriptors.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindSamplerHeapEXT.html>
    pub unsafe fn cmd_bind_sampler_heap_ext(
        &self,
        command_buffer: CommandBuffer,
        bind_info: &BindHeapInfoEXT<'_>,
    ) {
        unsafe { (self.cmd_bind_sampler_heap_ext)(command_buffer, bind_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindResourceHeapEXT.html>
    pub unsafe fn cmd_bind_resource_heap_ext(
        &self,
        command_buffer: CommandBuffer,
        bind_info: &BindHeapInfoEXT<'_>,
    ) {
        unsafe { (self.cmd_bind_resource_heap_ext)(command_buffer, bind_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPushDataEXT.html>
    pub unsafe fn cmd_push_data_ext(
        &self,
        command_buffer: CommandBuffer,
        push_data_info: &PushDataInfoEXT<'_>,
    ) {
        unsafe { (self.cmd_push_data_ext)(command_buffer, push_data_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetImageOpaqueCaptureDataEXT.html>
    pub unsafe fn get_image_opaque_capture_data_ext(
        &self,
        device: Device,
        images: &[Image],
        datas: &mut [HostAddressRangeEXT<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_image_opaque_capture_data_ext)(
                device,
                images.len().try_into().unwrap(),
                images.as_ptr() as _,
                datas.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkRegisterCustomBorderColorEXT.html>
    pub unsafe fn register_custom_border_color_ext(
        &self,
        device: Device,
        border_color: &SamplerCustomBorderColorCreateInfoEXT<'_>,
        request_index: bool,
    ) -> crate::Result<u32> {
        unsafe {
            let mut index = core::mem::MaybeUninit::uninit();
            let result = (self.register_custom_border_color_ext.unwrap())(
                device,
                border_color,
                request_index.into(),
                index.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(index.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkUnregisterCustomBorderColorEXT.html>
    pub unsafe fn unregister_custom_border_color_ext(&self, device: Device, index: u32) {
        unsafe { (self.unregister_custom_border_color_ext.unwrap())(device, index) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetTensorOpaqueCaptureDataARM.html>
    pub unsafe fn get_tensor_opaque_capture_data_arm(
        &self,
        device: Device,
        tensors: &[TensorARM],
        datas: &mut [HostAddressRangeEXT<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_tensor_opaque_capture_data_arm.unwrap())(
                device,
                tensors.len().try_into().unwrap(),
                tensors.as_ptr() as _,
                datas.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}

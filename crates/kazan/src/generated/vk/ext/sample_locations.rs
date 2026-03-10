//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_sample_locations.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_sample_locations";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSampleLocationEXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct SampleLocationEXT {
        pub x: f32,
        pub y: f32,
    }

    impl SampleLocationEXT {
        #[inline]
        pub fn x(mut self, x: f32) -> Self {
            self.x = x;
            self
        }

        #[inline]
        pub fn y(mut self, y: f32) -> Self {
            self.y = y;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSampleLocationsInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SampleLocationsInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub sample_locations_per_pixel: SampleCountFlagBits,
        pub sample_location_grid_size: Extent2D,
        pub sample_locations_count: u32,
        pub p_sample_locations: *const SampleLocationEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SampleLocationsInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SampleLocationsInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "sample_locations_per_pixel",
                    &self.sample_locations_per_pixel,
                )
                .field("sample_location_grid_size", &self.sample_location_grid_size)
                .field("sample_locations_count", &self.sample_locations_count)
                .field("p_sample_locations", &self.p_sample_locations)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SampleLocationsInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SAMPLE_LOCATIONS_INFO_EXT;
    }

    unsafe impl Extends<ImageMemoryBarrier<'_>> for SampleLocationsInfoEXT<'_> {}
    unsafe impl Extends<ImageMemoryBarrier2<'_>> for SampleLocationsInfoEXT<'_> {}

    impl Default for SampleLocationsInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                sample_locations_per_pixel: Default::default(),
                sample_location_grid_size: Default::default(),
                sample_locations_count: Default::default(),
                p_sample_locations: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SampleLocationsInfoEXT<'a> {
        #[inline]
        pub fn sample_locations_per_pixel(
            mut self,
            sample_locations_per_pixel: SampleCountFlagBits,
        ) -> Self {
            self.sample_locations_per_pixel = sample_locations_per_pixel;
            self
        }

        #[inline]
        pub fn sample_location_grid_size(mut self, sample_location_grid_size: Extent2D) -> Self {
            self.sample_location_grid_size = sample_location_grid_size;
            self
        }

        #[inline]
        pub fn sample_locations(mut self, sample_locations: &'a [SampleLocationEXT]) -> Self {
            self.sample_locations_count = sample_locations.len().try_into().unwrap();
            self.p_sample_locations = sample_locations.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAttachmentSampleLocationsEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AttachmentSampleLocationsEXT<'a> {
        pub attachment_index: u32,
        pub sample_locations_info: SampleLocationsInfoEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AttachmentSampleLocationsEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AttachmentSampleLocationsEXT")
                .field("attachment_index", &self.attachment_index)
                .field("sample_locations_info", &self.sample_locations_info)
                .finish()
        }
    }

    impl Default for AttachmentSampleLocationsEXT<'_> {
        fn default() -> Self {
            Self {
                attachment_index: Default::default(),
                sample_locations_info: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AttachmentSampleLocationsEXT<'a> {
        #[inline]
        pub fn attachment_index(mut self, attachment_index: u32) -> Self {
            self.attachment_index = attachment_index;
            self
        }

        #[inline]
        pub fn sample_locations_info(
            mut self,
            sample_locations_info: SampleLocationsInfoEXT<'a>,
        ) -> Self {
            self.sample_locations_info = sample_locations_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubpassSampleLocationsEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SubpassSampleLocationsEXT<'a> {
        pub subpass_index: u32,
        pub sample_locations_info: SampleLocationsInfoEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SubpassSampleLocationsEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SubpassSampleLocationsEXT")
                .field("subpass_index", &self.subpass_index)
                .field("sample_locations_info", &self.sample_locations_info)
                .finish()
        }
    }

    impl Default for SubpassSampleLocationsEXT<'_> {
        fn default() -> Self {
            Self {
                subpass_index: Default::default(),
                sample_locations_info: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SubpassSampleLocationsEXT<'a> {
        #[inline]
        pub fn subpass_index(mut self, subpass_index: u32) -> Self {
            self.subpass_index = subpass_index;
            self
        }

        #[inline]
        pub fn sample_locations_info(
            mut self,
            sample_locations_info: SampleLocationsInfoEXT<'a>,
        ) -> Self {
            self.sample_locations_info = sample_locations_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassSampleLocationsBeginInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RenderPassSampleLocationsBeginInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub attachment_initial_sample_locations_count: u32,
        pub p_attachment_initial_sample_locations: *const AttachmentSampleLocationsEXT<'a>,
        pub post_subpass_sample_locations_count: u32,
        pub p_post_subpass_sample_locations: *const SubpassSampleLocationsEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderPassSampleLocationsBeginInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderPassSampleLocationsBeginInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "attachment_initial_sample_locations_count",
                    &self.attachment_initial_sample_locations_count,
                )
                .field(
                    "p_attachment_initial_sample_locations",
                    &self.p_attachment_initial_sample_locations,
                )
                .field(
                    "post_subpass_sample_locations_count",
                    &self.post_subpass_sample_locations_count,
                )
                .field(
                    "p_post_subpass_sample_locations",
                    &self.p_post_subpass_sample_locations,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderPassSampleLocationsBeginInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT;
    }

    unsafe impl Extends<RenderPassBeginInfo<'_>> for RenderPassSampleLocationsBeginInfoEXT<'_> {}

    impl Default for RenderPassSampleLocationsBeginInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                attachment_initial_sample_locations_count: Default::default(),
                p_attachment_initial_sample_locations: ptr::null(),
                post_subpass_sample_locations_count: Default::default(),
                p_post_subpass_sample_locations: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RenderPassSampleLocationsBeginInfoEXT<'a> {
        #[inline]
        pub fn attachment_initial_sample_locations(
            mut self,
            attachment_initial_sample_locations: &'a [AttachmentSampleLocationsEXT<'_>],
        ) -> Self {
            self.attachment_initial_sample_locations_count = attachment_initial_sample_locations
                .len()
                .try_into()
                .unwrap();
            self.p_attachment_initial_sample_locations =
                attachment_initial_sample_locations.as_ptr() as _;
            self
        }

        #[inline]
        pub fn post_subpass_sample_locations(
            mut self,
            post_subpass_sample_locations: &'a [SubpassSampleLocationsEXT<'_>],
        ) -> Self {
            self.post_subpass_sample_locations_count =
                post_subpass_sample_locations.len().try_into().unwrap();
            self.p_post_subpass_sample_locations = post_subpass_sample_locations.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineSampleLocationsStateCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineSampleLocationsStateCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub sample_locations_enable: Bool32,
        pub sample_locations_info: SampleLocationsInfoEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineSampleLocationsStateCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineSampleLocationsStateCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("sample_locations_enable", &self.sample_locations_enable)
                .field("sample_locations_info", &self.sample_locations_info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineSampleLocationsStateCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT;
    }

    unsafe impl Extends<PipelineMultisampleStateCreateInfo<'_>>
        for PipelineSampleLocationsStateCreateInfoEXT<'_>
    {
    }

    impl Default for PipelineSampleLocationsStateCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                sample_locations_enable: Default::default(),
                sample_locations_info: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineSampleLocationsStateCreateInfoEXT<'a> {
        #[inline]
        pub fn sample_locations_enable(mut self, sample_locations_enable: bool) -> Self {
            self.sample_locations_enable = sample_locations_enable.into();
            self
        }

        #[inline]
        pub fn sample_locations_info(
            mut self,
            sample_locations_info: SampleLocationsInfoEXT<'a>,
        ) -> Self {
            self.sample_locations_info = sample_locations_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSampleLocationsPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceSampleLocationsPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub sample_location_sample_counts: SampleCountFlags,
        pub max_sample_location_grid_size: Extent2D,
        pub sample_location_coordinate_range: [f32; 2],
        pub sample_location_sub_pixel_bits: u32,
        pub variable_sample_locations: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceSampleLocationsPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceSampleLocationsPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "sample_location_sample_counts",
                    &self.sample_location_sample_counts,
                )
                .field(
                    "max_sample_location_grid_size",
                    &self.max_sample_location_grid_size,
                )
                .field(
                    "sample_location_coordinate_range",
                    &self.sample_location_coordinate_range,
                )
                .field(
                    "sample_location_sub_pixel_bits",
                    &self.sample_location_sub_pixel_bits,
                )
                .field("variable_sample_locations", &self.variable_sample_locations)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSampleLocationsPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceSampleLocationsPropertiesEXT<'_>
    {
    }

    impl Default for PhysicalDeviceSampleLocationsPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                sample_location_sample_counts: Default::default(),
                max_sample_location_grid_size: Default::default(),
                sample_location_coordinate_range: [Default::default(); _],
                sample_location_sub_pixel_bits: Default::default(),
                variable_sample_locations: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceSampleLocationsPropertiesEXT<'a> {
        #[inline]
        pub fn sample_location_sample_counts(
            mut self,
            sample_location_sample_counts: SampleCountFlags,
        ) -> Self {
            self.sample_location_sample_counts = sample_location_sample_counts;
            self
        }

        #[inline]
        pub fn max_sample_location_grid_size(
            mut self,
            max_sample_location_grid_size: Extent2D,
        ) -> Self {
            self.max_sample_location_grid_size = max_sample_location_grid_size;
            self
        }

        #[inline]
        pub fn sample_location_coordinate_range(
            mut self,
            sample_location_coordinate_range: [f32; 2],
        ) -> Self {
            self.sample_location_coordinate_range = sample_location_coordinate_range;
            self
        }

        #[inline]
        pub fn sample_location_sub_pixel_bits(
            mut self,
            sample_location_sub_pixel_bits: u32,
        ) -> Self {
            self.sample_location_sub_pixel_bits = sample_location_sub_pixel_bits;
            self
        }

        #[inline]
        pub fn variable_sample_locations(mut self, variable_sample_locations: bool) -> Self {
            self.variable_sample_locations = variable_sample_locations.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMultisamplePropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MultisamplePropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_sample_location_grid_size: Extent2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MultisamplePropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MultisamplePropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_sample_location_grid_size",
                    &self.max_sample_location_grid_size,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MultisamplePropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MULTISAMPLE_PROPERTIES_EXT;
    }

    impl Default for MultisamplePropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_sample_location_grid_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MultisamplePropertiesEXT<'a> {
        #[inline]
        pub fn max_sample_location_grid_size(
            mut self,
            max_sample_location_grid_size: Extent2D,
        ) -> Self {
            self.max_sample_location_grid_size = max_sample_location_grid_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetSampleLocationsEXT.html>
    pub type PFN_vkCmdSetSampleLocationsEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_sample_locations_info: *const SampleLocationsInfoEXT<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html>
    pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        samples: SampleCountFlagBits,
        p_multisample_properties: *mut MultisamplePropertiesEXT<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkSampleLocationEXT = SampleLocationEXT;
    pub type VkSampleLocationsInfoEXT = SampleLocationsInfoEXT<'static>;
    pub type VkAttachmentSampleLocationsEXT = AttachmentSampleLocationsEXT<'static>;
    pub type VkSubpassSampleLocationsEXT = SubpassSampleLocationsEXT<'static>;
    pub type VkRenderPassSampleLocationsBeginInfoEXT =
        RenderPassSampleLocationsBeginInfoEXT<'static>;
    pub type VkPipelineSampleLocationsStateCreateInfoEXT =
        PipelineSampleLocationsStateCreateInfoEXT<'static>;
    pub type VkPhysicalDeviceSampleLocationsPropertiesEXT =
        PhysicalDeviceSampleLocationsPropertiesEXT<'static>;
    pub type VkMultisamplePropertiesEXT = MultisamplePropertiesEXT<'static>;
    impl SampleLocationsInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSampleLocationsInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl AttachmentSampleLocationsEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkAttachmentSampleLocationsEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SubpassSampleLocationsEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSubpassSampleLocationsEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl RenderPassSampleLocationsBeginInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkRenderPassSampleLocationsBeginInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PipelineSampleLocationsStateCreateInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineSampleLocationsStateCreateInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceSampleLocationsPropertiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceSampleLocationsPropertiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MultisamplePropertiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMultisamplePropertiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    get_physical_device_multisample_properties_ext: PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_multisample_properties_ext: transmute(
                    load(c"vkGetPhysicalDeviceMultisamplePropertiesEXT")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html>
    #[inline]
    pub unsafe fn get_physical_device_multisample_properties_ext(
        &self,
        physical_device: PhysicalDevice,
        samples: SampleCountFlagBits,
        multisample_properties: &mut MultisamplePropertiesEXT<'_>,
    ) {
        unsafe {
            (self.get_physical_device_multisample_properties_ext)(
                physical_device,
                samples,
                multisample_properties,
            )
        }
    }
}

pub struct DeviceFn {
    cmd_set_sample_locations_ext: PFN_vkCmdSetSampleLocationsEXT,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_sample_locations_ext: transmute(
                    load(c"vkCmdSetSampleLocationsEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetSampleLocationsEXT.html>
    #[inline]
    pub unsafe fn cmd_set_sample_locations_ext(
        &self,
        command_buffer: CommandBuffer,
        sample_locations_info: &SampleLocationsInfoEXT<'_>,
    ) {
        unsafe { (self.cmd_set_sample_locations_ext)(command_buffer, sample_locations_info) }
    }
}

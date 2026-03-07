//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_subpass_merge_feedback.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_subpass_merge_feedback";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassCreationControlEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RenderPassCreationControlEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub disallow_merging: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderPassCreationControlEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderPassCreationControlEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("disallow_merging", &self.disallow_merging)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderPassCreationControlEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDER_PASS_CREATION_CONTROL_EXT;
    }

    unsafe impl<'a> Extends<RenderPassCreateInfo2<'a>> for RenderPassCreationControlEXT<'a> {}
    unsafe impl<'a> Extends<SubpassDescription2<'a>> for RenderPassCreationControlEXT<'a> {}

    impl Default for RenderPassCreationControlEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                disallow_merging: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RenderPassCreationControlEXT<'a> {
        #[inline]
        pub fn disallow_merging(mut self, disallow_merging: bool) -> Self {
            self.disallow_merging = disallow_merging.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassCreationFeedbackInfoEXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct RenderPassCreationFeedbackInfoEXT {
        pub post_merge_subpass_count: u32,
    }

    impl RenderPassCreationFeedbackInfoEXT {
        #[inline]
        pub fn post_merge_subpass_count(mut self, post_merge_subpass_count: u32) -> Self {
            self.post_merge_subpass_count = post_merge_subpass_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassCreationFeedbackCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RenderPassCreationFeedbackCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_render_pass_feedback: *mut RenderPassCreationFeedbackInfoEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderPassCreationFeedbackCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderPassCreationFeedbackCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_render_pass_feedback", &self.p_render_pass_feedback)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderPassCreationFeedbackCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<RenderPassCreateInfo2<'a>> for RenderPassCreationFeedbackCreateInfoEXT<'a> {}

    impl Default for RenderPassCreationFeedbackCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_render_pass_feedback: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RenderPassCreationFeedbackCreateInfoEXT<'a> {
        #[inline]
        pub fn render_pass_feedback(
            mut self,
            render_pass_feedback: &'a mut RenderPassCreationFeedbackInfoEXT,
        ) -> Self {
            self.p_render_pass_feedback = render_pass_feedback;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassSubpassFeedbackInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RenderPassSubpassFeedbackInfoEXT {
        pub subpass_merge_status: SubpassMergeStatusEXT,
        pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
        pub post_merge_index: u32,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderPassSubpassFeedbackInfoEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderPassSubpassFeedbackInfoEXT")
                .field("subpass_merge_status", &self.subpass_merge_status)
                .field(
                    "description",
                    &wrap_c_str_slice_until_nul(&self.description),
                )
                .field("post_merge_index", &self.post_merge_index)
                .finish()
        }
    }

    impl Default for RenderPassSubpassFeedbackInfoEXT {
        fn default() -> Self {
            Self {
                subpass_merge_status: Default::default(),
                description: [Default::default(); _],
                post_merge_index: Default::default(),
            }
        }
    }

    impl RenderPassSubpassFeedbackInfoEXT {
        #[inline]
        pub fn subpass_merge_status(mut self, subpass_merge_status: SubpassMergeStatusEXT) -> Self {
            self.subpass_merge_status = subpass_merge_status;
            self
        }

        #[inline]
        pub fn description(
            mut self,
            description: &CStr,
        ) -> core::result::Result<Self, CStrTooLargeForStaticArray> {
            write_c_str_slice_with_nul(&mut self.description, description)?;
            Ok(self)
        }

        #[inline]
        pub fn post_merge_index(mut self, post_merge_index: u32) -> Self {
            self.post_merge_index = post_merge_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassSubpassFeedbackCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RenderPassSubpassFeedbackCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_subpass_feedback: *mut RenderPassSubpassFeedbackInfoEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderPassSubpassFeedbackCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderPassSubpassFeedbackCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_subpass_feedback", &self.p_subpass_feedback)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderPassSubpassFeedbackCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<SubpassDescription2<'a>> for RenderPassSubpassFeedbackCreateInfoEXT<'a> {}

    impl Default for RenderPassSubpassFeedbackCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_subpass_feedback: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RenderPassSubpassFeedbackCreateInfoEXT<'a> {
        #[inline]
        pub fn subpass_feedback(
            mut self,
            subpass_feedback: &'a mut RenderPassSubpassFeedbackInfoEXT,
        ) -> Self {
            self.p_subpass_feedback = subpass_feedback;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub subpass_merge_feedback: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceSubpassMergeFeedbackFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("subpass_merge_feedback", &self.subpass_merge_feedback)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                subpass_merge_feedback: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'a> {
        #[inline]
        pub fn subpass_merge_feedback(mut self, subpass_merge_feedback: bool) -> Self {
            self.subpass_merge_feedback = subpass_merge_feedback.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubpassMergeStatusEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SubpassMergeStatusEXT(i32);

    impl SubpassMergeStatusEXT {
        pub const MERGED_EXT: Self = Self(0);
        pub const DISALLOWED_EXT: Self = Self(1);
        pub const NOT_MERGED_SIDE_EFFECTS_EXT: Self = Self(2);
        pub const NOT_MERGED_SAMPLES_MISMATCH_EXT: Self = Self(3);
        pub const NOT_MERGED_VIEWS_MISMATCH_EXT: Self = Self(4);
        pub const NOT_MERGED_ALIASING_EXT: Self = Self(5);
        pub const NOT_MERGED_DEPENDENCIES_EXT: Self = Self(6);
        pub const NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT: Self = Self(7);
        pub const NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT: Self = Self(8);
        pub const NOT_MERGED_INSUFFICIENT_STORAGE_EXT: Self = Self(9);
        pub const NOT_MERGED_DEPTH_STENCIL_COUNT_EXT: Self = Self(10);
        pub const NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT: Self = Self(11);
        pub const NOT_MERGED_SINGLE_SUBPASS_EXT: Self = Self(12);
        pub const NOT_MERGED_UNSPECIFIED_EXT: Self = Self(13);
    }

    impl fmt::Debug for SubpassMergeStatusEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::MERGED_EXT => Some("MERGED_EXT"),
                Self::DISALLOWED_EXT => Some("DISALLOWED_EXT"),
                Self::NOT_MERGED_SIDE_EFFECTS_EXT => Some("NOT_MERGED_SIDE_EFFECTS_EXT"),
                Self::NOT_MERGED_SAMPLES_MISMATCH_EXT => Some("NOT_MERGED_SAMPLES_MISMATCH_EXT"),
                Self::NOT_MERGED_VIEWS_MISMATCH_EXT => Some("NOT_MERGED_VIEWS_MISMATCH_EXT"),
                Self::NOT_MERGED_ALIASING_EXT => Some("NOT_MERGED_ALIASING_EXT"),
                Self::NOT_MERGED_DEPENDENCIES_EXT => Some("NOT_MERGED_DEPENDENCIES_EXT"),
                Self::NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT => {
                    Some("NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT")
                }
                Self::NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT => {
                    Some("NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT")
                }
                Self::NOT_MERGED_INSUFFICIENT_STORAGE_EXT => {
                    Some("NOT_MERGED_INSUFFICIENT_STORAGE_EXT")
                }
                Self::NOT_MERGED_DEPTH_STENCIL_COUNT_EXT => {
                    Some("NOT_MERGED_DEPTH_STENCIL_COUNT_EXT")
                }
                Self::NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT => {
                    Some("NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT")
                }
                Self::NOT_MERGED_SINGLE_SUBPASS_EXT => Some("NOT_MERGED_SINGLE_SUBPASS_EXT"),
                Self::NOT_MERGED_UNSPECIFIED_EXT => Some("NOT_MERGED_UNSPECIFIED_EXT"),
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

//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_AMD_pipeline_compiler_control.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_AMD_pipeline_compiler_control";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCompilerControlCreateInfoAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineCompilerControlCreateInfoAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub compiler_control_flags: PipelineCompilerControlFlagsAMD,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineCompilerControlCreateInfoAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineCompilerControlCreateInfoAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("compiler_control_flags", &self.compiler_control_flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineCompilerControlCreateInfoAMD<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD;
    }

    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>>
        for PipelineCompilerControlCreateInfoAMD<'a>
    {
    }
    unsafe impl<'a> Extends<ComputePipelineCreateInfo<'a>>
        for PipelineCompilerControlCreateInfoAMD<'a>
    {
    }
    #[cfg(feature = "provisional")]
    unsafe impl<'a> Extends<ExecutionGraphPipelineCreateInfoAMDX<'a>>
        for PipelineCompilerControlCreateInfoAMD<'a>
    {
    }

    impl Default for PipelineCompilerControlCreateInfoAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                compiler_control_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineCompilerControlCreateInfoAMD<'a> {
        #[inline]
        pub fn compiler_control_flags(
            mut self,
            compiler_control_flags: PipelineCompilerControlFlagsAMD,
        ) -> Self {
            self.compiler_control_flags = compiler_control_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCompilerControlFlagsAMD.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineCompilerControlFlagsAMD(Flags);
    vk_bitflags_wrapped!(PipelineCompilerControlFlagsAMD, Flags);

    impl PipelineCompilerControlFlagsAMD {}

    impl fmt::Debug for PipelineCompilerControlFlagsAMD {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineCompilerControlFlagBitsAMD.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PipelineCompilerControlFlagBitsAMD(u32);

    impl PipelineCompilerControlFlagBitsAMD {}

    impl fmt::Debug for PipelineCompilerControlFlagBitsAMD {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
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

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPipelineCompilerControlCreateInfoAMD = PipelineCompilerControlCreateInfoAMD<'static>;
    pub type VkPipelineCompilerControlFlagsAMD = PipelineCompilerControlFlagsAMD;
    pub type VkPipelineCompilerControlFlagBitsAMD = PipelineCompilerControlFlagBitsAMD;
    impl PipelineCompilerControlCreateInfoAMD<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPipelineCompilerControlCreateInfoAMD {
            unsafe { core::mem::transmute(self) }
        }
    }
}

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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineCompilerControlCreateInfoAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub compiler_control_flags: PipelineCompilerControlFlagsAMD,
        pub _marker: PhantomData<&'a ()>,
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
    unsafe impl<'a> Extends<ExecutionGraphPipelineCreateInfoAMDX<'a>>
        for PipelineCompilerControlCreateInfoAMD<'a>
    {
    }
    impl Default for PipelineCompilerControlCreateInfoAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                compiler_control_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineCompilerControlCreateInfoAMD<'a> {
        pub fn compiler_control_flags(
            mut self,
            compiler_control_flags: PipelineCompilerControlFlagsAMD,
        ) -> Self {
            self.compiler_control_flags = compiler_control_flags;
            self
        }
    }
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PipelineCompilerControlFlagBitsAMD(u32);
    impl PipelineCompilerControlFlagBitsAMD {}
}

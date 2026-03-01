#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct ShaderResourceUsageAMD {
        pub num_used_vgprs: u32,
        pub num_used_sgprs: u32,
        pub lds_size_per_local_work_group: u32,
        pub lds_usage_size_in_bytes: usize,
        pub scratch_mem_usage_in_bytes: usize,
    }
    impl ShaderResourceUsageAMD {
        pub fn num_used_vgprs(mut self, num_used_vgprs: u32) -> Self {
            self.num_used_vgprs = num_used_vgprs;
            self
        }
        pub fn num_used_sgprs(mut self, num_used_sgprs: u32) -> Self {
            self.num_used_sgprs = num_used_sgprs;
            self
        }
        pub fn lds_size_per_local_work_group(mut self, lds_size_per_local_work_group: u32) -> Self {
            self.lds_size_per_local_work_group = lds_size_per_local_work_group;
            self
        }
        pub fn lds_usage_size_in_bytes(mut self, lds_usage_size_in_bytes: usize) -> Self {
            self.lds_usage_size_in_bytes = lds_usage_size_in_bytes;
            self
        }
        pub fn scratch_mem_usage_in_bytes(mut self, scratch_mem_usage_in_bytes: usize) -> Self {
            self.scratch_mem_usage_in_bytes = scratch_mem_usage_in_bytes;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ShaderStatisticsInfoAMD {
        pub shader_stage_mask: ShaderStageFlags,
        pub resource_usage: ShaderResourceUsageAMD,
        pub num_physical_vgprs: u32,
        pub num_physical_sgprs: u32,
        pub num_available_vgprs: u32,
        pub num_available_sgprs: u32,
        pub compute_work_group_size: [u32; 3],
    }
    impl Default for ShaderStatisticsInfoAMD {
        fn default() -> Self {
            Self {
                shader_stage_mask: Default::default(),
                resource_usage: Default::default(),
                num_physical_vgprs: Default::default(),
                num_physical_sgprs: Default::default(),
                num_available_vgprs: Default::default(),
                num_available_sgprs: Default::default(),
                compute_work_group_size: [Default::default(); _],
            }
        }
    }
    impl ShaderStatisticsInfoAMD {
        pub fn shader_stage_mask(mut self, shader_stage_mask: ShaderStageFlags) -> Self {
            self.shader_stage_mask = shader_stage_mask;
            self
        }
        pub fn resource_usage(mut self, resource_usage: ShaderResourceUsageAMD) -> Self {
            self.resource_usage = resource_usage;
            self
        }
        pub fn num_physical_vgprs(mut self, num_physical_vgprs: u32) -> Self {
            self.num_physical_vgprs = num_physical_vgprs;
            self
        }
        pub fn num_physical_sgprs(mut self, num_physical_sgprs: u32) -> Self {
            self.num_physical_sgprs = num_physical_sgprs;
            self
        }
        pub fn num_available_vgprs(mut self, num_available_vgprs: u32) -> Self {
            self.num_available_vgprs = num_available_vgprs;
            self
        }
        pub fn num_available_sgprs(mut self, num_available_sgprs: u32) -> Self {
            self.num_available_sgprs = num_available_sgprs;
            self
        }
        pub fn compute_work_group_size(mut self, compute_work_group_size: [u32; 3]) -> Self {
            self.compute_work_group_size = compute_work_group_size;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ShaderInfoTypeAMD(i32);
    impl ShaderInfoTypeAMD {
        pub const STATISTICS_AMD: Self = Self(0);
        pub const BINARY_AMD: Self = Self(1);
        pub const DISASSEMBLY_AMD: Self = Self(2);
    }
    pub type PFN_vkGetShaderInfoAMD = unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlagBits,
        info_type: ShaderInfoTypeAMD,
        p_info_size: *mut usize,
        p_info: *mut c_void,
    ) -> vk::Result;
}
pub struct DeviceFn {
    get_shader_info_amd: PFN_vkGetShaderInfoAMD,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_shader_info_amd: transmute(load(c"vkGetShaderInfoAMD").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_shader_info_amd(
        &self,
        device: Device,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlagBits,
        info_type: ShaderInfoTypeAMD,
        info: impl ExtendUninit<u8>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(info, |info_size, info| {
                let result = (self.get_shader_info_amd)(
                    device,
                    pipeline,
                    shader_stage,
                    info_type,
                    info_size,
                    info as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
}

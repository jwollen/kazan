#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct CuModuleNVX(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct CuFunctionNVX(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CuModuleCreateInfoNVX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub data_size: usize,
    pub p_data: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CuModuleCreateInfoNVX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CU_MODULE_CREATE_INFO_NVX,
            p_next: core::ptr::null(),
            data_size: Default::default(),
            p_data: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> CuModuleCreateInfoNVX<'a> {
    pub fn data(mut self, data: &'a [u8]) -> Self {
        self.data_size = data.len().try_into().unwrap();
        self.p_data = data.as_ptr() as _;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CuModuleTexturingModeCreateInfoNVX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use64bit_texturing: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CuModuleTexturingModeCreateInfoNVX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX,
            p_next: core::ptr::null(),
            use64bit_texturing: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> CuModuleTexturingModeCreateInfoNVX<'a> {
    pub fn use64bit_texturing(mut self, use64bit_texturing: Bool32) -> Self {
        self.use64bit_texturing = use64bit_texturing;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CuFunctionCreateInfoNVX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub module: CuModuleNVX,
    pub p_name: *const c_char,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CuFunctionCreateInfoNVX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CU_FUNCTION_CREATE_INFO_NVX,
            p_next: core::ptr::null(),
            module: Default::default(),
            p_name: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> CuFunctionCreateInfoNVX<'a> {
    pub fn module(mut self, module: CuModuleNVX) -> Self {
        self.module = module;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CuLaunchInfoNVX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub function: CuFunctionNVX,
    pub grid_dim_x: u32,
    pub grid_dim_y: u32,
    pub grid_dim_z: u32,
    pub block_dim_x: u32,
    pub block_dim_y: u32,
    pub block_dim_z: u32,
    pub shared_mem_bytes: u32,
    pub param_count: usize,
    pub p_params: *const *const c_void,
    pub extra_count: usize,
    pub p_extras: *const *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CuLaunchInfoNVX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CU_LAUNCH_INFO_NVX,
            p_next: core::ptr::null(),
            function: Default::default(),
            grid_dim_x: Default::default(),
            grid_dim_y: Default::default(),
            grid_dim_z: Default::default(),
            block_dim_x: Default::default(),
            block_dim_y: Default::default(),
            block_dim_z: Default::default(),
            shared_mem_bytes: Default::default(),
            param_count: Default::default(),
            p_params: core::ptr::null(),
            extra_count: Default::default(),
            p_extras: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> CuLaunchInfoNVX<'a> {
    pub fn function(mut self, function: CuFunctionNVX) -> Self {
        self.function = function;
        self
    }
    pub fn grid_dim_x(mut self, grid_dim_x: u32) -> Self {
        self.grid_dim_x = grid_dim_x;
        self
    }
    pub fn grid_dim_y(mut self, grid_dim_y: u32) -> Self {
        self.grid_dim_y = grid_dim_y;
        self
    }
    pub fn grid_dim_z(mut self, grid_dim_z: u32) -> Self {
        self.grid_dim_z = grid_dim_z;
        self
    }
    pub fn block_dim_x(mut self, block_dim_x: u32) -> Self {
        self.block_dim_x = block_dim_x;
        self
    }
    pub fn block_dim_y(mut self, block_dim_y: u32) -> Self {
        self.block_dim_y = block_dim_y;
        self
    }
    pub fn block_dim_z(mut self, block_dim_z: u32) -> Self {
        self.block_dim_z = block_dim_z;
        self
    }
    pub fn shared_mem_bytes(mut self, shared_mem_bytes: u32) -> Self {
        self.shared_mem_bytes = shared_mem_bytes;
        self
    }
    pub fn params(mut self, params: &'a [&'a c_void]) -> Self {
        self.param_count = params.len().try_into().unwrap();
        self.p_params = params.as_ptr() as _;
        self
    }
    pub fn extras(mut self, extras: &'a [&'a c_void]) -> Self {
        self.extra_count = extras.len().try_into().unwrap();
        self.p_extras = extras.as_ptr() as _;
        self
    }
}
pub type PFN_vkCreateCuModuleNVX = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CuModuleCreateInfoNVX<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_module: *mut CuModuleNVX,
) -> Result;
pub type PFN_vkCreateCuFunctionNVX = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CuFunctionCreateInfoNVX<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_function: *mut CuFunctionNVX,
) -> Result;
pub type PFN_vkDestroyCuModuleNVX = unsafe extern "system" fn(
    device: Device,
    module: CuModuleNVX,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkDestroyCuFunctionNVX = unsafe extern "system" fn(
    device: Device,
    function: CuFunctionNVX,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCmdCuLaunchKernelNVX = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_launch_info: *const CuLaunchInfoNVX<'_>,
);

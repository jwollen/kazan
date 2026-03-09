//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NVX_binary_import.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NVX_binary_import";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    handle_nondispatchable!(
        CuModuleNVX,
        CU_MODULE_NVX,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkCuModuleNVX.html>"
    );
    handle_nondispatchable!(
        CuFunctionNVX,
        CU_FUNCTION_NVX,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkCuFunctionNVX.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCuModuleCreateInfoNVX.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CuModuleCreateInfoNVX<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub data_size: usize,
        pub p_data: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CuModuleCreateInfoNVX<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CuModuleCreateInfoNVX")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("data_size", &self.data_size)
                .field("p_data", &self.p_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CuModuleCreateInfoNVX<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::CU_MODULE_CREATE_INFO_NVX;
    }

    impl Default for CuModuleCreateInfoNVX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                data_size: Default::default(),
                p_data: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CuModuleCreateInfoNVX<'a> {
        #[inline]
        pub fn data(mut self, data: &'a [u8]) -> Self {
            self.data_size = data.len().try_into().unwrap();
            self.p_data = data.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCuModuleTexturingModeCreateInfoNVX.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CuModuleTexturingModeCreateInfoNVX<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub use64bit_texturing: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CuModuleTexturingModeCreateInfoNVX<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CuModuleTexturingModeCreateInfoNVX")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("use64bit_texturing", &self.use64bit_texturing)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CuModuleTexturingModeCreateInfoNVX<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX;
    }

    unsafe impl<'a> Extends<CuModuleCreateInfoNVX<'a>> for CuModuleTexturingModeCreateInfoNVX<'a> {}

    impl Default for CuModuleTexturingModeCreateInfoNVX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                use64bit_texturing: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CuModuleTexturingModeCreateInfoNVX<'a> {
        #[inline]
        pub fn use64bit_texturing(mut self, use64bit_texturing: bool) -> Self {
            self.use64bit_texturing = use64bit_texturing.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCuFunctionCreateInfoNVX.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CuFunctionCreateInfoNVX<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub module: CuModuleNVX,
        pub p_name: *const c_char,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CuFunctionCreateInfoNVX<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CuFunctionCreateInfoNVX")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("module", &self.module)
                .field("p_name", &unsafe { as_c_str(self.p_name) })
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CuFunctionCreateInfoNVX<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::CU_FUNCTION_CREATE_INFO_NVX;
    }

    impl Default for CuFunctionCreateInfoNVX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                module: Default::default(),
                p_name: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CuFunctionCreateInfoNVX<'a> {
        #[inline]
        pub fn module(mut self, module: CuModuleNVX) -> Self {
            self.module = module;
            self
        }

        #[inline]
        pub fn name(mut self, name: &'a CStr) -> Self {
            self.p_name = name.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCuLaunchInfoNVX.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for CuLaunchInfoNVX<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CuLaunchInfoNVX")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("function", &self.function)
                .field("grid_dim_x", &self.grid_dim_x)
                .field("grid_dim_y", &self.grid_dim_y)
                .field("grid_dim_z", &self.grid_dim_z)
                .field("block_dim_x", &self.block_dim_x)
                .field("block_dim_y", &self.block_dim_y)
                .field("block_dim_z", &self.block_dim_z)
                .field("shared_mem_bytes", &self.shared_mem_bytes)
                .field("param_count", &self.param_count)
                .field("p_params", &self.p_params)
                .field("extra_count", &self.extra_count)
                .field("p_extras", &self.p_extras)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CuLaunchInfoNVX<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::CU_LAUNCH_INFO_NVX;
    }

    impl Default for CuLaunchInfoNVX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                function: Default::default(),
                grid_dim_x: Default::default(),
                grid_dim_y: Default::default(),
                grid_dim_z: Default::default(),
                block_dim_x: Default::default(),
                block_dim_y: Default::default(),
                block_dim_z: Default::default(),
                shared_mem_bytes: Default::default(),
                param_count: Default::default(),
                p_params: ptr::null(),
                extra_count: Default::default(),
                p_extras: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CuLaunchInfoNVX<'a> {
        #[inline]
        pub fn function(mut self, function: CuFunctionNVX) -> Self {
            self.function = function;
            self
        }

        #[inline]
        pub fn grid_dim_x(mut self, grid_dim_x: u32) -> Self {
            self.grid_dim_x = grid_dim_x;
            self
        }

        #[inline]
        pub fn grid_dim_y(mut self, grid_dim_y: u32) -> Self {
            self.grid_dim_y = grid_dim_y;
            self
        }

        #[inline]
        pub fn grid_dim_z(mut self, grid_dim_z: u32) -> Self {
            self.grid_dim_z = grid_dim_z;
            self
        }

        #[inline]
        pub fn block_dim_x(mut self, block_dim_x: u32) -> Self {
            self.block_dim_x = block_dim_x;
            self
        }

        #[inline]
        pub fn block_dim_y(mut self, block_dim_y: u32) -> Self {
            self.block_dim_y = block_dim_y;
            self
        }

        #[inline]
        pub fn block_dim_z(mut self, block_dim_z: u32) -> Self {
            self.block_dim_z = block_dim_z;
            self
        }

        #[inline]
        pub fn shared_mem_bytes(mut self, shared_mem_bytes: u32) -> Self {
            self.shared_mem_bytes = shared_mem_bytes;
            self
        }

        #[inline]
        pub fn params(mut self, params: &'a [u8]) -> Self {
            self.param_count = params.len().try_into().unwrap();
            self.p_params = params.as_ptr() as _;
            self
        }

        #[inline]
        pub fn extras(mut self, extras: &'a [u8]) -> Self {
            self.extra_count = extras.len().try_into().unwrap();
            self.p_extras = extras.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateCuModuleNVX.html>
    pub type PFN_vkCreateCuModuleNVX = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const CuModuleCreateInfoNVX<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_module: *mut CuModuleNVX,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateCuFunctionNVX.html>
    pub type PFN_vkCreateCuFunctionNVX = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const CuFunctionCreateInfoNVX<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_function: *mut CuFunctionNVX,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyCuModuleNVX.html>
    pub type PFN_vkDestroyCuModuleNVX = unsafe extern "system" fn(
        device: Device,
        module: CuModuleNVX,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyCuFunctionNVX.html>
    pub type PFN_vkDestroyCuFunctionNVX = unsafe extern "system" fn(
        device: Device,
        function: CuFunctionNVX,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCuLaunchKernelNVX.html>
    pub type PFN_vkCmdCuLaunchKernelNVX = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_launch_info: *const CuLaunchInfoNVX<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkCuModuleNVX = CuModuleNVX;
    pub type VkCuFunctionNVX = CuFunctionNVX;
    pub type VkCuModuleCreateInfoNVX = CuModuleCreateInfoNVX<'static>;
    pub type VkCuModuleTexturingModeCreateInfoNVX = CuModuleTexturingModeCreateInfoNVX<'static>;
    pub type VkCuFunctionCreateInfoNVX = CuFunctionCreateInfoNVX<'static>;
    pub type VkCuLaunchInfoNVX = CuLaunchInfoNVX<'static>;
    impl CuModuleCreateInfoNVX<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCuModuleCreateInfoNVX {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CuModuleTexturingModeCreateInfoNVX<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCuModuleTexturingModeCreateInfoNVX {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CuFunctionCreateInfoNVX<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCuFunctionCreateInfoNVX {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CuLaunchInfoNVX<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCuLaunchInfoNVX {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    create_cu_module_nvx: PFN_vkCreateCuModuleNVX,
    create_cu_function_nvx: PFN_vkCreateCuFunctionNVX,
    destroy_cu_module_nvx: PFN_vkDestroyCuModuleNVX,
    destroy_cu_function_nvx: PFN_vkDestroyCuFunctionNVX,
    cmd_cu_launch_kernel_nvx: PFN_vkCmdCuLaunchKernelNVX,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_cu_module_nvx: transmute(
                    load(c"vkCreateCuModuleNVX").ok_or(MissingEntryPointError)?,
                ),
                create_cu_function_nvx: transmute(
                    load(c"vkCreateCuFunctionNVX").ok_or(MissingEntryPointError)?,
                ),
                destroy_cu_module_nvx: transmute(
                    load(c"vkDestroyCuModuleNVX").ok_or(MissingEntryPointError)?,
                ),
                destroy_cu_function_nvx: transmute(
                    load(c"vkDestroyCuFunctionNVX").ok_or(MissingEntryPointError)?,
                ),
                cmd_cu_launch_kernel_nvx: transmute(
                    load(c"vkCmdCuLaunchKernelNVX").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateCuModuleNVX.html>
    #[inline]
    pub unsafe fn create_cu_module_nvx(
        &self,
        device: Device,
        create_info: &CuModuleCreateInfoNVX<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<CuModuleNVX> {
        unsafe {
            let mut module = core::mem::MaybeUninit::uninit();
            let result = (self.create_cu_module_nvx)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                module.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(module.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateCuFunctionNVX.html>
    #[inline]
    pub unsafe fn create_cu_function_nvx(
        &self,
        device: Device,
        create_info: &CuFunctionCreateInfoNVX<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<CuFunctionNVX> {
        unsafe {
            let mut function = core::mem::MaybeUninit::uninit();
            let result = (self.create_cu_function_nvx)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                function.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(function.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyCuModuleNVX.html>
    #[inline]
    pub unsafe fn destroy_cu_module_nvx(
        &self,
        device: Device,
        module: CuModuleNVX,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_cu_module_nvx)(device, module, allocator.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyCuFunctionNVX.html>
    #[inline]
    pub unsafe fn destroy_cu_function_nvx(
        &self,
        device: Device,
        function: CuFunctionNVX,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_cu_function_nvx)(device, function, allocator.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCuLaunchKernelNVX.html>
    #[inline]
    pub unsafe fn cmd_cu_launch_kernel_nvx(
        &self,
        command_buffer: CommandBuffer,
        launch_info: &CuLaunchInfoNVX<'_>,
    ) {
        unsafe { (self.cmd_cu_launch_kernel_nvx)(command_buffer, launch_info) }
    }
}

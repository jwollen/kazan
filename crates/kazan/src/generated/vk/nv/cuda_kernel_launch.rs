#![cfg(feature = "provisional")]
//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_cuda_kernel_launch.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_cuda_kernel_launch";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    handle_nondispatchable!(
        CudaModuleNV,
        CUDA_MODULE_NV,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkCudaModuleNV.html>"
    );
    handle_nondispatchable!(
        CudaFunctionNV,
        CUDA_FUNCTION_NV,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkCudaFunctionNV.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCudaModuleCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CudaModuleCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub data_size: usize,
        pub p_data: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CudaModuleCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CudaModuleCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("data_size", &self.data_size)
                .field("p_data", &self.p_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CudaModuleCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::CUDA_MODULE_CREATE_INFO_NV;
    }

    impl Default for CudaModuleCreateInfoNV<'_> {
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

    impl<'a> CudaModuleCreateInfoNV<'a> {
        #[inline]
        pub fn data(mut self, data: &'a [u8]) -> Self {
            self.data_size = data.len().try_into().unwrap();
            self.p_data = data.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCudaFunctionCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CudaFunctionCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub module: CudaModuleNV,
        pub p_name: *const c_char,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CudaFunctionCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CudaFunctionCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("module", &self.module)
                .field("p_name", &unsafe { as_c_str(self.p_name) })
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CudaFunctionCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::CUDA_FUNCTION_CREATE_INFO_NV;
    }

    impl Default for CudaFunctionCreateInfoNV<'_> {
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

    impl<'a> CudaFunctionCreateInfoNV<'a> {
        #[inline]
        pub fn module(mut self, module: CudaModuleNV) -> Self {
            self.module = module;
            self
        }

        #[inline]
        pub fn name(mut self, name: &'a CStr) -> Self {
            self.p_name = name.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCudaLaunchInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CudaLaunchInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub function: CudaFunctionNV,
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
    impl fmt::Debug for CudaLaunchInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CudaLaunchInfoNV")
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

    unsafe impl<'a> TaggedStructure<'a> for CudaLaunchInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::CUDA_LAUNCH_INFO_NV;
    }

    impl Default for CudaLaunchInfoNV<'_> {
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

    impl<'a> CudaLaunchInfoNV<'a> {
        #[inline]
        pub fn function(mut self, function: CudaFunctionNV) -> Self {
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCudaKernelLaunchFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCudaKernelLaunchFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cuda_kernel_launch_features: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCudaKernelLaunchFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCudaKernelLaunchFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "cuda_kernel_launch_features",
                    &self.cuda_kernel_launch_features,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCudaKernelLaunchFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceCudaKernelLaunchFeaturesNV<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceCudaKernelLaunchFeaturesNV<'_> {}

    impl Default for PhysicalDeviceCudaKernelLaunchFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                cuda_kernel_launch_features: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCudaKernelLaunchFeaturesNV<'a> {
        #[inline]
        pub fn cuda_kernel_launch_features(mut self, cuda_kernel_launch_features: bool) -> Self {
            self.cuda_kernel_launch_features = cuda_kernel_launch_features.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCudaKernelLaunchPropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCudaKernelLaunchPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub compute_capability_minor: u32,
        pub compute_capability_major: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCudaKernelLaunchPropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCudaKernelLaunchPropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("compute_capability_minor", &self.compute_capability_minor)
                .field("compute_capability_major", &self.compute_capability_major)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCudaKernelLaunchPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceCudaKernelLaunchPropertiesNV<'_>
    {
    }

    impl Default for PhysicalDeviceCudaKernelLaunchPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                compute_capability_minor: Default::default(),
                compute_capability_major: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCudaKernelLaunchPropertiesNV<'a> {
        #[inline]
        pub fn compute_capability_minor(mut self, compute_capability_minor: u32) -> Self {
            self.compute_capability_minor = compute_capability_minor;
            self
        }

        #[inline]
        pub fn compute_capability_major(mut self, compute_capability_major: u32) -> Self {
            self.compute_capability_major = compute_capability_major;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateCudaModuleNV.html>
    pub type PFN_vkCreateCudaModuleNV = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const CudaModuleCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_module: *mut CudaModuleNV,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetCudaModuleCacheNV.html>
    pub type PFN_vkGetCudaModuleCacheNV = unsafe extern "system" fn(
        device: Device,
        module: CudaModuleNV,
        p_cache_size: *mut usize,
        p_cache_data: *mut c_void,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateCudaFunctionNV.html>
    pub type PFN_vkCreateCudaFunctionNV = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const CudaFunctionCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_function: *mut CudaFunctionNV,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyCudaModuleNV.html>
    pub type PFN_vkDestroyCudaModuleNV = unsafe extern "system" fn(
        device: Device,
        module: CudaModuleNV,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyCudaFunctionNV.html>
    pub type PFN_vkDestroyCudaFunctionNV = unsafe extern "system" fn(
        device: Device,
        function: CudaFunctionNV,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCudaLaunchKernelNV.html>
    pub type PFN_vkCmdCudaLaunchKernelNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_launch_info: *const CudaLaunchInfoNV<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkCudaModuleNV = CudaModuleNV;
    pub type VkCudaFunctionNV = CudaFunctionNV;
    pub type VkCudaModuleCreateInfoNV = CudaModuleCreateInfoNV<'static>;
    pub type VkCudaFunctionCreateInfoNV = CudaFunctionCreateInfoNV<'static>;
    pub type VkCudaLaunchInfoNV = CudaLaunchInfoNV<'static>;
    pub type VkPhysicalDeviceCudaKernelLaunchFeaturesNV =
        PhysicalDeviceCudaKernelLaunchFeaturesNV<'static>;
    pub type VkPhysicalDeviceCudaKernelLaunchPropertiesNV =
        PhysicalDeviceCudaKernelLaunchPropertiesNV<'static>;
    impl CudaModuleCreateInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCudaModuleCreateInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CudaFunctionCreateInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCudaFunctionCreateInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CudaLaunchInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCudaLaunchInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceCudaKernelLaunchFeaturesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceCudaKernelLaunchFeaturesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceCudaKernelLaunchPropertiesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceCudaKernelLaunchPropertiesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    create_cuda_module: PFN_vkCreateCudaModuleNV,
    get_cuda_module_cache: PFN_vkGetCudaModuleCacheNV,
    create_cuda_function: PFN_vkCreateCudaFunctionNV,
    destroy_cuda_module: PFN_vkDestroyCudaModuleNV,
    destroy_cuda_function: PFN_vkDestroyCudaFunctionNV,
    cmd_cuda_launch_kernel: PFN_vkCmdCudaLaunchKernelNV,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_cuda_module: transmute(
                    load(c"vkCreateCudaModuleNV").ok_or(MissingEntryPointError)?,
                ),
                get_cuda_module_cache: transmute(
                    load(c"vkGetCudaModuleCacheNV").ok_or(MissingEntryPointError)?,
                ),
                create_cuda_function: transmute(
                    load(c"vkCreateCudaFunctionNV").ok_or(MissingEntryPointError)?,
                ),
                destroy_cuda_module: transmute(
                    load(c"vkDestroyCudaModuleNV").ok_or(MissingEntryPointError)?,
                ),
                destroy_cuda_function: transmute(
                    load(c"vkDestroyCudaFunctionNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_cuda_launch_kernel: transmute(
                    load(c"vkCmdCudaLaunchKernelNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateCudaModuleNV.html>
    #[inline]
    pub unsafe fn create_cuda_module(
        &self,
        device: Device,
        create_info: &CudaModuleCreateInfoNV<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<CudaModuleNV> {
        unsafe {
            let mut module = core::mem::MaybeUninit::uninit();
            let result = (self.create_cuda_module)(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetCudaModuleCacheNV.html>
    #[inline]
    pub unsafe fn get_cuda_module_cache(
        &self,
        device: Device,
        module: CudaModuleNV,
        mut cache_data: impl ExtendUninit<u8>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |cache_size, cache_data| {
                let result =
                    (self.get_cuda_module_cache)(device, module, cache_size, cache_data as _);

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let cache_data_buf = cache_data.reserve(capacity);
            len = cache_data_buf.len().try_into().unwrap();
            let result = call(&mut len, cache_data_buf.as_mut_ptr() as *mut _)?;
            cache_data.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateCudaFunctionNV.html>
    #[inline]
    pub unsafe fn create_cuda_function(
        &self,
        device: Device,
        create_info: &CudaFunctionCreateInfoNV<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<CudaFunctionNV> {
        unsafe {
            let mut function = core::mem::MaybeUninit::uninit();
            let result = (self.create_cuda_function)(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyCudaModuleNV.html>
    #[inline]
    pub unsafe fn destroy_cuda_module(
        &self,
        device: Device,
        module: CudaModuleNV,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_cuda_module)(device, module, allocator.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyCudaFunctionNV.html>
    #[inline]
    pub unsafe fn destroy_cuda_function(
        &self,
        device: Device,
        function: CudaFunctionNV,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_cuda_function)(device, function, allocator.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCudaLaunchKernelNV.html>
    #[inline]
    pub unsafe fn cmd_cuda_launch_kernel(
        &self,
        command_buffer: CommandBuffer,
        launch_info: &CudaLaunchInfoNV<'_>,
    ) {
        unsafe { (self.cmd_cuda_launch_kernel)(command_buffer, launch_info) }
    }
}

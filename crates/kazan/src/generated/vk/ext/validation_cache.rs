//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_validation_cache.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_validation_cache";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    handle_nondispatchable!(
        ValidationCacheEXT,
        VALIDATION_CACHE_EXT,
        doc =
            "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkValidationCacheEXT.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkValidationCacheCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ValidationCacheCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: ValidationCacheCreateFlagsEXT,
        pub initial_data_size: usize,
        pub p_initial_data: *const c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ValidationCacheCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ValidationCacheCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("initial_data_size", &self.initial_data_size)
                .field("p_initial_data", &self.p_initial_data)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ValidationCacheCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VALIDATION_CACHE_CREATE_INFO_EXT;
    }

    impl Default for ValidationCacheCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                initial_data_size: Default::default(),
                p_initial_data: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ValidationCacheCreateInfoEXT<'a> {
        #[inline]
        pub fn flags(mut self, flags: ValidationCacheCreateFlagsEXT) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn initial_data(mut self, initial_data: &'a [u8]) -> Self {
            self.initial_data_size = initial_data.len().try_into().unwrap();
            self.p_initial_data = initial_data.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkShaderModuleValidationCacheCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ShaderModuleValidationCacheCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub validation_cache: ValidationCacheEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ShaderModuleValidationCacheCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ShaderModuleValidationCacheCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("validation_cache", &self.validation_cache)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ShaderModuleValidationCacheCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT;
    }

    unsafe impl Extends<ShaderModuleCreateInfo<'_>> for ShaderModuleValidationCacheCreateInfoEXT<'_> {}
    unsafe impl Extends<PipelineShaderStageCreateInfo<'_>>
        for ShaderModuleValidationCacheCreateInfoEXT<'_>
    {
    }

    impl Default for ShaderModuleValidationCacheCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                validation_cache: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ShaderModuleValidationCacheCreateInfoEXT<'a> {
        #[inline]
        pub fn validation_cache(mut self, validation_cache: ValidationCacheEXT) -> Self {
            self.validation_cache = validation_cache;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkValidationCacheHeaderVersionEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ValidationCacheHeaderVersionEXT(i32);

    impl ValidationCacheHeaderVersionEXT {
        pub const ONE_EXT: Self = Self(1);
    }

    impl fmt::Debug for ValidationCacheHeaderVersionEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ONE_EXT => Some("ONE_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkValidationCacheCreateFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ValidationCacheCreateFlagsEXT(Flags);
    vk_bitflags_wrapped!(ValidationCacheCreateFlagsEXT, Flags);

    impl fmt::Debug for ValidationCacheCreateFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateValidationCacheEXT.html>
    pub type PFN_vkCreateValidationCacheEXT = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ValidationCacheCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_validation_cache: *mut ValidationCacheEXT,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyValidationCacheEXT.html>
    pub type PFN_vkDestroyValidationCacheEXT = unsafe extern "system" fn(
        device: Device,
        validation_cache: ValidationCacheEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetValidationCacheDataEXT.html>
    pub type PFN_vkGetValidationCacheDataEXT = unsafe extern "system" fn(
        device: Device,
        validation_cache: ValidationCacheEXT,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkMergeValidationCachesEXT.html>
    pub type PFN_vkMergeValidationCachesEXT = unsafe extern "system" fn(
        device: Device,
        dst_cache: ValidationCacheEXT,
        src_cache_count: u32,
        p_src_caches: *const ValidationCacheEXT,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkValidationCacheEXT = ValidationCacheEXT;
    pub type VkValidationCacheCreateInfoEXT = ValidationCacheCreateInfoEXT<'static>;
    pub type VkShaderModuleValidationCacheCreateInfoEXT =
        ShaderModuleValidationCacheCreateInfoEXT<'static>;
    pub type VkValidationCacheHeaderVersionEXT = ValidationCacheHeaderVersionEXT;
    pub type VkValidationCacheCreateFlagsEXT = ValidationCacheCreateFlagsEXT;
    impl ValidationCacheCreateInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkValidationCacheCreateInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ShaderModuleValidationCacheCreateInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkShaderModuleValidationCacheCreateInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    create_validation_cache_ext: PFN_vkCreateValidationCacheEXT,
    destroy_validation_cache_ext: PFN_vkDestroyValidationCacheEXT,
    merge_validation_caches_ext: PFN_vkMergeValidationCachesEXT,
    get_validation_cache_data_ext: PFN_vkGetValidationCacheDataEXT,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_validation_cache_ext: transmute(
                    load(c"vkCreateValidationCacheEXT").ok_or(MissingEntryPointError)?,
                ),
                destroy_validation_cache_ext: transmute(
                    load(c"vkDestroyValidationCacheEXT").ok_or(MissingEntryPointError)?,
                ),
                merge_validation_caches_ext: transmute(
                    load(c"vkMergeValidationCachesEXT").ok_or(MissingEntryPointError)?,
                ),
                get_validation_cache_data_ext: transmute(
                    load(c"vkGetValidationCacheDataEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateValidationCacheEXT.html>
    #[inline]
    pub unsafe fn create_validation_cache_ext(
        &self,
        device: Device,
        create_info: &ValidationCacheCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<ValidationCacheEXT> {
        unsafe {
            let mut validation_cache = core::mem::MaybeUninit::uninit();
            let result = (self.create_validation_cache_ext)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                validation_cache.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(validation_cache.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyValidationCacheEXT.html>
    #[inline]
    pub unsafe fn destroy_validation_cache_ext(
        &self,
        device: Device,
        validation_cache: ValidationCacheEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_validation_cache_ext)(device, validation_cache, allocator.to_raw_ptr())
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkMergeValidationCachesEXT.html>
    #[inline]
    pub unsafe fn merge_validation_caches_ext(
        &self,
        device: Device,
        dst_cache: ValidationCacheEXT,
        src_caches: &[ValidationCacheEXT],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.merge_validation_caches_ext)(
                device,
                dst_cache,
                src_caches.len().try_into().unwrap(),
                src_caches.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetValidationCacheDataEXT.html>
    #[inline]
    pub unsafe fn get_validation_cache_data_ext(
        &self,
        device: Device,
        validation_cache: ValidationCacheEXT,
        mut data: impl ExtendUninit<u8>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |data_size, data| {
                let result = (self.get_validation_cache_data_ext)(
                    device,
                    validation_cache,
                    data_size,
                    data as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let data_buf = data.reserve(capacity);
            len = data_buf.len().try_into().unwrap();
            let result = call(&mut len, data_buf.as_mut_ptr() as *mut _)?;
            data.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}

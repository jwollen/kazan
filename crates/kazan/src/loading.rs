use core::fmt;
use core::{ffi::CStr, mem};

use crate::vk::PFN_vkVoidFunction;

/// Trait for dispatch tables loaded via `vkGetInstanceProcAddr`.
pub trait LoadInstanceFn: Sized {
    /// Load using a raw function pointer loader.
    ///
    /// # Safety
    /// The loader must return valid function pointers for the requested names.
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError>;

    /// Load from an `Entry` and `Instance`.
    ///
    /// # Safety
    /// `instance` must be a valid Vulkan instance.
    unsafe fn load(
        entry: &Entry,
        instance: crate::vk::Instance,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Self::load_with(|name| {
                mem::transmute((entry.static_fn.get_instance_proc_addr)(
                    instance,
                    name.as_ptr(),
                ))
            })
        }
    }
}

/// Trait for dispatch tables loaded via `vkGetDeviceProcAddr`.
pub trait LoadDeviceFn: Sized {
    /// Load using a raw function pointer loader.
    ///
    /// # Safety
    /// The loader must return valid function pointers for the requested names.
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError>;

    /// Load from a `vk1_0::InstanceFn` and `Device`.
    ///
    /// # Safety
    /// `device` must be a valid Vulkan device.
    unsafe fn load(
        instance_fn: &crate::vk::vk1_0::InstanceFn,
        device: crate::vk::Device,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Self::load_with(|name| mem::transmute(instance_fn.get_device_proc_addr(device, name)))
        }
    }
}

#[cfg(feature = "loaded")]
use std::ffi::OsStr;

#[cfg(feature = "loaded")]
use libloading::Library;

pub struct Entry {
    #[cfg(feature = "loaded")]
    _lib_guard: Option<Library>,
    pub static_fn: StaticFn,
    pub vk1_0: crate::vk::vk1_0::EntryFn,
    pub vk1_1: Option<crate::vk::vk1_1::EntryFn>,
}

impl Entry {
    #[cfg(feature = "linked")]
    pub fn linked() -> Result<Self, MissingEntryPointError> {
        // Sound because we're linking to Vulkan, which provides a vkGetInstanceProcAddr that has
        // defined behavior in this use.
        unsafe {
            Self::from_static_fn(StaticFn {
                get_instance_proc_addr: vkGetInstanceProcAddr,
            })
        }
    }

    #[cfg(feature = "loaded")]
    pub unsafe fn load() -> Result<Self, LoadingError> {
        #[cfg(windows)]
        const LIB_PATH: &str = "vulkan-1.dll";

        #[cfg(all(
            unix,
            not(any(
                target_os = "macos",
                target_os = "ios",
                target_os = "android",
                target_os = "fuchsia",
                target_env = "ohos"
            ))
        ))]
        const LIB_PATH: &str = "libvulkan.so.1";

        #[cfg(any(target_os = "android", target_os = "fuchsia", target_env = "ohos"))]
        const LIB_PATH: &str = "libvulkan.so";

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        const LIB_PATH: &str = "libvulkan.dylib";

        unsafe { Self::load_from(LIB_PATH) }
    }

    #[cfg(feature = "loaded")]
    #[cfg_attr(docsrs, doc(cfg(feature = "loaded")))]
    pub unsafe fn load_from(path: impl AsRef<OsStr>) -> Result<Self, LoadingError> {
        let lib = unsafe { Library::new(path.as_ref()).map_err(LoadingError::LibraryLoadFailure)? };

        let static_fn = unsafe {
            StaticFn::load(|name| {
                lib.get(name.to_bytes_with_nul())
                    .map(|symbol| *symbol)
                    .unwrap_or(None)
            })
        }?;

        Ok(Self {
            _lib_guard: Some(lib),
            ..unsafe { Self::from_static_fn(static_fn)? }
        })
    }

    pub unsafe fn from_static_fn(static_fn: StaticFn) -> Result<Self, MissingEntryPointError> {
        let load_fn = move |name: &CStr| unsafe {
            mem::transmute((static_fn.get_instance_proc_addr)(
                crate::vk::Instance::null(),
                name.as_ptr(),
            ))
        };

        let vk1_0 = unsafe { crate::vk::vk1_0::EntryFn::load(load_fn) }?;
        let vk1_1 = unsafe { crate::vk::vk1_1::EntryFn::load(load_fn) }.ok();

        Ok(Self {
            static_fn,
            vk1_0,
            vk1_1,
            #[cfg(feature = "loaded")]
            _lib_guard: None,
        })
    }
}

pub struct StaticFn {
    pub get_instance_proc_addr: crate::vk::PFN_vkGetInstanceProcAddr,
}

impl StaticFn {
    pub unsafe fn load<F>(mut f: F) -> Result<Self, MissingEntryPointError>
    where
        F: FnMut(&CStr) -> Option<crate::vk::PFN_vkVoidFunction>,
    {
        let get_instance_proc_addr = f(c"vkGetInstanceProcAddr").ok_or(MissingEntryPointError)?;

        Ok(Self {
            get_instance_proc_addr: unsafe { mem::transmute(get_instance_proc_addr) },
        })
    }
}

#[derive(Clone, Debug)]
pub struct MissingEntryPointError;
impl fmt::Display for MissingEntryPointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot load `vkGetInstanceProcAddr` symbol from library")
    }
}

impl core::error::Error for MissingEntryPointError {}

#[cfg(feature = "linked")]
unsafe extern "system" {
    fn vkGetInstanceProcAddr(
        instance: crate::vk::Instance,
        name: *const core::ffi::c_char,
    ) -> crate::vk::PFN_vkVoidFunction;
}

#[cfg(feature = "loaded")]
mod loaded {

    use super::*;

    #[derive(Debug)]
    #[cfg_attr(docsrs, doc(cfg(feature = "loaded")))]
    pub enum LoadingError {
        LibraryLoadFailure(libloading::Error),
        MissingEntryPoint(MissingEntryPointError),
    }

    impl fmt::Display for LoadingError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::LibraryLoadFailure(err) => fmt::Display::fmt(err, f),
                Self::MissingEntryPoint(err) => fmt::Display::fmt(err, f),
            }
        }
    }

    #[cfg(feature = "std")]
    impl std::error::Error for LoadingError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(match self {
                Self::LibraryLoadFailure(err) => err,
                Self::MissingEntryPoint(err) => err,
            })
        }
    }

    impl From<MissingEntryPointError> for LoadingError {
        fn from(err: MissingEntryPointError) -> Self {
            Self::MissingEntryPoint(err)
        }
    }
}
#[cfg(feature = "loaded")]
pub use self::loaded::*;

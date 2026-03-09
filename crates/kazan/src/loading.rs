use core::fmt;
use core::{ffi::CStr, mem};

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

#[cfg(all(feature = "loaded", test))]
mod tests {
    use crate::vk;

    use super::*;

    #[derive(Debug)]
    enum TestError {
        Loading(LoadingError),
        Vulkan(crate::vk::Result),
    }
    impl From<LoadingError> for TestError {
        fn from(err: LoadingError) -> Self {
            Self::Loading(err)
        }
    }
    impl From<crate::vk::Result> for TestError {
        fn from(err: crate::vk::Result) -> Self {
            Self::Vulkan(err)
        }
    }

    struct InstanceFn {
        vk1_0: vk::vk1_0::InstanceFn,
    }

    #[test]
    fn test_loaded() -> Result<(), TestError> {
        let entry = unsafe { Entry::load()? };

        let api_version = if let Some(vk1_1) = entry.vk1_1 {
            crate::ApiVersion::from_raw(unsafe { vk1_1.enumerate_instance_version()? })
        } else {
            vk::vk1_0::API_VERSION
        };
        println!("API version: {}", api_version);

        let mut extensions = Vec::new();
        unsafe {
            entry
                .vk1_0
                .enumerate_instance_extension_properties(None, &mut extensions)?
        };
        for extension in extensions {
            println!("Extension: {}", unsafe {
                CStr::from_ptr(extension.extension_name.as_ptr()).to_string_lossy()
            });
        }

        let application_info = vk::ApplicationInfo::default().api_version(api_version);

        let instance_create_info =
            vk::InstanceCreateInfo::default().application_info(&application_info);

        let instance = unsafe { entry.vk1_0.create_instance(&instance_create_info, None)? };
        println!("Instance: {:?}", instance);

        let instance_fn = InstanceFn {
            vk1_0: unsafe {
                vk::vk1_0::InstanceFn::load(|name| {
                    mem::transmute((entry.static_fn.get_instance_proc_addr)(
                        instance,
                        name.as_ptr(),
                    ))
                })
                .map_err(|e| TestError::Loading(LoadingError::MissingEntryPoint(e)))?
            },
        };

        unsafe {
            let mut physical_devices = Vec::new();
            instance_fn
                .vk1_0
                .enumerate_physical_devices(instance, &mut physical_devices)?;

            for physical_device in &physical_devices {
                let mut extension_properties = Vec::new();
                instance_fn.vk1_0.enumerate_device_extension_properties(
                    *physical_device,
                    None,
                    &mut extension_properties,
                )?;
                for extension_property in &extension_properties {
                    println!(
                        "Extension property: {}",
                        CStr::from_ptr(extension_property.extension_name.as_ptr())
                            .to_string_lossy()
                    );
                }
            }
        }

        unsafe { instance_fn.vk1_0.destroy_instance(instance, None) };

        Ok(())
    }
}

pub use crate::generated::vk::*;

mod chain;
pub use chain::*;

mod version;
pub use version::ApiVersion;

use core::{
    ffi::{CStr, c_char},
    fmt,
};

pub trait Handle: Sized + Copy {
    const TYPE: ObjectType;
    fn to_raw(self) -> u64;
    fn from_raw(_: u64) -> Self;

    /// Returns whether the handle is a `NULL` value.
    ///
    /// # Example
    ///
    /// ```
    /// # use kazan::vk::{Handle, Instance};
    /// let instance = Instance::null();
    /// assert!(instance.is_null());
    /// ```
    fn is_null(self) -> bool {
        self.to_raw() == 0
    }
}

/// Helper for Debug-formatting bitflag types. Prints known flags by name,
/// separated by `|`, and appends any remaining unknown bits as hex.
pub fn debug_flags<F: Into<u64> + Copy>(
    f: &mut fmt::Formatter<'_>,
    known: &[(F, &str)],
    value: F,
) -> fmt::Result {
    let mut first = true;
    let mut remaining: u64 = value.into();
    for &(bit, name) in known {
        let bit: u64 = bit.into();
        if bit != 0 && remaining & bit == bit {
            if !first {
                f.write_str(" | ")?;
            }
            f.write_str(name)?;
            first = false;
            remaining &= !bit;
        }
    }
    if remaining != 0 {
        if !first {
            f.write_str(" | ")?;
        }
        write!(f, "{remaining:#x}")?;
    } else if first {
        f.write_str("(empty)")?;
    }
    Ok(())
}

/// Converts a possibly-null `*const c_char` to `Option<&CStr>`.
///
/// # Safety
/// If non-null, the pointer must point to a valid nul-terminated C string.
#[cfg(feature = "debug")]
#[inline]
pub(crate) unsafe fn as_c_str<'a>(ptr: *const c_char) -> Option<&'a CStr> {
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(ptr) })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Result {}

impl fmt::Display for Result {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::SUCCESS => Some("Command completed successfully"),
            Self::NOT_READY => Some("A fence or query has not yet completed"),
            Self::TIMEOUT => Some("A wait operation has not completed in the specified time"),
            Self::EVENT_SET => Some("An event is signaled"),
            Self::EVENT_RESET => Some("An event is unsignaled"),
            Self::INCOMPLETE => Some("A return array was too small for the result"),
            Self::ERROR_OUT_OF_HOST_MEMORY => Some("A host memory allocation has failed"),
            Self::ERROR_OUT_OF_DEVICE_MEMORY => Some("A device memory allocation has failed"),
            Self::ERROR_INITIALIZATION_FAILED => Some("Initialization of an object has failed"),
            Self::ERROR_DEVICE_LOST => Some(
                "The logical device has been lost. See <https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-lost-device>",
            ),
            Self::ERROR_MEMORY_MAP_FAILED => Some("Mapping of a memory object has failed"),
            Self::ERROR_LAYER_NOT_PRESENT => Some("Layer specified does not exist"),
            Self::ERROR_EXTENSION_NOT_PRESENT => Some("Extension specified does not exist"),
            Self::ERROR_FEATURE_NOT_PRESENT => {
                Some("Requested feature is not available on this device")
            }
            Self::ERROR_INCOMPATIBLE_DRIVER => Some("Unable to find a Vulkan driver"),
            Self::ERROR_TOO_MANY_OBJECTS => {
                Some("Too many objects of the type have already been created")
            }
            Self::ERROR_FORMAT_NOT_SUPPORTED => {
                Some("Requested format is not supported on this device")
            }
            Self::ERROR_FRAGMENTED_POOL => Some(
                "A requested pool allocation has failed due to fragmentation of the pool's memory",
            ),
            Self::ERROR_UNKNOWN => {
                Some("An unknown error has occurred, due to an implementation or application bug")
            }
            _ => None,
        };
        if let Some(x) = name {
            fmt.write_str(x)
        } else {
            <Self as fmt::Debug>::fmt(self, fmt)
        }
    }
}

impl From<Extent2D> for Extent3D {
    #[inline]
    fn from(e: Extent2D) -> Self {
        Self {
            width: e.width,
            height: e.height,
            depth: 1,
        }
    }
}

impl From<Extent2D> for Rect2D {
    #[inline]
    fn from(extent: Extent2D) -> Self {
        Self {
            offset: Offset2D { x: 0, y: 0 },
            extent,
        }
    }
}

impl ShaderModuleCreateInfo<'_> {
    /// Sets the shader code from a `&[u32]` slice (SPIR-V words).
    ///
    /// This sets both `code_size` (in bytes) and `p_code`.
    #[inline]
    pub fn code(mut self, code: &[u32]) -> Self {
        self.code_size = code.len() * 4;
        self.p_code = code.as_ptr();
        self
    }
}

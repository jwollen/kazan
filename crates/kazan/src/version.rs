use core::fmt;

/// A packed Vulkan API version number.
///
/// Encodes variant, major, minor, and patch into a single `u32`,
/// matching the layout of `VK_MAKE_API_VERSION`.
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApiVersion(u32);

impl ApiVersion {
    #[inline]
    pub const fn new(variant: u32, major: u32, minor: u32, patch: u32) -> Self {
        Self((variant << 29) | (major << 22) | (minor << 12) | patch)
    }

    #[inline]
    pub const fn variant(self) -> u32 {
        self.0 >> 29
    }

    #[inline]
    pub const fn major(self) -> u32 {
        (self.0 >> 22) & 0x7F
    }

    #[inline]
    pub const fn minor(self) -> u32 {
        (self.0 >> 12) & 0x3FF
    }

    #[inline]
    pub const fn patch(self) -> u32 {
        self.0 & 0xFFF
    }

    #[inline]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    #[inline]
    pub const fn to_raw(self) -> u32 {
        self.0
    }
}

impl fmt::Debug for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}

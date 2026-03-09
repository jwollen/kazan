use core::ffi::CStr;
use core::fmt;

use crate::generated::extensions::{EXTENSION_COUNT, EXTENSIONS, extension_index};

const WORDS: usize = (EXTENSION_COUNT + 63) / 64;

/// Error returned when an unknown extension name is passed to [`ExtensionSet`].
#[derive(Debug, Clone, Copy)]
pub struct UnknownExtensionError;

impl fmt::Display for UnknownExtensionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unknown Vulkan extension")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UnknownExtensionError {}

/// A set of Vulkan extensions, stored as a compact bit array.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ExtensionSet {
    bits: [u64; WORDS],
}

impl ExtensionSet {
    /// Creates an empty `ExtensionSet`.
    pub const fn empty() -> Self {
        Self { bits: [0; WORDS] }
    }

    /// Returns the extension name for a given bit index, or `None` if out of range.
    pub fn name(index: usize) -> Option<&'static CStr> {
        EXTENSIONS.get(index).copied()
    }

    /// Returns the bit index for a given extension name, or `None` if unknown.
    pub fn index(name: &CStr) -> Option<usize> {
        extension_index(name)
    }

    /// Returns `true` if the set contains the extension with the given name.
    pub fn contains(&self, name: &CStr) -> bool {
        Self::index(name).is_some_and(|i| self.bits[i / 64] & (1 << (i % 64)) != 0)
    }

    /// Inserts the extension with the given name. Returns `true` if it was newly inserted.
    pub fn insert(&mut self, name: &CStr) -> Result<bool, UnknownExtensionError> {
        let i = Self::index(name).ok_or_else(|| UnknownExtensionError)?;
        let word = &mut self.bits[i / 64];
        let bit = 1 << (i % 64);
        let was_absent = *word & bit == 0;
        *word |= bit;
        Ok(was_absent)
    }

    /// Removes the extension with the given name. Returns `true` if it was present.
    pub fn remove(&mut self, name: &CStr) -> Result<bool, UnknownExtensionError> {
        let i = Self::index(name).ok_or_else(|| UnknownExtensionError)?;
        let word = &mut self.bits[i / 64];
        let bit = 1 << (i % 64);
        let was_present = *word & bit != 0;
        *word &= !bit;
        Ok(was_present)
    }

    /// Returns the number of extensions in the set.
    pub fn count(&self) -> usize {
        self.bits.iter().map(|w| w.count_ones() as usize).sum()
    }

    /// Returns `true` if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.bits.iter().all(|&w| w == 0)
    }

    /// Returns an iterator over the extension names in the set.
    pub fn names(&self) -> impl Iterator<Item = &'static CStr> + '_ {
        self.bits.iter().enumerate().flat_map(|(word_idx, &word)| {
            (0..64).filter_map(move |bit| {
                if word & (1u64 << bit) != 0 {
                    EXTENSIONS.get(word_idx * 64 + bit).copied()
                } else {
                    None
                }
            })
        })
    }
}

impl fmt::Debug for ExtensionSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.names()).finish()
    }
}

impl<'a> TryFrom<&'a [&'a CStr]> for ExtensionSet {
    type Error = UnknownExtensionError;

    fn try_from(names: &'a [&'a CStr]) -> Result<Self, Self::Error> {
        let mut set = Self::empty();
        for &name in names {
            set.insert(name)?;
        }
        Ok(set)
    }
}

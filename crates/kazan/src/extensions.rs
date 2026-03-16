use core::fmt;

/// Error returned when an unknown extension name is passed to an extension set.
#[derive(Debug, Clone, Copy)]
pub struct UnknownExtensionError;

impl fmt::Display for UnknownExtensionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unknown Vulkan extension")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UnknownExtensionError {}

/// Defines an extension set type with a built-in extension table and index lookup.
///
/// Invoked by generated code with the extension name and a `(identifier, CStr)` pair
/// for each extension.  The index of each extension is derived automatically.
#[macro_export]
macro_rules! define_extension_set {
    ($Name:ident, [$($(#[$meta:meta])* ($id:ident, $ext:expr)),* $(,)?]) => {
        #[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $Name {
            bits: [u64; [$($(#[$meta])* $ext),*].len().div_ceil(64)],
        }

        impl $Name {
            const EXTENSIONS: &[&core::ffi::CStr] = &[$($(#[$meta])* $ext),*];

            /// Creates an empty set.
            pub const fn empty() -> Self {
                Self { bits: [0; Self::EXTENSIONS.len().div_ceil(64)] }
            }

            /// Returns a set containing all known extensions.
            pub const fn all_known() -> Self {
                const {
                    let len = [$($(#[$meta])* $ext),*].len();
                    let full_words = len / 64;
                    let remainder = len % 64;
                    let mut bits = [!0u64; [$($(#[$meta])* $ext),*].len().div_ceil(64)];
                    if remainder != 0 {
                        bits[full_words] = (1u64 << remainder) - 1;
                    }
                    Self { bits }
                }
            }

            /// Returns the extension name for a given bit index, or `None` if out of range.
            pub fn name(index: usize) -> Option<&'static core::ffi::CStr> {
                Self::EXTENSIONS.get(index).copied()
            }

            /// Returns the bit index for a given extension name, or `None` if unknown.
            pub fn index(name: &core::ffi::CStr) -> Option<usize> {
                #[allow(non_camel_case_types)]
                #[repr(usize)]
                enum Idx { $($(#[$meta])* $id),* }

                #[allow(non_upper_case_globals, unreachable_patterns)]
                {
                    $($(#[$meta])* const $id: &[u8] = $ext.to_bytes_with_nul();)*

                    match name.to_bytes_with_nul() {
                        $($(#[$meta])* $id => Some(Idx::$id as usize),)*
                        _ => None,
                    }
                }
            }

            /// Returns `true` if the set contains the extension with the given name.
            pub fn contains(&self, name: &core::ffi::CStr) -> bool {
                Self::index(name).is_some_and(|i| self.bits[i / 64] & (1 << (i % 64)) != 0)
            }

            /// Inserts the extension with the given name. Returns `true` if it was newly inserted.
            pub fn insert(&mut self, name: &core::ffi::CStr) -> core::result::Result<bool, $crate::UnknownExtensionError> {
                let i = Self::index(name).ok_or($crate::UnknownExtensionError)?;
                let word = &mut self.bits[i / 64];
                let bit = 1 << (i % 64);
                let was_absent = *word & bit == 0;
                *word |= bit;
                core::result::Result::Ok(was_absent)
            }

            /// Removes the extension with the given name. Returns `true` if it was present.
            pub fn remove(&mut self, name: &core::ffi::CStr) -> core::result::Result<bool, $crate::UnknownExtensionError> {
                let i = Self::index(name).ok_or($crate::UnknownExtensionError)?;
                let word = &mut self.bits[i / 64];
                let bit = 1 << (i % 64);
                let was_present = *word & bit != 0;
                *word &= !bit;
                core::result::Result::Ok(was_present)
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
            pub fn names(&self) -> impl Iterator<Item = &'static core::ffi::CStr> + '_ {
                self.bits.iter().enumerate().flat_map(|(word_idx, &word)| {
                    (0..64).filter_map(move |bit| {
                        if word & (1u64 << bit) != 0 {
                            Self::EXTENSIONS.get(word_idx * 64 + bit).copied()
                        } else {
                            None
                        }
                    })
                })
            }
        }

        impl core::fmt::Debug for $Name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_set().entries(self.names()).finish()
            }
        }

        impl<'a> core::convert::TryFrom<&'a [&'a core::ffi::CStr]> for $Name {
            type Error = $crate::UnknownExtensionError;

            fn try_from(names: &'a [&'a core::ffi::CStr]) -> core::result::Result<Self, Self::Error> {
                let mut set = Self::empty();
                for &name in names {
                    set.insert(name)?;
                }
                core::result::Result::Ok(set)
            }
        }
    };
}

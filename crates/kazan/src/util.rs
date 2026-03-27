#[cfg(feature = "std")]
use std::io;

/// Decode SPIR-V from bytes.
///
/// This function handles SPIR-V of arbitrary endianness gracefully, and returns correctly aligned
/// storage.
///
/// # Examples
/// ```no_run
/// // Decode SPIR-V from a file
/// let mut file = std::fs::File::open("/path/to/shader.spv").unwrap();
/// let words = kazan::util::read_spv(&mut file).unwrap();
/// ```
/// ```
/// // Decode SPIR-V from memory
/// const SPIRV: &[u8] = &[
///     // ...
/// #   0x03, 0x02, 0x23, 0x07,
/// ];
/// let words = kazan::util::read_spv(&mut std::io::Cursor::new(&SPIRV[..])).unwrap();
/// ```
#[cfg(feature = "std")]
pub fn read_spv<R: io::Read + io::Seek>(x: &mut R) -> io::Result<Vec<u32>> {
    // TODO use stream_len() once it is stabilized and remove the subsequent rewind() call
    let size = x.seek(io::SeekFrom::End(0))?;
    x.rewind()?;
    if !size.is_multiple_of(4) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "input length not divisible by 4",
        ));
    }
    if size > usize::MAX as u64 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "input too long"));
    }
    let words = (size / 4) as usize;
    // https://github.com/kazan-rs/kazan/issues/354:
    // Zero-initialize the result to prevent read_exact from possibly
    // reading uninitialized memory.
    let mut result = vec![0u32; words];
    x.read_exact(unsafe {
        core::slice::from_raw_parts_mut(result.as_mut_ptr().cast::<u8>(), words * 4)
    })?;
    const MAGIC_NUMBER: u32 = 0x0723_0203;
    if !result.is_empty() && result[0] == MAGIC_NUMBER.swap_bytes() {
        for word in &mut result {
            *word = word.swap_bytes();
        }
    }
    if result.is_empty() || result[0] != MAGIC_NUMBER {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "input missing SPIR-V magic number",
        ));
    }
    Ok(result)
}

#[inline]
pub fn copy(src: &[u8], dst: &mut [u8]) {
    debug_assert!(src.len() <= dst.len());
    // dst[..src.len()].copy_from_slice(src);
    for i in 0..src.len() {
        dst[i] = src[i];
    }
}

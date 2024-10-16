#[repr(C)]
pub struct File(nitro_sys::FSFile, bool);

impl core::fmt::Debug for File {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("File").field(&(self as *const _)).finish()
    }
}

impl File {
    #[inline(always)]
    pub fn open(path: &str) -> Self {
        debug_assert!(path.ends_with('\0'), "path {path:?} must end with '\\0'");
        let mut file = Self(nitro_sys::FSFile::default(), false);
        file.init();
        unsafe {
            debug_assert_ne!(
                nitro_sys::FS_OpenFile(&mut file.0, path.as_ptr() as _,),
                0,
                "Failed to open file: {path}"
            );
        }
        file.1 = true;
        file
    }

    #[inline(always)]
    pub fn init(&mut self) {
        unsafe {
            nitro_sys::FS_InitFile(&mut self.0);
        }
    }

    #[inline(always)]
    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        unsafe {
            nitro_sys::FS_ReadFile(&mut self.0, buf.as_mut_ptr() as _, buf.len() as _) as usize
        }
    }

    #[inline(always)]
    pub fn seek(&mut self, pos: isize, seek_mode: nitro_sys::FSSeekFileMode) -> bool {
        unsafe { nitro_sys::FS_SeekFile(&mut self.0, pos as _, seek_mode) != 0 }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        if self.1 {
            unsafe { nitro_sys::FS_CloseFile(&mut self.0) };
            self.1 = false;
        }
    }
}

pub use nitro_sys::FSSeekFileMode;

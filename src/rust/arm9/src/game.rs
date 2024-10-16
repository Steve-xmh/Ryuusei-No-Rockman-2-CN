use core::ffi::c_char;

use crate::font::{get_font_graph_id_by_addr, FontId};

#[repr(transparent)]
pub struct GameCtx(usize);

impl GameCtx {
    pub fn get_script_data(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(*((self.0 + 0x10) as *mut *mut u8), 2) }
    }
    
    pub unsafe fn get_raw_script_data(&self) -> *mut u8 {
        *((self.0 + 0x10) as *mut *mut u8)
    }

    pub fn move_script_data(&mut self, offset: isize) {
        unsafe {
            let script_data = (self.0 + 0x10) as *mut usize;
            *script_data = (*script_data as isize + offset) as usize;
        }
    }

    pub fn set_char_code(&mut self, code: u16) {
        unsafe {
            *((self.0 + 0x44) as *mut u16) = code;
            *((self.0 + 0x6E) as *mut u16) = code;
        }
    }

    pub fn get_font_addr(&self) -> *const u8 {
        unsafe { *((self.0 + 0x24) as *const *const u8) }
    }

    pub fn get_font_id(&self) -> Option<FontId> {
        get_font_graph_id_by_addr(self.get_font_addr() as usize).map(|x| x.0)
    }

    pub fn get_char_code(&self) -> u16 {
        unsafe { *((self.0 + 0x44) as *mut u16) }
    }

    pub fn move_draw_cursor(&mut self, x: i16) {
        unsafe {
            *((self.0 + 0x60) as *mut i16) += x * 4;
        }
    }

    pub fn get_is_font3(&self) -> bool {
        unsafe { *((self.0 + 0x60) as *mut c_char) != 0 }
    }

    pub fn get_render_method(&self) -> u32 {
        unsafe { *((self.0 + 0x128) as *mut u32) }
    }
}

impl From<usize> for GameCtx {
    fn from(ptr: usize) -> Self {
        Self(ptr)
    }
}

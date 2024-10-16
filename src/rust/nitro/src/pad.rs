use nitro_sys::*;

pub fn read() -> u16 {
    unsafe {
        ((*(0x04000130 as *const u16) | *(0x027FFFA8 as *const u16))
            ^ (PAD_PLUS_KEY_MASK | PAD_BUTTON_MASK) as u16)
            & (PAD_PLUS_KEY_MASK | PAD_BUTTON_MASK) as u16
    }
}

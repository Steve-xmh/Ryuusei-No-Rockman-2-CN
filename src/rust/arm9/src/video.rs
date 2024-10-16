pub fn set_brightness(level: i8) {
    const A: *mut u32 = nitro::sys::REG_MASTER_BRIGHT_ADDR as *mut u32;
    const B: *mut u32 = nitro::sys::REG_DB_MASTER_BRIGHT_ADDR as *mut u32;
    let mode = if level < 0 { 2 << 14 } else { 1 << 14 };
    let v = mode | (level.abs().min(16) as u32);
    unsafe {
        *A = v;
        *B = v;
    }
}

pub fn fade_in() {
    for i in (0..=16).rev() {
        set_brightness(i);
        nitro::irq::wait_vblank();
    }
}

pub fn fade_out() {
    for i in 0..=16 {
        set_brightness(i);
        nitro::irq::wait_vblank();
    }
}

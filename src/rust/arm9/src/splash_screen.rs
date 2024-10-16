use nitro::{
    fs::File,
    sys::{
        HW_BG_PLTT, HW_BG_PLTT_SIZE, HW_DB_BG_PLTT, HW_DB_BG_PLTT_SIZE, REG_BG0CNT_ADDR,
        REG_DB_BG0CNT_ADDR, REG_DB_DISPCNT_ADDR, REG_DISPCNT_ADDR, REG_VRAMCNT_A_ADDR,
        REG_VRAMCNT_C_ADDR,
    },
};

pub fn load_splash_screen() {
    let mut file = File::open("splash-screen.bin\0");
    // 读取并设置调色板
    let pltt =
        unsafe { core::slice::from_raw_parts_mut(HW_BG_PLTT as *mut u8, HW_BG_PLTT_SIZE as usize) };
    let db_pltt = unsafe {
        core::slice::from_raw_parts_mut(HW_DB_BG_PLTT as *mut u8, HW_DB_BG_PLTT_SIZE as usize)
    };
    let mut palette = [0u8; 256 * 2];
    file.read(&mut palette);

    nitro::mem::copy(&palette, pltt);
    nitro::mem::copy(&palette, db_pltt);

    // 初始化上下屏显存总线，设置它们的显存映射范围
    unsafe {
        // 设置上屏幕显存范围到 [0x06000000..0x06020000]
        *(REG_VRAMCNT_A_ADDR as *mut u8) = (1 << 7) | 1;
        // 设置下屏幕显存范围到 [0x06200000..0x06220000]
        *(REG_VRAMCNT_C_ADDR as *mut u8) = (1 << 7) | 4;
    }

    unsafe {
        // 1 << 8: 启用 BG0
        // 1 << 16: 设置成 BG 模式
        let v = (1 << 16) | (1 << 8);
        *(REG_DISPCNT_ADDR as *mut u32) = v;
        *(REG_DB_DISPCNT_ADDR as *mut u32) = v;
    }

    unsafe {
        // 1 << 2: 设置图块集初始内存位置为 显存位置+0x4000
        // 1 << 7: 设置成 256 色模式（8bpp）
        let v = (1 << 7) | (1 << 2);
        *(REG_BG0CNT_ADDR as *mut u16) = v;
        *(REG_DB_BG0CNT_ADDR as *mut u16) = v;
    }

    for i in 0..32 * 24 {
        unsafe {
            *((0x06000000 as *mut u16).offset(i as isize)) = i;
        }
    }

    for i in 0..32 * 24 {
        unsafe {
            *((0x06200000 as *mut u16).offset(i as isize)) = i;
        }
    }

    file.read(unsafe {
        core::slice::from_raw_parts_mut((0x06000000 + 0x4000) as _, 8 * 8 * 32 * 24)
    });

    file.read(unsafe {
        core::slice::from_raw_parts_mut((0x06200000 + 0x4000) as _, 8 * 8 * 32 * 24)
    });

    // REG_VRAMCNT_ADDR
}

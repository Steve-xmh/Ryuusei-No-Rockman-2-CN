#![no_std]
#![no_main]
#![allow(clippy::missing_safety_doc)]

mod font;
mod game;
mod script;
mod splash_screen;
mod video;
mod vram_font;

extern crate alloc;

use font::FontLoader;
use nitro::{nogba::nogba_breakpoint, *};
use vram_font::VRamFontLoader;

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InitData {
    pub font1_pos: usize,
    pub font2_pos: usize,
    pub font3_pos: usize,
    pub font3_graph_amount: usize,
}

#[allow(clippy::mut_from_ref)]
impl InitData {
    pub fn zero_font1(&self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut((self.font1_pos + 0x40) as *mut u8, 0x40) }
    }

    pub fn zero_font2(&self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut((self.font2_pos + 0x40) as *mut u8, 0x40) }
    }

    pub fn zero_font3(&self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut((self.font3_pos + 0x80) as *mut u8, 0x80) }
    }

    pub fn font3_graph_amount(&self) -> usize {
        self.font3_graph_amount
    }
}

#[derive(Debug)]
pub struct GlobalData {
    pub init_data: InitData,
    pub font_loader: FontLoader,
    pub vram_font_loader: VRamFontLoader<192>,
}

static mut GLOBAL_DATA: Option<GlobalData> = None;
// NDS 是单核心的，所以不需要特别担心多线程问题
pub fn global_data() -> &'static mut GlobalData {
    unsafe {
        GLOBAL_DATA
            .as_mut()
            .expect("can't get global data, maybe it's been overrided!")
    }
}
fn init_global_data(data: GlobalData) {
    unsafe { GLOBAL_DATA = Some(data) }
}

/// 汇编引导代码将会加载的第一个 Rust 函数
#[no_mangle]
pub unsafe extern "C" fn fontapi_main(init_data: *const InitData) {
    let init_data = init_data.as_ref().unwrap();
    println!("Ryuusei No Rockman 2 CN Patch");
    println!("By SteveXMH written in ASM/Rust");
    init_global_data(GlobalData {
        init_data: init_data.clone(),
        font_loader: FontLoader::default(),
        vram_font_loader: {
            let mut l = VRamFontLoader::default();
            l.reset(0x0600C040);
            l
        },
    });
    println!(
        "Loaded {} font graphs",
        global_data().init_data.font3_graph_amount()
    );
    video::set_brightness(16);
    splash_screen::load_splash_screen();
    video::fade_in();
    while (nitro::pad::read() & 1) == 0 {
        nitro::irq::set_key_irq(true);
        nitro::irq::wait_any_intr();
    }
    video::fade_out();
    println!("Finished loading patch");
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_read_script_font(mut game_ctx: crate::game::GameCtx) {
    let data = game_ctx.get_script_data();
    let (code, is_double_encode) = crate::script::decode_script(data);
    // println!(
    //     "fontapi_read_script_font: {:02x} {:02x} -> {} {}",
    //     data[0], data[1], code, is_double_encode
    // );
    if let Some(font_id) = game_ctx.get_font_id() {
        let graph = global_data().font_loader.get_graph(code, font_id);
        match font_id {
            font::FontId::Font1 => nitro::mem::copy(graph, global_data().init_data.zero_font1()),
            font::FontId::Font2 => nitro::mem::copy(graph, global_data().init_data.zero_font2()),
            font::FontId::Font3 => nitro::mem::copy(graph, global_data().init_data.zero_font3()),
        }
    }
    game_ctx.set_char_code(code);
    game_ctx.move_script_data(if is_double_encode { 2 } else { 1 });
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_read_script_font_1(mut game_ctx: crate::game::GameCtx) {
    let data = game_ctx.get_script_data();
    let (code, is_double_encode) = crate::script::decode_script(data);
    // println!(
    //     "fontapi_read_script_font_1: {:02x} {:02x} -> {}",
    //     data[0], data[1], code
    // );
    let graph = global_data().font_loader.get_graph_font2(code);
    nitro::mem::copy(graph, global_data().init_data.zero_font1());
    game_ctx.set_char_code(code);
    game_ctx.move_script_data(if is_double_encode { 2 } else { 1 });
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_read_script_font_2(mut game_ctx: crate::game::GameCtx) {
    let data = game_ctx.get_script_data();
    let (code, is_double_encode) = crate::script::decode_script(data);
    // println!(
    //     "fontapi_read_script_font_2: {:02x} {:02x} -> {}",
    //     data[0], data[1], code
    // );
    let graph = global_data().font_loader.get_graph_font2(code);
    nitro::mem::copy(graph, global_data().init_data.zero_font2());
    game_ctx.set_char_code(code);
    game_ctx.move_script_data(if is_double_encode { 2 } else { 1 });
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_read_script_font_3(mut game_ctx: crate::game::GameCtx) {
    let data = game_ctx.get_script_data();
    let (code, is_double_encode) = crate::script::decode_script(data);
    // println!(
    //     "fontapi_read_script_font_3: {:02x} {:02x} -> {} {}",
    //     data[0], data[1], code, is_double_encode
    // );
    let graph = global_data().font_loader.get_graph_font3(code);
    nitro::mem::copy(graph, global_data().init_data.zero_font3());
    game_ctx.set_char_code(code);
    game_ctx.move_script_data(if is_double_encode { 2 } else { 1 });
    // game_ctx.move_draw_cursor(
    //     global_data()
    //         .font_loader
    //         .get_graph_font3_width(code) as i16,
    // );
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_move_draw_cursor(mut game_ctx: crate::game::GameCtx) {
    let code = game_ctx.get_char_code();
    // println!(
    //     "fontapi_move_draw_cursor: {} {:08X}",
    //     code,
    //     game_ctx.get_render_method()
    // );
    match game_ctx.get_render_method() {
        0x400 | 0x600 => {
            // println!("Print using fixed 11px");
            game_ctx.move_draw_cursor(11);
            // nogba_breakpoint();
        }
        _ => {
            const BUFFER_START: usize = 0x20F75A8;
            const BUFFER_END: usize = BUFFER_START + 0x24 * 4 - 1;
            match game_ctx.get_raw_script_data() as usize {
                BUFFER_START..=BUFFER_END => {
                    // println!("Print using fixed 11px");
                    game_ctx.move_draw_cursor(11);
                }
                _ => {
                    game_ctx.move_draw_cursor(
                        global_data().font_loader.get_graph_font3_width(code) as i16,
                    );
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_read_script_font_3_fixed_width(
    mut game_ctx: crate::game::GameCtx,
) {
    let data = game_ctx.get_script_data();
    let (code, is_double_encode) = crate::script::decode_script(data);
    // println!(
    //     "fontapi_read_script_font_3_fixed_width: {:02x} {:02x} -> {} {}",
    //     data[0], data[1], code, is_double_encode
    // );
    let graph = global_data().font_loader.get_graph_font3(code);
    nitro::mem::copy(graph, global_data().init_data.zero_font3());
    game_ctx.set_char_code(code);
    game_ctx.move_script_data(if is_double_encode { 2 } else { 1 });
    // game_ctx.move_draw_cursor(11);
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_read_script(mut game_ctx: crate::game::GameCtx) {
    let data = game_ctx.get_script_data();
    let (code, is_double_encode) = crate::script::decode_script(data);
    println!(
        "fontapi_read_script: {:02x} {:02x} -> {}",
        data[0], data[1], code
    );
    game_ctx.set_char_code(code);
    game_ctx.move_script_data(if is_double_encode { 2 } else { 1 });
}

// sub_217E07C
#[no_mangle]
pub unsafe extern "C" fn fontapi_transform_multi_line_script(
    mut dest_script: *mut u8,
    mut src_script: *const u8,
    line_pad_size: usize,
) {
    nitro::sys::MI_CpuFill8(dest_script as _, 0, 0x6C);
    let mut line_cursor = 0;
    loop {
        match *src_script {
            0xE9 => {
                dest_script = dest_script.add(line_pad_size - line_cursor);
                *dest_script = 0xE9;
                src_script = src_script.add(1);
                dest_script = dest_script.add(1);
                line_cursor = 0;
            }
            0xE6 => {
                *dest_script = 0xE6;
                break;
            }
            _ => {
                let width = fontapi_get_code_size(*src_script);
                for _ in 0..width {
                    *dest_script = *src_script;
                    src_script = src_script.add(1);
                    dest_script = dest_script.add(1);
                }
                line_cursor += 1;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_get_code_size(first_code: u8) -> usize {
    // println!("fontapi_get_code_size: {:02x} -> {}", first_code, size);
    match first_code {
        0x00..=0xCF => 1,
        0xD0..=0xE4 => 2,
        _ => 1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_get_script_chars_len_loop(script_data: *const u8) -> usize {
    // println!("fontapi_get_script_chars_len: {:?}", script_data);
    let mut result = 0;

    let mut script_data = script_data;
    loop {
        let (code, is_double_encode) =
            crate::script::decode_script(core::slice::from_raw_parts(script_data, 2));
        // println!("{}, {}", code, is_double_encode);
        if code == u16::MAX {
            break;
        }
        script_data = script_data.add(if is_double_encode { 2 } else { 1 });
        result += 1;
    }

    // println!("fontapi_get_script_chars_len: {}", result);

    result
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_get_encode_size_by_index(
    script_data: *const u8,
    index: usize,
    width_result: *mut u8,
) -> *const u8 {
    let mut script_data = script_data;
    // println!(
    //     "get_encode_size_by_index: {:08X} {} {:08X}",
    //     script_data as usize, index, width_result as usize
    // );
    for _i in 0..index {
        let width = fontapi_get_code_size(*script_data);
        // println!(
        //     "   {} {:02X?} -> {}",
        //     _i,
        //     core::slice::from_raw_parts(script_data, width),
        //     width
        // );
        script_data = script_data.add(width);
    }
    *width_result = fontapi_get_code_size(*script_data) as u8;
    // println!(
    //     "   {} {:02X?} -> {}",
    //     index,
    //     core::slice::from_raw_parts(script_data, *width_result as usize),
    //     *width_result
    // );
    script_data
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_get_script_chars_len(script_data: *const u8) -> usize {
    // println!("fontapi_get_script_chars_len: {:?}", script_data);
    let mut result = 0;

    let mut script_data = script_data;
    while *script_data != 0xE6 {
        let (code, is_double_encode) =
            crate::script::decode_script(core::slice::from_raw_parts(script_data, 2));
        // println!("{}, {}", code, is_double_encode);
        if code == u16::MAX {
            break;
        }
        script_data = script_data.add(if is_double_encode { 2 } else { 1 });
        result += 1;
    }

    // println!("fontapi_get_script_chars_len: {}", result);

    result
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_get_script_bytes_len(script_data: *const u8) -> usize {
    // println!("fontapi_get_script_bytes_len: {:?}", script_data);

    let mut cur_script_data = script_data;
    while *cur_script_data != 0xE6 {
        let (_code, is_double_encode) =
            crate::script::decode_script(core::slice::from_raw_parts(cur_script_data, 2));
        // println!("{}, {}", code, is_double_encode);
        // if code == u16::MAX {
        //     break;
        // }
        cur_script_data = cur_script_data.add(if is_double_encode { 2 } else { 1 });
    }

    // println!("fontapi_get_script_bytes_len: {}", result);

    cur_script_data as usize - script_data as usize
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_get_font_graph_addr(graph_id: u16, font_id: usize) -> *const u8 {
    // println!("fontapi_get_font_graph_addr: {:04X} {}", graph_id, font_id);
    let init_data = &global_data().init_data;
    let font_loader = &mut global_data().font_loader;
    let addr = match font_id {
        0 => {
            let addr = font_loader.get_graph_font3(graph_id).as_ptr();
            let dest = init_data.font3_pos + 0x80;
            addr.copy_to_nonoverlapping(dest as _, 0x80);
            dest as _
        }
        1 => {
            let addr = font_loader.get_graph_font1(graph_id).as_ptr();
            let dest = init_data.font1_pos + 0x40;
            addr.copy_to_nonoverlapping(dest as _, 0x40);
            dest as _
        }
        2 => {
            let addr = font_loader.get_graph_font2(graph_id).as_ptr();
            let dest = init_data.font2_pos + 0x40;
            addr.copy_to_nonoverlapping(dest as _, 0x40);
            dest as _
        }
        _ => core::ptr::null(),
    };
    addr
}

fn get_gb2_tile_base() -> usize {
    unsafe {
        let bg2cnt = *(nitro::sys::REG_BG2CNT_ADDR as usize as *const u16);
        let tile_offset = bg2cnt >> 2 & 0b1111;

        0x06000000usize + tile_offset as usize * 0x4000
    }
}

static mut CUR_BG2CNT: u16 = 0;
fn check_vram_and_reset() {
    unsafe {
        let bg2cnt = *(nitro::sys::REG_BG2CNT_ADDR as usize as *const u16);
        if CUR_BG2CNT != bg2cnt {
            CUR_BG2CNT = bg2cnt;
            let tile_offset = bg2cnt >> 2 & 0b1111;
            let tile_base = 0x06000000usize + tile_offset as usize * 0x4000;
            println!(
                "tile base addr has been changed to {:08X} (offset bit {:02X}, CNT {:04X})",
                tile_base, tile_offset, bg2cnt
            );
            nitro::sys::MI_CpuFill8(tile_base as *mut _, 0, 0x40 * 2);
            global_data().vram_font_loader.reset(tile_base + 0x40 * 2);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_reset_vram_cache_library(
    _a1: usize,
    _a2: usize,
    _a3: usize,
    _a4: usize,
    _a5: usize,
) {
    println!("fontapi_reset_vram_cache_library");
    CUR_BG2CNT = 0;
    check_vram_and_reset();
    *(nitro::sys::REG_BG2CNT_ADDR as usize as *mut u16) = 0x0355;
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_reset_vram_cache_folder(
    _a1: usize,
    _a2: usize,
    _a3: usize,
    _a4: usize,
    _a5: usize,
) {
    println!("fontapi_reset_vram_cache_folder");
    *(nitro::sys::REG_BG2CNT_ADDR as usize as *mut u16) = 0x034D;
    CUR_BG2CNT = 0;
    check_vram_and_reset();
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_place_char(dest: *mut u16, index: u16, next_pos: usize) {
    check_vram_and_reset();
    let graph_index = index / 2;
    println!(
        "fontapi_place_char: {:?} {:04X} ({}) {}",
        dest, index, graph_index, next_pos
    );
    let graph_index = global_data().vram_font_loader.fetch(graph_index) * 2 + 4;
    *dest = graph_index as u16;
    (*dest.add(next_pos)) = graph_index as u16 + 1;
}

const ARCHIVE_FILE_NAMES: &[&str] = &[
    ":null:",
    "models.bin",
    "motions.bin",
    "stages.bin",
    "panel.bin",
    "messobj.bin",
    "subscreen.bin",
    "subscreentuto.bin",
    "field.bin",
    "fieldscrchg.bin",
    "fieldbg.bin",
    "fieldbgefc.bin",
    "fieldobj.bin",
    "fieldface.bin",
    "fieldcockpit.bin",
    "minigameskigame.bin",
    "erandgame.bin",
    "custom.bin",
    "battlecard.bin",
    "cockpit.bin",
    "capcomlogo.bin",
    "result.bin",
    "wavain_demo.bin",
    "emscript.bin",
    "antenna.bin",
    "mess.bin",
    "subscreen_local.bin",
    "fieldobj_local.bin",
    "erandgame_local.bin",
    "custom_local.bin",
    "cockpit_local.bin",
    "capcomlogo_local.bin",
    "result_local.bin",
];

#[no_mangle]
pub unsafe extern "C" fn fontapi_log_get_archive_path(archive_id: usize, callee: usize) {
    let archive_file = *ARCHIVE_FILE_NAMES
        .get((archive_id & 0xFFFF0000) >> 16)
        .unwrap_or(&"unknown file");

    static mut LAST_ARCHIVE_ID: usize = 0;
    if LAST_ARCHIVE_ID != archive_id {
        LAST_ARCHIVE_ID = archive_id;
        if archive_file == "mess.bin" {
            println!(
                "log get archive id: {:08X} -> {} ({})",
                archive_id,
                archive_file,
                archive_id & 0xFFFF
            );
            println!("  from {:08X}", callee);
            if archive_id & 0xFFFF == 86 {
                nogba::nogba_breakpoint();
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_sub_2002E0C_hook(
    dest_script: *mut u8,
    src_script: *const u8,
    limit: usize,
) -> usize {
    let mut cur_dest_script = dest_script;
    let mut cur_src_script = src_script;
    for i in 0..limit {
        print!("{:02X} ", *cur_src_script);
        if *cur_src_script == 0xE6 {
            *cur_dest_script = 0xE6;
            println!(
                "\nfontapi_sub_2002E0C_hook {:?} -> {:?} {}",
                src_script, dest_script, i
            );
            return i + 1;
        }
        *cur_dest_script = *cur_src_script;
        cur_src_script = cur_src_script.add(1);
        cur_dest_script = cur_dest_script.add(1);
        if script::decode_script(core::slice::from_raw_parts(cur_src_script, 2)).1 {
            print!("{:02X} ", *cur_src_script);
            *cur_dest_script = *cur_src_script;
            cur_dest_script = cur_dest_script.add(1);
            cur_src_script = cur_src_script.add(1);
        }
    }
    println!(
        "\nfontapi_sub_2002E0C_hook {:?} -> {:?} overflowed {}",
        src_script, dest_script, limit
    );
    *dest_script = 0xE6;
    1
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_sub_217BBC8_hook(
    mut dest_script: *mut u8,
    mut src_script: *const u8,
    mut script_limit: usize,
) -> *mut u8 {
    // TODO: 是否需要考虑换行符
    println!(
        "fontapi_sub_217BBC8_hook: {:?} -> {:?} ({})",
        src_script, dest_script, script_limit,
    );
    while script_limit > 0 {
        match script::decode_script(core::slice::from_raw_parts(src_script, 2)) {
            (u16::MAX, _) => {
                break;
            }
            (_, false) => {
                print!("{:02X} ", *src_script);
                *dest_script = *src_script;
                dest_script = dest_script.add(1);
                src_script = src_script.add(1);
            }
            (_, true) => {
                print!("{:02X} {:02X} ", *src_script, *src_script.add(1));
                src_script.copy_to(dest_script, 2);
                dest_script = dest_script.add(2);
                src_script = src_script.add(2);
            }
        }
        script_limit = script_limit.wrapping_sub(1);
    }
    print!("\n");
    dest_script
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_input_char(
    encode_width_dest: usize,
    encode_width_src: usize,
    encode_addr_dest: *mut u8,
    encode_addr_src: *const u8,
    is_update: bool,
) -> bool {
    println!("fontapi_input_char");
    {
        println!("  is_update: {}", is_update);
        println!("  encode_width_dest: {}", encode_width_dest);
        println!("  encode_width_src:  {}", encode_width_src);
        println!(
            "  encode_addr_dest:  {:08X} ({:02X?})",
            encode_addr_dest as usize,
            core::slice::from_raw_parts(encode_addr_dest, 16)
        );
        println!(
            "  encode_addr_src:   {:08X} ({:02X?})",
            encode_addr_src as usize,
            core::slice::from_raw_parts(encode_addr_src, encode_width_src)
        );
    }
    // 在内容改变时应该返回 false
    if encode_width_dest == encode_width_src && is_update {
        if core::slice::from_raw_parts_mut(encode_addr_dest, encode_width_dest)
            == core::slice::from_raw_parts(encode_addr_src, encode_width_src)
        {
            true
        } else {
            // if *encode_addr_dest == 0xE6 {
            //     encode_addr_dest.copy_to(encode_addr_dest.add(encode_width_src), 1);
            // }
            encode_addr_src.copy_to(encode_addr_dest, encode_width_src);
            {
                println!(
                    "  result:   {:02X?}",
                    core::slice::from_raw_parts(encode_addr_dest, 16)
                );
            }
            false
        }
    } else {
        let rest_data = if is_update {
            encode_addr_dest.add(encode_width_dest)
        } else {
            encode_addr_dest
        };
        let new_pos = encode_addr_dest.add(encode_width_src);
        let dest_len = fontapi_get_script_bytes_len(rest_data);
        rest_data.copy_to(new_pos, dest_len + 1);
        encode_addr_src.copy_to(encode_addr_dest, encode_width_src);
        {
            println!(
                "  result:   {:02X?}",
                core::slice::from_raw_parts(encode_addr_dest, 16)
            );
        }
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn fontapi_shrink_script_data_sub_2002F18(
    src_script: *const u8,
    dest_script: *mut u8,
) {
    println!(
        "Shrink script data {:08X} -> {:08X}",
        src_script as usize, dest_script as usize
    );
    let src_len = fontapi_get_script_bytes_len(src_script);
    dest_script.copy_from(src_script, src_len + 1);
    let dest_script = dest_script.add(src_len + 1);
    *dest_script = 0;
    let dest_script = dest_script.add(1);
    *dest_script = 0;
}

#[cfg(target_arch = "arm")]
mod patch {
    #[global_allocator]
    static ALLOCATOR: nitro::alloc::NitroAllocator = nitro::alloc::NitroAllocator;

    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        use nitro::println;
        println!("Patch code panicked!");
        println!("{}", _info);
        unsafe {
            nitro::sys::OS_Terminate();
        }
    }
}

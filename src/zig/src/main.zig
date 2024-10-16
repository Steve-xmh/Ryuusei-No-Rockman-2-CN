const std = @import("std");
const p = std.builtin.default_panic;
const nitro = @import("./nitro.zig");
const nogba = @import("./nogba.zig");
const font = @import("./font.zig");
const FontId = font.FontId;
const GameCtx = @import("./game.zig").GameCtx;
const print = nogba.print;
const println = nogba.println;
const Allocator = std.mem.Allocator;
const FixedBufferAllocator = std.heap.FixedBufferAllocator;

const InitData = extern struct {
    font1_pos: u32,
    font2_pos: u32,
    font3_pos: u32,
    font3_graph_amount: u32,
    heap_start: u32,
    heap_end: u32,
};

const Font1Loader = font.SimpleFontLoader("fonts/font1.bin", 0x40, 128);
const Font2Loader = font.SimpleFontLoader("fonts/font2.bin", 0x40, 128);
const Font3Loader = font.FontWithGraphWidthLoader("fonts/font3.bin", "fonts/font3_width.bin", 0x80, 256 + 64);

const GlobalState = struct {
    cur_bg2cnt: u16,
    init_data: *const InitData,
    font1_loader: *Font1Loader,
    font2_loader: *Font2Loader,
    font3_loader: *Font3Loader,
};

// ARMIPS 不会导入 .bss 段
var orig_allocator: FixedBufferAllocator linksection(".text") = undefined;
pub var global_state: *GlobalState linksection(".text") = undefined;

pub fn allocator() Allocator {
    return orig_allocator.allocator();
}

extern "c" fn OS_Terminate() noreturn;

pub fn fatal_error(msg: []const u8, err: anyerror) noreturn {
    const err_name = @errorName(err);
    println("--- FATAL ERROR ---");
    println("An error from patch code occurred and the game cannot continue.");
    print("Error Msg:  ");
    println(msg);
    print("Error Code: ");
    println(err_name);
    OS_Terminate();
}

export fn fontapi_main(init_data: *const InitData) void {
    println("Ryuusei No Rockman 2 CN Patch");
    println("By SteveXMH written in ASM/Zig");

    const heap_size: usize = init_data.heap_end - init_data.heap_start;
    print("Available heap size for Zig: ");
    nogba.print_hex(usize, heap_size);
    println("");
    const raw_heap: [*]u8 = @ptrFromInt(init_data.heap_start);
    const heap = raw_heap[0..heap_size];
    for (heap) |*ptr| {
        ptr.* = 0;
    }
    orig_allocator = FixedBufferAllocator.init(heap[0..heap_size]);
    const alloc = allocator();
    global_state = alloc.create(GlobalState) catch |err| fatal_error("Failed to allocate global state", err);

    global_state.init_data = init_data;
    global_state.font1_loader = Font1Loader.init(alloc, init_data.font1_pos) catch |err| fatal_error("Failed to load font1", err);
    global_state.font2_loader = Font2Loader.init(alloc, init_data.font2_pos) catch |err| fatal_error("Failed to load font2", err);
    global_state.font3_loader = Font3Loader.init(alloc, init_data.font3_pos) catch |err| fatal_error("Failed to load font3", err);

    print("Rest heap size for Zig: ");
    const last_index = orig_allocator.end_index;
    nogba.print_hex(usize, heap_size - last_index);
    println("");
    println("Finished load patch!");
}

export fn fontapi_read_script_font(game_ctx: *GameCtx) void {
    print("fontapi_read_script_font ");
    nogba.print_hex(u32, @intFromPtr(game_ctx));
    const code = game_ctx.read_script_code();
    game_ctx.set_char_code(code);
    const font_id = game_ctx.get_font_id() orelse return;
    switch (font_id) {
        FontId.Font1 => {
            global_state.font1_loader.load_graph(code);
        },
        FontId.Font2 => {
            global_state.font2_loader.load_graph(code);
        },
        FontId.Font3 => {
            global_state.font3_loader.load_graph(code);
        },
    }
}

export fn fontapi_read_script_font_1(game_ctx: *GameCtx) void {
    println("fontapi_read_script_font_1");
    fontapi_read_script_font(game_ctx);
}

export fn fontapi_read_script_font_2(game_ctx: *GameCtx) void {
    println("fontapi_read_script_font_2");
    fontapi_read_script_font(game_ctx);
}

export fn fontapi_read_script_font_3(game_ctx: *GameCtx) void {
    println("fontapi_read_script_font_3");
    fontapi_read_script_font(game_ctx);
}

export fn fontapi_move_draw_cursor(game_ctx: *GameCtx) void {
    println("fontapi_move_draw_cursor");
    switch (game_ctx.get_render_method()) {
        0x400, 0x600 => {
            game_ctx.move_draw_cursor(11);
        },
        else => {
            // 动态内容，需要定长字宽
            const BUFFER_START: usize = 0x20F75A8;
            const BUFFER_END: usize = BUFFER_START + 0x24 * 4 - 1;
            switch (@intFromPtr(game_ctx.get_script_data())) {
                BUFFER_START...BUFFER_END => {
                    game_ctx.move_draw_cursor(11);
                },
                else => {
                    const code = game_ctx.get_char_code();
                    game_ctx.move_draw_cursor(global_state.font3_loader.get_graph_width(code));
                },
            }
        },
    }
}

export fn fontapi_read_script_font_3_fixed_width(game_ctx: *GameCtx) void {
    _ = game_ctx;
    fatal_error("unimplemented fontapi fontapi_read_script_font_3_fixed_width", undefined);
}

export fn fontapi_read_script(game_ctx: *GameCtx) void {
    _ = game_ctx;
    fatal_error("unimplemented fontapi fontapi_read_script", undefined);
}

export fn fontapi_transform_multi_line_script(game_ctx: *GameCtx) void {
    _ = game_ctx;
    fatal_error("unimplemented fontapi fontapi_transform_multi_line_script", undefined);
}

export fn fontapi_get_code_size(first_code: u8) u32 {
    _ = first_code;
    fatal_error("unimplemented fontapi fontapi_get_code_size", undefined);
}

export fn fontapi_sub_2002E0C_hook(
    dest_script: *u8,
    src_script: *const u8,
    limit: usize,
) void {
    _ = dest_script;
    _ = src_script;
    _ = limit;
    // fatal_error("unimplemented fontapi fontapi_sub_2002E0C_hook", undefined);
}

export fn fontapi_shrink_script_data_sub_2002F18(
    src_script: *const u8,
    dest_script: *u8,
) void {
    _ = src_script;
    _ = dest_script;
    fatal_error("unimplemented fontapi fontapi_shrink_script_data_sub_2002F18", undefined);
}

export fn fontapi_get_script_chars_len(script_data: *const u8) usize {
    _ = script_data;
    fatal_error("unimplemented fontapi fontapi_get_script_chars_len", undefined);
}

export fn fontapi_get_script_chars_len_loop(script_data: *const u8) usize {
    _ = script_data;
    fatal_error("unimplemented fontapi fontapi_get_script_chars_len_loop", undefined);
}

export fn fontapi_get_font_graph_addr(graph_id: u16, font_id: usize) *const u8 {
    _ = graph_id;
    _ = font_id;
    fatal_error("unimplemented fontapi fontapi_get_font_graph_addr", undefined);
}

export fn fontapi_log_get_archive_path() void {
    if (comptime false) {
        fatal_error("unimplemented fontapi fontapi_log_get_archive_path", undefined);
    }
}

export fn fontapi_place_char(dest: *u16, index: u16, next_pos: usize) void {
    _ = dest;
    _ = index;
    _ = next_pos;
    fatal_error("unimplemented fontapi fontapi_place_char", undefined);
}

fn check_vram_and_reset() void {}

export fn fontapi_reset_vram_cache_library(
    _: usize,
    _: usize,
    _: usize,
    _: usize,
    _: usize,
) void {
    const REG_BG2CNT_ADDR: *u16 = @ptrFromInt(0x0400000C);
    global_state.cur_bg2cnt = 0x034D;
    check_vram_and_reset();
    REG_BG2CNT_ADDR.* = 0x0355;
}

export fn fontapi_reset_vram_cache_folder(
    _: usize,
    _: usize,
    _: usize,
    _: usize,
    _: usize,
) void {
    const REG_BG2CNT_ADDR: *u16 = @ptrFromInt(0x0400000C);
    REG_BG2CNT_ADDR.* = 0x034D;
    global_state.cur_bg2cnt = 0x034D;
    check_vram_and_reset();
}

export fn fontapi_get_encode_size_by_index(
    script_data: *const u8,
    index: usize,
    width_result: *u8,
) *const u8 {
    _ = script_data;
    _ = index;
    _ = width_result;
    fatal_error("unimplemented fontapi fontapi_get_encode_size_by_index", undefined);
}

export fn fontapi_input_char(
    encode_width_dest: usize,
    encode_width_src: usize,
    encode_addr_dest: *u8,
    encode_addr_src: *const u8,
    is_update: bool,
) bool {
    _ = encode_width_dest;
    _ = encode_width_src;
    _ = encode_addr_dest;
    _ = encode_addr_src;
    _ = is_update;
    fatal_error("unimplemented fontapi fontapi_input_char", undefined);
}

export fn fontapi_sub_217BBC8_hook(
    dest_script: *u8,
    src_script: *const u8,
    script_limit: usize,
) *u8 {
    _ = dest_script;
    _ = src_script;
    _ = script_limit;
    fatal_error("unimplemented fontapi fontapi_sub_217BBC8_hook", undefined);
}

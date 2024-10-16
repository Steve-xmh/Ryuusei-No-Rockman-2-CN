const fatal_error = @import("./main.zig").fatal_error;
const std = @import("std");
const font = @import("./font.zig");
const nogba = @import("./nogba.zig");
const FontId = font.FontId;
const maxInt = std.math.maxInt;

pub const GameCtx = extern struct {
    pub inline fn get_script_data(self: *GameCtx) [*]u8 {
        const script_data_ptr: *[*]u8 = @ptrFromInt(@intFromPtr(self) + 0x10);
        return script_data_ptr.*;
    }

    pub inline fn move_script_data(self: *GameCtx, offset: usize) void {
        const script_data_ptr: *usize = @ptrFromInt(@intFromPtr(self) + 0x10);
        script_data_ptr.* += offset;
    }

    pub fn get_script_code_size(self: *GameCtx) usize {
        const data = self.get_script_data();
        const first_code = data[0];
        switch (first_code) {
            0x00...0xCF => {
                return 1;
            },
            0xD0...0xE4 => {
                return 2;
            },
            else => {
                return 1;
            },
        }
    }

    pub fn read_script_code(self: *GameCtx) u16 {
        const data = self.get_script_data();
        const first_code = @as(u16, data[0]);
        const second_code = @as(u16, data[1]);
        self.move_script_data(self.get_script_code_size());
        switch (first_code) {
            0x00...0xCF => {
                return first_code;
            },
            0xD0...0xE3 => {
                return (first_code - 0xD0) * 0xE4 + second_code + 0xD0;
            },
            0xE4 => {
                return first_code + second_code;
            },
            0xE9 => {
                return maxInt(u16) - 1;
            },
            else => {
                fatal_error("Invalid script code", undefined);
                unreachable;
            },
        }
    }

    pub inline fn get_font_addr(self: *GameCtx) *const u8 {
        const script_data_ptr: **const u8 = @ptrFromInt(@intFromPtr(self) + 0x24);
        return script_data_ptr.*;
    }

    pub inline fn set_char_code(self: *GameCtx, code: u16) void {
        @as(*u16, @ptrFromInt(@intFromPtr(self) + 0x44)).* = code;
        @as(*u16, @ptrFromInt(@intFromPtr(self) + 0x64)).* = code;
    }

    pub inline fn get_char_code(self: *GameCtx) u16 {
        return @as(*u16, @ptrFromInt(@intFromPtr(self) + 0x44)).*;
    }

    pub inline fn move_draw_cursor(self: *GameCtx, offset: i16) void {
        @as(*i16, @ptrFromInt(@intFromPtr(self) + 0x60)).* += offset * 4;
    }

    pub inline fn get_is_font3(self: *GameCtx) bool {
        return @as(*c_char, @ptrFromInt(@intFromPtr(self) + 0x60)).* != 0;
    }

    pub inline fn get_render_method(self: *GameCtx) u32 {
        return @as(*u32, @ptrFromInt(@intFromPtr(self) + 0x128)).*;
    }

    pub fn get_font_id(self: *GameCtx) ?FontId {
        const FONT1_POS: usize = 0x020B7898;
        const FONT2_POS: usize = 0x020C0098;
        const FONT3_POS: usize = 0x020C8898;
        const FONT1_END_POS: usize = FONT1_POS + 0x40 * 0x1E3;
        const FONT2_END_POS: usize = FONT2_POS + 0x40 * 0x1E3;
        const FONT3_END_POS: usize = FONT3_POS + 0x80 * 0x1E3;
        const FONT1_END_POS_E: usize = FONT1_END_POS - 1;
        const FONT2_END_POS_E: usize = FONT2_END_POS - 1;
        const FONT3_END_POS_E: usize = FONT3_END_POS - 1;

        const addr = @intFromPtr(self.get_font_addr());
        switch (addr) {
            FONT1_POS...FONT1_END_POS_E => return FontId.Font1,
            FONT2_POS...FONT2_END_POS_E => return FontId.Font2,
            FONT3_POS...FONT3_END_POS_E => return FontId.Font3,
            else => return null,
        }
    }
};

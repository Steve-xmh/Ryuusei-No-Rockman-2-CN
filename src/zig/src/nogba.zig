const NOGBA_STRING_OUT: *volatile u32 = @ptrFromInt(0x04FFFA1C);

pub inline fn nogba_breakpoint() void {
    asm volatile ("mov r11, r11");
}

pub fn print(msg: []const u8) void {
    for (msg) |c| {
        NOGBA_STRING_OUT.* = c;
    }
}

const NUM_CHARS: []const u8 = "0123456789ABCDEF";
pub fn print_hex(comptime T: type, num: u32) void {
    NOGBA_STRING_OUT.* = '0';
    NOGBA_STRING_OUT.* = 'x';
    const hex_size = @bitSizeOf(T) / 4;
    for (0..hex_size) |i| {
        NOGBA_STRING_OUT.* = NUM_CHARS[(num >> @as(u5, @intCast(hex_size - 1 - i)) * 4) & 0xF];
    }
}

pub fn println(msg: []const u8) void {
    print(msg);
    NOGBA_STRING_OUT.* = '\n';
}

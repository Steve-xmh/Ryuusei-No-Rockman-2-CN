const Allocator = @import("std").mem.Allocator;

pub const FileError = error{
    CantOpen,
};

pub const FSFile = extern struct {
    file: [0x3C]u8,

    pub fn open(self: *FSFile, path: []const u8) !void {
        FS_InitFile(self);
        if (FS_OpenFile(self, path.ptr) == 0) {
            return FileError.CantOpen;
        }
    }

    pub fn close(self: *FSFile, alloc: *Allocator) void {
        FS_CloseFile(self);
        alloc.deinit(self);
    }

    pub fn seek(self: *FSFile, offset: i32, origin: FSSeekFileMode) bool {
        return FS_SeekFile(self, offset, origin) != 0;
    }

    pub fn read(self: *FSFile, buf: []u8) i32 {
        return FS_ReadFile(self, buf.ptr, buf.len);
    }
};

pub const FSSeekFileMode = enum(u32) {
    Set = 0,
    Cur = 1,
    End = 2,
};

extern "c" fn FS_InitFile(p_file: *FSFile) void;
extern "c" fn FS_CloseFile(p_file: *FSFile) c_int;
extern "c" fn FS_OpenFile(p_file: *FSFile, path: [*]const u8) c_int;
extern "c" fn FS_SeekFile(p_file: *FSFile, offset: i32, origin: FSSeekFileMode) c_int;
extern "c" fn FS_ReadFile(p_file: *FSFile, dest: [*]u8, len: u32) c_int;
pub extern "c" fn memcpy(dest: [*]u8, src: [*]const u8, len: u32) void;
pub extern "c" fn memset(dest: [*]u8, value: u8, len: u32) void;

export fn __aeabi_memcpy4(dest: [*]u8, src: [*]const u8, len: u32) void {
    memcpy(dest, src, len);
}

export fn __aeabi_memcpy(dest: [*]u8, src: [*]const u8, len: u32) void {
    memcpy(dest, src, len);
}

export fn __aeabi_memclr4(dest: [*]u8, len: u32) void {
    memset(dest, 0, len);
}

const std = @import("std");
const LRUCache = @import("./lru.zig").LRUCache;
const nitro = @import("./nitro.zig");
const nogba = @import("./nogba.zig");

pub const FontId = enum {
    Font1,
    Font2,
    Font3,
};

pub fn SimpleFontLoader(
    comptime file_path: []const u8,
    comptime graph_size: usize,
    comptime cache_capacity: usize,
) type {
    const FontGraph = [graph_size]u8;
    return struct {
        file: nitro.FSFile,
        cache: LRUCache(usize, FontGraph, cache_capacity),
        zero_graph_pos: *[graph_size]u8,

        const Self = @This();
        pub fn init(allocator: std.mem.Allocator, zero_graph_pos: u32) !*Self {
            const self = try allocator.create(Self);
            self.zero_graph_pos = @as(*[graph_size]u8, @ptrFromInt(zero_graph_pos));
            try self.file.open(file_path);
            nogba.print("Opened font file: ");
            nogba.println(file_path[0..file_path.len]);
            return self;
        }

        pub fn load_graph(self: *Self, graph_id: usize) void {
            const zero_graph_pos = self.zero_graph_pos;
            const cached = self.cache.get(graph_id) orelse {
                _ = self.file.seek(@as(i32, @intCast(graph_id * graph_size)), nitro.FSSeekFileMode.Set);
                var graph = [_]u8{0} ** graph_size;
                _ = self.file.read(graph[0..graph.len]);
                const src = self.cache.put(graph_id, graph);
                nitro.memcpy(zero_graph_pos, src.ptr, graph_size);
                return;
            };
            nitro.memcpy(zero_graph_pos, cached.ptr, graph_size);
        }
    };
}

pub fn FontWithGraphWidthLoader(
    comptime file_path: []const u8,
    comptime graph_file_path: []const u8,
    comptime graph_size: usize,
    comptime cache_capacity: usize,
) type {
    const FontGraph = struct {
        font_data: [graph_size]u8,
        width_data: u8,
    };
    return struct {
        file: nitro.FSFile,
        width_file: nitro.FSFile,
        cache: LRUCache(usize, FontGraph, cache_capacity),
        zero_graph_pos: *[graph_size]u8,

        const Self = @This();
        pub fn init(allocator: std.mem.Allocator, zero_graph_pos: u32) !*Self {
            const self = try allocator.create(Self);
            self.zero_graph_pos = @as(*[graph_size]u8, @ptrFromInt(zero_graph_pos));
            try self.file.open(file_path);
            nogba.print("Opened font file: ");
            nogba.println(file_path);
            try self.width_file.open(graph_file_path);
            nogba.print("Opened font graph width file: ");
            nogba.println(graph_file_path);
            return self;
        }

        pub fn load_graph(self: *Self, graph_id: usize) void {
            const zero_graph_pos = self.zero_graph_pos;
            const cached = self.cache.get(graph_id) orelse {
                _ = self.file.seek(@as(i32, @intCast(graph_id * graph_size)), nitro.FSSeekFileMode.Set);
                var graph = [_]u8{0} ** graph_size;
                _ = self.file.read(graph[0..]);

                _ = self.width_file.seek(@as(i32, @intCast(graph_id)), nitro.FSSeekFileMode.Set);
                var width = [_]u8{0};
                _ = self.width_file.read(width[0..]);

                const src = self.cache.put(graph_id, .{
                    .font_data = graph,
                    .width_data = width[0],
                });

                nitro.memcpy(zero_graph_pos, &src.font_data, graph_size);
                return;
            };
            nitro.memcpy(zero_graph_pos, &cached.font_data, graph_size);
        }

        pub fn get_graph_width(self: *Self, graph_id: usize) i16 {
            const cached = self.cache.get(graph_id) orelse {
                return 11;
            };
            return cached.width_data;
        }
    };
}

const nogba = @import("./nogba.zig");
const fatal_error = @import("./main.zig").fatal_error;
const std = @import("std");
const BoundedArray = std.BoundedArray;

const CacheError = error{
    NotFound,
};

pub fn LRUCache(comptime K: type, comptime T: type, comptime capacity: usize) type {
    return struct {
        cache: BoundedArray(struct {
            key: K,
            value: T,
        }, capacity),
        pub fn init(allocator: *std.mem.Allocator) *@This() {
            var self = allocator.create(@This());
            self.cache.init(0);
            return self;
        }
        pub fn deinit(self: *@This(), allocator: *std.mem.Allocator) void {
            allocator.free(self.cache);
        }
        pub fn get(self: *@This(), key: K) ?*const T {
            nogba.println("LRUCache: get");
            for (self.cache.slice(), 0..) |entry, i| {
                if (entry.key == key) {
                    const target = self.cache.orderedRemove(i);
                    self.cache.insert(0, target) catch fatal_error("LRUCache: orderedRemove failed", CacheError.NotFound);
                    return &self.cache.get(0).value;
                }
            }
            return null;
        }
        pub fn put(self: *@This(), key: K, value: T) *const T {
            nogba.println("LRUCache: put");
            if (self.cache.len == capacity) {
                _ = self.cache.pop();
            }
            self.cache.insert(0, .{ .key = key, .value = value }) catch fatal_error("LRUCache: insert failed", CacheError.NotFound);
            return &self.cache.get(0).value;
        }
    };
}

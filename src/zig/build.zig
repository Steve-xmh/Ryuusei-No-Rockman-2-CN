const std = @import("std");
const zig = std.zig;
const LazyPath = @import("std").Build.LazyPath;

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{ .default_target = .{ .abi = .eabi, .cpu_arch = .arm, .os_tag = .freestanding, .cpu_model = .{ .explicit = &.{
        .name = "arm946e_s",
        .llvm_name = "armv5te",
        .features = std.Target.Cpu.Feature.Set.empty,
    } } } });
    const optimize = b.standardOptimizeOption(.{
        .preferred_optimize_mode = .ReleaseSmall,
    });
    const root_source_file = b.path("src/main.zig");
    const linker_script = b.path("linker.ld");

    const ninja_options = b.addOptions();
    ninja_options.addOption(bool, "is_saurian", false);

    const saurian_options = b.addOptions();
    saurian_options.addOption(bool, "is_saurian", true);

    const ninja_lib = b.addStaticLibrary(.{
        .name = "ninja",
        .root_source_file = root_source_file,
        .link_libc = true,
        .single_threaded = true,
        .error_tracing = false,
        .optimize = optimize,
        .target = target,
    });
    ninja_lib.linker_script = linker_script;
    ninja_lib.root_module.addOptions("build_opt", ninja_options);

    const saurian_lib = b.addStaticLibrary(.{
        .name = "saurian",
        .root_source_file = root_source_file,
        .link_libc = true,
        .single_threaded = true,
        .error_tracing = false,
        .optimize = optimize,
        .target = target,
    });
    saurian_lib.linker_script = linker_script;
    saurian_lib.root_module.addOptions("build_opt", saurian_options);

    const ninja_install = b.addInstallArtifact(ninja_lib, .{});
    const ninja_asm_code = b.addInstallFile(ninja_lib.getEmittedAsm(), "asm-ninja.txt");
    b.default_step.dependOn(&ninja_asm_code.step);
    b.default_step.dependOn(&ninja_install.step);

    const saurian_install = b.addInstallArtifact(saurian_lib, .{});
    const saurian_asm_code = b.addInstallFile(saurian_lib.getEmittedAsm(), "asm-saurian.txt");
    b.default_step.dependOn(&saurian_asm_code.step);
    b.default_step.dependOn(&saurian_install.step);
}

const std = @import("std");
const zig = std.zig;
const LazyPath = @import("std").build.LazyPath;

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{ .default_target = .{ .abi = .eabi, .cpu_arch = .arm, .os_tag = .freestanding, .cpu_model = .{ .explicit = &.{
        .name = "arm946e_s",
        .llvm_name = "armv5te",
        .features = std.Target.Cpu.Feature.Set.empty,
    } } } });
    const optimize = b.standardOptimizeOption(.{
        .preferred_optimize_mode = .ReleaseSmall,
    });
    const root_source_file = LazyPath.relative("src/main.zig");
    // const linker_script = LazyPath.relative("linker.ld");

    const ninja_options = b.addOptions();
    ninja_options.addOption(bool, "is_saurian", false);

    const saurian_options = b.addOptions();
    saurian_options.addOption(bool, "is_saurian", true);

    // const ninja_bin = b.addExecutable(.{
    //     .name = "ninja",
    //     .root_source_file = root_source_file,
    //     .linkage = .static,
    //     .link_libc = false,
    //     .single_threaded = true,
    //     .optimize = optimize,
    //     .target = target,
    // });
    // ninja_bin.linker_script = linker_script;
    // ninja_bin.strip = false;
    // ninja_bin.link_emit_relocs = true;
    // ninja_bin.addOptions("build_opt", ninja_options);

    // const saurian_bin = b.addExecutable(.{
    //     .name = "saurian",
    //     .root_source_file = root_source_file,
    //     .linkage = .static,
    //     .link_libc = false,
    //     .single_threaded = true,
    //     .optimize = optimize,
    //     .target = target,
    // });
    // saurian_bin.linker_script = linker_script;
    // saurian_bin.strip = false;
    // saurian_bin.link_emit_relocs = true;
    // saurian_bin.addOptions("build_opt", saurian_options);

    // const ninja_install = b.addInstallArtifact(ninja_bin, .{});
    // const ninja_asm_code = b.addInstallFile(ninja_bin.getEmittedAsm(), "asm-ninja.txt");
    // b.default_step.dependOn(&ninja_asm_code.step);
    // b.default_step.dependOn(&ninja_install.step);

    // const saurian_install = b.addInstallArtifact(saurian_bin, .{});
    // const saurian_asm_code = b.addInstallFile(saurian_bin.getEmittedAsm(), "asm-saurian.txt");
    // b.default_step.dependOn(&saurian_asm_code.step);
    // b.default_step.dependOn(&saurian_install.step);

    const ninja_lib = b.addStaticLibrary(.{
        .name = "ninja",
        .root_source_file = root_source_file,
        .link_libc = true,
        .single_threaded = true,
        .optimize = optimize,
        .target = target,
    });
    ninja_lib.addOptions("build_opt", ninja_options);

    const saurian_lib = b.addStaticLibrary(.{
        .name = "saurian",
        .root_source_file = root_source_file,
        .link_libc = true,
        .single_threaded = true,
        .optimize = optimize,
        .target = target,
    });
    saurian_lib.addOptions("build_opt", saurian_options);

    const ninja_install = b.addInstallArtifact(ninja_lib, .{});
    const ninja_asm_code = b.addInstallFile(ninja_lib.getEmittedAsm(), "asm-ninja.txt");
    b.default_step.dependOn(&ninja_asm_code.step);
    b.default_step.dependOn(&ninja_install.step);

    const saurian_install = b.addInstallArtifact(saurian_lib, .{});
    const saurian_asm_code = b.addInstallFile(saurian_lib.getEmittedAsm(), "asm-saurian.txt");
    b.default_step.dependOn(&saurian_asm_code.step);
    b.default_step.dependOn(&saurian_install.step);
}

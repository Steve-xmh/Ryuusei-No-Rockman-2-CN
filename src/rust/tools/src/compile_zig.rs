use anyhow::*;
use tools::utils::ToolsRunner;

pub fn main() -> anyhow::Result<()> {
    let cwd = std::env::current_dir().unwrap();
    let _ = ToolsRunner::new(Some(&cwd))?;

    // let target = "thumbv5te-none-eabi";

    ensure!(std::process::Command::new("zig")
        .arg("build")
        .arg("-Drelease=true")
        .current_dir(cwd.join("src/zig"))
        .status()?
        .success(),);

    let ninja_target_bin_path = cwd.join("src/zig/zig-out/lib/libninja.a");
    let saurian_target_bin_path = cwd.join("src/zig/zig-out/lib/libsaurian.a");

    std::fs::copy(
        ninja_target_bin_path,
        cwd.join("src/asm/ninja/zig-code.bin"),
    )?;
    std::fs::copy(
        saurian_target_bin_path,
        cwd.join("src/asm/saurian/zig-code.bin"),
    )?;

    Ok(())
}

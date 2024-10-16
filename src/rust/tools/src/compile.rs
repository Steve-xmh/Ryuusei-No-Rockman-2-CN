use anyhow::*;
use tools::utils::ToolsRunner;

pub fn main() -> anyhow::Result<()> {
    let cwd = std::env::current_dir().unwrap();
    let _ = ToolsRunner::new(Some(&cwd))?;

    let ninja_target_dir_path = cwd.join("_temp/target-ninja");
    let saurian_target_dir_path = cwd.join("_temp/target-saurian");

    let target = "armv5te-none-eabi";
    // let target = "thumbv5te-none-eabi";

    ensure!(std::process::Command::new("rustup")
        .arg("run")
        .arg("nightly")
        .arg("cargo")
        .arg("build")
        .arg("-Zbuild-std=core,alloc")
        .arg("--release")
        .arg("--no-default-features")
        .arg("--features")
        .arg("ninja")
        .arg("--package")
        .arg("arm9")
        .arg("--target")
        .arg(target)
        .arg("--target-dir")
        .arg(&ninja_target_dir_path)
        .status()?
        .success(),);

    ensure!(std::process::Command::new("rustup")
        .arg("run")
        .arg("nightly")
        .arg("cargo")
        .arg("build")
        .arg("-Zbuild-std=core,alloc")
        .arg("--release")
        .arg("--no-default-features")
        .arg("--features")
        .arg("saurian")
        .arg("--package")
        .arg("arm9")
        .arg("--target")
        .arg(target)
        .arg("--target-dir")
        .arg(&saurian_target_dir_path)
        .status()?
        .success(),);

    let ninja_target_bin_path = ninja_target_dir_path.join(target).join("release/arm9");
    let saurian_target_bin_path = saurian_target_dir_path.join(target).join("release/arm9");

    std::fs::copy(
        ninja_target_bin_path,
        cwd.join("src/asm/ninja/rust-code.bin"),
    )?;
    std::fs::copy(
        saurian_target_bin_path,
        cwd.join("src/asm/saurian/rust-code.bin"),
    )?;

    Ok(())
}

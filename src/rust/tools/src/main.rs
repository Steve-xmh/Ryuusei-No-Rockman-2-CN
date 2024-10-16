use anyhow::*;
use tools::utils::{fs::copy_dir_all, ToolsRunner};

mod compile;

fn main() -> anyhow::Result<()> {
    let cwd = std::env::current_dir().unwrap();
    let temp_trans_dir_path = cwd.join("_temp/translator-pack");
    let pack_target_dir_path = temp_trans_dir_path.join("target");
    let pack_dir_path = temp_trans_dir_path.join("pack");
    let _ = ToolsRunner::new(Some(&cwd))?;

    std::fs::create_dir_all(&pack_target_dir_path)?;
    let _ = std::fs::remove_dir_all(&pack_dir_path);
    std::fs::create_dir_all(&pack_dir_path)?;

    compile::main()?;

    ensure!(std::process::Command::new("rustup")
        .arg("run")
        .arg("stable")
        .arg("cargo")
        .arg("build")
        .arg("--release")
        .arg("--bin")
        .arg("setup")
        .arg("--target")
        .arg("x86_64-pc-windows-msvc")
        .arg("--target-dir")
        .arg(&pack_target_dir_path)
        .status()?
        .success(),);

    ensure!(std::process::Command::new("rustup")
        .arg("run")
        .arg("stable")
        .arg("cargo")
        .arg("build")
        .arg("--release")
        .arg("--bin")
        .arg("genfont")
        .arg("--target")
        .arg("x86_64-pc-windows-msvc")
        .arg("--target-dir")
        .arg(&pack_target_dir_path)
        .status()?
        .success(),);

    ensure!(std::process::Command::new("rustup")
        .arg("run")
        .arg("stable")
        .arg("cargo")
        .arg("build")
        .arg("--release")
        .arg("--bin")
        .arg("pack")
        .arg("--target")
        .arg("x86_64-pc-windows-msvc")
        .arg("--target-dir")
        .arg(&pack_target_dir_path)
        .status()?
        .success(),);

    std::fs::copy(
        pack_target_dir_path.join("x86_64-pc-windows-msvc/release/setup.exe"),
        temp_trans_dir_path.join("pack/初始化工作环境.exe"),
    )?;
    std::fs::copy(
        pack_target_dir_path.join("x86_64-pc-windows-msvc/release/genfont.exe"),
        temp_trans_dir_path.join("pack/生成字库.exe"),
    )?;
    std::fs::copy(
        pack_target_dir_path.join("x86_64-pc-windows-msvc/release/pack.exe"),
        temp_trans_dir_path.join("pack/打包游戏.exe"),
    )?;

    std::fs::copy(
        pack_target_dir_path.join("x86_64-pc-windows-msvc/release/setup.exe"),
        cwd.join("setup.exe"),
    )?;
    std::fs::copy(
        pack_target_dir_path.join("x86_64-pc-windows-msvc/release/genfont.exe"),
        cwd.join("genfont.exe"),
    )?;
    std::fs::copy(
        pack_target_dir_path.join("x86_64-pc-windows-msvc/release/pack.exe"),
        cwd.join("pack.exe"),
    )?;

    copy_dir_all(cwd.join("src/asm"), pack_dir_path.join("src/asm"))?;
    copy_dir_all(cwd.join("images"), pack_dir_path.join("images"))?;

    std::fs::create_dir_all(pack_dir_path.join("_rom"))?;
    std::fs::copy(
        cwd.join("_rom/ninja.nds"),
        pack_dir_path.join("_rom/ninja.nds"),
    )?;
    std::fs::copy(
        cwd.join("_rom/saurian.nds"),
        pack_dir_path.join("_rom/saurian.nds"),
    )?;
    copy_dir_all(cwd.join("tools"), pack_dir_path.join("tools"))?;
    sevenz_rust::compress_to_path(
        pack_dir_path,
        temp_trans_dir_path.join("Ryuusei-No-Rockman-2-CN.7z"),
    )?;

    Ok(())
}

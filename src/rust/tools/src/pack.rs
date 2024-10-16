use std::{io::Write, path::Path};

use anyhow::*;
use tools::utils::ToolsRunner;

pub fn main() -> anyhow::Result<()> {
    let cwd = std::env::current_dir().unwrap();
    let workspace_path = cwd.join("_workspace");
    let rom_path = cwd.join("_rom");
    let build_path = cwd.join("_build");
    let images_path = cwd.join("images");
    let temp_path = cwd.join("_temp/pack");
    let mess_path = temp_path.join("mess");
    let ninja_temp_path = temp_path.join("ninja");
    let saurian_temp_path = temp_path.join("saurian");
    let tools = ToolsRunner::new(Some(&cwd))?;

    std::fs::create_dir_all(cwd.join("tpl"))?;

    ensure!(tools
        .textpet_checker()
        .arg(cwd.join("tpl"))
        .status()?
        .success(),);

    let _ = std::fs::remove_dir_all(&mess_path);
    let _ = std::fs::remove_dir_all(&ninja_temp_path);
    let _ = std::fs::remove_dir_all(&saurian_temp_path);
    std::fs::create_dir_all(&mess_path)?;
    std::fs::create_dir_all(&ninja_temp_path)?;
    std::fs::create_dir_all(&saurian_temp_path)?;
    std::fs::create_dir_all(&build_path)?;

    std::fs::copy(
        workspace_path.join("ninja/arm9.bin"),
        ninja_temp_path.join("arm9.bin"),
    )?;
    std::fs::copy(
        workspace_path.join("saurian/arm9.bin"),
        saurian_temp_path.join("arm9.bin"),
    )?;
    std::fs::copy(
        workspace_path.join("ninja/y9.bin"),
        ninja_temp_path.join("y9.bin"),
    )?;
    std::fs::copy(
        workspace_path.join("saurian/y9.bin"),
        saurian_temp_path.join("y9.bin"),
    )?;
    tools::utils::fs::copy_dir_all(
        workspace_path.join("ninja/overlay"),
        ninja_temp_path.join("overlay"),
    )?;
    tools::utils::fs::copy_dir_all(
        workspace_path.join("saurian/overlay"),
        saurian_temp_path.join("overlay"),
    )?;
    tools::utils::fs::copy_dir_all(
        workspace_path.join("ninja/data"),
        ninja_temp_path.join("data"),
    )?;
    tools::utils::fs::copy_dir_all(
        workspace_path.join("saurian/data"),
        saurian_temp_path.join("data"),
    )?;

    ensure!(tools
        .armips()
        .arg("-strequ")
        .arg("TEMP")
        .arg(&ninja_temp_path)
        .arg("-sym")
        .arg(build_path.join("ninja.sym"))
        .arg(cwd.join("src/asm/ninja/_main.asm"))
        .current_dir(cwd.join("src/asm"))
        .status()?
        .success(),);

    ensure!(tools
        .armips()
        .arg("-strequ")
        .arg("TEMP")
        .arg(&saurian_temp_path)
        .arg("-sym")
        .arg(build_path.join("saurian.sym"))
        .arg(cwd.join("src/asm/saurian/_main.asm"))
        .current_dir(cwd.join("src/asm"))
        .status()?
        .success(),);

    let should_repack_bins = vec![
        "capcomlogo_local.bin".to_string(),
        "subscreen_local.bin".to_string(),
        "result_local.bin".to_string(),
    ];

    tools::dump_images::save_images()?;

    // 处理精灵图
    println!("正在处理精灵图");
    let sprites_out_dir = cwd.join("images/sfsprites_out");
    let mut should_repack_bins = should_repack_bins
        .into_iter()
        .chain(std::thread::scope(|s| {
            let mut bin_names = Vec::with_capacity(16);

            for dir in std::fs::read_dir(sprites_out_dir)?.flatten() {
                if dir.path().is_dir() {
                    let bin_name = dir.file_name().to_string_lossy().into_owned();
                    bin_names.push(bin_name.to_owned());
                    for sprite_png in std::fs::read_dir(dir.path())?.flatten() {
                        if sprite_png.path().extension().map(|x| x.to_string_lossy())
                            == Some("png".into())
                        {
                            let sprite_bin_path = sprite_png.path().to_owned();
                            let sprite_bin = sprite_png
                                .path()
                                .with_extension("")
                                .with_extension("bin")
                                .file_name()
                                .unwrap()
                                .to_string_lossy()
                                .into_owned();
                            let sprite_bin_ninja_path = cwd
                                .join("_workspace/unpacked_bins/ninja")
                                .join(&bin_name)
                                .join(&sprite_bin);
                            let sprite_bin_saurian_path = cwd
                                .join("_workspace/unpacked_bins/saurian")
                                .join(&bin_name)
                                .join(&sprite_bin);
                            println!("  - {:?} -> {:?}", sprite_bin_path, sprite_bin_ninja_path);
                            s.spawn(|| {
                                ensure!(tools
                                    .sfspatcher()
                                    .arg("patch")
                                    .arg("--buildin-palette-only")
                                    .arg("true")
                                    .arg("-t")
                                    .arg(sprite_bin_path)
                                    .arg("-i")
                                    .arg(&sprite_bin_ninja_path)
                                    .arg("-o")
                                    .arg(&sprite_bin_ninja_path)
                                    .status()?
                                    .success(),);
                                std::fs::copy(sprite_bin_ninja_path, sprite_bin_saurian_path)?;
                                Ok(())
                            });
                        }
                    }
                }
            }

            Ok(bin_names)
        })?)
        .collect::<Vec<_>>();

    should_repack_bins.dedup();

    std::thread::scope(|s| {
        for bin_name in should_repack_bins {
            let saurian_path = cwd.join("_workspace/unpacked_bins/saurian").join(&bin_name);
            let ninja_path = cwd.join("_workspace/unpacked_bins/ninja").join(&bin_name);
            let saurian_out_path = saurian_temp_path.join("data/datbin/jpn").join(&bin_name);
            let ninja_out_path = ninja_temp_path.join("data/datbin/jpn").join(&bin_name);

            s.spawn(|| {
                ensure!(tools
                    .sfarctool()
                    .arg("-p")
                    .arg("--ignore-zero")
                    .arg("-c")
                    .arg("-i")
                    .arg(saurian_path)
                    .arg("-o")
                    .arg(saurian_out_path)
                    .status()?
                    .success(),);
                Ok(())
            });

            s.spawn(|| {
                ensure!(tools
                    .sfarctool()
                    .arg("-p")
                    .arg("--ignore-zero")
                    .arg("-c")
                    .arg("-i")
                    .arg(ninja_path)
                    .arg("-o")
                    .arg(ninja_out_path)
                    .status()?
                    .success(),);
                Ok(())
            });
        }

        Ok(())
    })?;

    ensure!(tools
        .textpet()
        .arg("Load-Plugins")
        .arg(cwd.join("tools/plugins"))
        .arg("Game")
        .arg("rnr2-cn")
        .arg("Read-Text-Archives")
        .arg(cwd.join("_workspace/mess_tpl"))
        .arg("-f")
        .arg("tpl")
        .arg("Read-Text-Archives")
        .arg(cwd.join("tpl/mess_tpl"))
        .arg("-r")
        .arg("-f")
        .arg("tpl")
        .arg("-p")
        .arg("Write-Text-Archives")
        .arg(&mess_path)
        .arg("-f")
        .arg("msg")
        .status()?
        .success(),);

    ensure!(tools
        .sfarctool()
        .arg("-p")
        .arg("--ignore-zero")
        .arg("-c")
        .arg("-i")
        .arg(&mess_path)
        .arg("-o")
        .arg(temp_path.join("mess.bin"))
        .status()?
        .success(),);

    std::fs::copy(
        images_path.join("splash-screen.ninja.bin"),
        ninja_temp_path.join("data/splash-screen.bin"),
    )?;
    std::fs::copy(
        images_path.join("splash-screen.saurian.bin"),
        saurian_temp_path.join("data/splash-screen.bin"),
    )?;

    std::fs::copy(
        temp_path.join("mess.bin"),
        ninja_temp_path.join("data/datbin/jpn/mess.bin"),
    )?;
    std::fs::copy(
        temp_path.join("mess.bin"),
        saurian_temp_path.join("data/datbin/jpn/mess.bin"),
    )?;

    tools::utils::fs::copy_dir_all(cwd.join("_temp/fonts"), ninja_temp_path.join("data/fonts"))?;
    tools::utils::fs::copy_dir_all(
        cwd.join("_temp/fonts"),
        saurian_temp_path.join("data/fonts"),
    )?;

    ensure!(tools
        .ndstool()
        .arg("-c")
        .arg(build_path.join("ninja.nds"))
        .arg("-9")
        .arg(ninja_temp_path.join("arm9.bin"))
        .arg("-7")
        .arg(workspace_path.join("ninja/arm7.bin"))
        .arg("-y9")
        .arg(workspace_path.join("ninja/y9.bin"))
        .arg("-y7")
        .arg(workspace_path.join("ninja/y7.bin"))
        .arg("-d")
        .arg(ninja_temp_path.join("data"))
        .arg("-y")
        .arg(ninja_temp_path.join("overlay"))
        .arg("-t")
        .arg(workspace_path.join("ninja/banner.bin"))
        .arg("-h")
        .arg(workspace_path.join("ninja/header.bin"))
        .status()?
        .success(),);

    ensure!(tools
        .ndstool()
        .arg("-c")
        .arg(build_path.join("saurian.nds"))
        .arg("-9")
        .arg(saurian_temp_path.join("arm9.bin"))
        .arg("-7")
        .arg(workspace_path.join("saurian/arm7.bin"))
        .arg("-y9")
        .arg(workspace_path.join("saurian/y9.bin"))
        .arg("-y7")
        .arg(workspace_path.join("saurian/y7.bin"))
        .arg("-d")
        .arg(saurian_temp_path.join("data"))
        .arg("-y")
        .arg(saurian_temp_path.join("overlay"))
        .arg("-t")
        .arg(workspace_path.join("saurian/banner.bin"))
        .arg("-h")
        .arg(workspace_path.join("saurian/header.bin"))
        .status()?
        .success(),);

    if std::env::args().any(|x| &x == "--test-release") {
        let test_rel_path = build_path.join("测试版本");
        std::fs::create_dir_all(&test_rel_path)?;
        let mut rel_id = 0;
        loop {
            rel_id += 1;
            let rel_ninja_path =
                test_rel_path.join(format!("流星洛克人2 忍者 测试版本（{rel_id}）.nds"));
            let rel_saurian_path =
                test_rel_path.join(format!("流星洛克人2 恐龙 测试版本（{rel_id}）.nds"));
            if !rel_ninja_path.exists() && !rel_saurian_path.exists() {
                std::fs::copy(build_path.join("ninja.nds"), rel_ninja_path)?;
                std::fs::copy(build_path.join("saurian.nds"), rel_saurian_path)?;
                break;
            }
        }
    }

    if std::env::args().any(|x| &x == "--release") {
        let test_rel_path = build_path.join("正式版本");
        std::fs::create_dir_all(&test_rel_path)?;
        let rel_id = "Ver.2";

        let rel_ninja_path =
            test_rel_path.join(format!("流星洛克人 2 忍者 完全汉化版 {rel_id}.nds"));
        let rel_saurian_path =
            test_rel_path.join(format!("流星洛克人 2 恐龙 完全汉化版 {rel_id}.nds"));
        std::fs::copy(build_path.join("ninja.nds"), &rel_ninja_path)?;
        std::fs::copy(build_path.join("saurian.nds"), &rel_saurian_path)?;

        let rel_ninja_bps_path = rel_ninja_path.with_extension("bps");
        let rel_saurian_bps_path = rel_saurian_path.with_extension("bps");

        fn gen_bps_patch(
            src: impl AsRef<Path>,
            target: impl AsRef<Path>,
            dest: impl AsRef<Path>,
        ) -> anyhow::Result<()> {
            let source = std::fs::read(src)?;
            let target = std::fs::read(target)?;

            let mut bps = flips::BpsDeltaBuilder::new();

            bps.source(source);
            bps.target(target);
            let result = bps.build()?;

            let mut patch_file = std::fs::File::create(dest)?;
            patch_file.write_all(result.as_ref())?;

            Ok(())
        }

        gen_bps_patch(
            rom_path.join("ninja.nds"),
            build_path.join("ninja.nds"),
            rel_ninja_bps_path,
        )?;
        gen_bps_patch(
            rom_path.join("saurian.nds"),
            build_path.join("saurian.nds"),
            rel_saurian_bps_path,
        )?;
    }

    Ok(())
}

use anyhow::*;
use tools::utils::ToolsRunner;

pub fn main() -> anyhow::Result<()> {
    let cwd = std::env::current_dir().unwrap();
    let tools = ToolsRunner::new(Some(&cwd))?;

    let _ = std::fs::remove_dir_all(cwd.join("_workspace"));
    let _ = std::fs::create_dir_all(cwd.join("_workspace"));
    std::fs::create_dir_all(cwd.join("tpl"))?;

    tools
        .ndstool_extract(cwd.join("_rom/ninja.nds"), cwd.join("_workspace/ninja"))
        .context("解包 忍者版 原始游戏文件失败")?;
    tools
        .ndstool_extract(cwd.join("_rom/saurian.nds"), cwd.join("_workspace/saurian"))
        .context("解包 恐龙版 原始游戏文件失败")?;

    std::thread::scope(|s| {
        for entry in std::fs::read_dir(cwd.join("_workspace/ninja/data/datbin/com"))?.flatten() {
            let filename = entry.file_name();
            let filename = filename.to_string_lossy().into_owned();
            if filename.ends_with(".bin") {
                s.spawn(|| {
                    let filename_owned = filename;
                    let unpack_dest = cwd
                        .join("_workspace/unpacked_bins/common")
                        .join(&filename_owned);
                    tools
                        .sfarctool()
                        .arg("-x")
                        .arg("--ignore-zero")
                        .arg("-i")
                        .arg(
                            cwd.join("_workspace/ninja/data/datbin/com")
                                .join(&filename_owned),
                        )
                        .arg("-o")
                        .arg(&unpack_dest)
                        .status()
                        .with_context(|| {
                            format!("解包 忍者版 通用 游戏归档文件 {filename_owned} 失败")
                        })?;
                    Ok(())
                });
            }
        }
        for entry in std::fs::read_dir(cwd.join("_workspace/ninja/data/datbin/jpn"))?.flatten() {
            let filename = entry.file_name();
            let filename = filename.to_string_lossy().into_owned();
            if filename.ends_with(".bin") {
                s.spawn(|| {
                    let filename_owned = filename;
                    let unpack_dest = cwd
                        .join("_workspace/unpacked_bins/ninja")
                        .join(&filename_owned);
                    tools
                        .sfarctool()
                        .arg("-x")
                        .arg("--ignore-zero")
                        .arg("-i")
                        .arg(
                            cwd.join("_workspace/ninja/data/datbin/jpn")
                                .join(&filename_owned),
                        )
                        .arg("-o")
                        .arg(&unpack_dest)
                        .status()
                        .with_context(|| {
                            format!("解包 忍者版 游戏归档文件 {filename_owned} 失败")
                        })?;
                    tools
                        .sfspatcher()
                        .arg("--ignore-errors")
                        .arg("extract")
                        .arg("-i")
                        .arg(&unpack_dest)
                        .arg("-o")
                        .arg(cwd.join("images/sfsprites").join(&filename_owned))
                        .status()
                        .with_context(|| {
                            format!("解包 忍者版 游戏归档文件 {filename_owned} 失败")
                        })?;
                    Ok(())
                });
            }
        }
        for entry in std::fs::read_dir(cwd.join("_workspace/saurian/data/datbin/jpn"))?.flatten() {
            let filename = entry.file_name();
            let filename = filename.to_string_lossy().into_owned();
            if filename.ends_with(".bin") {
                s.spawn(|| {
                    let filename_owned = filename;
                    let unpack_dest = cwd
                        .join("_workspace/unpacked_bins/saurian")
                        .join(&filename_owned);
                    tools
                        .sfarctool()
                        .arg("-x")
                        .arg("--ignore-zero")
                        .arg("-i")
                        .arg(
                            cwd.join("_workspace/saurian/data/datbin/jpn")
                                .join(&filename_owned),
                        )
                        .arg("-o")
                        .arg(&unpack_dest)
                        .status()
                        .with_context(|| {
                            format!("解包 恐龙版 游戏归档文件 {filename_owned} 失败")
                        })?;
                    tools
                        .sfspatcher()
                        .arg("--ignore-errors")
                        .arg("extract")
                        .arg("-i")
                        .arg(&unpack_dest)
                        .arg("-o")
                        .arg(cwd.join("images/sfsprites").join(&filename_owned))
                        .status()
                        .with_context(|| {
                            format!("解包 恐龙版 游戏归档文件 {filename_owned} 失败")
                        })?;
                    Ok(())
                });
            }
        }
        Ok(())
    })?;

    tools::dump_images::dump_images()?;

    tools
        .textpet()
        .arg("Load-Plugins")
        .arg(cwd.join("tools/plugins"))
        .arg("Game")
        .arg("rnr2")
        .arg("Read-Text-Archives")
        .arg(cwd.join("_workspace/unpacked_bins/ninja/mess.bin"))
        .arg("-f")
        .arg("msg")
        .arg("Write-Text-Archives")
        .arg(cwd.join("_workspace/mess_tpl"))
        .arg("-f")
        .arg("tpl")
        .status()
        .context("解包 忍者版 游戏脚本文件失败")?;

    Ok(())
}

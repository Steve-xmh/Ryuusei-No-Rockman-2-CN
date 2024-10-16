use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::ensure;

pub mod fs;
pub mod path;
pub mod tile_img;
pub mod buildin_palette;

pub struct ToolsRunner {
    textpet_path: PathBuf,
    armips_path: PathBuf,
    sfarctool_path: PathBuf,
    sfspatcher_path: PathBuf,
    ndstool_path: PathBuf,
    sfont_gen_path: PathBuf,
    textpet_checker_path: PathBuf,
}

impl ToolsRunner {
    pub fn new(root_path: Option<&Path>) -> anyhow::Result<Self> {
        let root_path = root_path
            .map(|x| x.to_path_buf())
            .unwrap_or_else(|| std::env::current_dir().unwrap());
        let textpet_path = root_path.join("./tools/textpet.exe");
        let armips_path = root_path.join("./tools/armips.exe");
        let sfarctool_path = root_path.join("./tools/sfarctool.exe");
        let sfspatcher_path = root_path.join("./tools/sfspatcher.exe");
        let ndstool_path = root_path.join("./tools/ndstool.exe");
        let sfont_gen_path = root_path.join("./tools/sfont-gen.exe");
        let textpet_checker_path = root_path.join("./tools/textpet-checker.exe");

        if !textpet_path.is_file() {
            anyhow::bail!("无法找到工具 textpet.exe，请检查本程序所在文件夹是否存在 tools 文件夹并存放了对应的工具文件");
        }

        if !armips_path.is_file() {
            anyhow::bail!("无法找到工具 armips.exe，请检查本程序所在文件夹是否存在 tools 文件夹并存放了对应的工具文件");
        }

        if !sfarctool_path.is_file() {
            anyhow::bail!("无法找到工具 sfarctool.exe，请检查本程序所在文件夹是否存在 tools 文件夹并存放了对应的工具文件");
        }

        if !sfspatcher_path.is_file() {
            anyhow::bail!("无法找到工具 sfspatcher.exe，请检查本程序所在文件夹是否存在 tools 文件夹并存放了对应的工具文件");
        }

        if !ndstool_path.is_file() {
            anyhow::bail!("无法找到工具 ndstool.exe，请检查本程序所在文件夹是否存在 tools 文件夹并存放了对应的工具文件");
        }

        if !sfont_gen_path.is_file() {
            anyhow::bail!("无法找到工具 sfont-gen.exe，请检查本程序所在文件夹是否存在 tools 文件夹并存放了对应的工具文件");
        }

        if !textpet_checker_path.is_file() {
            anyhow::bail!("无法找到工具 textpet-checker.exe，请检查本程序所在文件夹是否存在 tools 文件夹并存放了对应的工具文件");
        }

        Ok(Self {
            textpet_path,
            armips_path,
            sfarctool_path,
            sfspatcher_path,
            ndstool_path,
            sfont_gen_path,
            textpet_checker_path,
        })
    }

    pub fn textpet(&self) -> Command {
        Command::new(&self.textpet_path)
    }

    pub fn armips(&self) -> Command {
        Command::new(&self.armips_path)
    }

    pub fn sfarctool(&self) -> Command {
        Command::new(&self.sfarctool_path)
    }

    pub fn sfspatcher(&self) -> Command {
        Command::new(&self.sfspatcher_path)
    }

    pub fn ndstool(&self) -> Command {
        Command::new(&self.ndstool_path)
    }

    pub fn textpet_checker(&self) -> Command {
        Command::new(&self.textpet_checker_path)
    }

    pub fn ndstool_extract(
        &self,
        nds_file: impl AsRef<Path>,
        dir_path: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        let dir_path = dir_path.as_ref();
        std::fs::create_dir_all(dir_path)?;

        let mut cmd = self.ndstool();
        cmd.arg("-x").arg(nds_file.as_ref());
        cmd.arg("-9").arg(dir_path.join("arm9.bin"));
        cmd.arg("-7").arg(dir_path.join("arm7.bin"));
        cmd.arg("-y9").arg(dir_path.join("y9.bin"));
        cmd.arg("-y7").arg(dir_path.join("y7.bin"));
        cmd.arg("-d").arg(dir_path.join("data"));
        cmd.arg("-y").arg(dir_path.join("overlay"));
        cmd.arg("-t").arg(dir_path.join("banner.bin"));
        cmd.arg("-h").arg(dir_path.join("header.bin"));

        ensure!(cmd.status()?.success(), "ndstool 执行解包游戏文件失败");

        Ok(())
    }

    pub fn sfont_gen(&self) -> Command {
        Command::new(&self.sfont_gen_path)
    }
}

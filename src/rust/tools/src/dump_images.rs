use std::io::Write;

use crate::utils::{tile_img::TileImage, ToolsRunner};
use anyhow::*;
use image::GenericImageView;
use sfbase::Tile4BPP;

#[allow(unused)]
trait FileSize {
    fn file_size(&self) -> u64;
}

#[cfg(target_os = "windows")]
impl<T: std::os::windows::fs::MetadataExt> FileSize for T {
    fn file_size(&self) -> u64 {
        self.file_size()
    }
}

#[cfg(target_os = "linux")]
impl<T: std::os::linux::fs::MetadataExt> FileSize for T {
    fn file_size(&self) -> u64 {
        self.st_size()
    }
}

#[cfg(unix)]
impl<T: std::os::unix::fs::MetadataExt> FileSize for T {
    fn file_size(&self) -> u64 {
        self.size()
    }
}

#[cfg(target_os = "wasi")]
impl<T: std::os::wasi::fs::MetadataExt> FileSize for T {
    fn file_size(&self) -> u64 {
        self.size()
    }
}

pub fn dump_images() -> anyhow::Result<()> {
    let cwd = std::env::current_dir().unwrap();
    let _workspace_path = cwd.join("_workspace");
    let _build_path = cwd.join("_build");
    let _images_path = cwd.join("images");
    let temp_path = cwd.join("_temp/pack");
    let _mess_path = temp_path.join("mess");
    let _ninja_temp_path = temp_path.join("ninja");
    let _saurian_temp_path = temp_path.join("saurian");
    let _tools = ToolsRunner::new(Some(&cwd))?;

    let saurian_ccl_path = cwd.join("_workspace/unpacked_bins/saurian/capcomlogo_local.bin");
    let ninja_ccl_path = cwd.join("_workspace/unpacked_bins/ninja/capcomlogo_local.bin");
    let image_path = cwd.join("_workspace/images");
    std::fs::create_dir_all(&image_path)?;

    let dirs = &[(&saurian_ccl_path, "saurian"), (&ninja_ccl_path, "ninja")];

    for (p, n) in dirs {
        let t = TileImage::new()
            .with_tileset(p.join("capcomlogo_local_00.bin"), 1)
            .with_tilemap(p.join("capcomlogo_local_01.bin"), 32, 24)
            .with_palette(p.join("capcomlogo_local_02.bin"))
            .with_output(image_path.join(format!("capcomlogo_local_{n}_0.png")));
        t.read_tileimg()?;

        let t = TileImage::new()
            .with_palette(p.join("capcomlogo_local_07.bin"))
            .with_tileset(p.join("capcomlogo_local_08.bin"), 1)
            .with_tilemap(p.join("capcomlogo_local_09.bin"), 32, 24)
            .with_tileset(p.join("capcomlogo_local_10.bin"), 1)
            .with_tilemap(p.join("capcomlogo_local_11.bin"), 32, 24)
            .with_tileset(p.join("capcomlogo_local_12.bin"), 1)
            .with_tilemap(p.join("capcomlogo_local_13.bin"), 32, 24)
            .with_tileset(p.join("capcomlogo_local_14.bin"), 1)
            .with_tilemap(p.join("capcomlogo_local_15.bin"), 32, 24)
            .with_tileset(p.join("capcomlogo_local_16.bin"), 1)
            .with_tilemap(p.join("capcomlogo_local_17.bin"), 32, 24)
            .with_tileset(p.join("capcomlogo_local_18.bin"), 1)
            .with_tilemap(p.join("capcomlogo_local_19.bin"), 32, 24)
            .with_output(image_path.join(format!("capcomlogo_local_{n}_1.png")));
        t.read_tileimg()?;

        let t = TileImage::new()
            .with_tileset(p.join("capcomlogo_local_22.bin"), 1)
            .with_tilemap(p.join("capcomlogo_local_23.bin"), 32, 48)
            .with_tileset(p.join("capcomlogo_local_24.bin"), 1)
            .with_tilemap(p.join("capcomlogo_local_25.bin"), 32, 48)
            .with_palette(p.join("capcomlogo_local_26.bin"))
            .with_output(image_path.join(format!("capcomlogo_local_{n}_2.png")));
        t.read_tileimg()?;

        let t = TileImage::new()
            .with_tileset(p.join("capcomlogo_local_28.bin"), 1)
            .with_palette(p.join("capcomlogo_local_29.bin"))
            .with_tilemap(p.join("capcomlogo_local_30.bin"), 32, 24)
            .with_output(image_path.join(format!("capcomlogo_local_{n}_3.png")));
        t.read_tileimg()?;

        let t = TileImage::new()
            .with_tileset(p.join("capcomlogo_local_32.bin"), 1)
            .with_palette(p.join("capcomlogo_local_33.bin"))
            .with_tilemap(p.join("capcomlogo_local_34.bin"), 32, 24)
            .with_output(image_path.join(format!("capcomlogo_local_{n}_4.png")));
        t.read_tileimg()?;

        let t = TileImage::new()
            .with_tileset(p.join("capcomlogo_local_35.bin"), 1)
            .with_palette(p.join("capcomlogo_local_36.bin"))
            .with_tilemap(p.join("capcomlogo_local_37.bin"), 32, 24)
            .with_output(image_path.join(format!("capcomlogo_local_{n}_5.png")));
        t.read_tileimg()?;

        let t = TileImage::new()
            .with_tileset(p.join("capcomlogo_local_38.bin"), 1)
            .with_palette(p.join("capcomlogo_local_39.bin"))
            .with_tilemap(p.join("capcomlogo_local_40.bin"), 32, 24)
            .with_output(image_path.join(format!("capcomlogo_local_{n}_6.png")));
        t.read_tileimg()?;
    }

    let saurian_ssl_path = cwd.join("_workspace/unpacked_bins/saurian/subscreen_local.bin");
    let ninja_ssl_path = cwd.join("_workspace/unpacked_bins/ninja/subscreen_local.bin");
    let dirs = &[(&saurian_ssl_path, "saurian"), (&ninja_ssl_path, "ninja")];

    for (p, n) in dirs {
        let t = TileImage::new()
            .with_palette(p.join("subscreen_local_026.bin"))
            .with_tileset(p.join("subscreen_local_027.bin"), 0)
            .with_tilemap(p.join("subscreen_local_028.bin"), 11, 17)
            .with_tilemap(p.join("subscreen_local_029.bin"), 11, 14)
            .with_tilemap(p.join("subscreen_local_030.bin"), 11, 19)
            .with_tilemap(p.join("subscreen_local_031.bin"), 11, 16)
            .with_4bpp()
            .with_output(image_path.join(format!("subscreen_local_{n}_0.png")));
        t.read_tileimg()?;

        let t = TileImage::new()
            .with_palette(p.join("subscreen_local_036.bin"))
            .with_tileset(p.join("subscreen_local_037.bin"), 1)
            .with_tilemap(p.join("subscreen_local_038.bin"), 32, 24)
            .with_tilemap(p.join("subscreen_local_039.bin"), 32, 24)
            .with_tilemap(p.join("subscreen_local_040.bin"), 32, 24)
            .with_tilemap(p.join("subscreen_local_041.bin"), 32, 24)
            .with_tilemap(p.join("subscreen_local_042.bin"), 32, 24)
            .with_output(image_path.join(format!("subscreen_local_{n}_1.png")));
        t.read_tileimg()?;

        let t = TileImage::new()
            .with_palette(p.join("subscreen_local_087.bin"))
            .with_tilemap(p.join("subscreen_local_088.bin"), 32, 24)
            .with_tileset(p.join("subscreen_local_089.bin"), 1)
            .with_output(image_path.join(format!("subscreen_local_{n}_2.png")));
        t.read_tileimg()?;

        let t = TileImage::new()
            .with_palette(p.join("subscreen_local_069.bin"))
            .with_tilemap(p.join("subscreen_local_072.bin"), 12, 5)
            .with_tileset(p.join("subscreen_local_071.bin"), 1)
            .with_4bpp()
            .with_output(image_path.join(format!("subscreen_local_{n}_3.png")));
        t.read_tileimg()?;
    }

    let saurian_rl_path = cwd.join("_workspace/unpacked_bins/saurian/result_local.bin");
    let ninja_rl_path = cwd.join("_workspace/unpacked_bins/ninja/result_local.bin");
    let dirs = &[(&saurian_rl_path, "saurian"), (&ninja_rl_path, "ninja")];

    for (p, n) in dirs {
        let t = TileImage::new()
            .with_palette(p.join("result_local_00.bin"))
            .with_tilemap(p.join("result_local_17.bin"), 32, 8)
            .with_tileset(p.join("result_local_10.bin"), 0)
            .with_4bpp()
            .with_output(image_path.join(format!("result_local_{n}_0.png")));
        t.read_tileimg()?;

        let t = TileImage::new()
            .with_palette(p.join("result_local_00.bin"))
            .with_tilemap(p.join("result_local_21.bin"), 32, 8)
            .with_tileset(p.join("result_local_14.bin"), 0)
            .with_4bpp()
            .with_output(image_path.join(format!("result_local_{n}_1.png")));
        t.read_tileimg()?;
    }

    let ninja_cp_path = cwd.join("_workspace/unpacked_bins/ninja/cockpit_local.bin");

    let tmp_tilemap_path = cwd.join("_temp/cockpit_customing_text_tilemap.bin");

    std::fs::write(&tmp_tilemap_path, [0x00, 0x00, 0x01, 0x00])?;

    for i in 13..=18 {
        let t = TileImage::new()
            .with_tilemap(tmp_tilemap_path.clone(), 1, 2)
            .with_tileset(ninja_cp_path.join(format!("cockpit_local_{i}.bin")), 0)
            .with_4bpp()
            .with_output(image_path.join(format!("cockpit_customing_text_{}.png", i - 13)));
        t.read_tileimg()?;
    }

    // let mut has_space = false;
    // for entry in
    //     std::fs::read_dir(cwd.join("_workspace/unpacked_bins/ninja/subscreen_local.bin"))?.flatten()
    // {
    //     if let std::result::Result::Ok(meta) = entry.metadata() {
    //         if meta.is_file() {
    //             let size = meta.file_size();
    //             let data = std::fs::read(entry.path())?;
    //             if sfsprite::SFSpriteEditor::from_data(&data).is_ok() {
    //                 if !has_space {
    //                     println!();
    //                     has_space = true
    //                 };
    //                 continue;
    //             }
    //             match size / 2 {
    //                 768 => {
    //                     println!(
    //                         "{}: {:<16} 疑似为图块表，大小可能是 32x24",
    //                         entry
    //                             .path()
    //                             .file_name()
    //                             .map(|x| x.to_string_lossy())
    //                             .unwrap_or_default(),
    //                         size,
    //                     );
    //                     has_space = false;
    //                 }
    //                 1536 => {
    //                     println!(
    //                         "{}: {:<16} 疑似为图块表，大小可能是 32x48",
    //                         entry
    //                             .path()
    //                             .file_name()
    //                             .map(|x| x.to_string_lossy())
    //                             .unwrap_or_default(),
    //                         size,
    //                     );
    //                     has_space = false;
    //                 }
    //                 _ => {
    //                     if size % 2 == 0 && size <= 256 * 2 {
    //                         println!(
    //                             "{}: {:<16} 疑似为调色板",
    //                             entry
    //                                 .path()
    //                                 .file_name()
    //                                 .map(|x| x.to_string_lossy())
    //                                 .unwrap_or_default(),
    //                             size,
    //                         );
    //                         has_space = false;
    //                     } else if size % (8 * 8) == 0 {
    //                         println!(
    //                             "{}: {:<16} 疑似为 8BPP 图块集，可能有 {} 个图块",
    //                             entry
    //                                 .path()
    //                                 .file_name()
    //                                 .map(|x| x.to_string_lossy())
    //                                 .unwrap_or_default(),
    //                             size,
    //                             size / (8 * 8)
    //                         );
    //                         has_space = false;
    //                     } else if size % (8 * 8 / 2) == 0 {
    //                         println!(
    //                             "{}: {:<16} 疑似为 4BPP 图块集，可能有 {} 个图块",
    //                             entry
    //                                 .path()
    //                                 .file_name()
    //                                 .map(|x| x.to_string_lossy())
    //                                 .unwrap_or_default(),
    //                             size,
    //                             size / (8 * 8 / 2)
    //                         );
    //                         has_space = false;
    //                     } else if !has_space {
    //                         println!();
    //                         has_space = true
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    Ok(())
}

pub fn save_images() -> anyhow::Result<()> {
    let cwd = std::env::current_dir().unwrap();
    let _workspace_path = cwd.join("_workspace");
    let _build_path = cwd.join("_build");
    let images_path = cwd.join("images");
    let temp_path = cwd.join("_temp/pack");
    let _mess_path = temp_path.join("mess");
    let _ninja_temp_path = temp_path.join("ninja");
    let _saurian_temp_path = temp_path.join("saurian");
    let _tools = ToolsRunner::new(Some(&cwd))?;

    let saurian_ccl_path = cwd.join("_workspace/unpacked_bins/saurian/capcomlogo_local.bin");
    let ninja_ccl_path = cwd.join("_workspace/unpacked_bins/ninja/capcomlogo_local.bin");
    let t = TileImage::new()
        .with_tileset(ninja_ccl_path.join("capcomlogo_local_22.bin"), 1)
        .with_tilemap(ninja_ccl_path.join("capcomlogo_local_23.bin"), 32, 48)
        .with_tileset(ninja_ccl_path.join("capcomlogo_local_24.bin"), 1)
        .with_tilemap(ninja_ccl_path.join("capcomlogo_local_25.bin"), 32, 48)
        .with_palette(ninja_ccl_path.join("capcomlogo_local_26.bin"))
        .with_output(images_path.join("capcomlogo_local_ninja_0.png"));
    t.save_tileimg()?;

    let t = TileImage::new()
        .with_tileset(saurian_ccl_path.join("capcomlogo_local_22.bin"), 1)
        .with_tilemap(saurian_ccl_path.join("capcomlogo_local_23.bin"), 32, 48)
        .with_tileset(saurian_ccl_path.join("capcomlogo_local_24.bin"), 1)
        .with_tilemap(saurian_ccl_path.join("capcomlogo_local_25.bin"), 32, 48)
        .with_palette(saurian_ccl_path.join("capcomlogo_local_26.bin"))
        .with_output(images_path.join("capcomlogo_local_saurian_0.png"));
    t.save_tileimg()?;

    let t = TileImage::new()
        .with_tileset(ninja_ccl_path.join("capcomlogo_local_00.bin"), 1)
        .with_tilemap(ninja_ccl_path.join("capcomlogo_local_01.bin"), 32, 24)
        .with_palette(ninja_ccl_path.join("capcomlogo_local_02.bin"))
        .with_output(images_path.join("capcomlogo_local_ninja_1.png"));
    t.save_tileimg()?;

    let t = TileImage::new()
        .with_tileset(saurian_ccl_path.join("capcomlogo_local_00.bin"), 1)
        .with_tilemap(saurian_ccl_path.join("capcomlogo_local_01.bin"), 32, 24)
        .with_palette(saurian_ccl_path.join("capcomlogo_local_02.bin"))
        .with_output(images_path.join("capcomlogo_local_saurian_1.png"));
    t.save_tileimg()?;

    let p = ninja_ccl_path.to_owned();
    let t = TileImage::new()
        .with_palette(p.join("capcomlogo_local_07.bin"))
        .with_tileset(p.join("capcomlogo_local_08.bin"), 1)
        .with_tilemap(p.join("capcomlogo_local_09.bin"), 32, 24)
        .with_tileset(p.join("capcomlogo_local_10.bin"), 1)
        .with_tilemap(p.join("capcomlogo_local_11.bin"), 32, 24)
        .with_tileset(p.join("capcomlogo_local_12.bin"), 1)
        .with_tilemap(p.join("capcomlogo_local_13.bin"), 32, 24)
        .with_tileset(p.join("capcomlogo_local_14.bin"), 1)
        .with_tilemap(p.join("capcomlogo_local_15.bin"), 32, 24)
        .with_tileset(p.join("capcomlogo_local_16.bin"), 1)
        .with_tilemap(p.join("capcomlogo_local_17.bin"), 32, 24)
        .with_tileset(p.join("capcomlogo_local_18.bin"), 1)
        .with_tilemap(p.join("capcomlogo_local_19.bin"), 32, 24)
        .with_output(images_path.join("capcomlogo_local_achivements.png"));
    t.save_tileimg()?;

    // #[rustfmt::skip]
    {
        std::fs::copy(
            p.join("capcomlogo_local_07.bin"),
            saurian_ccl_path.join("capcomlogo_local_07.bin"),
        )?;
        std::fs::copy(
            p.join("capcomlogo_local_08.bin"),
            saurian_ccl_path.join("capcomlogo_local_08.bin"),
        )?;
        std::fs::copy(
            p.join("capcomlogo_local_09.bin"),
            saurian_ccl_path.join("capcomlogo_local_09.bin"),
        )?;
        std::fs::copy(
            p.join("capcomlogo_local_10.bin"),
            saurian_ccl_path.join("capcomlogo_local_10.bin"),
        )?;
        std::fs::copy(
            p.join("capcomlogo_local_11.bin"),
            saurian_ccl_path.join("capcomlogo_local_11.bin"),
        )?;
        std::fs::copy(
            p.join("capcomlogo_local_12.bin"),
            saurian_ccl_path.join("capcomlogo_local_12.bin"),
        )?;
        std::fs::copy(
            p.join("capcomlogo_local_13.bin"),
            saurian_ccl_path.join("capcomlogo_local_13.bin"),
        )?;
        std::fs::copy(
            p.join("capcomlogo_local_14.bin"),
            saurian_ccl_path.join("capcomlogo_local_14.bin"),
        )?;
        std::fs::copy(
            p.join("capcomlogo_local_15.bin"),
            saurian_ccl_path.join("capcomlogo_local_15.bin"),
        )?;
        std::fs::copy(
            p.join("capcomlogo_local_16.bin"),
            saurian_ccl_path.join("capcomlogo_local_16.bin"),
        )?;
        std::fs::copy(
            p.join("capcomlogo_local_17.bin"),
            saurian_ccl_path.join("capcomlogo_local_17.bin"),
        )?;
        std::fs::copy(
            p.join("capcomlogo_local_18.bin"),
            saurian_ccl_path.join("capcomlogo_local_18.bin"),
        )?;
        std::fs::copy(
            p.join("capcomlogo_local_19.bin"),
            saurian_ccl_path.join("capcomlogo_local_19.bin"),
        )?;
    }

    let saurian_ssl_path = cwd.join("_workspace/unpacked_bins/saurian/subscreen_local.bin");
    let ninja_ssl_path = cwd.join("_workspace/unpacked_bins/ninja/subscreen_local.bin");
    let p = ninja_ssl_path.to_owned();

    let t = TileImage::new()
        .with_palette(images_path.join("subscreen_local_026.bin")) // 将第一个颜色修改成了透明色，即黑色
        .with_tileset(p.join("subscreen_local_027.bin"), 0)
        .with_tilemap(p.join("subscreen_local_028.bin"), 11, 17)
        .with_tilemap(p.join("subscreen_local_029.bin"), 11, 14)
        .with_tilemap(p.join("subscreen_local_030.bin"), 11, 19)
        .with_tilemap(p.join("subscreen_local_031.bin"), 11, 16)
        .with_palette_reference()
        .with_4bpp()
        .with_output(images_path.join("subscreen_local_sort.png"));
    t.save_tileimg()?;

    // #[rustfmt::skip]
    {
        std::fs::copy(
            p.join("subscreen_local_027.bin"),
            saurian_ssl_path.join("subscreen_local_027.bin"),
        )?;
        std::fs::copy(
            p.join("subscreen_local_028.bin"),
            saurian_ssl_path.join("subscreen_local_028.bin"),
        )?;
        std::fs::copy(
            p.join("subscreen_local_029.bin"),
            saurian_ssl_path.join("subscreen_local_029.bin"),
        )?;
        std::fs::copy(
            p.join("subscreen_local_030.bin"),
            saurian_ssl_path.join("subscreen_local_030.bin"),
        )?;
        std::fs::copy(
            p.join("subscreen_local_031.bin"),
            saurian_ssl_path.join("subscreen_local_031.bin"),
        )?;
    }

    let t = TileImage::new()
        .with_palette(p.join("subscreen_local_036.bin"))
        .with_tileset(p.join("subscreen_local_037.bin"), 1)
        .with_tilemap(p.join("subscreen_local_038.bin"), 32, 24)
        .with_tilemap(p.join("subscreen_local_039.bin"), 32, 24)
        .with_tilemap(p.join("subscreen_local_040.bin"), 32, 24)
        .with_tilemap(p.join("subscreen_local_041.bin"), 32, 24)
        .with_tilemap(p.join("subscreen_local_042.bin"), 32, 24)
        .with_palette_reference()
        .with_output(images_path.join("subscreen_local_input.png"));
    t.save_tileimg()?;

    // #[rustfmt::skip]
    {
        std::fs::copy(
            p.join("subscreen_local_037.bin"),
            saurian_ssl_path.join("subscreen_local_037.bin"),
        )?;
        std::fs::copy(
            p.join("subscreen_local_038.bin"),
            saurian_ssl_path.join("subscreen_local_038.bin"),
        )?;
        std::fs::copy(
            p.join("subscreen_local_039.bin"),
            saurian_ssl_path.join("subscreen_local_039.bin"),
        )?;
        std::fs::copy(
            p.join("subscreen_local_040.bin"),
            saurian_ssl_path.join("subscreen_local_040.bin"),
        )?;
        std::fs::copy(
            p.join("subscreen_local_041.bin"),
            saurian_ssl_path.join("subscreen_local_041.bin"),
        )?;
        std::fs::copy(
            p.join("subscreen_local_042.bin"),
            saurian_ssl_path.join("subscreen_local_042.bin"),
        )?;
    }

    let t = TileImage::new()
        .with_palette(p.join("subscreen_local_087.bin"))
        .with_tilemap(p.join("subscreen_local_088.bin"), 32, 24)
        .with_tileset(p.join("subscreen_local_089.bin"), 1)
        .with_palette_reference()
        .with_output(images_path.join("subscreen_local_shop.png"));
    t.save_tileimg()?;

    // #[rustfmt::skip]
    {
        std::fs::copy(
            p.join("subscreen_local_088.bin"),
            saurian_ssl_path.join("subscreen_local_088.bin"),
        )?;
        std::fs::copy(
            p.join("subscreen_local_089.bin"),
            saurian_ssl_path.join("subscreen_local_089.bin"),
        )?;
    }

    let t = TileImage::new()
        .with_palette(p.join("subscreen_local_069.bin"))
        .with_tilemap(p.join("subscreen_local_072.bin"), 12, 5)
        .with_tileset(p.join("subscreen_local_071.bin"), 1)
        .with_palette_reference()
        .with_4bpp()
        .with_output(images_path.join("subscreen_local_mailer_list.png"));
    t.save_tileimg()?;

    {
        std::fs::copy(
            p.join("subscreen_local_072.bin"),
            saurian_ssl_path.join("subscreen_local_072.bin"),
        )?;
        std::fs::copy(
            p.join("subscreen_local_071.bin"),
            saurian_ssl_path.join("subscreen_local_071.bin"),
        )?;
    }

    let saurian_rl_path = cwd.join("_workspace/unpacked_bins/saurian/result_local.bin");
    let ninja_rl_path = cwd.join("_workspace/unpacked_bins/ninja/result_local.bin");
    let p = ninja_rl_path.to_owned();

    let t = TileImage::new()
        .with_palette(p.join("result_local_00.bin"))
        .with_tilemap(p.join("result_local_17.bin"), 32, 8)
        .with_tileset(p.join("result_local_10.bin"), 0)
        .with_palette_reference()
        .with_4bpp()
        .with_output(images_path.join("result_local_delete_panel.png"));
    t.save_tileimg()?;

    {
        std::fs::copy(
            p.join("result_local_17.bin"),
            saurian_rl_path.join("result_local_17.bin"),
        )?;
        std::fs::copy(
            p.join("result_local_10.bin"),
            saurian_rl_path.join("result_local_10.bin"),
        )?;
    }

    let t = TileImage::new()
        .with_palette(p.join("result_local_00.bin"))
        .with_tilemap(p.join("result_local_21.bin"), 32, 8)
        .with_tileset(p.join("result_local_14.bin"), 0)
        .with_palette_reference()
        .with_4bpp()
        .with_output(images_path.join("result_local_loser_delete_panel.png"));
    t.save_tileimg()?;

    {
        std::fs::copy(
            p.join("result_local_21.bin"),
            saurian_rl_path.join("result_local_21.bin"),
        )?;
        std::fs::copy(
            p.join("result_local_14.bin"),
            saurian_rl_path.join("result_local_14.bin"),
        )?;
    }

    let ninja_cp_path = cwd.join("_workspace/unpacked_bins/ninja/cockpit_local.bin");
    let saurian_cp_path = cwd.join("_workspace/unpacked_bins/saurian/cockpit_local.bin");

    let cockpit_customing_text = image::open(images_path.join("cockpit_customing_text.png"))?;

    let mut t = Tile4BPP::new();
    for i in 0..6 {
        let mut out =
            std::fs::File::create(ninja_cp_path.join(format!("cockpit_local_{}.bin", i + 13)))?;
        let croped = cockpit_customing_text.crop_imm(i * 8, 0, 8, 16);
        let get_p = |x: usize, y: usize| -> u8 {
            let pixel = croped.get_pixel(x as u32, y as u32);
            // a80840 3010e8
            if pixel.0 == [0xA8, 0x08, 0x40, 0xFF] {
                2
            } else if pixel.0 == [0x30, 0x10, 0xE8, 0xFF] {
                1
            } else {
                0
            }
        };
        t.fill(0);
        for y in 0..8 {
            for x in 0..8 {
                let pixel = get_p(x, y);
                t.set_pixel(x, y, pixel);
            }
        }
        out.write_all(t.as_raw())?;
        t.fill(0);
        for y in 0..8 {
            for x in 0..8 {
                let pixel = get_p(x, y + 8);
                t.set_pixel(x, y, pixel);
            }
        }
        out.write_all(t.as_raw())?;
        drop(out);
        std::fs::copy(
            ninja_cp_path.join(format!("cockpit_local_{}.bin", i + 13)),
            saurian_cp_path.join(format!("cockpit_local_{}.bin", i + 13)),
        )?;
    }

    Ok(())
}

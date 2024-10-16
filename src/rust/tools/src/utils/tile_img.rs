use std::{io::Write, path::PathBuf};

use anyhow::ensure;
use image::RgbImage;
use serde::{Deserialize, Serialize};
use sfbase::{GBAColor, Tile, Tile4BPP, Tile8BPP};

use super::buildin_palette::BUILDIN_COLOR_PALETTES;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TilesetMeta {
    pub input_file: PathBuf,
    pub output_file: PathBuf,
    pub base_tile: usize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TilemapMeta {
    pub input_file: PathBuf,
    pub output_file: PathBuf,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TileImage {
    pub tileset: Vec<TilesetMeta>,
    pub tilemap: Vec<TilemapMeta>,
    pub first_color_pos: (usize, usize),
    pub palette: Option<PathBuf>,
    // 将原色表作为参考使用，而不从图块集中提取
    pub palette_reference: bool,
    // 当颜色不存在时使用的默认颜色
    pub default_color_index: Option<u8>,
    pub output: PathBuf,
    pub is_4bpp: bool,
}

impl TileImage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_4bpp(mut self) -> Self {
        self.is_4bpp = true;
        self
    }

    pub fn with_8bpp(mut self) -> Self {
        self.is_4bpp = false;
        self
    }

    pub fn with_first_color_pos(mut self, x: usize, y: usize) -> Self {
        self.first_color_pos = (x, y);
        self
    }

    pub fn with_palette_reference(mut self) -> Self {
        self.palette_reference = true;
        self
    }

    pub fn with_default_color_index(mut self, index: u8) -> Self {
        self.default_color_index = Some(index);
        self
    }

    pub fn with_tileset(
        mut self,
        input_file: impl AsRef<std::path::Path>,
        // output_file: impl AsRef<std::path::Path>,
        base_tile: usize,
    ) -> Self {
        self.tileset.push(TilesetMeta {
            input_file: input_file.as_ref().to_path_buf(),
            output_file: input_file.as_ref().to_path_buf(),
            base_tile,
        });
        self
    }

    pub fn with_tilemap(
        mut self,
        input_file: impl AsRef<std::path::Path>,
        // output_file: impl AsRef<std::path::Path>,
        width: usize,
        height: usize,
    ) -> Self {
        self.tilemap.push(TilemapMeta {
            input_file: input_file.as_ref().to_path_buf(),
            output_file: input_file.as_ref().to_path_buf(),
            width,
            height,
        });
        self
    }

    pub fn with_output(mut self, output: impl AsRef<std::path::Path>) -> Self {
        self.output = output.as_ref().to_path_buf();
        self
    }

    pub fn with_palette(mut self, palette: impl AsRef<std::path::Path>) -> Self {
        self.palette = Some(palette.as_ref().to_path_buf());
        self
    }

    pub fn save_meta(&self, output: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        let output = std::fs::File::create(output.as_ref())?;
        serde_yaml::to_writer(output, self)?;
        Ok(())
    }

    pub fn from_meta(input: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let input = std::fs::File::open(input.as_ref())?;
        let meta: Self = serde_yaml::from_reader(input)?;
        Ok(meta)
    }

    fn verify_tilemap_tileset_len(&self) -> anyhow::Result<()> {
        match (self.tilemap.len(), self.tileset.len()) {
            (1, _) => {}
            (_, 1) => {}
            (a, b) => {
                if a != b {
                    anyhow::bail!("tilemap and tileset count mismatch: {} != {}", a, b);
                }
            }
        }
        Ok(())
    }

    fn verify_input_meta(&self) -> anyhow::Result<()> {
        anyhow::ensure!(!self.tileset.is_empty(), "tileset is empty",);
        anyhow::ensure!(!self.tilemap.is_empty(), "tilemap meta is empty");
        anyhow::ensure!(
            self.palette.as_ref().map(|x| x.is_file()).unwrap_or(true),
            "palette path {:?} does not exist",
            self.palette
        );

        self.verify_tilemap_tileset_len()?;

        for set in &self.tileset {
            anyhow::ensure!(
                set.input_file.is_file(),
                "tileset {} does not exist",
                set.input_file.display()
            );
        }

        for map in &self.tilemap {
            anyhow::ensure!(
                map.input_file.is_file(),
                "tilemap {} does not exist",
                map.input_file.display()
            );
        }

        Ok(())
    }

    fn verify_output_meta(&self) -> anyhow::Result<()> {
        anyhow::ensure!(!self.tilemap.is_empty(), "tilemap meta is empty");

        self.verify_tilemap_tileset_len()?;

        anyhow::ensure!(
            self.output.is_file(),
            "output {} does not exist",
            self.output.display()
        );

        Ok(())
    }

    fn get_image_size(&self) -> (usize, usize) {
        let mut width = 0;
        let mut height = 0;
        for map in &self.tilemap {
            width = width.max(map.width * 8);
            height += map.height * 8;
        }
        (width, height)
    }

    fn read_palette(&self) -> anyhow::Result<Vec<GBAColor>> {
        if let Some(palette_path) = &self.palette {
            println!("Reading palette {}", palette_path.display());
            let palette = std::fs::read(palette_path.as_path())?;
            let palette = palette
                .chunks_exact(2)
                .map(|x| GBAColor::from_raw(u16::from_le_bytes([x[0], x[1]])))
                .collect::<Vec<_>>();
            Ok(palette)
        } else {
            Ok(BUILDIN_COLOR_PALETTES.to_vec())
        }
    }

    fn read_tileimg_inner<T: Tile>(&self) -> anyhow::Result<()> {
        let (img_width, img_height) = self.get_image_size();
        let mut img = RgbImage::new(img_width as _, img_height as _);

        let palette = self.read_palette()?;
        println!("Read {} colors", palette.len());

        let mut tilesets = Vec::new();
        for set in &self.tileset {
            println!("Reading tileset {}", set.input_file.display());
            let raw_tileset = std::fs::read(set.input_file.as_path())?;
            let mut tileset = Vec::with_capacity(raw_tileset.len() / 8 / 8);
            for raw_tile in raw_tileset.chunks_exact(T::DATA_SIZE) {
                tileset.push(T::from_raw(raw_tile.to_vec()));
            }
            println!("Read {} tiles", tileset.len());
            tilesets.push(tileset);
        }

        let mut draw_tile = |tile: &T, gx: usize, gy: usize| {
            for y in 0..8 {
                for x in 0..8 {
                    let color = palette
                        .get(tile.get_pixel(x, y) as usize)
                        .copied()
                        .unwrap_or_default();
                    img.put_pixel((gx + x) as _, (gy + y) as _, color.into());
                }
            }
        };

        // 共用调色板，但是图块集和图块表分别存放
        if self.tilemap.len() == self.tileset.len() {
            let mut global_y = 0;
            for ((tilemap_meta, tileset), tileset_meta) in self
                .tilemap
                .iter()
                .zip(tilesets.iter())
                .zip(self.tileset.iter())
            {
                println!("Reading tilemap {}", tilemap_meta.input_file.display());
                let raw_tilemap = std::fs::read(tilemap_meta.input_file.as_path())?;
                let mut tilemap = Vec::with_capacity(raw_tilemap.len() / 2);
                for raw_tile in raw_tilemap.chunks_exact(2) {
                    tilemap.push(u16::from_le_bytes([raw_tile[0], raw_tile[1]]));
                }
                println!("Read {} tiles", tilemap.len());

                let mut x = 0;
                for tile in tilemap {
                    if let Some(tile) =
                        tileset.get((tile as usize).wrapping_sub(tileset_meta.base_tile))
                    {
                        draw_tile(tile, x * 8, global_y * 8);
                    }
                    x += 1;
                    if x >= tilemap_meta.width {
                        x = 0;
                        global_y += 1;
                    }
                }
            }
        } else if self.tileset.len() == 1 {
            let mut global_y = 0;
            let tileset = &tilesets[0];
            let tileset_meta = &self.tileset[0];
            for tilemap_meta in self.tilemap.iter() {
                println!("Reading tilemap {}", tilemap_meta.input_file.display());
                let raw_tilemap = std::fs::read(tilemap_meta.input_file.as_path())?;
                let mut tilemap = Vec::with_capacity(raw_tilemap.len() / 2);
                for raw_tile in raw_tilemap.chunks_exact(2) {
                    tilemap.push(u16::from_le_bytes([raw_tile[0], raw_tile[1]]));
                }
                println!("Read {} tiles", tilemap.len());

                let mut x = 0;
                for tile in tilemap {
                    if tile as usize >= tileset_meta.base_tile {
                        if let Some(tile) =
                            tileset.get((tile as usize).wrapping_sub(tileset_meta.base_tile))
                        {
                            draw_tile(tile, x * 8, global_y * 8);
                        }
                    }
                    x += 1;
                    if x >= tilemap_meta.width {
                        x = 0;
                        global_y += 1;
                    }
                }
            }
        } else {
            anyhow::bail!(
                "not implemented for this case (tilemap {} with tileset {})",
                self.tilemap.len(),
                self.tileset.len()
            );
        }

        img.save(&self.output)?;

        Ok(())
    }

    pub fn read_tileimg(&self) -> anyhow::Result<()> {
        self.verify_input_meta()?;

        if self.is_4bpp {
            self.read_tileimg_inner::<Tile4BPP>()
        } else {
            self.read_tileimg_inner::<Tile8BPP>()
        }
    }

    fn save_tileimg_inner<T: Tile>(&self) -> anyhow::Result<()> {
        let (img_width, img_height) = self.get_image_size();
        let img = image::open(&self.output)?.into_rgb8();

        ensure!(
            img.width() == img_width as u32,
            "image width is not {}",
            img_width
        );
        ensure!(
            img.height() == img_height as u32,
            "image height is not {}",
            img_height
        );

        let palette = if self.palette_reference {
            self.read_palette()?
                .into_iter()
                .enumerate()
                .map(|x| (x.1, x.0))
                .collect::<Vec<_>>()
        } else {
            let first_color_pixel = GBAColor::from(
                img.get_pixel(self.first_color_pos.0 as _, self.first_color_pos.1 as _),
            );
            let mut palette = Vec::<(GBAColor, usize)>::with_capacity(256);
            for p in img.pixels() {
                let p = GBAColor::from(p);
                if let Some(p) = palette.iter_mut().find(|x| x.0 == p) {
                    p.1 = p.1.wrapping_add(1);
                } else if first_color_pixel == p {
                    palette.push((p, usize::MAX));
                } else {
                    palette.push((p, 1));
                }
            }
            println!("palette size: {}", palette.len());
            ensure!(
                palette.len() <= 256,
                "palette size should not be more than 256"
            );
            ensure!(self.palette.is_some(), "palette path is not set");

            let palette_path = self.palette.as_ref().unwrap();

            palette.sort_by(|a, b| b.1.cmp(&a.1));

            let mut palette_output = std::fs::File::create(palette_path)?;

            for c in &palette {
                palette_output.write_all(&c.0.to_le_bytes())?;
            }

            palette_output.sync_all()?;
            palette
        };

        let empty_tile = T::default();

        if self.tilemap.len() == self.tileset.len() {
            let mut global_y = 0;
            for (tilemap_meta, tileset_meta) in self.tilemap.iter().zip(self.tileset.iter()) {
                let mut tileset = Vec::with_capacity(tilemap_meta.width * tilemap_meta.height);
                let mut tilemap: Vec<u16> =
                    Vec::with_capacity(tilemap_meta.width * tilemap_meta.height);

                for ty in 0..tilemap_meta.height {
                    for tx in 0..tilemap_meta.width {
                        let mut tile = T::default();
                        for y in 0..8 {
                            for x in 0..8 {
                                let px = tx * 8 + x;
                                let py = ty * 8 + y;
                                let p =
                                    GBAColor::from(img.get_pixel(px as _, (py + global_y) as _));
                                let pv = palette.iter().position(|x| x.0 == p);
                                match pv {
                                    Some(pv) => {
                                        tile.set_pixel(x, y, pv as u8);
                                    }
                                    None => {
                                        if let Some(default_color_index) = self.default_color_index
                                        {
                                            tile.set_pixel(x, y, default_color_index);
                                        } else {
                                            anyhow::bail!("无法获取位于 ({},{}) 的像素颜色值 {:?} 在调色板中的位置", px, py + global_y, p);
                                        }
                                    }
                                }
                            }
                        }
                        if tile == empty_tile && tileset_meta.base_tile > 0 {
                            tilemap.push(0);
                        } else if let Some(pos) = tileset.iter().position(|x| x == &tile) {
                            tilemap.push((pos + tileset_meta.base_tile) as u16);
                        } else {
                            tilemap.push((tileset.len() + tileset_meta.base_tile) as u16);
                            tileset.push(tile);
                        }
                    }
                }

                let mut tilemap_output = std::fs::File::create(&tilemap_meta.input_file)?;
                for tile in tilemap {
                    tilemap_output.write_all(&tile.to_le_bytes())?;
                }

                let mut tileset_output = std::fs::File::create(&tileset_meta.input_file)?;
                for tile in tileset {
                    tileset_output.write_all(tile.as_raw())?;
                }

                tilemap_output.sync_all()?;
                tileset_output.sync_all()?;

                global_y += tilemap_meta.height * 8;
            }
        } else if self.tileset.len() == 1 {
            let mut global_y = 0;
            let tileset_meta = &self.tileset[0];
            let mut tileset = Vec::with_capacity(256);

            for tilemap_meta in &self.tilemap {
                let mut tilemap: Vec<u16> = Vec::with_capacity(256);

                for ty in 0..tilemap_meta.height {
                    for tx in 0..tilemap_meta.width {
                        let mut tile = T::default();
                        for y in 0..8 {
                            for x in 0..8 {
                                let px = tx * 8 + x;
                                let py = ty * 8 + y;
                                let p =
                                    GBAColor::from(img.get_pixel(px as _, (py + global_y) as _));
                                let pv = palette.iter().position(|x| x.0 == p);
                                match pv {
                                    Some(pv) => {
                                        tile.set_pixel(x, y, pv as u8);
                                    }
                                    None => {
                                        if let Some(default_color_index) = self.default_color_index
                                        {
                                            tile.set_pixel(x, y, default_color_index);
                                        } else {
                                            anyhow::bail!("无法获取位于 ({},{}) 的像素颜色值 {:?} 在调色板中的位置", px, py + global_y, p);
                                        }
                                    }
                                }
                            }
                        }
                        if tile == empty_tile && tileset_meta.base_tile > 0 {
                            tilemap.push(0);
                        } else if let Some(pos) = tileset.iter().position(|x| x == &tile) {
                            tilemap.push((pos + tileset_meta.base_tile) as u16);
                        } else {
                            tilemap.push((tileset.len() + tileset_meta.base_tile) as u16);
                            tileset.push(tile);
                        }
                    }
                }

                let mut tilemap_output = std::fs::File::create(&tilemap_meta.input_file)?;
                for tile in tilemap {
                    tilemap_output.write_all(&tile.to_le_bytes())?;
                }
                tilemap_output.sync_all()?;

                global_y += tilemap_meta.height * 8;
            }

            let mut tileset_output = std::fs::File::create(&self.tileset[0].input_file)?;
            for tile in tileset {
                tileset_output.write_all(tile.as_raw())?;
            }

            tileset_output.sync_all()?;
        } else {
            anyhow::bail!(
                "not implemented for this case (tilemap {} with tileset {})",
                self.tilemap.len(),
                self.tileset.len()
            );
        }

        Ok(())
    }

    pub fn save_tileimg(&self) -> anyhow::Result<()> {
        self.verify_output_meta()?;

        if self.is_4bpp {
            self.save_tileimg_inner::<Tile4BPP>()
        } else {
            self.save_tileimg_inner::<Tile8BPP>()
        }
    }
}

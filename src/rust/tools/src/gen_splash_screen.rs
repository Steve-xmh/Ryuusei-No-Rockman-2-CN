use std::{
    collections::HashMap,
    io::{Read, Seek, Write},
    path::Path,
    time::Instant,
};

use anyhow::*;
use rscolorq::*;
use sfbase::{GBAColor, Tile8BPP};

fn get_file_md5(input: impl AsRef<Path>) -> anyhow::Result<String> {
    let mut buf = [0u8; 1024];
    let mut ctx = md5::Context::new();
    let mut file = std::fs::File::open(input)?;
    loop {
        let len = file.read(&mut buf)?;
        if len == 0 {
            break;
        }
        ctx.consume(&buf[..len]);
    }
    Ok(format!("{:?}", ctx.compute()))
}

fn process_splash_screen_image_to_tiles(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let input = input.as_ref();
    let output = output.as_ref();
    let input_md5_cache = input.with_extension(format!(
        "{}.md5",
        input.extension().unwrap_or_default().to_string_lossy()
    ));
    let patch_output = dbg!(input.with_extension(format!(
        "patch.{}",
        input.extension().unwrap_or_default().to_string_lossy()
    )));

    // let should_generate_patch = {
    //     let before_hash = std::fs::read_to_string(&input_md5_cache).unwrap_or_default();
    //     let now_hash = get_file_md5(input)?;
    //     std::fs::write(&input_md5_cache, &now_hash)?;
    //     !patch_output.is_file() || now_hash != before_hash
    // };
    let should_generate_patch = false;

    if should_generate_patch {
        let img = image::open(input)?;
        ensure!(img.width() == 256, "image width is not 256");
        ensure!(img.height() == 384, "image height is not 384");
        let mut img = img.into_rgb8();

        let mut result = Matrix2d::new(img.width() as _, img.height() as _);
        let mut conditions = Params::new();
        let palette_size = 64;
        conditions.palette_size(palette_size);
        conditions.dithering_level_auto(img.width(), img.height(), palette_size as _);
        conditions.filter_size(FilterSize::Three);
        conditions.verify_parameters()?;

        let input_img = Matrix2d::from_vec(
            img.enumerate_pixels()
                .map(|x| color::Rgb {
                    red: x.2 .0[0] as f64 / 255.0,
                    green: x.2 .0[1] as f64 / 255.0,
                    blue: x.2 .0[2] as f64 / 255.0,
                })
                .collect(),
            img.width() as _,
            img.height() as _,
        );

        let mut palette = Vec::with_capacity(u8::MAX as _);

        println!("processing image {}", input.to_string_lossy());
        let t = Instant::now();
        rscolorq::spatial_color_quant(&input_img, &mut result, &mut palette, &conditions)?;
        println!(
            "processed image {} in {:?}",
            input.to_string_lossy(),
            t.elapsed()
        );

        let palette = palette
            .into_iter()
            .map(|c| {
                image::Rgb([
                    (c.red * u8::MAX as f64) as u8,
                    (c.green * u8::MAX as f64) as u8,
                    (c.blue * u8::MAX as f64) as u8,
                ])
            })
            .collect::<Vec<_>>();

        for (i, &c) in result.iter().enumerate() {
            let c = palette
                .get(c as usize)
                .context("Could not retrieve color from palette")?;
            let x = i as u32 % img.width();
            let y = i as u32 / img.width();
            img.put_pixel(x, y, *c);
        }
        println!("output image {}", patch_output.to_string_lossy());
        img.save(&patch_output)?;
    } else {
        println!("skipped processing image {}", input.to_string_lossy());
    }

    println!("generating palette");
    let img = image::open(&patch_output)?.into_rgb8();
    ensure!(img.width() == 256, "image width is not 256");
    ensure!(img.height() == 384, "image height is not 384");
    let mut palettes = Vec::<(GBAColor, usize)>::with_capacity(256);
    for p in img.pixels() {
        let p = GBAColor::from(p);
        if let Some(p) = palettes.iter_mut().find(|x| x.0 == p) {
            p.1 += 1;
        } else {
            palettes.push((p, 1));
        }
    }
    println!("palette size: {}", palettes.len());

    palettes.sort_by(|a, b| b.1.cmp(&a.1));

    ensure!(
        palettes.len() <= 256,
        "palette size should not be more than 256"
    );

    println!("generating tileset");
    let mut output = std::fs::File::create(output)?;

    for c in &palettes {
        output.write_all(&c.0.to_le_bytes())?;
    }

    output.seek(std::io::SeekFrom::Start(256 * 2))?;

    let palettes = palettes
        .iter()
        .enumerate()
        .map(|x| (x.1 .0, x.0))
        .collect::<HashMap<_, _>>();

    let mut tile = Tile8BPP::new();

    for tile_y in 0..(384 / 8) {
        for tile_x in 0..(256 / 8) {
            tile.fill(0);
            for y in 0..8 {
                for x in 0..8 {
                    let px = tile_x * 8 + x;
                    let py = tile_y * 8 + y;
                    let p = GBAColor::from(img.get_pixel(px as _, py as _));
                    tile.set_pixel(x, y, palettes[&p] as u8)
                }
            }
            output.write_all(tile.as_raw())?;
        }
    }

    Ok(())
}

pub fn main() -> anyhow::Result<()> {
    let cwd = std::env::current_dir().unwrap();
    let image_path = cwd.join("images");

    std::thread::scope(|s| {
        s.spawn(|| {
            process_splash_screen_image_to_tiles(
                image_path.join("splash-screen.ninja.png"),
                image_path.join("splash-screen.ninja.bin"),
            )
            .unwrap();
        });
        s.spawn(|| {
            process_splash_screen_image_to_tiles(
                image_path.join("splash-screen.saurian.png"),
                image_path.join("splash-screen.saurian.bin"),
            )
            .unwrap();
        });
    });

    Ok(())
}

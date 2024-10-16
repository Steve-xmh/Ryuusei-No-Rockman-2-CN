use anyhow::*;
use tools::utils::ToolsRunner;

pub fn main() -> anyhow::Result<()> {
    let cwd = std::env::current_dir().unwrap();
    let tpl_path = cwd.join("tpl");
    let workspace_tpl_path = cwd.join("_workspace/mess_out_tpl");
    let sfonts_path = cwd.join("tools/sfonts");
    let original_tbl_path = cwd.join("tools/plugins/rnr2-utf8-cn-base.tbl");
    let generated_tbl_path = cwd.join("tools/plugins/rnr2-utf8-cn.tbl");
    let temp_fonts_path = cwd.join("_temp/fonts");
    let tools = ToolsRunner::new(Some(&cwd))?;

    let _ = std::fs::remove_dir_all(&temp_fonts_path);
    let _ = std::fs::create_dir_all(&temp_fonts_path)?;
    let _ = std::fs::create_dir_all(&tpl_path)?;

    ensure!(tools
        .sfont_gen()
        .arg("gen-table")
        .arg("-c")
        .arg("cn-patch-sf2")
        .arg("-i")
        .arg(&tpl_path)
        .arg("-i")
        .arg(&workspace_tpl_path)
        .arg("-b")
        .arg(&original_tbl_path)
        .arg("-o")
        .arg(&generated_tbl_path)
        .status()?
        .success());

    ensure!(tools
        .sfont_gen()
        .arg("gen-font")
        .arg("--output-base-font")
        .arg(sfonts_path.join("font3.original.bin"))
        .arg("--full-space-width")
        .arg("12")
        .arg("--half-space-width")
        .arg("6")
        .arg("-t")
        .arg(&generated_tbl_path)
        .arg("-o")
        .arg(temp_fonts_path.join("font3.bin"))
        .arg("-w")
        .arg(temp_fonts_path.join("font3_width.bin"))
        .arg("-f")
        .arg(sfonts_path.join("cn/sf1-jp-font3.sfont"))
        .arg("-f")
        .arg(sfonts_path.join("us/font-12x12-us.resized.sfont"))
        .arg("-f")
        .arg(sfonts_path.join("simsun/font-simsun-12x12.cliped.sfont"))
        .status()?
        .success());

    ensure!(tools
        .sfont_gen()
        .arg("gen-font")
        .arg("--output-base-font")
        .arg(sfonts_path.join("font2.original.bin"))
        .arg("-t")
        .arg(&generated_tbl_path)
        .arg("-o")
        .arg(temp_fonts_path.join("font2.bin"))
        .arg("-f")
        .arg(sfonts_path.join("cn/sf1-jp-font2.sfont"))
        .arg("-f")
        .arg(sfonts_path.join("muzai/font-muzai-8x12.mod.shadow.bold.sfont"))
        .arg("-f")
        .arg(sfonts_path.join("gb2312/gb2312.purified.shifted.shadow.bold.sfont"))
        .status()?
        .success());

    ensure!(tools
        .sfont_gen()
        .arg("gen-font")
        .arg("--output-base-font")
        .arg(sfonts_path.join("font1.original.bin"))
        .arg("-t")
        .arg(&generated_tbl_path)
        .arg("-o")
        .arg(temp_fonts_path.join("font1.bin"))
        .arg("-f")
        .arg(sfonts_path.join("cn/sf1-jp-font1.sfont"))
        .arg("-f")
        .arg(sfonts_path.join("muzai/font-muzai-8x12.mod.shadow.sfont"))
        .arg("-f")
        .arg(sfonts_path.join("gb2312/gb2312.purified.shifted.shadow.sfont"))
        .status()?
        .success());

    Ok(())
}

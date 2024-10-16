// 根据 CrystalTile2 导出的符号表，生成链接脚本提供给 NitroSDK 的 API 以定位函数
#[allow(dead_code)]
fn gen_api_linker_script(sym_file: &str) {
    use std::io::Write;
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let out_path = out_path.join("nitro.ld");
    println!("cargo:rustc-link-arg=-T{}", out_path.to_string_lossy());
    println!("cargo:rerun-if-changed={sym_file}");

    let mut sym_map = std::collections::HashMap::<String, (String, String)>::with_capacity(2048);
    for line in std::fs::read_to_string(sym_file).unwrap().lines() {
        let splited = line.split_whitespace().collect::<Vec<_>>();
        if splited.len() == 2 {
            let addr = splited[0].trim();
            let symbol = splited[1].trim();
            sym_map.insert(
                symbol.to_owned().to_lowercase(),
                (symbol.to_owned(), addr.to_owned()),
            );
        }
    }

    let mut file = std::fs::File::create(&out_path).unwrap();

    writeln!(file, "/* Collected {} symbols */", sym_map.len()).unwrap();
    for (symbol, addr) in sym_map.values() {
        writeln!(file, "{symbol} = 0x{addr};").unwrap();
    }
}

fn main() {
    println!("cargo:rustc-link-arg=-T./.cargo/linker.ld"); // 遵循链接脚本
    println!("cargo:rustc-link-arg=-r"); // 导出可再分配的 ELF 文件
    println!("cargo:rerun-if-changed=./.cargo/linker.ld"); // 每次更改链接脚本时需要更新

    #[cfg(feature = "ninja")]
    {
        gen_api_linker_script("../../../_rom/ninja.txt");
    }
    #[cfg(feature = "saurian")]
    {
        gen_api_linker_script("../../../_rom/saurian.txt");
    }
    #[cfg(not(any(feature = "ninja", feature = "saurian")))]
    {
        panic!("ninja or saurian feature must be enabled");
    }
}

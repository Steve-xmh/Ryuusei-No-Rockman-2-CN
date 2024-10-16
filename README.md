# Ryuusei No Rockman 2 CN

本项目为 流星之洛克人 2 狂战士/忍者/恐龙 的游戏汉化字库扩展代码，首次采用 Rust/ASM 混编的方式编写复杂字库读取代码。同时使用 Rust 编写了各种工具以更加便携地生成汉化工具包为翻译工作者更好地预览测试文字效果。

开源以提供汉化代码学习参考，部分版权受限代码已做屏蔽删除处理，翻译文本因翻译者要求不作开源处理。

## 构建指南

### 准备材料
- 可启动的 流星之洛克人 2 狂战士/忍者/恐龙 游戏本体，并按以下文件名命名：
    - 狂战士/忍者 - ninja.nds
    - 狂战士/恐龙 - saurian.nds
- Rust 工具链
- ARMIPS

首先，你需要预先准备好一个拥有完整头文件的 NitroSDK，然后使用 Rust 的 Bindgen 框架生成 Rust 绑定即可。

`build.rs` 大致构建脚本如下：

```rust
fn main() {
    bindgen::Builder::default()
        .header("./include/nitro.h")
        .use_core()
        .merge_extern_blocks(true)
        .generate_inline_functions(true)
        .generate_comments(true)
        .derive_debug(true)
        .derive_default(true)
        .impl_debug(true)
        .size_t_is_usize(true)
        .rustified_enum(".*")
        .clang_arg("-DSDK_ARM9")
        .clang_arg("-DSDK_FINALROM")
        .clang_arg("-DNITRO_OS_COMMON_PRINTF_H_")
        .clang_arg("-DNITRO_STD_STRING_H_")
        .clang_arg("-I./include")
        .clang_arg("-fparse-all-comments")
        .generate_comments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap()
        .write_to_file("./src/bindings.rs")
        .unwrap();
}
```

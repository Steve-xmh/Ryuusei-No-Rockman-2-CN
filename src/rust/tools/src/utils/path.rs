use std::path::{Path, PathBuf};

/// 根据传入的参数，返回一个可执行文件的绝对路径，如有必要会加上 `.exe` 后缀
///
/// 首先会确认路径是否存在，存在就直接返回，否则获取 `PATH` 环境变量并根据其中的路径逐个查询。
pub fn locate_path(exe_name: impl AsRef<Path>) -> PathBuf {
    if exe_name.as_ref().is_file() {
        return exe_name.as_ref().to_path_buf();
    } else {
        #[cfg(target_os = "windows")]
        {
            let exe_path = exe_name.as_ref().with_extension("exe");
            if exe_path.is_file() {
                return exe_path;
            }
        }
    }
    std::env::var_os("PATH")
        .and_then(|paths| {
            std::env::split_paths(&paths).find_map(|dir| {
                let full_path = dir.join(&exe_name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    #[cfg(target_os = "windows")]
                    {
                        let exe_path = full_path.with_extension("exe");
                        if exe_path.is_file() {
                            Some(exe_path)
                        } else {
                            None
                        }
                    }
                    #[cfg(not(target_os = "windows"))]
                    {
                        None
                    }
                }
            })
        })
        .unwrap_or_else(|| exe_name.as_ref().to_path_buf())
}

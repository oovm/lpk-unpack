use std::path::{Path, PathBuf};

// 递归扫描目录中的所有LPK文件
pub fn scan_directory_for_lpk(dir: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();

            if path.is_dir() {
                // 递归扫描子目录
                let mut sub_results = scan_directory_for_lpk(&path);
                result.append(&mut sub_results);
            }
            else if let Some(extension) = path.extension() {
                // 检查文件扩展名是否为lpk
                if extension.to_string_lossy().to_lowercase() == "lpk" {
                    result.push(path);
                }
            }
        }
    }

    result
}
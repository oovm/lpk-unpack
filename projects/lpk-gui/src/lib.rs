use dioxus::prelude::*;
use lpk::LpkLoader;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

// 应用状态
#[derive(Clone)]
struct AppState {
    // 扫描到的LPK文件列表
    lpk_files: Arc<Mutex<Vec<PathBuf>>>,
    // 选中的LPK文件
    selected_files: Arc<Mutex<HashMap<String, bool>>>,
    // 当前选择的目录
    current_dir: Arc<Mutex<Option<PathBuf>>>,
    // 解压状态信息
    status_message: Arc<Mutex<String>>,
    // 是否正在处理
    is_processing: Arc<Mutex<bool>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            lpk_files: Arc::new(Mutex::new(Vec::new())),
            selected_files: Arc::new(Mutex::new(HashMap::new())),
            current_dir: Arc::new(Mutex::new(None)),
            status_message: Arc::new(Mutex::new(String::new())),
            is_processing: Arc::new(Mutex::new(false)),
        }
    }
}

// 主应用组件
pub fn app() -> Element {
    let state = use_ref(AppState::default());

    rsx! {
        div { class: "container",
            style: "padding: 20px; display: flex; flex-direction: column; height: 100vh;",
            div { class: "header", style: "margin-bottom: 20px;",
                h1 { "LPK文件解包器" }
                p { "选择文件夹，扫描并解压LPK文件" }
            }
            div { class: "actions", style: "display: flex; gap: 10px; margin-bottom: 20px;",
                button {
                    onclick: move |_| select_directory(state.clone()),
                    disabled: *state.read().is_processing.lock().unwrap(),
                    "选择文件夹"
                }
                button {
                    onclick: move |_| extract_selected(state.clone()),
                    disabled: *state.read().is_processing.lock().unwrap() || state.read().lpk_files.lock().unwrap().is_empty(),
                    "解压选中文件"
                }
                button {
                    onclick: move |_| select_all(state.clone(), true),
                    disabled: *state.read().is_processing.lock().unwrap() || state.read().lpk_files.lock().unwrap().is_empty(),
                    "全选"
                }
                button {
                    onclick: move |_| select_all(state.clone(), false),
                    disabled: *state.read().is_processing.lock().unwrap() || state.read().lpk_files.lock().unwrap().is_empty(),
                    "取消全选"
                }
            }
            div { class: "status", style: "margin-bottom: 10px;",
                p { "{state.read().status_message.lock().unwrap()}" }
            }
            div { class: "file-list", style: "flex-grow: 1; overflow-y: auto; border: 1px solid #ccc; padding: 10px;",
                if state.read().lpk_files.lock().unwrap().is_empty() {
                    p { "未选择文件夹或未找到LPK文件" }
                } else {
                    render_file_list(state.clone())
                }
            }
        }
    }
}

// 渲染文件列表
fn render_file_list(state: UseRef<AppState>) -> Element {
    let files = state.read().lpk_files.lock().unwrap().clone();
    let selected = state.read().selected_files.lock().unwrap().clone();

    rsx! {
        ul { style: "list-style-type: none; padding: 0;",
            files.iter().map(|file| {
                let file_path = file.to_string_lossy().to_string();
                let is_checked = selected.get(&file_path).copied().unwrap_or(false);
                let file_name = file.file_name().unwrap_or_default().to_string_lossy().to_string();
                
                rsx! {
                    li { style: "margin-bottom: 5px; display: flex; align-items: center;",
                        input {
                            r#type: "checkbox",
                            checked: is_checked,
                            onchange: move |_| toggle_file_selection(state.clone(), file_path.clone()),
                            style: "margin-right: 10px;"
                        }
                        span { "{file_name} ({file_path})" }
                    }
                }
            })
        }
    }
}

// 选择目录并扫描LPK文件
fn select_directory(state: UseRef<AppState>) {
    let mut is_processing = state.write().is_processing.lock().unwrap();
    *is_processing = true;
    drop(is_processing);

    let mut status = state.write().status_message.lock().unwrap();
    *status = "正在选择文件夹...".to_string();
    drop(status);

    // 使用rfd打开文件夹选择对话框
    let task = async move {
        if let Some(folder) = rfd::AsyncFileDialog::new().pick_folder().await {
            let folder_path = folder.path().to_path_buf();

            let mut current_dir = state.write().current_dir.lock().unwrap();
            *current_dir = Some(folder_path.clone());
            drop(current_dir);

            let mut status = state.write().status_message.lock().unwrap();
            *status = format!("正在扫描文件夹: {}", folder_path.display());
            drop(status);

            // 扫描LPK文件
            let lpk_files = scan_directory_for_lpk(&folder_path);

            let mut files = state.write().lpk_files.lock().unwrap();
            *files = lpk_files.clone();
            drop(files);

            // 初始化选择状态
            let mut selected = state.write().selected_files.lock().unwrap();
            selected.clear();
            for file in &lpk_files {
                selected.insert(file.to_string_lossy().to_string(), true);
            }
            drop(selected);

            let mut status = state.write().status_message.lock().unwrap();
            *status = format!("找到 {} 个LPK文件", lpk_files.len());
        } else {
            let mut status = state.write().status_message.lock().unwrap();
            *status = "未选择文件夹".to_string();
        }

        let mut is_processing = state.write().is_processing.lock().unwrap();
        *is_processing = false;
    };

    spawn(task);
}

// 递归扫描目录中的所有LPK文件
fn scan_directory_for_lpk(dir: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();

            if path.is_dir() {
                // 递归扫描子目录
                let mut sub_results = scan_directory_for_lpk(&path);
                result.append(&mut sub_results);
            } else if let Some(extension) = path.extension() {
                // 检查文件扩展名是否为lpk
                if extension.to_string_lossy().to_lowercase() == "lpk" {
                    result.push(path);
                }
            }
        }
    }

    result
}

// 切换文件选择状态
fn toggle_file_selection(state: UseRef<AppState>, file_path: String) {
    let mut selected = state.write().selected_files.lock().unwrap();
    let current = selected.get(&file_path).copied().unwrap_or(false);
    selected.insert(file_path, !current);
}

// 全选或取消全选
fn select_all(state: UseRef<AppState>, select: bool) {
    let files = state.read().lpk_files.lock().unwrap().clone();
    let mut selected = state.write().selected_files.lock().unwrap();

    for file in files {
        selected.insert(file.to_string_lossy().to_string(), select);
    }
}

// 解压选中的文件
fn extract_selected(state: UseRef<AppState>) {
    let mut is_processing = state.write().is_processing.lock().unwrap();
    *is_processing = true;
    drop(is_processing);

    let files = state.read().lpk_files.lock().unwrap().clone();
    let selected = state.read().selected_files.lock().unwrap().clone();

    let task = async move {
        let mut status = state.write().status_message.lock().unwrap();
        *status = "开始解压文件...".to_string();
        drop(status);

        let mut success_count = 0;
        let mut error_count = 0;

        for file in files {
            let file_path = file.to_string_lossy().to_string();
            if !selected.get(&file_path).copied().unwrap_or(false) {
                continue;
            }

            let mut status = state.write().status_message.lock().unwrap();
            *status = format!("正在解压: {}", file.display());
            drop(status);

            // 创建输出目录（与文件同名，不含扩展名）
            let file_stem = file.file_stem().unwrap_or_default().to_string_lossy().to_string();
            let parent_dir = file.parent().unwrap_or_else(|| Path::new(""));
            let output_dir = parent_dir.join(&file_stem);

            // 解压文件
            match extract_lpk_file(&file, &output_dir) {
                Ok(_) => {
                    success_count += 1;
                }
                Err(err) => {
                    error_count += 1;
                    let mut status = state.write().status_message.lock().unwrap();
                    *status = format!("解压失败: {} - {}", file.display(), err);
                    drop(status);
                }
            }
        }

        let mut status = state.write().status_message.lock().unwrap();
        *status = format!("解压完成: {} 成功, {} 失败", success_count, error_count);

        let mut is_processing = state.write().is_processing.lock().unwrap();
        *is_processing = false;
    };

    spawn(task);
}

// 解压单个LPK文件
fn extract_lpk_file(file_path: &Path, output_dir: &Path) -> Result<(), String> {
    // 创建LPK加载器
    let mut loader = LpkLoader::open(file_path, None)
        .map_err(|e| format!("加载LPK文件失败: {}", e))?;

    // 解压文件
    loader.extract(output_dir)
        .map_err(|e| format!("解压LPK文件失败: {}", e))
}
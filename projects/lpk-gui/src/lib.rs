use crate::utils::scan_directory_for_lpk;
use dioxus::prelude::*;
use lpk::{LpkError, LpkLoader};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

mod utils;

// 应用状态
#[derive(Clone, Default)]
pub struct AppState {
    // 扫描到的 LPK 文件列表
    lpk_files: Vec<PathBuf>,
    // 选中的 LPK 文件
    selected_files: HashMap<String, bool>,
    // 当前选择的目录
    current_folder: Option<PathBuf>,
    // 解压状态信息
    status_message: String,
    // 是否正在处理
    is_processing: bool,
}

// 主应用组件
pub fn app() -> Element {
    let state = use_signal(AppState::default);

    rsx! {
        document::Stylesheet { href: asset!("/assets/index.scss") }
        div { class: "container",
            div { class: "header",
                h1 { "LPK文件解包器" }
                p { "选择文件夹，扫描并解压LPK文件" }
            }
            div { class: "actions",
                button {
                    class: "primary",
                    onclick: move |_| {
                        let mut state = state.clone();
                        spawn(async move {
                            let mut state = state.write();
                            state.select_directory().await;
                        });
                    },
                    disabled: state.read().is_processing,
                    div {
                        "选择文件夹"
                    }
                }
                button {
                    class: "secondary",
                    onclick: move |_| {
                        let mut state = state.clone();
                        let mut state = state.write();
                        state.extract_selected();
                    },
                    disabled: state.read().is_processing || state.read().lpk_files.is_empty(),
                    div {
                        "解压选中文件"
                    }
                }
                button {
                    class: "auxiliary",
                    onclick: move |_| select_all(state.clone(), true),
                    disabled: state.read().is_processing || state.read().lpk_files.is_empty(),
                    div {
                        "全选"
                    }
                }
                button {
                    class: "auxiliary",
                    onclick: move |_| select_all(state.clone(), false),
                    disabled: state.read().is_processing || state.read().lpk_files.is_empty(),
                    div {
                        "取消全选"
                    }
                }
            }
            div { class: "status",
                p { "{state.read().status_message}" }
            }
            div { class: "file-list",
                if state.read().lpk_files.is_empty() {
                    div { class: "empty-state",
                        p { "未选择文件夹或未找到LPK文件" }
                        p { class: "hint", "点击'选择文件夹'按钮开始" }
                    }
                } else {
                    div { class: "file-list-header",
                        div { class: "checkbox-header" }
                        div { class: "filename-header", "文件名" }
                    }
                    render_file_list { state: state.clone() }
                }
            }
        }
    }
}

// 渲染文件列表
#[component]
fn render_file_list(state: Signal<AppState>) -> Element {
    let files = state.read().lpk_files.clone();
    let selected = state.clone().read().selected_files.clone();
    let file_elements = files.iter().map(|file| {
        let file_path = file.to_string_lossy().to_string();
        let is_checked = selected.get(&file_path).copied().unwrap_or(false);
        let file_name = file.file_name().unwrap_or_default().to_string_lossy().to_string();
        let state = state.clone();
        let key = file_path.clone();

        rsx! {
            li {
                key: key,
                input {
                    r#type: "checkbox",
                    checked: is_checked,
                    onchange: move |_| toggle_file_selection(state.clone(), file_path.clone()),
                }
                span { "{file_name} ({file_path})" }
            }
        }
    });

    rsx! {
        ul {
            {file_elements}
        }
    }
}

impl AppState {
    // 选择目录并扫描 LPK 文件
    pub async fn select_directory(&mut self) {
        self.is_processing = true;
        self.status_message = "正在选择文件夹...".to_string();
        // 使用 rfd 打开文件夹选择对话框
        if let Some(folder) = rfd::AsyncFileDialog::new().pick_folder().await {
            let folder_path = folder.path().to_path_buf();
            self.current_folder = Some(folder_path.clone());
            self.status_message = format!("正在扫描文件夹: {}", folder_path.display());

            // 扫描LPK文件
            let lpk_files = scan_directory_for_lpk(&folder_path);
            self.lpk_files = lpk_files.clone();

            // 初始化选择状态
            self.selected_files.clear();
            for file in &lpk_files {
                self.selected_files.insert(file.to_string_lossy().to_string(), true);
            }

            self.status_message = format!("找到 {} 个LPK文件", lpk_files.len());
        }
        else {
            self.status_message = "未选择文件夹".to_string();
        }

        self.is_processing = false;
    }
}

// 切换文件选择状态
fn toggle_file_selection(mut state: Signal<AppState>, file_path: String) {
    let mut state = state.write();
    let current = state.selected_files.get(&file_path).copied().unwrap_or(false);
    state.selected_files.insert(file_path, !current);
}

// 全选或取消全选
fn select_all(mut state: Signal<AppState>, select: bool) {
    let mut state = state.write();
    let files = state.lpk_files.clone();
    for file in files {
        state.selected_files.insert(file.to_string_lossy().to_string(), select);
    }
}

impl AppState {
    // 解压选中的文件
    fn extract_selected(&mut self) {
        self.is_processing = true;
        self.status_message = "开始解压文件...".to_string();

        let files = self.lpk_files.clone();
        let selected = self.selected_files.clone();

        let mut success_count = 0;
        let mut error_count = 0;

        for file in files {
            let file_path = file.to_string_lossy().to_string();
            if !selected.get(&file_path).copied().unwrap_or(false) {
                continue;
            }

            self.status_message = format!("正在解压: {}", file.display());

            // 解压文件
            match extract_lpk_file(&file) {
                Ok(_) => {
                    success_count += 1;
                }
                Err(err) => {
                    error_count += 1;
                    self.status_message = format!("解压失败: {} - {}", file.display(), err);
                }
            }
        }

        self.status_message = format!("解压完成: {} 成功, {} 失败", success_count, error_count);
        self.is_processing = false;
    }
}

// 解压单个LPK文件
fn extract_lpk_file(lpk_path: &Path) -> Result<(), LpkError> {
    match lpk_path.parent() {
        Some(s) => {
            let mut loader = LpkLoader::open(lpk_path)?;
            loader.extract(s)
        }
        None => Err(LpkError::ConfigMissing),
    }
}

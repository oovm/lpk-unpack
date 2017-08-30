use dioxus::prelude::*;
use lpk_unpack::app;

fn main() {
    // 初始化日志
    dioxus::desktop::launch_cfg(
        app,
        dioxus::desktop::Config::new().with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("LPK解包器")
                .with_inner_size(dioxus::desktop::LogicalSize::new(800.0, 600.0)),
        ),
    );
}

//! 请先运行`cargo install cargo-apk2`工具。
//! cargo-apk2可以轻松构建代码并部署到您的安卓设备。
//! 运行示例`cargo apk2 run -p system-ui-android-example`

#![allow(unused_must_use)]
use system_ui::*;

#[mobile_entry_point::mobile_entry_point]
fn main() -> anyhow::Result<()> {
    init()?;
    let window = Window::new("测试", 800, 300, false)?;
    window.show();
    window.set_child(Label::new("Hello, world! 你好世界！")?);
    window.on_closing(
        |_, _| {
            quit_loop();
            true
        },
        &mut (),
    )?;

    Ok(main_loop())
}

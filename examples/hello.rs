use system_ui::*;

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

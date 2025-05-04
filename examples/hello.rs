use system_ui::*;

fn main() -> anyhow::Result<()> {
    init()?;
    let window = Window::new("测试", 800, 300, false)?;
    println!("{}", window.handle());
    println!("{:?}", window.parent::<Window>());
    window.show();
    let button = Button::new("确定")?;
    let mut cnt = 0;
    button.on_clicked(
        |_, c| {
            *c += 1;
            println!("Clicked {}", c)
        },
        &mut cnt,
    )?;
    window.set_child(button);
    //window.destroy();
    window.on_closing(|_, _| true, &mut ())?;
    ui_main();

    Ok(())
}

use system_ui::*;

fn main() -> anyhow::Result<()> {
    init()?;
    let window = Window::new("测试", 800, 300, false)?;
    window.show();
    let button = Button::new("增加")?;
    let mut cnt = 0;
    button.on_clicked(
        |_, c| {
            *c += 1;
            println!("Clicked {}", c)
        },
        &mut cnt,
    )?;
    window.set_child(button);
    window.on_closing(
        |_, _| {
            quit_loop();
            true
        },
        &mut (),
    )?;

    Ok(main_loop())
}

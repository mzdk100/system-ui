use system_ui::*;

pub fn test_button() -> anyhow::Result<()> {
    let button = Button::new("测试")?;
    assert_eq!("测试", button.text()?);
    button.set_text("test")?;
    assert_eq!("test", button.text()?);

    Ok(())
}

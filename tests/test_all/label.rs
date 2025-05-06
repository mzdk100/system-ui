use system_ui::*;

pub fn test_label() -> anyhow::Result<()> {
    let label = Label::new("测试")?;
    assert_eq!("测试", label.text()?);
    label.set_text("test")?;
    assert_eq!("test", label.text()?);

    Ok(())
}

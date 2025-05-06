use system_ui::*;

pub fn test_check_box() -> anyhow::Result<()> {
    let check_box = CheckBox::new("测试")?;
    assert_eq!("测试", check_box.text()?);
    check_box.set_text("test")?;
    assert_eq!("test", check_box.text()?);
    check_box.set_checked(true);
    assert!(check_box.checked());

    Ok(())
}

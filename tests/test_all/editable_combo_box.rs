use system_ui::*;

pub fn test_editable_combo_box() -> anyhow::Result<()> {
    let combobox = EditableCombobox::new()?;
    combobox.append("测试")?;
    combobox.set_text("测试")?;
    assert_eq!("测试", combobox.text()?);

    Ok(())
}

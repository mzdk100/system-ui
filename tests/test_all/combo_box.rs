use system_ui::*;

pub fn test_combo_box() -> anyhow::Result<()> {
    let combobox = ComboBox::new();
    combobox.append("item1")?;
    combobox.append("item2")?;
    assert_eq!(2, combobox.num_items());
    combobox.insert_at(1, "item3")?;
    assert_eq!(3, combobox.num_items());
    combobox.delete(1);
    assert_eq!(2, combobox.num_items());
    combobox.set_selected(0);
    assert_eq!(0, combobox.selected());
    combobox.clear();
    assert_eq!(0, combobox.num_items());
    assert_eq!(-1, combobox.selected());

    Ok(())
}

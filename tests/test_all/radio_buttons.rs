use system_ui::*;

pub fn test_radio_buttons() -> anyhow::Result<()> {
    let radio_buttons = RadioButtons::new();
    radio_buttons.append("item1")?;
    radio_buttons.append("item2")?;
    radio_buttons.set_selected(0);
    assert_eq!(0, radio_buttons.selected());
    radio_buttons.set_selected(-1);
    assert_eq!(-1, radio_buttons.selected());

    Ok(())
}

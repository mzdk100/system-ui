use system_ui::*;

pub fn test_spin_box() -> anyhow::Result<()> {
    let spinbox = Spinbox::new(0, 10);
    assert_eq!(0, spinbox.value());
    spinbox.set_value(5);
    assert_eq!(5, spinbox.value());

    Ok(())
}

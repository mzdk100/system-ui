use system_ui::*;

pub fn test_box() -> anyhow::Result<()> {
    let r#box = Box::new_horizontal();
    r#box.set_padded(true);
    assert!(r#box.padded());
    assert_eq!(0, r#box.num_children());

    Ok(())
}

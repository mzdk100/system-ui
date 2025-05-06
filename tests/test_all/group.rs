use system_ui::*;

pub fn test_group() -> anyhow::Result<()> {
    let group = Group::new("test")?;
    group.set_title("new_title")?;
    assert_eq!("new_title", group.title()?);
    group.set_margined(true);
    assert!(group.margined());

    Ok(())
}

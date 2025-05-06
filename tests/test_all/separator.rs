use system_ui::*;

pub fn test_separator() -> anyhow::Result<()> {
    let _horizontal_separator = Separator::new_horizontal();
    let _vertical_separator = Separator::new_vertical();

    Ok(())
}

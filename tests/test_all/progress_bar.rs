use system_ui::*;

pub fn test_progress_bar() -> anyhow::Result<()> {
    let progress_bar = ProgressBar::new();
    assert_eq!(0, progress_bar.value());
    progress_bar.set_value(50);
    assert_eq!(50, progress_bar.value());

    Ok(())
}

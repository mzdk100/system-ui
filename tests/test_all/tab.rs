use system_ui::*;

pub fn test_tab() -> anyhow::Result<()> {
    let tab = Tab::new();
    assert_eq!(-1, tab.selected());
    tab.set_selected(0);
    // assert_eq!(0, tab.selected());
    assert_eq!(0, tab.num_pages());

    Ok(())
}

use system_ui::*;

pub(super) fn test_menu() -> anyhow::Result<()> {
    let menu = Menu::new("文件")?;
    let item = menu.append_check_item("省电模式")?;
    item.set_checked(true);
    assert!(item.checked());
    let item = menu.append_quit_item();
    item.disable();
    item.enable();

    Ok(())
}

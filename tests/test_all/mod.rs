//! 测试每一个小部件功能的正确性。
//! **请注意： UI组件应该在主线程中运行，并且不应该在多线程之间共享数据。**

mod r#box;
mod button;
mod check_box;
mod combo_box;
mod control;
mod date_time_picker;
mod editable_combo_box;
mod entry;
mod group;
mod label;
mod menu;
mod multi_line_entry;
mod progress_bar;
mod radio_buttons;
mod separator;
mod slider;
mod spin_box;
mod tab;
mod window;

use system_ui::*;

#[test]
fn test_all() -> anyhow::Result<()> {
    init()?;

    // ui操作必须单线程，所以这里收集所有要测试的函数顺序运行
    r#box::test_box()?;
    button::test_button()?;
    check_box::test_check_box()?;
    combo_box::test_combo_box()?;
    control::test_control()?;
    date_time_picker::test_date_time_picker()?;
    editable_combo_box::test_editable_combo_box()?;
    entry::test_entry()?;
    group::test_group()?;
    label::test_label()?;
    menu::test_menu()?;
    multi_line_entry::test_multi_line_entry()?;
    progress_bar::test_progress_bar()?;
    radio_buttons::test_radio_buttons()?;
    separator::test_separator()?;
    slider::test_slider()?;
    spin_box::test_spin_box()?;
    tab::test_tab()?;
    window::test_window()?;

    Ok(())
}
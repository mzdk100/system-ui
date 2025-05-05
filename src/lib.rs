mod r#box;
mod button;
mod check_box;
mod combo_box;
mod control;
mod date_time_picker;
mod editable_combo_box;
mod entry;
mod error;
mod group;
mod label;
mod macros;
mod menu;
mod multi_line_entry;
mod progress_bar;
mod radio_buttons;
pub mod raw;
mod separator;
mod slider;
mod spin_box;
mod tab;
mod window;

pub use {
    r#box::*, button::*, check_box::*, combo_box::*, control::*, date_time_picker::*,
    editable_combo_box::*, entry::*, group::*, label::*, menu::*, multi_line_entry::*,
    progress_bar::*, radio_buttons::*, separator::*, slider::*, spin_box::*, tab::*, window::*,
};

use {
    error::UiError,
    raw::{uiFreeInitError, uiInit, uiInitOptions, uiMain, uiQuit, uiUninit},
    std::{ffi::CStr, mem::size_of},
};

pub fn init() -> Result<(), UiError> {
    let mut init_options = uiInitOptions {
        Size: size_of::<uiInitOptions>(),
    };

    let ptr = unsafe { uiInit(&mut init_options) };
    if ptr.is_null() {
        return Ok(());
    }

    let msg = unsafe { CStr::from_ptr(ptr).to_str()?.into() };
    unsafe {
        uiFreeInitError(ptr);
    }
    Err(UiError::Init(msg))
}

pub fn uninit() {
    unsafe { uiUninit() }
}

pub fn main_loop() {
    unsafe { uiMain() }
}

pub fn quit_loop() {
    unsafe { uiQuit() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() -> anyhow::Result<()> {
        init()?;

        // ui操作必须单线程，所以这里收集所有要测试的函数顺序运行
        test_box()?;
        test_button()?;
        test_check_box()?;
        test_combo_box()?;
        test_control()?;
        test_date_time_picker()?;
        test_editable_combo_box()?;
        test_entry()?;
        test_group()?;
        test_label()?;
        test_menu()?;
        test_multi_line_entry()?;
        test_progress_bar()?;
        test_radio_buttons()?;
        test_separator()?;
        test_slider()?;
        test_spin_box()?;
        test_tab()?;
        test_window()?;

        Ok(())
    }
}

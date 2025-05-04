mod r#box;
mod button;
mod check_box;
mod combo_box;
mod control;
mod entry;
mod error;
mod group;
mod label;
mod macros;
mod progress_bar;
pub mod raw;
mod separator;
mod slider;
mod spin_box;
mod tab;
mod window;

pub use {
    r#box::*, button::*, check_box::*, combo_box::*, control::*, entry::*, group::*, label::*,
    progress_bar::*, separator::*, slider::*, spin_box::*, tab::*, window::*,
};

use {
    error::UiError,
    raw::{uiInit, uiInitOptions, uiMain},
    std::{ffi::CStr, mem::size_of},
};

pub fn init() -> Result<(), UiError> {
    let mut init_options = uiInitOptions {
        Size: size_of::<uiInitOptions>(),
    };

    let res = unsafe { uiInit(&mut init_options) };
    if res.is_null() {
        return Ok(());
    }
    let msg = unsafe { CStr::from_ptr(res).to_str()?.into() };
    Err(UiError::Init(msg))
}

pub fn ui_main() {
    unsafe { uiMain() }
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
        test_entry()?;
        test_group()?;
        test_label()?;
        test_progress_bar()?;
        test_separator()?;
        test_slider()?;
        test_spin_box()?;
        test_tab()?;
        test_window()?;

        Ok(())
    }
}

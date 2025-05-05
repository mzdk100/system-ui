use crate::{
    Control,
    raw::{uiControl, uiNewProgressBar, uiProgressBar, uiProgressBarSetValue, uiProgressBarValue},
};

pub struct ProgressBar {
    _inner: *mut uiProgressBar,
}

impl AsRef<Self> for ProgressBar {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Control for ProgressBar {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl ProgressBar {
    /// Returns the progress bar value.
    ///
    /// # returns
    /// * Progress bar value. `[Default 0]`
    pub fn value(&self) -> i32 {
        unsafe { uiProgressBarValue(self._inner) }
    }

    /// Sets the progress bar value.
    /// Valid values are `[0, 100]` for displaying a solid bar imitating a percent
    /// value.
    /// Use a value of `-1` to render an animated bar to convey an indeterminate
    /// value.
    ///
    /// # arguments
    /// * `n`: Value to set. Integer in the range of `[-1, 100]`.
    pub fn set_value(&self, n: i32) {
        unsafe { uiProgressBarSetValue(self._inner, n) }
    }

    /// Creates a new progress bar.
    ///
    /// # returns
    /// * A new uiProgressBar instance.
    pub fn new() -> Self {
        let ptr = unsafe { uiNewProgressBar() };
        Self { _inner: ptr }.into()
    }
}

#[cfg(test)]
pub(super) fn test_progress_bar() -> anyhow::Result<()> {
    let progress_bar = ProgressBar::new();
    assert_eq!(0, progress_bar.value());
    progress_bar.set_value(50);
    assert_eq!(50, progress_bar.value());

    Ok(())
}

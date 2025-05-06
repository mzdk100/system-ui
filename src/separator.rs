use crate::{
    Control,
    raw::{uiControl, uiNewHorizontalSeparator, uiNewVerticalSeparator, uiSeparator},
};

pub struct Separator {
    _inner: *mut uiSeparator,
}

impl AsRef<Self> for Separator {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Control for Separator {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl Separator {
    /// Creates a new horizontal separator.
    /// This separator will separate controls being stacked vertically.
    ///
    /// # returns
    /// * A new uiSeparator instance.
    pub fn new_horizontal() -> Self {
        let ptr = unsafe { uiNewHorizontalSeparator() };
        Self { _inner: ptr }.into()
    }

    /// Creates a new vertical separator.
    /// This separator will separate controls being stacked horizontally.
    ///
    /// # returns
    /// * A new uiSeparator instance.
    pub fn new_vertical() -> Self {
        let ptr = unsafe { uiNewVerticalSeparator() };
        Self { _inner: ptr }.into()
    }
}

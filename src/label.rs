use {
    crate::{
        control::{BaseControl, Control},
        raw::{uiControl, uiFreeText, uiLabel, uiLabelSetText, uiLabelText, uiNewLabel},
    },
    std::{
        ffi::{CStr, CString, NulError},
        str::Utf8Error,
    },
};

pub struct Label {
    _inner: *mut uiLabel,
}

impl BaseControl for Label {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl Label {
    /// Returns the label text.
    ///
    /// # returns
    /// * The text of the label.
    pub fn text(&self) -> Result<String, Utf8Error> {
        let ptr = unsafe { uiLabelText(self._inner) };
        let text = unsafe { CStr::from_ptr(ptr) }.to_str()?.into();
        unsafe { uiFreeText(ptr) };
        Ok(text)
    }

    /// Sets the label text.
    ///
    /// # arguments
    /// * `text`: Label text.
    pub fn set_text(&self, text: &str) -> Result<(), NulError> {
        let text = CString::new(text)?;
        Ok(unsafe { uiLabelSetText(self._inner, text.as_ptr()) })
    }

    /// Creates a new label.
    ///
    /// # arguments
    /// * `text`: Label text.
    ///
    /// # returns
    /// * A new uiLabel instance.
    pub fn new(text: &str) -> Result<Control<Self>, NulError> {
        let text = CString::new(text)?;
        let ptr = unsafe { uiNewLabel(text.as_ptr()) };
        Ok(Self { _inner: ptr }.into())
    }
}

#[cfg(test)]
pub(super) fn test_label() -> anyhow::Result<()> {
    let label = Label::new("测试")?;
    assert_eq!("测试", label.text()?);
    label.set_text("test")?;
    assert_eq!("test", label.text()?);

    Ok(())
}

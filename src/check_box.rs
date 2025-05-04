use {
    crate::{
        control::{BaseControl, Control},
        define_callback_function,
        error::UiError,
        modify_callback,
        raw::{
            uiCheckbox, uiCheckboxChecked, uiCheckboxOnToggled, uiCheckboxSetChecked,
            uiCheckboxSetText, uiCheckboxText, uiControl, uiFreeText, uiNewCheckbox,
        },
    },
    log::error,
    std::{
        collections::HashMap,
        ffi::{CStr, CString, NulError, c_void},
        mem::transmute,
        str::Utf8Error,
        sync::Mutex,
    },
};

pub struct CheckBox {
    _inner: *mut uiCheckbox,
}

impl BaseControl for CheckBox {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl CheckBox {
    /// Returns the checkbox label text.
    ///
    /// # returns
    /// * The text of the label.
    pub fn text(&self) -> Result<String, Utf8Error> {
        let ptr = unsafe { uiCheckboxText(self._inner) };
        let text = unsafe { CStr::from_ptr(ptr) }.to_str()?.into();
        unsafe { uiFreeText(ptr) };
        Ok(text)
    }

    /// Sets the checkbox label text.
    ///
    /// # arguments
    /// * `text`: Label text.
    pub fn set_text(&self, text: &str) -> Result<(), NulError> {
        let text = CString::new(text)?;
        Ok(unsafe { uiCheckboxSetText(self._inner, text.as_ptr()) })
    }

    define_callback_function!(_on_toggled, uiCheckboxOnToggled, (), uiCheckbox);
    /// Registers a callback for when the checkbox is toggled by the user.
    ///
    /// # arguments
    /// * `f`: Callback function.
    ///          @p sender Back reference to the instance that initiated the callback.
    ///          @p senderData User data registered with the sender instance.
    /// * `data`: User data to be passed to the callback.
    ///
    /// # note
    /// * The callback is not triggered when calling uiCheckboxSetChecked().
    /// * Only one callback can be registered at a time.
    pub fn on_toggled<'a, 'b, F, T>(&self, f: F, data: &'a mut T) -> Result<(), UiError>
    where
        T: Copy + 'b,
        F: FnMut(Control<Self>, &'b mut T) + Send + 'static,
        'b: 'a,
    {
        self._on_toggled(Some(f), data)
    }

    /// Unregisters a callback for when the checkbox is toggled by the user.
    pub fn clear_toggled(&self) -> Result<(), UiError> {
        #[allow(unused_assignments)]
        let mut func = Some(|_, _| ());
        func = None;
        self._on_toggled(func, &mut ())
    }

    /// Returns whether or the checkbox is checked.
    ///
    /// # returns
    /// * `true` if checked, `FALSE` otherwise. [Default: `FALSE`]
    pub fn checked(&self) -> bool {
        unsafe { uiCheckboxChecked(self._inner) != 0 }
    }

    /// Sets whether or not the checkbox is checked.
    ///
    /// # arguments
    /// * `checked`: `true` to check box, `FALSE` otherwise.
    pub fn set_checked(&self, checked: bool) {
        let checked = if checked { 1 } else { 0 };
        unsafe { uiCheckboxSetChecked(self._inner, checked) }
    }

    /// Creates a new checkbox.
    ///
    /// # arguments
    /// * `text`: Label text.
    ///
    /// # returns
    /// A new uiCheckbox instance.
    pub fn new(text: &str) -> Result<Control<Self>, NulError> {
        let text = CString::new(text)?;
        let ptr = unsafe { uiNewCheckbox(text.as_ptr()) };
        Ok(Self { _inner: ptr }.into())
    }
}

#[cfg(test)]
pub(super) fn test_check_box() -> anyhow::Result<()> {
    let check_box = CheckBox::new("测试")?;
    assert_eq!("测试", check_box.text()?);
    check_box.set_text("test")?;
    assert_eq!("test", check_box.text()?);
    check_box.set_checked(true);
    assert!(check_box.checked());

    Ok(())
}

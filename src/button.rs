use {
    crate::{
        Control, define_callback_function,
        error::UiError,
        modify_callback,
        raw::{
            uiButton, uiButtonOnClicked, uiButtonSetText, uiButtonText, uiControl, uiFreeText,
            uiNewButton,
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

pub struct Button {
    _inner: *mut uiButton,
}

impl AsRef<Self> for Button {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Control for Button {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl Button {
    /// Returns the button label text.
    ///
    /// # returns
    /// * The text of the label.
    pub fn text(&self) -> Result<String, Utf8Error> {
        let ptr = unsafe { uiButtonText(self._inner) };
        let text = unsafe { CStr::from_ptr(ptr) }.to_str()?.into();
        unsafe { uiFreeText(ptr) };
        Ok(text)
    }

    /// Sets the button label text.
    ///
    /// # arguments
    /// * `text`: Label text.
    pub fn set_text(&self, text: &str) -> Result<(), NulError> {
        let text = CString::new(text)?;
        Ok(unsafe { uiButtonSetText(self._inner, text.as_ptr()) })
    }

    define_callback_function!(_on_clicked, uiButtonOnClicked, (), uiButton);
    /// Registers a callback for when the button is clicked.
    ///
    /// # arguments
    /// * `f`: Callback function.
    ///          @p sender Back reference to the instance that triggered the callback.
    ///          @p senderData User data registered with the sender instance.
    /// * `data`: User data to be passed to the callback.
    ///
    /// # note
    /// * Only one callback can be registered at a time.
    pub fn on_clicked<'a, 'b, F, T>(&self, f: F, data: &'a mut T) -> Result<(), UiError>
    where
        T: Copy + 'b,
        F: FnMut(Self, &'b mut T) + Send + 'static,
        'b: 'a,
    {
        self._on_clicked(Some(f), data)
    }

    /// Unregisters a callback for when the button is clicked.
    pub fn clear_clicked(&self) -> Result<(), UiError> {
        #[allow(unused_assignments)]
        let mut func = Some(|_, _| ());
        func = None;
        self._on_clicked(func, &mut ())
    }

    /// Creates a new button.
    ///
    /// # arguments
    /// * `text`: Label text.
    ///
    /// # returns
    /// * A new uiButton instance.
    pub fn new(text: &str) -> Result<Self, NulError> {
        let text = CString::new(text)?;
        let ptr = unsafe { uiNewButton(text.as_ptr()) };
        Ok(Self { _inner: ptr })
    }
}

#[cfg(test)]
pub(super) fn test_button() -> anyhow::Result<()> {
    let button = Button::new("测试")?;
    assert_eq!("测试", button.text()?);
    button.set_text("test")?;
    assert_eq!("test", button.text()?);

    Ok(())
}

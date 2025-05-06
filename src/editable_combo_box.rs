use {
    crate::{
        Control, define_callback_function,
        error::UiError,
        modify_callback,
        raw::{
            uiControl, uiEditableCombobox, uiEditableComboboxAppend, uiEditableComboboxOnChanged,
            uiEditableComboboxSetText, uiEditableComboboxText, uiFreeText, uiNewEditableCombobox,
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

pub struct EditableCombobox {
    _inner: *mut uiEditableCombobox,
}

impl AsRef<Self> for EditableCombobox {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Control for EditableCombobox {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl EditableCombobox {
    /// Appends an item to the editable combo box.
    ///
    /// # arguments
    /// * `text`: Item text.
    pub fn append(&self, text: &str) -> Result<(), NulError> {
        let text = CString::new(text)?;
        Ok(unsafe { uiEditableComboboxAppend(self._inner, text.as_ptr()) })
    }

    /// Returns the text of the editable combo box.
    ///
    /// # returns
    /// * The text of the editable combo box.
    pub fn text(&self) -> Result<String, Utf8Error> {
        let ptr = unsafe { uiEditableComboboxText(self._inner) };
        let text = unsafe { CStr::from_ptr(ptr) }.to_str()?.into();
        unsafe { uiFreeText(ptr) };
        Ok(text)
    }

    /// Sets the editable combo box text.
    ///
    /// # arguments
    /// * `text`: Text field text.
    pub fn set_text(&self, text: &str) -> Result<(), NulError> {
        let text = CString::new(text)?;
        Ok(unsafe { uiEditableComboboxSetText(self._inner, text.as_ptr()) })
    }

    define_callback_function!(
        _on_changed,
        uiEditableComboboxOnChanged,
        (),
        uiEditableCombobox
    );
    /// Registers a callback for when an editable combo box item is selected or user text changed.
    ///
    /// # arguments
    /// * `f`: Callback function.
    ///          @p sender Back reference to the instance that triggered the callback.
    ///          @p senderData User data registered with the sender instance.
    /// * `data`: User data to be passed to the callback.
    ///
    /// # note
    /// * Only one callback can be registered at a time.
    pub fn on_changed<'a, 'b, F, T>(&self, f: F, data: &'a mut T) -> Result<(), UiError>
    where
        T: Copy + 'b,
        F: FnMut(Self, &'b mut T) + Send + 'static,
        'b: 'a,
    {
        self._on_changed(Some(f), data)
    }

    /// Unregisters a callback for when an editable combo box item is selected or user text changed.
    pub fn clear_changed(&self) -> Result<(), UiError> {
        #[allow(unused_assignments)]
        let mut func = Some(|_, _| ());
        func = None;
        self._on_changed(func, &mut ())
    }

    /// Creates a new editable combo box.
    ///
    /// # returns
    /// * A new uiEditableCombobox instance.
    pub fn new() -> Result<Self, NulError> {
        let ptr = unsafe { uiNewEditableCombobox() };
        Ok(Self { _inner: ptr }.into())
    }
}

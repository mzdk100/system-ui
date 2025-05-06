use {
    crate::{
        Control, define_callback_function,
        error::UiError,
        modify_callback,
        raw::{
            uiControl, uiEntry, uiEntryOnChanged, uiEntryReadOnly, uiEntrySetReadOnly,
            uiEntrySetText, uiEntryText, uiFreeText, uiNewEntry, uiNewPasswordEntry,
            uiNewSearchEntry,
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

pub struct Entry {
    _inner: *mut uiEntry,
}

impl AsRef<Self> for Entry {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Control for Entry {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl Entry {
    /// Returns the entry's text.
    ///
    /// # returns
    /// * The text of the entry.
    pub fn text(&self) -> Result<String, Utf8Error> {
        let ptr = unsafe { uiEntryText(self._inner) };
        let text = unsafe { CStr::from_ptr(ptr) }.to_str()?.into();
        unsafe { uiFreeText(ptr) };
        Ok(text)
    }

    /// Sets the entry's text.
    ///
    /// # arguments
    /// * `text`: Entry text.
    pub fn set_text(&self, text: &str) -> Result<(), NulError> {
        let text = CString::new(text)?;
        Ok(unsafe { uiEntrySetText(self._inner, text.as_ptr()) })
    }

    define_callback_function!(_on_changed, uiEntryOnChanged, (), uiEntry);
    /// Registers a callback for when the user changes the entry's text.
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

    /// Unregisters a callback for when the user changes the entry's text.
    pub fn clear_changed(&self) -> Result<(), UiError> {
        #[allow(unused_assignments)]
        let mut func = Some(|_, _| ());
        func = None;
        self._on_changed(func, &mut ())
    }

    /// Returns whether the entry's text is read only.
    ///
    /// # returns
    /// * `true` if read only, `false` otherwise.
    pub fn read_only(&self) -> bool {
        unsafe { uiEntryReadOnly(self._inner) != 0 }
    }

    /// Sets whether the entry's text is read only.
    ///
    /// # arguments
    /// * `readonly`: `true` to make read only, `false` otherwise.
    pub fn set_read_only(&self, readonly: bool) {
        unsafe { uiEntrySetReadOnly(self._inner, readonly as _) }
    }

    /// Creates a new entry.
    ///
    /// # returns
    /// * A new uiEntry instance.
    pub fn new() -> Self {
        let ptr = unsafe { uiNewEntry() };
        Self { _inner: ptr }.into()
    }

    /// Creates a new entry suitable for sensitive inputs like passwords.
    ///
    /// The entered text is NOT readable by the user but masked as *******.
    ///
    /// # returns
    /// * A new uiEntry instance.
    pub fn new_password() -> Self {
        let ptr = unsafe { uiNewPasswordEntry() };
        Self { _inner: ptr }.into()
    }

    /// Creates a new entry suitable for search.
    ///
    /// Some systems will deliberately delay the uiEntryOnChanged() callback for
    /// a more natural feel.
    ///
    /// # returns
    /// * A new uiEntry instance.
    pub fn new_search() -> Self {
        let ptr = unsafe { uiNewSearchEntry() };
        Self { _inner: ptr }.into()
    }
}

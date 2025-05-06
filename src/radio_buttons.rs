use {
    crate::{
        Control, define_callback_function,
        error::UiError,
        modify_callback,
        raw::{
            uiControl, uiNewRadioButtons, uiRadioButtons, uiRadioButtonsAppend,
            uiRadioButtonsOnSelected, uiRadioButtonsSelected, uiRadioButtonsSetSelected,
        },
    },
    log::error,
    std::{
        collections::HashMap,
        ffi::{CString, NulError, c_void},
        mem::transmute,
        sync::Mutex,
    },
};

#[derive(Debug)]
pub struct RadioButtons {
    _inner: *mut uiRadioButtons,
}

impl AsRef<Self> for RadioButtons {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Control for RadioButtons {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl RadioButtons {
    /// Appends a radio button.
    ///
    /// # arguments
    /// * `text`: Radio button text.
    pub fn append(&self, text: &str) -> Result<(), NulError> {
        let text = CString::new(text)?;
        Ok(unsafe { uiRadioButtonsAppend(self._inner, text.as_ptr()) })
    }

    /// Returns the index of the selected radio button.
    ///
    /// # returns
    /// * Index of the selected radio button, `-1` on empty selection.
    pub fn selected(&self) -> i32 {
        unsafe { uiRadioButtonsSelected(self._inner) }
    }

    /// Sets the selected radio button.
    ///
    /// # arguments
    /// * `index`: Index of the radio button to be selected, `-1` to clear selection.
    pub fn set_selected(&self, index: i32) {
        unsafe { uiRadioButtonsSetSelected(self._inner, index) }
    }

    define_callback_function!(_on_selected, uiRadioButtonsOnSelected, (), uiRadioButtons);
    /// Registers a callback for when a radio button is selected.
    ///
    /// # arguments
    /// * `f`: Callback function.
    ///          @p sender Back reference to the instance that triggered the callback.
    ///          @p senderData User data registered with the sender instance.
    /// * `data`: User data to be passed to the callback.
    ///
    /// # note
    /// * The callback is not triggered when calling uiRadioButtonsSetSelected().
    /// * Only one callback can be registered at a time.
    pub fn on_selected<'a, 'b, F, T>(&self, f: F, data: &'a mut T) -> Result<(), UiError>
    where
        T: Copy + 'b,
        F: FnMut(Self, &'b mut T) + Send + 'static,
        'b: 'a,
    {
        self._on_selected(Some(f), data)
    }

    /// Unregisters a callback for when a radio button is selected.
    pub fn clear_selected(&self) -> Result<(), UiError> {
        #[allow(unused_assignments)]
        let mut func = Some(|_, _| ());
        func = None;
        self._on_selected(func, &mut ())
    }

    /// Creates a new radio buttons instance.
    ///
    /// # returns
    /// * A new uiRadioButtons instance.
    pub fn new() -> Self {
        let ptr = unsafe { uiNewRadioButtons() };
        Self { _inner: ptr }.into()
    }
}

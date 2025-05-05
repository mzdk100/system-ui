use {
    crate::{
        Control, define_callback_function,
        error::UiError,
        modify_callback,
        raw::{
            uiCombobox, uiComboboxAppend, uiComboboxClear, uiComboboxDelete, uiComboboxInsertAt,
            uiComboboxNumItems, uiComboboxOnSelected, uiComboboxSelected, uiComboboxSetSelected,
            uiControl, uiNewCombobox,
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
pub struct ComboBox {
    _inner: *mut uiCombobox,
}

impl AsRef<Self> for ComboBox {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Control for ComboBox {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl ComboBox {
    /// Appends an item to the combo box.
    ///
    /// # arguments
    /// * `text`: Item text.
    pub fn append(&self, text: &str) -> Result<(), NulError> {
        let text = CString::new(text)?;
        Ok(unsafe { uiComboboxAppend(self._inner, text.as_ptr()) })
    }

    /// Inserts an item at `index` to the combo box.
    ///
    /// # arguments
    /// * `index`: Index at which to insert the item.
    /// * `text`: Item text.
    pub fn insert_at(&self, index: i32, text: &str) -> Result<(), NulError> {
        let text = CString::new(text)?;
        Ok(unsafe { uiComboboxInsertAt(self._inner, index, text.as_ptr()) })
    }

    /// Deletes an item at @p index from the combo box.
    ///
    /// # arguments
    /// * `index`: Index of the item to be deleted.
    pub fn delete(&self, index: i32) {
        unsafe { uiComboboxDelete(self._inner, index) }
    }

    /// Deletes all items from the combo box.
    pub fn clear(&self) {
        unsafe { uiComboboxClear(self._inner) }
    }

    /// Returns the number of items contained within the combo box.
    ///
    /// # returns
    /// * Number of items.
    pub fn num_items(&self) -> i32 {
        unsafe { uiComboboxNumItems(self._inner) }
    }

    /// Returns the index of the item selected.
    ///
    /// # returns
    /// * Index of the item selected, `-1` on empty selection. [Default `-1`]
    pub fn selected(&self) -> i32 {
        unsafe { uiComboboxSelected(self._inner) }
    }

    /// Sets the item selected.
    ///
    /// # arguments
    /// * `index`: Index of the item to be selected, `-1` to clear selection.
    pub fn set_selected(&self, index: i32) {
        unsafe { uiComboboxSetSelected(self._inner, index) }
    }

    define_callback_function!(_on_selected, uiComboboxOnSelected, (), uiCombobox);
    /// Registers a callback for when a combo box item is selected.
    ///
    /// # arguments
    /// * `f`: Callback function.
    ///          @p sender Back reference to the instance that triggered the callback.
    ///          @p senderData User data registered with the sender instance.
    /// * `data`: User data to be passed to the callback.
    ///
    /// # note
    /// * The callback is not triggered when calling uiComboboxSetSelected(), uiComboboxInsertAt(), uiComboboxDelete(), or uiComboboxClear().
    /// * Only one callback can be registered at a time.
    pub fn on_selected<'a, 'b, F, T>(&self, f: F, data: &'a mut T) -> Result<(), UiError>
    where
        T: Copy + 'b,
        F: FnMut(Self, &'b mut T) + Send + 'static,
        'b: 'a,
    {
        self._on_selected(Some(f), data)
    }

    /// Unregisters a callback for when a combo box item is selected.
    pub fn clear_selected(&self) -> Result<(), UiError> {
        #[allow(unused_assignments)]
        let mut func = Some(|_, _| ());
        func = None;
        self._on_selected(func, &mut ())
    }

    /// Creates a new combo box.
    ///
    /// # returns
    /// * A new uiCombobox instance.
    pub fn new() -> Self {
        let ptr = unsafe { uiNewCombobox() };
        Self { _inner: ptr }
    }
}

#[cfg(test)]
pub(super) fn test_combo_box() -> anyhow::Result<()> {
    let combobox = ComboBox::new();
    combobox.append("item1")?;
    combobox.append("item2")?;
    assert_eq!(2, combobox.num_items());
    combobox.insert_at(1, "item3")?;
    assert_eq!(3, combobox.num_items());
    combobox.delete(1);
    assert_eq!(2, combobox.num_items());
    combobox.set_selected(0);
    assert_eq!(0, combobox.selected());
    combobox.clear();
    assert_eq!(0, combobox.num_items());
    assert_eq!(-1, combobox.selected());

    Ok(())
}

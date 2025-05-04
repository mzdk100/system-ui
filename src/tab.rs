use {
    crate::{
        control::{BaseControl, Control},
        define_callback_function,
        error::UiError,
        modify_callback,
        raw::{
            uiControl, uiNewTab, uiTab, uiTabAppend, uiTabDelete, uiTabInsertAt, uiTabMargined,
            uiTabNumPages, uiTabOnSelected, uiTabSelected, uiTabSetMargined, uiTabSetSelected,
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

pub struct Tab {
    _inner: *mut uiTab,
}

impl BaseControl for Tab {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl Tab {
    /// Returns the index of the tab selected.
    ///
    /// # returns
    /// * Index of the tab selected.
    pub fn selected(&self) -> i32 {
        unsafe { uiTabSelected(self._inner) }
    }

    /// Sets the tab selected.
    ///
    /// # arguments
    /// * `index`: Index of the tab to be selected.
    ///
    /// # note
    /// * The index must be in the range [0, uiTabNumPages(t) - 1].
    ///   If out of bounds, the selection is not changed.
    pub fn set_selected(&self, index: i32) {
        unsafe { uiTabSetSelected(self._inner, index) }
    }

    define_callback_function!(_on_selected, uiTabOnSelected, (), uiTab);
    /// Registers a callback for when a tab is selected.
    ///
    /// # arguments
    /// * `f`: Callback function.
    ///          @p sender Back reference to the instance that triggered the callback.
    ///          @p senderData User data registered with the sender instance.
    /// * `data`: User data to be passed to the callback.
    ///
    /// # note
    /// * The callback is not triggered when calling uiTabSetSelected().
    /// * Only one callback can be registered at a time.
    pub fn on_selected<'a, 'b, F, T>(&self, f: F, data: &'a mut T) -> Result<(), UiError>
    where
        T: Copy + 'b,
        F: FnMut(Control<Self>, &'b mut T) + Send + 'static,
        'b: 'a,
    {
        self._on_selected(Some(f), data)
    }

    /// Unregisters a callback for when a tab is selected.
    pub fn clear_selected(&self) -> Result<(), UiError> {
        #[allow(unused_assignments)]
        let mut func = Some(|_, _| ());
        func = None;
        self._on_selected(func, &mut ())
    }

    /// Appends a control in form of a page/tab with label.
    ///
    /// # arguments
    /// * `name`: Label text.
    /// * `c`: Control to append.
    pub fn append<C, I>(&self, name: &str, c: C) -> Result<(), NulError>
    where
        C: AsRef<Control<I>>,
        I: BaseControl,
    {
        let name = CString::new(name)?;
        Ok(unsafe { uiTabAppend(self._inner, name.as_ptr(), c.as_ref().as_ptr_mut()) })
    }

    /// Inserts a control in form of a page/tab with label at `index`.
    ///
    /// # arguments
    /// * `name`: Label text.
    /// * `index`: Index at which to insert the control.
    /// * `c`: Control to insert.
    pub fn insert_at<C, I>(&self, name: &str, index: i32, c: C) -> Result<(), NulError>
    where
        C: AsRef<Control<I>>,
        I: BaseControl,
    {
        let name = CString::new(name)?;
        Ok(unsafe { uiTabInsertAt(self._inner, name.as_ptr(), index, c.as_ref().as_ptr_mut()) })
    }

    /// Removes the control at `index`.
    ///
    /// # arguments
    /// * `index`: Index of the control to be removed.
    ///
    /// # note
    /// * The control is neither destroyed nor freed.
    pub fn delete(&self, index: i32) {
        unsafe { uiTabDelete(self._inner, index) }
    }

    /// Returns the number of pages contained.
    ///
    /// # returns
    /// * Number of pages.
    pub fn num_pages(&self) -> i32 {
        unsafe { uiTabNumPages(self._inner) }
    }

    /// Returns whether or not the page/tab at `index` has a margin.
    ///
    /// # arguments
    /// * `index`: Index to check if it has a margin.
    ///
    /// # returns
    /// * `true` if the tab has a margin, `false` otherwise. [Default: `TODO`]
    pub fn margined(&self, index: i32) -> bool {
        unsafe { uiTabMargined(self._inner, index) != 0 }
    }

    /// Sets whether or not the page/tab at `index` has a margin.
    ///
    /// The margin size is determined by the OS defaults.
    ///
    /// # arguments
    /// * `index`: Index of the tab/page to un/set margin for.
    /// * `margined`: `true` to set a margin for tab at `index`, `false` otherwise.
    pub fn set_margined(&self, index: i32, margined: bool) {
        unsafe { uiTabSetMargined(self._inner, index, margined as _) }
    }

    /// Creates a new tab container.
    ///
    /// # returns
    /// * A new uiTab instance.
    pub fn new() -> Control<Self> {
        let ptr = unsafe { uiNewTab() };
        Self { _inner: ptr }.into()
    }
}

#[cfg(test)]
pub(super) fn test_tab() -> anyhow::Result<()> {
    let tab = Tab::new();
    assert_eq!(-1, tab.selected());
    tab.set_selected(0);
    // assert_eq!(0, tab.selected());
    assert_eq!(0, tab.num_pages());

    Ok(())
}

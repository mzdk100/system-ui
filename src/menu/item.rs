use {
    crate::{
        Control, Window, define_callback_function,
        error::UiError,
        modify_callback,
        raw::{
            uiControl, uiMenuItem, uiMenuItemChecked, uiMenuItemDisable, uiMenuItemEnable,
            uiMenuItemOnClicked, uiMenuItemSetChecked, uiWindow,
        },
    },
    log::error,
    std::{collections::HashMap, ffi::c_void, mem::transmute, sync::Mutex},
};

pub struct MenuItem {
    _inner: *mut uiMenuItem,
}

impl AsRef<Self> for MenuItem {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Control for MenuItem {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl MenuItem {
    /// Enables the menu item.
    pub fn enable(&self) {
        unsafe { uiMenuItemEnable(self._inner) }
    }

    /// Disables the menu item.
    /// Menu item is grayed out and user interaction is not possible.
    pub fn disable(&self) {
        unsafe { uiMenuItemDisable(self._inner) }
    }

    define_callback_function!(
        _on_clicked,
        uiMenuItemOnClicked,
        (),
        uiMenuItem,
        (ww, uiWindow, Window)
    );
    /// Registers a callback for when the menu item is clicked.
    ///
    /// # arguments
    /// * `f`: Callback function.
    ///          @p sender Back reference to the instance that triggered the callback.
    ///          @p window Reference to the window from which the callback got triggered.
    ///          @p senderData User data registered with the sender instance.
    /// * `data`: User data to be passed to the callback.
    ///
    /// # note
    /// * Only one callback can be registered at a time.
    pub fn on_clicked<'a, 'b, F, T>(&self, f: F, data: &'a mut T) -> Result<(), UiError>
    where
        T: Copy + 'b,
        F: FnMut(Self, Window, &'b mut T) + Send + 'static,
        'b: 'a,
    {
        self._on_clicked(Some(f), data)
    }

    /// Unregisters a callback for when the menu item is clicked.
    pub fn clear_clicked(&self) -> Result<(), UiError> {
        #[allow(unused_assignments)]
        let mut func = Some(|_, _, _| ());
        func = None;
        self._on_clicked(func, &mut ())
    }

    /// Returns whether the menu item's checkbox is checked.
    /// To be used only with items created via uiMenuAppendCheckItem().
    ///
    /// # returns
    /// * `true` if checked, `FALSE` otherwise. [Default: `FALSE`]
    pub fn checked(&self) -> bool {
        unsafe { uiMenuItemChecked(self._inner) != 0 }
    }

    /// Sets whether the menu item's checkbox is checked.
    /// To be used only with items created via uiMenuAppendCheckItem().
    ///
    /// # arguments
    /// * `checked`: `true` to check menu item checkbox, `FALSE` otherwise.
    pub fn set_checked(&self, checked: bool) {
        unsafe { uiMenuItemSetChecked(self._inner, checked as _) }
    }
}

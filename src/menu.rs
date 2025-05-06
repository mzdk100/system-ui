mod item;

pub use item::MenuItem;
use {
    crate::{
        Control,
        raw::{
            uiControl, uiMenu, uiMenuAppendAboutItem, uiMenuAppendCheckItem, uiMenuAppendItem,
            uiMenuAppendPreferencesItem, uiMenuAppendQuitItem, uiMenuAppendSeparator, uiNewMenu,
        },
    },
    std::ffi::{CString, NulError},
};

pub struct Menu {
    _inner: *mut uiMenu,
}

impl AsRef<Self> for Menu {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Control for Menu {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl Menu {
    /// Appends a generic menu item.
    ///
    /// # arguments
    /// * `name`: Menu item text.
    ///
    /// # returns
    /// * A new uiMenuItem instance.
    pub fn append_item(&self, name: &str) -> Result<MenuItem, NulError> {
        let name = CString::new(name)?;
        let ptr = unsafe { uiMenuAppendItem(self._inner, name.as_ptr()) };
        Ok(MenuItem::from_ptr(ptr as _).into())
    }

    /// Appends a generic menu item with a checkbox.
    ///
    /// # arguments
    /// * `name`: Menu item text.
    ///
    /// # returns
    /// * A new uiMenuItem instance.
    pub fn append_check_item(&self, name: &str) -> Result<MenuItem, NulError> {
        let name = CString::new(name)?;
        let ptr = unsafe { uiMenuAppendCheckItem(self._inner, name.as_ptr()) };
        Ok(MenuItem::from_ptr(ptr as _).into())
    }

    /// Appends a new `Quit` menu item.
    ///
    /// # returns
    /// * A new uiMenuItem instance.
    ///
    /// # warning
    /// * Only one such menu item may exist per application.
    pub fn append_quit_item(&self) -> MenuItem {
        let ptr = unsafe { uiMenuAppendQuitItem(self._inner) };
        MenuItem::from_ptr(ptr as _).into()
    }

    /// Appends a new `Preferences` menu item.
    ///
    /// # returns
    /// * A new uiMenuItem instance.
    ///
    /// # warning
    /// * Only one such menu item may exist per application.
    pub fn append_preferences_item(&self) -> MenuItem {
        let ptr = unsafe { uiMenuAppendPreferencesItem(self._inner) };
        MenuItem::from_ptr(ptr as _).into()
    }

    /// Appends a new `About` menu item.
    ///
    /// # warning
    /// * Only one such menu item may exist per application.
    ///
    /// # returns
    /// * A new uiMenuItem instance.
    pub fn append_about_item(&self) -> MenuItem {
        let ptr = unsafe { uiMenuAppendAboutItem(self._inner) };
        MenuItem::from_ptr(ptr as _).into()
    }

    /// Appends a new separator.
    pub fn append_separator(&self) {
        unsafe { uiMenuAppendSeparator(self._inner) }
    }

    /// Creates a new menu.
    /// Typical values are `File`, `Edit`, `Help`.
    ///
    /// # arguments
    /// * `name`: Menu label.
    ///
    /// # returns
    /// * A new uiMenu instance.
    pub fn new(name: &str) -> Result<Self, NulError> {
        let name = CString::new(name)?;
        let ptr = unsafe { uiNewMenu(name.as_ptr()) };
        Ok(Self { _inner: ptr }.into())
    }
}

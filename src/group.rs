use {
    crate::{
        Control,
        raw::{
            uiControl, uiFreeText, uiGroup, uiGroupMargined, uiGroupSetChild, uiGroupSetMargined,
            uiGroupSetTitle, uiGroupTitle, uiNewGroup,
        },
    },
    std::{
        ffi::{CStr, CString, NulError},
        str::Utf8Error,
    },
};

#[derive(Debug)]
pub struct Group {
    _inner: *mut uiGroup,
}

impl AsRef<Self> for Group {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Control for Group {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl Group {
    /// Returns the group title.
    ///
    /// # returns
    /// * The group title text.
    pub fn title(&self) -> Result<String, Utf8Error> {
        let ptr = unsafe { uiGroupTitle(self._inner) };
        let text = unsafe { CStr::from_ptr(ptr) }.to_str()?.into();
        unsafe { uiFreeText(ptr) };
        Ok(text)
    }

    /// Sets the group title.
    ///
    /// # arguments
    /// * `title`: Group title text.
    ///
    /// # note
    /// * This method is merely a hint and may be ignored on unix platforms.
    pub fn set_title(&self, title: &str) -> Result<(), NulError> {
        let title = CString::new(title)?;
        Ok(unsafe { uiGroupSetTitle(self._inner, title.as_ptr()) })
    }

    /// Sets the group's child.
    ///
    /// # arguments
    /// * `child`: Control to be made child.
    pub fn set_child<C, I>(&self, child: C)
    where
        C: AsRef<I>,
        I: Control,
    {
        unsafe { uiGroupSetChild(self._inner, child.as_ref().as_ptr_mut()) }
    }

    /// Returns whether the group has a margin.
    ///
    /// # returns
    /// * `true` if group has a margin, `false` otherwise. [Default: `false`]
    pub fn margined(&self) -> bool {
        unsafe { uiGroupMargined(self._inner) != 0 }
    }

    /// Sets whether the group has a margin.
    /// The margin size is determined by the OS defaults.
    ///
    /// # arguments
    /// * `margined`: `true` to set a group margin, `false` otherwise.
    pub fn set_margined(&self, margined: bool) {
        unsafe { uiGroupSetMargined(self._inner, margined as _) }
    }

    /// Creates a new uiGroup.
    ///
    /// # arguments
    /// * `title`: Group title text.
    ///
    /// # returns
    /// * A new uiGroup instance.
    pub fn new(title: &str) -> Result<Self, NulError> {
        let title = CString::new(title)?;
        let ptr = unsafe { uiNewGroup(title.as_ptr()) };
        Ok(Self { _inner: ptr }.into())
    }
}

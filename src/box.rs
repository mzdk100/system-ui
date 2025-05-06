use crate::{
    Control,
    raw::{
        uiBox, uiBoxAppend, uiBoxDelete, uiBoxNumChildren, uiBoxPadded, uiBoxSetPadded, uiControl,
        uiNewHorizontalBox, uiNewVerticalBox,
    },
};

pub struct Box {
    _inner: *mut uiBox,
}

impl AsRef<Self> for Box {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Control for Box {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl Box {
    /// Appends a control to the box.
    /// Stretchy items expand to use the remaining space within the box.
    /// In the case of multiple stretchy items the space is shared equally.
    ///
    /// # arguments
    /// * `child`: Control instance to append.
    /// * `stretchy`: `true` to stretch control, `FALSE` otherwise.
    pub fn append<C, I>(&self, child: C, stretchy: bool)
    where
        C: AsRef<I>,
        I: Control,
    {
        let stretchy = if stretchy { 1 } else { 0 };
        unsafe { uiBoxAppend(self._inner, child.as_ref().as_ptr_mut(), stretchy) }
    }

    /// Returns the number of controls contained within the box.
    ///
    /// #returns
    /// * Number of children.
    pub fn num_children(&self) -> i32 {
        unsafe { uiBoxNumChildren(self._inner) }
    }

    /// Removes the control at `index` from the box.
    ///
    /// # arguments
    /// * `index`: Index of control to be removed.
    ///
    /// # note
    /// * The control neither destroyed nor freed.
    pub fn delete(&self, index: i32) {
        unsafe { uiBoxDelete(self._inner, index) }
    }

    /// Returns whether or not controls within the box are padded.
    /// Padding is defined as space between individual controls.
    ///
    /// # returns
    /// * `true` if controls are padded, `FALSE` otherwise. [Default: `TODO`]
    pub fn padded(&self) -> bool {
        unsafe { uiBoxPadded(self._inner) != 0 }
    }

    /// Sets whether controls within the box are padded.
    /// Padding is defined as space between individual controls.
    /// The padding size is determined by the OS defaults.
    ///
    /// # arguments
    /// * `padded`:  `true` to make controls padded, `FALSE` otherwise.
    pub fn set_padded(&self, padded: bool) {
        let padded = if padded { 1 } else { 0 };
        unsafe { uiBoxSetPadded(self._inner, padded) }
    }

    /// Creates a new horizontal box.
    /// Controls within the box are placed next to each other horizontally.
    ///
    /// # returns
    /// * A new uiBox instance.
    pub fn new_horizontal() -> Self {
        let ptr = unsafe { uiNewHorizontalBox() };
        Self { _inner: ptr }.into()
    }

    /// Creates a new vertical box.
    /// Controls within the box are placed next to each other vertically.
    ///
    /// # returns
    /// * A new uiBox instance.
    pub fn new_vertical() -> Self {
        let ptr = unsafe { uiNewVerticalBox() };
        Self { _inner: ptr }.into()
    }
}

use {
    crate::raw::{
        uiAllocControl, uiControl, uiControlDestroy, uiControlDisable, uiControlEnable,
        uiControlEnabled, uiControlEnabledToUser, uiControlHandle, uiControlHide, uiControlParent,
        uiControlSetParent, uiControlShow, uiControlToplevel, uiControlVerifySetParent,
        uiControlVisible, uiFreeControl,
    },
    std::{
        ffi::{CString, NulError},
        ops::Deref,
        ptr::null_mut,
    },
};

pub trait BaseControl {
    fn as_ptr(&self) -> *const uiControl {
        self.as_ptr_mut() as _
    }

    fn as_ptr_mut(&self) -> *mut uiControl;
    fn from_ptr(ptr: *mut uiControl) -> Self;
}

#[derive(Debug)]
pub struct Control<T> {
    _inner: T,
}

impl<T> Control<T> {
    /// Frees the memory associated with the control reference.
    ///
    /// # note
    /// This method is public only for writing custom controls.
    pub fn free(&self)
    where
        T: BaseControl,
    {
        unsafe { uiFreeControl(self._inner.as_ptr_mut()) }
    }

    /// Makes sure the control's parent can be set to `parent`.
    ///
    /// # arguments
    /// * `parent`: uiControl instance.
    ///
    /// # todo
    /// Make sure all controls have these
    ///
    /// # warning
    /// This will crash the application if `FALSE`.
    pub fn verify_set_parent<C, I>(&self, parent: C)
    where
        C: AsRef<Control<I>>,
        I: BaseControl,
        T: BaseControl,
    {
        unsafe { uiControlVerifySetParent(self._inner.as_ptr_mut(), parent.as_ref().as_ptr_mut()) }
    }

    /// Returns whether or not the control can be interacted with by the user.
    /// Checks if the control and all it's parents are enabled to make sure it can
    /// be interacted with by the user.
    ///
    /// # returns
    /// `true` if enabled, `FALSE` otherwise.
    ///
    /// # see
    /// - `uiControlEnabled`
    pub fn enabled_to_user(&self) -> bool
    where
        T: BaseControl,
    {
        unsafe { uiControlEnabledToUser(self._inner.as_ptr_mut()) != 0 }
    }

    /// Dispose and free all allocated resources.
    /// The platform specific APIs that actually destroy a control (and its children) are called.
    ///
    /// # note
    /// * Most of the time is needed to be used directly only on the top level windows.
    ///
    /// # todo
    /// Document ownership.
    pub fn destroy(&self)
    where
        T: BaseControl,
    {
        unsafe { uiControlDestroy(self._inner.as_ptr_mut()) }
    }

    /// Returns the control's OS-level handle.
    ///
    /// # returns
    /// * OS-level handle.
    pub fn handle(&self) -> usize
    where
        T: BaseControl,
    {
        unsafe { uiControlHandle(self._inner.as_ptr_mut()) }
    }

    /// Returns the parent control.
    ///
    /// # returns
    /// * The parent control, `NULL` if detached.
    pub fn parent<O>(&self) -> Option<Control<O>>
    where
        O: BaseControl,
        T: BaseControl,
    {
        let ptr = unsafe { uiControlParent(self._inner.as_ptr_mut()) };
        if ptr.is_null() {
            None
        } else {
            Some(Control {
                _inner: O::from_ptr(ptr),
            })
        }
    }

    /// Sets the control's parent.
    ///
    /// # arguments
    /// * `parent`: The parent control, `NULL` to detach.
    ///
    /// # todo
    /// Document ownership.
    pub fn set_parent<P, I>(&self, parent: Option<P>)
    where
        P: AsRef<Control<I>>,
        I: BaseControl,
        T: BaseControl,
    {
        let parent = match parent {
            None => null_mut(),
            Some(p) => p.as_ref().as_ptr_mut(),
        };

        unsafe { uiControlSetParent(self._inner.as_ptr_mut(), parent) }
    }

    /// Returns whether or not the control is a top level control.
    ///
    /// # returns
    /// * `true` if top level control, `FALSE` otherwise.
    pub fn toplevel(&self) -> bool
    where
        T: BaseControl,
    {
        unsafe { uiControlToplevel(self._inner.as_ptr_mut()) != 0 }
    }

    /// Returns whether or not the control is visible.
    ///
    /// # returns
    /// * `true` if visible, `FALSE` otherwise.
    pub fn visible(&self) -> bool
    where
        T: BaseControl,
    {
        unsafe { uiControlVisible(self._inner.as_ptr_mut()) != 0 }
    }

    /// Shows the control.
    pub fn show(&self)
    where
        T: BaseControl,
    {
        unsafe { uiControlShow(self._inner.as_ptr_mut()) }
    }

    /// Hides the control.
    ///
    /// # note
    /// * Hidden controls do not take up space within the layout.
    pub fn hide(&self)
    where
        T: BaseControl,
    {
        unsafe { uiControlHide(self._inner.as_ptr_mut()) }
    }

    /// Returns whether or not the control is enabled.
    /// Defaults to `true`.
    ///
    /// # see
    /// - `uiControlEnabledToUser`
    pub fn enabled(&self) -> bool
    where
        T: BaseControl,
    {
        unsafe { uiControlEnabled(self._inner.as_ptr_mut()) != 0 }
    }

    /// Enables the control.
    pub fn enable(&self)
    where
        T: BaseControl,
    {
        unsafe { uiControlEnable(self._inner.as_ptr_mut()) }
    }

    /// Disables the control.
    pub fn disable(&self)
    where
        T: BaseControl,
    {
        unsafe { uiControlDisable(self._inner.as_ptr_mut()) }
    }

    /// Allocates a uiControl.
    /// Helper to allocate new controls.
    ///
    /// # todo
    /// Document parameters
    pub fn alloc(os_sig: u32, type_sig: u32, type_name_str: &str) -> Result<Self, NulError>
    where
        T: BaseControl,
    {
        let type_name_str = CString::new(type_name_str)?;
        let ptr = unsafe {
            uiAllocControl(
                size_of::<uiControl>(),
                os_sig,
                type_sig,
                type_name_str.as_ptr(),
            )
        };
        Ok(Self {
            _inner: T::from_ptr(ptr),
        })
    }
}

impl<T> From<T> for Control<T> {
    fn from(value: T) -> Self {
        Self { _inner: value }
    }
}

impl<T> Deref for Control<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self._inner
    }
}

impl<T> AsRef<Self> for Control<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[cfg(test)]
pub(super) fn test_control() -> anyhow::Result<()> {
    struct MyControl {
        _inner: *mut uiControl,
    }

    impl BaseControl for MyControl {
        fn as_ptr_mut(&self) -> *mut uiControl {
            self._inner
        }

        fn from_ptr(ptr: *mut uiControl) -> Self {
            Self { _inner: ptr }
        }
    }

    let control = Control::<MyControl>::alloc(0, 0, "MyControl")?;
    control.show();
    control.hide();
    assert!(!control.enabled());
    control.enable();
    control.disable();
    assert!(!control.visible());
    assert!(!control.toplevel());
    control.set_parent::<Control<MyControl>, _>(None);
    assert!(control.parent::<MyControl>().is_none());
    assert!(control.handle() == 0);
    assert!(!control.enabled_to_user());
    control.destroy();
    control.verify_set_parent(&control);
    control.free();

    Ok(())
}

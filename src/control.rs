use {
    crate::raw::{
        uiAllocControl, uiControl, uiControlDestroy, uiControlDisable, uiControlEnable,
        uiControlEnabled, uiControlEnabledToUser, uiControlHandle, uiControlHide, uiControlParent,
        uiControlSetParent, uiControlShow, uiControlToplevel, uiControlVerifySetParent,
        uiControlVisible, uiFreeControl,
    },
    std::{
        ffi::{CString, NulError},
        ptr::null_mut,
    },
};

pub trait Control: AsRef<Self> {
    fn as_ptr(&self) -> *const uiControl {
        self.as_ptr_mut() as _
    }

    fn as_ptr_mut(&self) -> *mut uiControl;

    fn from_ptr(ptr: *mut uiControl) -> Self;

    /// Frees the memory associated with the control reference.
    ///
    /// # note
    /// This method is public only for writing custom controls.
    fn free(&self) {
        unsafe { uiFreeControl(self.as_ptr_mut()) }
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
    fn verify_set_parent<C, I>(&self, parent: C)
    where
        C: AsRef<I>,
        I: Control,
    {
        unsafe { uiControlVerifySetParent(self.as_ptr_mut(), parent.as_ref().as_ptr_mut()) }
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
    fn enabled_to_user(&self) -> bool {
        unsafe { uiControlEnabledToUser(self.as_ptr_mut()) != 0 }
    }

    /// Dispose and free all allocated resources.
    /// The platform specific APIs that actually destroy a control (and its children) are called.
    ///
    /// # note
    /// * Most of the time is needed to be used directly only on the top level windows.
    ///
    /// # todo
    /// Document ownership.
    fn destroy(&self) {
        unsafe { uiControlDestroy(self.as_ptr_mut()) }
    }

    /// Returns the control's OS-level handle.
    ///
    /// # returns
    /// * OS-level handle.
    fn handle(&self) -> usize {
        unsafe { uiControlHandle(self.as_ptr_mut()) }
    }

    /// Returns the parent control.
    ///
    /// # returns
    /// * The parent control, `NULL` if detached.
    fn parent<O>(&self) -> Option<O>
    where
        O: Control,
    {
        let ptr = unsafe { uiControlParent(self.as_ptr_mut()) };
        if ptr.is_null() {
            None
        } else {
            Some(O::from_ptr(ptr))
        }
    }

    /// Sets the control's parent.
    ///
    /// # arguments
    /// * `parent`: The parent control, `NULL` to detach.
    ///
    /// # todo
    /// Document ownership.
    fn set_parent<P, I>(&self, parent: Option<P>)
    where
        P: AsRef<I>,
        I: Control,
    {
        let parent = match parent {
            None => null_mut(),
            Some(p) => p.as_ref().as_ptr_mut(),
        };

        unsafe { uiControlSetParent(self.as_ptr_mut(), parent) }
    }

    /// Returns whether or not the control is a top level control.
    ///
    /// # returns
    /// * `true` if top level control, `FALSE` otherwise.
    fn toplevel(&self) -> bool {
        unsafe { uiControlToplevel(self.as_ptr_mut()) != 0 }
    }

    /// Returns whether or not the control is visible.
    ///
    /// # returns
    /// * `true` if visible, `FALSE` otherwise.
    fn visible(&self) -> bool {
        unsafe { uiControlVisible(self.as_ptr_mut()) != 0 }
    }

    /// Shows the control.
    fn show(&self) {
        unsafe { uiControlShow(self.as_ptr_mut()) }
    }

    /// Hides the control.
    ///
    /// # note
    /// * Hidden controls do not take up space within the layout.
    fn hide(&self) {
        unsafe { uiControlHide(self.as_ptr_mut()) }
    }

    /// Returns whether or not the control is enabled.
    /// Defaults to `true`.
    ///
    /// # see
    /// - `uiControlEnabledToUser`
    fn enabled(&self) -> bool {
        unsafe { uiControlEnabled(self.as_ptr_mut()) != 0 }
    }

    /// Enables the control.
    fn enable(&self) {
        unsafe { uiControlEnable(self.as_ptr_mut()) }
    }

    /// Disables the control.
    fn disable(&self) {
        unsafe { uiControlDisable(self.as_ptr_mut()) }
    }

    /// Allocates a uiControl.
    /// Helper to allocate new controls.
    ///
    /// # todo
    /// Document parameters
    fn alloc(os_sig: u32, type_sig: u32, type_name_str: &str) -> Result<Self, NulError>
    where
        Self: Sized,
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
        Ok(Self::from_ptr(ptr))
    }
}

#[cfg(test)]
pub(super) fn test_control() -> anyhow::Result<()> {
    struct MyControl {
        _inner: *mut uiControl,
    }

    impl AsRef<Self> for MyControl {
        fn as_ref(&self) -> &Self {
            self
        }
    }

    impl Control for MyControl {
        fn as_ptr_mut(&self) -> *mut uiControl {
            self._inner
        }

        fn from_ptr(ptr: *mut uiControl) -> Self {
            Self { _inner: ptr }
        }
    }

    let control = MyControl::alloc(0, 0, "MyControl")?;
    control.show();
    control.hide();
    assert!(!control.enabled());
    control.enable();
    control.disable();
    assert!(!control.visible());
    assert!(!control.toplevel());
    control.set_parent::<MyControl, _>(None);
    assert!(control.parent::<MyControl>().is_none());
    assert!(control.handle() == 0);
    assert!(!control.enabled_to_user());
    control.destroy();
    control.verify_set_parent(&control);
    control.free();

    Ok(())
}

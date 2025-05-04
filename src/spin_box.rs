use {
    crate::{
        control::{BaseControl, Control},
        define_callback_function,
        error::UiError,
        modify_callback,
        raw::{
            uiControl, uiNewSpinbox, uiSpinbox, uiSpinboxOnChanged, uiSpinboxSetValue,
            uiSpinboxValue,
        },
    },
    log::error,
    std::{collections::HashMap, ffi::c_void, mem::transmute, sync::Mutex},
};

pub struct Spinbox {
    _inner: *mut uiSpinbox,
}

impl BaseControl for Spinbox {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl Spinbox {
    /// Returns the spinbox value.
    ///
    /// # returns
    /// * The spinbox value.
    pub fn value(&self) -> i32 {
        unsafe { uiSpinboxValue(self._inner) }
    }

    /// Sets the spinbox value.
    ///
    /// # arguments
    /// * `value`: Value to set.
    /// # note
    /// * Setting a value out of range will clamp to the nearest value in range.
    pub fn set_value(&self, value: i32) {
        unsafe { uiSpinboxSetValue(self._inner, value) }
    }

    define_callback_function!(_on_changed, uiSpinboxOnChanged, (), uiSpinbox);
    /// Registers a callback for when the spinbox value is changed by the user.
    ///
    /// # arguments
    /// * `f`: Callback function.
    ///          @p sender Back reference to the instance that initiated the callback.
    ///          @p senderData User data registered with the sender instance.
    /// * `data`: User data to be passed to the callback.
    ///
    /// # note
    /// * The callback is not triggered when calling uiSpinboxSetValue().
    /// * Only one callback can be registered at a time.
    pub fn on_changed<'a, 'b, F, T>(&self, f: F, data: &'a mut T) -> Result<(), UiError>
    where
        T: Copy + 'b,
        F: FnMut(Control<Self>, &'b mut T) + Send + 'static,
        'b: 'a,
    {
        self._on_changed(Some(f), data)
    }

    /// Unregisters a callback for when the spinbox value is changed by the user.
    pub fn clear_changed(&self) -> Result<(), UiError> {
        #[allow(unused_assignments)]
        let mut func = Some(|_, _| ());
        func = None;
        self._on_changed(func, &mut ())
    }

    /// Creates a new spinbox.
    ///
    /// The initial spinbox value equals the minimum value.
    ///
    /// In the current implementation @p min and @p max are swapped if `min>max`.
    /// This may change in the future though. See TODO.
    ///
    /// # arguments
    /// * `min`: Minimum value.
    /// * `max`: Maximum value.
    ///
    /// # returns
    /// A new uiSpinbox instance.
    pub fn new(min: i32, max: i32) -> Control<Self> {
        let ptr = unsafe { uiNewSpinbox(min, max) };
        Self { _inner: ptr }.into()
    }
}

#[cfg(test)]
pub(super) fn test_spin_box() -> anyhow::Result<()> {
    let spinbox = Spinbox::new(0, 10);
    assert_eq!(0, spinbox.value());
    spinbox.set_value(5);
    assert_eq!(5, spinbox.value());

    Ok(())
}

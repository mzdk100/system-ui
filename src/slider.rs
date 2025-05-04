use {
    crate::{error::UiError,
        control::{BaseControl, Control},
        define_callback_function,
        modify_callback,
        raw::{
            uiControl, uiNewSlider, uiSlider, uiSliderHasToolTip, uiSliderOnChanged,
            uiSliderOnReleased, uiSliderSetValue, uiSliderSetHasToolTip, uiSliderSetRange,
            uiSliderValue,
        },
    },
    log::error,
    std::{collections::HashMap, ffi::c_void, mem::transmute, sync::Mutex},
};

pub struct Slider {
    _inner: *mut uiSlider,
}

impl BaseControl for Slider {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl Slider {
    /// Returns the slider value.
    ///
    /// # returns
    /// * The slider value.
    pub fn value(&self) -> i32 {
        unsafe { uiSliderValue(self._inner) }
    }

    /// Sets the slider value.
    ///
    /// # arguments
    /// * `value`: Value to set.
    /// 
    /// # note
    /// * Setting a value out of range will clamp to the nearest value in range.
    pub fn set_value(&self, value: i32) {
        unsafe { uiSliderSetValue(self._inner, value) }
    }

    /// Returns whether the slider has a tool tip.
    ///
    /// # returns
    /// * `true` if a tool tip is present, `FALSE` otherwise. [Default `true`]
    pub fn has_tool_tip(&self) -> bool {
        unsafe { uiSliderHasToolTip(self._inner) != 0 }
    }

    /// Sets whether or not the slider has a tool tip.
    ///
    /// # arguments
    /// * `has_tool_tip`: `true` to display a tool tip, `FALSE` to display no tool tip.
    pub fn set_has_tool_tip(&self, has_tool_tip: bool) {
        unsafe { uiSliderSetHasToolTip(self._inner, has_tool_tip as _) }
    }

    define_callback_function!(_on_changed, uiSliderOnChanged, (), uiSlider);
    /// Registers a callback for when the slider value is changed by the user.
    ///
    /// # arguments
    /// * `f`: Callback function.
    ///          @p sender Back reference to the instance that initiated the callback.
    ///          @p senderData User data registered with the sender instance.
    /// * `data`: User data to be passed to the callback.
    ///
    /// # note
    /// * The callback is not triggered when calling uiSliderSetValue().
    /// * Only one callback can be registered at a time.
    pub fn on_changed<'a, 'b, F, T>(&self, f: F, data: &'a mut T) -> Result<(), UiError>
    where
        T: Copy + 'b,
        F: FnMut(Control<Self>, &'b mut T) + Send + 'static,
        'b: 'a,
    {
        self._on_changed(Some(f), data)
    }

    /// Unregisters a callback for when the slider value is changed by the user.
    pub fn clear_changed(&self) -> Result<(), UiError> {
        #[allow(unused_assignments)]
        let mut func = Some(|_, _| ());
        func = None;
        self._on_changed(func, &mut ())
    }

    define_callback_function!(_on_released, uiSliderOnReleased, (), uiSlider);
    /// Registers a callback for when the slider is released from dragging.
    ///
    /// # arguments
    /// * `f`: Callback function.
    ///          @p sender Back reference to the instance that initiated the callback.
    ///          @p senderData User data registered with the sender instance.
    /// * `data`: User data to be passed to the callback.
    ///
    /// # note
    /// * Only one callback can be registered at a time.
    pub fn on_released<'a, 'b, F, T>(&self, f: F, data: &'a mut T) -> Result<(), UiError>
    where
        T: Copy + 'b,
        F: FnMut(Control<Self>, &'b mut T) + Send + 'static,
        'b: 'a,
    {
        self._on_released(Some(f), data)
    }

    /// Unregisters a callback for when the slider is released from dragging.
    pub fn clear_released(&self) -> Result<(), UiError> {
        #[allow(unused_assignments)]
        let mut func = Some(|_, _| ());
        func = None;
        self._on_released(func, &mut ())
    }

    /// Sets the slider range.
    ///
    /// # arguments
    /// * `min`: Minimum value.
    /// * `max`: Maximum value.
    ///
    /// # note
    /// * Make sure to clamp the slider value to the nearest value in range - should
    ///   it be out of range. Call uiSliderOnChanged() in such a case.
    pub fn set_range(&self, min: i32, max: i32) {
        unsafe { uiSliderSetRange(self._inner, min, max) }
    }

    /// Creates a new slider.
    ///
    /// The initial slider value equals the minimum value.
    ///
    /// In the current implementation @p min and @p max are swapped if `min>max`.
    /// This may change in the future though. See TODO.
    ///
    /// # arguments
    /// * `min`: Minimum value.
    /// * `max`: Maximum value.
    ///
    /// # returns
    /// A new uiSlider instance.
    pub fn new(min: i32, max: i32) -> Control<Self> {
        let ptr = unsafe { uiNewSlider(min, max) };
        Self { _inner: ptr }.into()
    }
}

#[cfg(test)]
pub(super) fn test_slider() -> anyhow::Result<()> {
    let slider = Slider::new(0, 10);
    assert_eq!(0, slider.value());
    slider.set_value(5);
    assert_eq!(5, slider.value());
    slider.set_has_tool_tip(true);
    assert!(slider.has_tool_tip());

    Ok(())
}

use {
    crate::{
        Control, define_callback_function,
        error::UiError,
        modify_callback,
        raw::{
            tm, uiControl, uiDateTimePicker, uiDateTimePickerOnChanged, uiDateTimePickerSetTime,
            uiDateTimePickerTime, uiNewDatePicker, uiNewDateTimePicker, uiNewTimePicker,
        },
    },
    log::error,
    std::{
        collections::HashMap,
        ffi::c_void,
        mem::transmute,
        sync::Mutex,
        time::{Duration, SystemTime, SystemTimeError},
    },
};

unsafe extern "C" {
    fn mktime(time: *mut tm) -> u64;
    fn localtime(timer: *const u64) -> *mut tm;
}

const TM_SIZE: usize = 28;

pub struct DateTimePicker {
    _inner: *mut uiDateTimePicker,
}

impl AsRef<Self> for DateTimePicker {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Control for DateTimePicker {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl DateTimePicker {
    /// Returns date and time stored in the data time picker.
    ///
    /// # returns
    /// * Date and/or time as local time.
    pub fn time(&self) -> SystemTime {
        let mut t = [0u8; TM_SIZE];
        let ptr = &mut t as *mut [u8; TM_SIZE] as *mut tm;
        unsafe { uiDateTimePickerTime(self._inner, ptr) };
        let ts = unsafe { mktime(ptr) };
        SystemTime::UNIX_EPOCH + Duration::from_secs(ts)
    }

    /// Sets date and time of the data time picker.
    ///
    /// # arguments
    /// * `time`: Date and/or time as local time.
    pub fn set_time(&self, time: SystemTime) -> Result<(), SystemTimeError> {
        let ts = time.duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
        let ptr = unsafe { localtime(&ts as *const _) };
        Ok(unsafe { uiDateTimePickerSetTime(self._inner, ptr) })
    }

    define_callback_function!(_on_changed, uiDateTimePickerOnChanged, (), uiDateTimePicker);
    /// Registers a callback for when the date time picker value is changed by the user.
    ///
    /// # arguments
    /// * `f`: Callback function.
    ///          @p sender Back reference to the instance that triggered the callback.
    ///          @p senderData User data registered with the sender instance.
    /// * `data`: User data to be passed to the callback.
    ///
    /// # note
    /// * The callback is not triggered when calling  uiDateTimePickerSetTime().
    /// * Only one callback can be registered at a time.
    pub fn on_changed<'a, 'b, F, T>(&self, f: F, data: &'a mut T) -> Result<(), UiError>
    where
        T: Copy + 'b,
        F: FnMut(Self, &'b mut T) + Send + 'static,
        'b: 'a,
    {
        self._on_changed(Some(f), data)
    }

    /// Unregisters a callback for when the date time picker value is changed by the user.
    pub fn clear_changed(&self) -> Result<(), UiError> {
        #[allow(unused_assignments)]
        let mut func = Some(|_, _| ());
        func = None;
        self._on_changed(func, &mut ())
    }

    /// Creates a new date picker.
    ///
    /// # returns
    /// * A new uiDateTimePicker instance.
    pub fn new_date() -> Self {
        let ptr = unsafe { uiNewDatePicker() };
        Self { _inner: ptr }.into()
    }

    /// Creates a new time picker.
    ///
    /// # returns
    /// * A new uiDateTimePicker instance.
    pub fn new_time() -> Self {
        let ptr = unsafe { uiNewTimePicker() };
        Self { _inner: ptr }.into()
    }

    /// Creates a new date and time picker.
    ///
    /// # returns
    /// * A new uiDateTimePicker instance.
    pub fn new() -> Self {
        let ptr = unsafe { uiNewDateTimePicker() };
        Self { _inner: ptr }.into()
    }
}

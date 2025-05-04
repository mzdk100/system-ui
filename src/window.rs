use {
    crate::{
        control::{BaseControl, Control},
        define_callback_function,
        error::UiError,
        modify_callback,
        raw::{
            uiControl, uiFreeText, uiNewWindow, uiWindow, uiWindowBorderless, uiWindowContentSize,
            uiWindowFocused, uiWindowFullscreen, uiWindowMargined, uiWindowOnClosing,
            uiWindowOnContentSizeChanged, uiWindowOnFocusChanged, uiWindowOnPositionChanged,
            uiWindowPosition, uiWindowResizeable, uiWindowSetBorderless, uiWindowSetChild,
            uiWindowSetContentSize, uiWindowSetFullscreen, uiWindowSetMargined,
            uiWindowSetPosition, uiWindowSetResizeable, uiWindowSetTitle, uiWindowTitle,
        },
    },
    log::error,
    std::{
        collections::HashMap,
        ffi::{CStr, CString, NulError, c_void},
        mem::transmute,
        str::Utf8Error,
        sync::Mutex,
    },
};

#[derive(Debug)]
pub struct Window {
    _inner: *mut uiWindow,
}

impl BaseControl for Window {
    fn as_ptr_mut(&self) -> *mut uiControl {
        self._inner as _
    }

    fn from_ptr(ptr: *mut uiControl) -> Self {
        Self { _inner: ptr as _ }
    }
}

impl Window {
    /// Returns whether or not the window is borderless.
    ///
    /// # returns
    /// * `true` if window is borderless, `false` otherwise. [Default: `TODO`]
    pub fn borderless(&self) -> bool {
        unsafe { uiWindowBorderless(self._inner) != 0 }
    }

    /// Sets whether the window is borderless.
    ///
    /// # arguments
    /// * `borderless`: `true` to make window borderless, `false` otherwise.
    ///
    /// # note
    /// * This method is merely a hint and may be ignored by the system.
    pub fn set_borderless(&self, borderless: bool) {
        unsafe { uiWindowSetBorderless(self._inner, borderless as _) }
    }

    /// Returns whether the window is focused.
    ///
    /// # returns
    /// * `true` if window is focused, `false` otherwise.
    pub fn focused(&self) -> bool {
        unsafe { uiWindowFocused(self._inner) != 0 }
    }

    define_callback_function!(_on_focus_changed, uiWindowOnFocusChanged, (), uiWindow);
    /// Registers a callback for when the window focus changes.
    ///
    /// # arguments
    /// * `f`: Callback function.
    ///          @p sender Back reference to the instance that triggered the callback.
    ///          @p senderData User data registered with the sender instance.
    ///
    /// * `data`: User data to be passed to the callback.
    ///
    /// # note
    /// * Only one callback can be registered at a time.
    pub fn on_focus_changed<'a, 'b, F, T>(&self, f: F, data: &'a mut T) -> Result<(), UiError>
    where
        T: Copy + 'b,
        F: FnMut(Control<Self>, &'b mut T) + Send + 'static,
        'b: 'a,
    {
        self._on_focus_changed(Some(f), data)
    }

    /// Unregisters a callback for when the window focus changes.
    pub fn clear_focus_changed(&self) -> Result<(), UiError> {
        #[allow(unused_assignments)]
        let mut func = Some(|_, _| ());
        func = None;
        self._on_focus_changed(func, &mut ())
    }

    define_callback_function!(_on_closing, uiWindowOnClosing, i32, uiWindow);
    /// Registers a callback for when the window is to be closed.
    ///
    /// # arguments
    /// * `f`: Callback function.
    ///          @p sender Back reference to the instance that triggered the callback.
    ///          @p senderData User data registered with the sender instance.
    ///          Return:
    ///          `true` to destroys the window.
    ///          `false` to abort closing and keep the window alive and visible.
    /// * `data`: User data to be passed to the callback.
    ///
    /// # note
    /// * Only one callback can be registered at a time.
    pub fn on_closing<'a, 'b, F, T>(&self, mut f: F, data: &'a mut T) -> Result<(), UiError>
    where
        T: Copy + 'b,
        F: FnMut(Control<Self>, &'b mut T) -> bool + Send + 'static,
        'b: 'a,
    {
        self._on_closing(Some(move |w, d| if f(w, d) { 1 } else { 0 }), data)
    }

    /// Unregisters a callback for when the window is to be closed.
    pub fn clear_closing(&self) -> Result<(), UiError> {
        #[allow(unused_assignments)]
        let mut func = Some(|_, _| 0);
        func = None;
        self._on_closing(func, &mut ())
    }

    define_callback_function!(
        _on_content_size_changed,
        uiWindowOnContentSizeChanged,
        (),
        uiWindow
    );
    /// Registers a callback for when the window content size is changed.
    ///
    /// # arguments
    /// * `f`: Callback function.
    ///          @p sender Back reference to the instance that triggered the callback.
    ///          @p senderData User data registered with the sender instance.
    /// * `data`: User data to be passed to the callback.
    ///
    /// # note
    /// * The callback is not triggered when calling uiWindowSetContentSize().
    /// Only one callback can be registered at a time.
    pub fn on_content_size_changed<'a, 'b, F, T>(&self, f: F, data: &mut T) -> Result<(), UiError>
    where
        T: Copy + 'b,
        F: FnMut(Control<Self>, &'b mut T) + Send + 'static,
        'b: 'a,
    {
        self._on_content_size_changed(Some(f), data)
    }

    /// Unregisters a callback for when the window content size is changed.
    pub fn clear_content_size_changed(&self) -> Result<(), UiError> {
        #[allow(unused_assignments)]
        let mut func = Some(|_, _| ());
        func = None;
        self._on_content_size_changed(func, &mut ())
    }

    /// Sets whether the window is full screen.
    ///
    /// # arguments
    /// * `fullscreen`: `true` to make window full screen, `false` otherwise.
    ///
    /// # note
    /// * This method is merely a hint and may be ignored by the system.
    pub fn set_fullscreen(&self, fullscreen: bool) {
        unsafe { uiWindowSetFullscreen(self._inner, fullscreen as _) }
    }

    /// Returns whether the window is full screen.
    ///
    /// # returns
    /// * `true` if full screen, `false` otherwise. [Default: `false`]
    pub fn fullscreen(&self) -> bool {
        unsafe { uiWindowFullscreen(self._inner) != 0 }
    }

    /// Sets the window content size.
    ///
    /// # arguments
    /// * `width`: Window content width to set.
    /// * `height`: Window content height to set.
    ///
    /// # note
    /// * The content size does NOT include window decorations like menus or title bars.
    /// * This method is merely a hint and may be ignored by the system.
    pub fn set_content_size(&self, width: i32, height: i32) {
        unsafe { uiWindowSetContentSize(self._inner, width, height) }
    }

    /// Gets the window content size.
    ///
    /// # returns
    /// * `width`: Window content width.
    /// * `height`: Window content height.
    ///
    /// # note
    /// * The content size does NOT include window decorations like menus or title bars.
    pub fn content_size(&self) -> (i32, i32) {
        let (mut width, mut height) = (0, 0);
        unsafe {
            uiWindowContentSize(self._inner, &mut width, &mut height);
        }
        (width, height)
    }

    define_callback_function!(
        _on_position_changed,
        uiWindowOnPositionChanged,
        (),
        uiWindow
    );
    /// Registers a callback for when the window moved.
    ///
    /// # arguments
    /// * `f`: Callback function.
    ///          @p sender Back reference to the instance that triggered the callback.
    ///          @p senderData User data registered with the sender instance.
    /// * `data`: User data to be passed to the callback.
    ///
    /// # note
    /// * Only one callback can be registered at a time.
    /// * The callback is not triggered when calling uiWindowSetPosition().
    pub fn on_position_changed<'a, 'b, F, T>(&self, f: F, data: &mut T) -> Result<(), UiError>
    where
        T: Copy + 'b,
        F: FnMut(Control<Self>, &'b mut T) + Send + 'static,
        'b: 'a,
    {
        self._on_position_changed(Some(f), data)
    }

    /// Unregisters a callback for when the window moved.
    pub fn clear_position_changed(&self) -> Result<(), UiError> {
        #[allow(unused_assignments)]
        let mut func = Some(|_, _| ());
        func = None;
        self._on_position_changed(func, &mut ())
    }

    /// Moves the window to the specified position.
    /// Coordinates are measured from the top left corner of the screen.
    ///
    /// # arguments
    /// * `x`: New x position of the window.
    /// * `y`: New y position of the window.
    ///
    /// #note
    /// * This method is merely a hint and may be ignored on Unix platforms.
    pub fn set_position(&self, x: i32, y: i32) {
        unsafe { uiWindowSetPosition(self._inner, x, y) }
    }

    /// Gets the window position.
    /// Coordinates are measured from the top left corner of the screen.
    ///
    /// # returns
    /// * `x`: X position of the window.
    /// * `y`: Y position of the window.
    ///
    /// # note
    /// * This method may return inaccurate or dummy values on Unix platforms.
    pub fn position(&self) -> (i32, i32) {
        let (mut x, mut y) = (0, 0);
        unsafe {
            uiWindowPosition(self._inner, &mut x, &mut y);
        }
        (x, y)
    }

    /// Sets the window title.
    ///
    /// # arguments
    /// * `title`: Window title text.
    ///
    /// # note
    /// * This method is merely a hint and may be ignored on unix platforms.
    pub fn set_title(&self, title: &str) -> Result<(), NulError> {
        let title = CString::new(title)?;
        Ok(unsafe { uiWindowSetTitle(self._inner, title.as_ptr()) })
    }

    /// Returns the window title.
    ///
    /// # returns
    /// * The window title text.
    pub fn title(&self) -> Result<String, Utf8Error> {
        let ptr = unsafe { uiWindowTitle(self._inner) };
        let text = unsafe { CStr::from_ptr(ptr) }.to_str()?.into();
        unsafe { uiFreeText(ptr) };
        Ok(text)
    }

    /// Sets the window's child.
    ///
    /// # arguments
    /// * `child`: Control to be made child.
    pub fn set_child<C, I>(&self, child: C)
    where
        C: AsRef<Control<I>>,
        I: BaseControl,
    {
        unsafe { uiWindowSetChild(self._inner, child.as_ref().as_ptr_mut()) }
    }

    /// Returns whether the window has a margin.
    ///
    /// # returns
    /// * `true` if window has a margin, `false` otherwise. [Default: `false`]
    pub fn margined(&self) -> bool {
        unsafe { uiWindowMargined(self._inner) != 0 }
    }

    /// Sets whether the window has a margin.
    /// The margin size is determined by the OS defaults.
    ///
    /// # arguments
    /// * `margined`: `true` to set a window margin, `false` otherwise.
    pub fn set_margined(&self, margined: bool) {
        unsafe { uiWindowSetMargined(self._inner, margined as _) }
    }

    /// Returns whether the window is user resizeable.
    ///
    /// # returns
    /// * `true` if window is resizable, `false` otherwise. [Default: `true`]
    pub fn resizeable(&self) -> bool {
        unsafe { uiWindowResizeable(self._inner) != 0 }
    }

    /// Sets whether the window is user resizeable.
    ///
    /// # arguments
    /// * `resizeable`: `true` to make window resizable, `false` otherwise.
    ///
    /// # note
    /// * This method is merely a hint and may be ignored by the system.
    pub fn set_resizeable(&self, resizeable: bool) {
        unsafe { uiWindowSetResizeable(self._inner, resizeable as _) }
    }

    /// Creates a new uiWindow.
    ///
    /// # arguments
    /// * `title`: Window title text.
    /// * `width`: Window width.
    /// * `height`: Window height.
    /// * `has_menubar`: Whether or not the window should display a menu bar.
    ///
    /// # returns
    /// * A new uiWindow instance.
    pub fn new(
        title: &str,
        width: i32,
        height: i32,
        has_menubar: bool,
    ) -> Result<Control<Self>, NulError> {
        let title = CString::new(title)?;
        let has_menubar = if has_menubar { 1 } else { 0 };
        let ptr = unsafe { uiNewWindow(title.as_ptr(), width, height, has_menubar) };
        Ok(Self { _inner: ptr }.into())
    }
}

#[cfg(test)]
pub(super) fn test_window() -> anyhow::Result<()> {
    let window = Window::new("test", 80, 80, false)?;
    window.set_title("new_title")?;
    assert_eq!("new_title", window.title()?);
    window.set_fullscreen(true);
    assert!(window.fullscreen());
    window.set_borderless(true);
    assert!(window.borderless());
    window.set_content_size(100, 100);
    assert_eq!((100, 100), window.content_size());
    window.set_margined(true);
    assert!(window.margined());
    window.set_position(40, 40);
    assert_eq!((40, 40), window.position());
    assert!(window.focused());

    Ok(())
}

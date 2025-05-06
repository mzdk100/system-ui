use system_ui::*;

pub fn test_control() -> anyhow::Result<()> {
    struct MyControl {
        _inner: *mut raw::uiControl,
    }

    impl AsRef<Self> for MyControl {
        fn as_ref(&self) -> &Self {
            self
        }
    }

    impl Control for MyControl {
        fn as_ptr_mut(&self) -> *mut raw::uiControl {
            self._inner
        }

        fn from_ptr(ptr: *mut raw::uiControl) -> Self {
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
    assert_eq!(control.handle(), 0);
    assert!(!control.enabled_to_user());
    control.destroy();
    control.verify_set_parent(&control);
    control.free();

    Ok(())
}

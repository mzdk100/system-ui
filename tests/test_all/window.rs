use system_ui::*;

pub fn test_window() -> anyhow::Result<()> {
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
    window.show();
    window.hide();
    assert!(!window.focused());

    Ok(())
}

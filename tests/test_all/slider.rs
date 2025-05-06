use system_ui::*;

pub fn test_slider() -> anyhow::Result<()> {
    let slider = Slider::new(0, 10);
    assert_eq!(0, slider.value());
    slider.set_value(5);
    assert_eq!(5, slider.value());
    slider.set_has_tool_tip(true);
    assert!(slider.has_tool_tip());

    Ok(())
}

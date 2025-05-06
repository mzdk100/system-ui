use system_ui::*;

pub fn test_multi_line_entry() -> anyhow::Result<()> {
    // 创建一个新的MultilineEntry实例
    let entry = MultiLineEntry::new();

    // 测试设置和获取文本
    entry.set_text("测试")?;
    assert_eq!("测试", entry.text()?);

    // 测试追加文本
    entry.append("追加")?;
    assert_eq!("测试追加", entry.text()?);

    // 测试只读属性
    assert!(!entry.read_only());
    entry.set_read_only(true);
    assert!(entry.read_only());

    Ok(())
}

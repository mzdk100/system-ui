use std::time::{Duration, SystemTime};
use system_ui::*;

pub fn test_date_time_picker() -> anyhow::Result<()> {
    use chrono::{DateTime, Local};
    let picker = DateTimePicker::new_date();
    let _time: DateTime<Local> = picker.time().into();
    // println!("{:?}", _time);
    let picker = DateTimePicker::new();
    let time = SystemTime::UNIX_EPOCH + Duration::from_secs(10000);
    picker.set_time(time)?;
    assert_eq!(time, picker.time());

    Ok(())
}

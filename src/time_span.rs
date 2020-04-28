use crate::windows::foundation::TimeSpan;
use std::convert::{From, TryInto};
use std::time::Duration;

// TODO: Better conversion, this is quick and dirty
impl From<Duration> for TimeSpan {
    fn from(duration: Duration) -> Self {
        TimeSpan { duration: (duration.as_millis() * 10000).try_into().unwrap() }
    }
}

#[test]
fn duration_to_time_span() {
    let value1 = TimeSpan{ duration: 1000000 }; // 100 ms
    let value2 = TimeSpan::from(Duration::from_millis(100));
    assert_eq!(value1, value2);

    let value1 = TimeSpan{ duration: 50000000 }; // 5 seconds
    let value2 = TimeSpan::from(Duration::from_secs(5));
    assert_eq!(value1, value2);
}
fn main() {}

#[cfg(test)]
fn bed_time(input: &[(&str, &str)]) -> Vec<String> {
    let mut res = Vec::new();

    for (alarm_time, sleep_duration) in input {
        res.push(single_bed_time(alarm_time, sleep_duration));
    }

    res
}

#[cfg(test)]
fn single_bed_time(alarm_time: &str, sleep_duration: &str) -> String {
    let mut alarm_time_iter = alarm_time.split(':');
    let mut sleep_duration_iter = sleep_duration.split(':');
    let alarm_time_hour: isize = alarm_time_iter.nth(0).unwrap().parse().unwrap();
    let alarm_time_minute: isize = alarm_time_iter.nth(0).unwrap().parse().unwrap();
    let sleep_duration_hour: isize = sleep_duration_iter.nth(0).unwrap().parse().unwrap();
    let sleep_duration_minute: isize = sleep_duration_iter.nth(0).unwrap().parse().unwrap();

    let mut bed_time_hour = alarm_time_hour - sleep_duration_hour;
    let mut bed_time_minute = alarm_time_minute - sleep_duration_minute;

    if bed_time_hour < 0 {
        bed_time_hour = 24 + bed_time_hour;
    }
    if bed_time_minute < 0 {
        bed_time_minute = 60 + bed_time_minute;
        bed_time_hour -= 1;
    }

    format!("{bed_time_hour:02}:{bed_time_minute:02}")
}

#[test]
fn test_function() {
    assert_eq!(bed_time(&[("07:50", "07:50")]), vec!["00:00"]);
    assert_eq!(
        bed_time(&[("06:15", "10:00"), ("08:00", "10:00"), ("09:30", "10:00")]),
        vec!["20:15", "22:00", "23:30"]
    );
    assert_eq!(
        bed_time(&[("05:45", "04:00"), ("07:10", "04:30")]),
        vec!["01:45", "02:40"]
    );
}

pub fn format_number(num: i64) -> String {
    if num.abs() >= 1_000_000 {
        format!("{:.1}M", num as f64 / 1_000_000.0)
    } else if num.abs() >= 1_000 {
        format!("{:.1}K", num as f64 / 1_000.0)
    } else {
        num.to_string()
    }
}

pub fn format_time(seconds: u64) -> String {
    if seconds < 60 {
        format!("{seconds:02}s")
    } else if seconds < 3600 {
        format!("{}m{:02}s", seconds / 60, seconds % 60)
    } else if seconds < 86400 {
        format!(
            "{}h{:02}m{:02}s",
            seconds / 3600,
            (seconds % 3600) / 60,
            seconds % 60
        )
    } else {
        format!(
            "{}d{:02}h{:02}m",
            seconds / 86400,
            (seconds % 86400) / 3600,
            (seconds % 3600) / 60
        )
    }
}

pub fn format_bytes_per_second(bytes_per_second: f64) -> String {
    let abs_rate = bytes_per_second.abs();

    if abs_rate < 1024.0 {
        format!("{bytes_per_second:.0}B/s")
    } else if abs_rate < 1024.0 * 1024.0 {
        format!("{:.1}KB/s", bytes_per_second / 1024.0)
    } else if abs_rate < 1024.0 * 1024.0 * 1024.0 {
        format!("{:.1}MB/s", bytes_per_second / (1024.0 * 1024.0))
    } else {
        format!("{:.1}GB/s", bytes_per_second / (1024.0 * 1024.0 * 1024.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(123), "123");
        assert_eq!(format_number(1500), "1.5K");
        assert_eq!(format_number(1500000), "1.5M");
        assert_eq!(format_number(-1500), "-1.5K");
    }

    #[test]
    fn test_format_time() {
        assert_eq!(format_time(45), "45s");
        assert_eq!(format_time(90), "1m30s");
        assert_eq!(format_time(3661), "1h01m01s");
        assert_eq!(format_time(90061), "1d01h01m");
    }

    #[test]
    fn test_format_bytes_per_second() {
        assert_eq!(format_bytes_per_second(512.0), "512B/s");
        assert_eq!(format_bytes_per_second(1536.0), "1.5KB/s");
        assert_eq!(format_bytes_per_second(1572864.0), "1.5MB/s");
        assert_eq!(format_bytes_per_second(1610612736.0), "1.5GB/s");
    }
}

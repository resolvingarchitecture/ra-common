/// Common Utils Module

/// Wait Module
pub mod wait {
    use std::{thread, time};
    pub fn wait_a_day(days: u64) {
        thread::sleep(time::Duration::from_secs(days * 24 * 60 * 60));
    }
    pub fn wait_a_hour(hours: u64) {
        thread::sleep(time::Duration::from_secs(hours * 60 * 60));
    }
    pub fn wait_a_minute(minutes: u64) {
        thread::sleep(time::Duration::from_secs(minutes * 60));
    }
    pub fn wait_a_sec(seconds: u64) {
        thread::sleep(time::Duration::from_secs(seconds));
    }
    pub fn wait_a_ms(millis: u64) {
        thread::sleep(time::Duration::from_millis(millis));
    }
    pub fn wait_a_mic(mics: u64) {
        thread::sleep(time::Duration::from_micros(mics));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
use std::thread;
use std::time::Duration;

/// Örnekte thread'leri belli süre durdurup uzun çalışmaları simüle etmek içindir.
pub fn sleep_while(seconds: f32) {
    thread::sleep(Duration::from_secs_f32(seconds));
}

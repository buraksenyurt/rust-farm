/*
   Örnek senaryodaki amacımız Update fonksiyonu için test metodu yazmak.
   Ancak bunu Watcher trait'ini mock'layarak yapmak istiyoruz.
*/
pub trait Watcher {
    fn send(&self, message: &str);
}

pub struct RequestTracker<'a, T: 'a + Watcher> {
    watcher: &'a T,
    current_size: usize,
    max: usize,
}

impl<'a, T> RequestTracker<'a, T>
where
    T: Watcher,
{
    pub fn new(watcher: &T, max: usize) -> RequestTracker<T> {
        RequestTracker {
            watcher,
            current_size: 0,
            max,
        }
    }
    pub fn update(&mut self, value: usize) {
        self.current_size = value;

        let percentage = self.current_size as f64 / self.max as f64;

        match SystemStatus::from(percentage) {
            SystemStatus::Normal => self.watcher.send("Sistem nominal"),
            SystemStatus::Overloading => self.watcher.send("Sistem aşırı yüklenmekte"),
            SystemStatus::Critical => self.watcher.send("Sistem kritik seviyede"),
            SystemStatus::Overloaded => self.watcher.send("Sistem aşırı yüklendi"),
            SystemStatus::Unknown => {}
        }
    }
}

enum SystemStatus {
    Normal,
    Overloading,
    Critical,
    Overloaded,
    Unknown,
}

impl From<f64> for SystemStatus {
    fn from(value: f64) -> Self {
        if value > 0.0 && value < 0.50 {
            Self::Normal
        } else if value > 0.50 && value < 0.80 {
            Self::Overloading
        } else if value > 0.80 && value < 0.90 {
            Self::Critical
        } else if value > 0.90 {
            Self::Overloaded
        } else {
            Self::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockWatcher {
        messages: Vec<String>,
    }
    impl MockWatcher {
        fn new() -> MockWatcher {
            MockWatcher { messages: vec![] }
        }
    }

    impl Watcher for MockWatcher {
        /*
            Mecburen mutable self kullandık ancak bu
            error[E0053]: method `send` has an incompatible type for trait
            hatasına sebep olur çünkü Watcher trait'inin bildirdiği ile uyumlu değildir.
         */
        fn send(&mut self, message: &str) {
            self.messages.push(String::from(message))
        }
    }

    #[test]
    fn should_send_message_over_90_percent() {
        let mock_watcher = MockWatcher::new();
        let mut request_tracker = RequestTracker::new(&mock_watcher, 100);
        request_tracker.update(91);
        assert_eq!(mock_watcher.messages.len(), 1);
    }
}

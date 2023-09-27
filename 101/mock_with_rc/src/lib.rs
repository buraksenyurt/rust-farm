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

    #[test]
    fn it_works() {}
}

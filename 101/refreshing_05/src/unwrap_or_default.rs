pub struct Window {
    title: String,
    width: u32,
    height: u32,
}
pub struct WindowSettings {
    title: String,
    width: u32,
    height: u32,
}

impl Default for WindowSettings {
    fn default() -> WindowSettings {
        WindowSettings {
            title: String::from("Game Window"),
            width: 640,
            height: 480,
        }
    }
}

fn add_window(windows_settings: Option<WindowSettings>) -> Window {
    let settings = windows_settings.unwrap_or_default();
    Window {
        title: settings.title,
        width: settings.width,
        height: settings.height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_window_with_default_settings_test() {
        let window = add_window(None);
        assert_eq!(window.title, "Game Window");
        assert_eq!(window.width, 640);
        assert_eq!(window.height, 480);
    }

    #[test]
    fn add_window_with_settings_test() {
        let window_settings = WindowSettings {
            title: String::from("Airborne"),
            width: 1280,
            height: 960,
        };
        let window = add_window(Some(window_settings));
        assert_eq!(window.title, "Airborne");
        assert_eq!(window.width, 1280);
        assert_eq!(window.height, 960);
    }
}

use std::env;

pub fn get_file_path(relative_path: &str) -> String {
    match env::var("DOCKER_ENV") {
        Ok(_) => format!("/usr/local/bin/{}", relative_path),
        Err(_) => relative_path.to_string(),
    }
}

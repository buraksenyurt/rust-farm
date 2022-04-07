pub struct Path {
    pub prefix: String,
}

impl Path {
    pub fn define(&self, following_path: String) -> String {
        self.prefix.to_string() + &following_path
    }
}

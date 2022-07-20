pub struct Logger {
    pub enabled: bool,
}

impl Logger {
    pub fn log(&self, message: &str) {
        if self.enabled {
            println!("{}", message);
        }
    }
}

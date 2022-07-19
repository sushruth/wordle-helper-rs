pub struct Logger {
    pub enabled: bool,
}

impl Logger {
    pub fn silence(&mut self) {
        self.enabled = false;
    }

    pub fn log(&self, message: &str) {
        if self.enabled {
            println!("{}", message);
        }
    }
}

pub(crate) static LOGGER: Logger = Logger { enabled: true };

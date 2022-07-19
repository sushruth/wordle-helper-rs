static enabled: bool = true;

pub fn log(message: &str) {
    if enabled {
        println!("{}", message);
    }
}

pub fn silence() {
    enabled = false;
}

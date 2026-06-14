pub struct Config {
    pub indent_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self { indent_size: 2 }
    }
}


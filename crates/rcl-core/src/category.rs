pub struct ConfigCategory {
    pub name: String,
    pub description: Option<String>,
}

impl ConfigCategory {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
        }
    }
}

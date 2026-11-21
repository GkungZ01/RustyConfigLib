pub struct OptionGroup {
    pub name: String,
    pub description: Option<String>,
    pub collapsed: bool,
}

impl OptionGroup {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            collapsed: false,
        }
    }
}

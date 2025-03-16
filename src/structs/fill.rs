pub struct Fill {
    pub fill_string: Option<String>,
    pub gradient: Option<crate::types::G>,
}

impl Fill {
    pub fn new() -> Self {
        Self {
            fill_string: None,
            gradient: None,
        }
    }
}

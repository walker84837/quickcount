pub struct QCEditor {
    pub content: String,
}

impl Default for QCEditor {
    fn default() -> Self {
        Self {
            content: String::new(),
        }
    }
}

impl QCEditor {
    pub fn add_new_content(&mut self, content: String) {
        self.content += &content;
    }
}

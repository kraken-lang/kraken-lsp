use std::sync::Arc;

pub struct Document {
    text: Arc<String>,
    lines: Vec<String>,
}

impl Document {
    #[allow(dead_code)]
    pub fn new(text: String) -> Self {
        let lines = text.lines().map(|s| s.to_string()).collect();
        Self {
            text: Arc::new(text),
            lines,
        }
    }

    pub fn update(&mut self, text: String) {
        self.lines = text.lines().map(|s| s.to_string()).collect();
        self.text = Arc::new(text);
    }

    #[allow(dead_code)]
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    #[allow(dead_code)]
    pub fn line(&self, index: usize) -> Option<&str> {
        self.lines.get(index).map(|s| s.as_str())
    }

    #[allow(dead_code)]
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

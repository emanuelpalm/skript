use crate::dia::Diagnostic;

#[derive(Debug)]
pub struct Report {
    diagnostics: Vec<Diagnostic>,
}

impl Report {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}

use crate::analysis::Document;
use tower_lsp::lsp_types::*;

pub fn analyze_document(doc: &Document) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for (line_num, line) in doc.lines().iter().enumerate() {
        if line.contains("TODO") {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: line_num as u32,
                        character: 0,
                    },
                    end: Position {
                        line: line_num as u32,
                        character: line.len() as u32,
                    },
                },
                severity: Some(DiagnosticSeverity::INFORMATION),
                message: "TODO comment found".to_string(),
                source: Some("kraken-lsp".to_string()),
                ..Default::default()
            });
        }
    }

    diagnostics
}

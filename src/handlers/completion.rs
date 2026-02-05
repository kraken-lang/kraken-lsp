use crate::analysis::Document;
use tower_lsp::lsp_types::*;

pub fn provide_completions(_doc: &Document) -> Vec<CompletionItem> {
    let mut completions = Vec::new();

    let keywords = vec![
        "fn", "let", "mut", "const", "if", "else", "while", "for", "in", "match", "return",
        "struct", "enum", "impl", "trait", "type", "unsafe", "move", "async", "await", "dyn",
    ];

    for keyword in keywords {
        completions.push(CompletionItem {
            label: keyword.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Keyword".to_string()),
            ..Default::default()
        });
    }

    let types = vec!["int", "float", "bool", "string", "void"];

    for type_name in types {
        completions.push(CompletionItem {
            label: type_name.to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            detail: Some("Primitive type".to_string()),
            ..Default::default()
        });
    }

    completions
}

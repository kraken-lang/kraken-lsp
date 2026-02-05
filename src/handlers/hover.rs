use crate::analysis::Document;
use tower_lsp::lsp_types::*;

pub fn provide_hover(_doc: &Document, _position: Position) -> Option<Hover> {
    Some(Hover {
        contents: HoverContents::Scalar(MarkedString::String("Kraken Language Server".to_string())),
        range: None,
    })
}

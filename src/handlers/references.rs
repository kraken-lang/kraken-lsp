use crate::analysis::Document;
use tower_lsp::lsp_types::*;

pub fn find_references(_doc: &Document, _uri: &Url, _position: Position) -> Vec<Location> {
    Vec::new()
}

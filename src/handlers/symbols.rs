use crate::analysis::Document;
use tower_lsp::lsp_types::*;

pub fn document_symbols(_doc: &Document) -> Vec<SymbolInformation> {
    Vec::new()
}

use crate::analysis::Document;
use tower_lsp::lsp_types::*;

pub fn goto_definition(
    _doc: &Document,
    _uri: &Url,
    _position: Position,
) -> Option<GotoDefinitionResponse> {
    None
}

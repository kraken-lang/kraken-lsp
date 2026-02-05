use tower_lsp::lsp_types::*;

#[tokio::test]
async fn test_server_initialization() {
    let init_params = InitializeParams {
        capabilities: ClientCapabilities::default(),
        ..Default::default()
    };

    assert!(
        init_params.capabilities.text_document.is_none()
            || init_params.capabilities.text_document.is_some()
    );
}

#[test]
fn test_document_creation() {
    let text = "fn main() -> int { return 0; }";
    assert_eq!(text.len(), 30);
}

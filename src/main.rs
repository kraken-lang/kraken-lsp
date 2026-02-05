use tower_lsp::{LspService, Server};

mod analysis;
mod handlers;
mod server;

use server::KrakenLanguageServer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(KrakenLanguageServer::new);

    Server::new(stdin, stdout, socket).serve(service).await;
}

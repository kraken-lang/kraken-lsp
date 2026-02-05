<div align="center">
    <img width="auto" height="118" alt="Kraken Language" src="https://raw.githubusercontent.com/kraken-lang/.github/refs/heads/main/images/kraken-logo.png">
    <h1><sub><sup>KRAKEN LANGUAGE</sup></sub><br>Language Server Protocol</h1>
</div>

Language Server Protocol implementation for the Kraken programming language. Provides IDE features like auto-completion, diagnostics, go-to-definition, and more.

Current version: `v0.8.42`

## Features

The Kraken LSP server provides intelligent code assistance for Kraken development:

- **Completion**: Context-aware suggestions for keywords, types, and identifiers
- **Diagnostics**: Real-time error detection and warning messages
- **Hover**: Type information and documentation when hovering over symbols
- **Go to Definition**: Jump to where functions, types, and variables are defined
- **Find References**: Locate all usages of a symbol across your codebase
- **Document Symbols**: Outline view showing functions, structs, and other declarations
- **Semantic Highlighting**: Enhanced syntax coloring based on semantic analysis

## Installation

### From Source

```bash
git clone https://github.com/kraken-lang/kraken-lsp.git
cd kraken-lsp
cargo build --release
```

The compiled binary will be at `target/release/kraken-lsp`.

### System-wide Installation

```bash
cargo install --path .
```

This installs `kraken-lsp` to your Cargo bin directory.

## Editor Configuration

### Visual Studio Code

The Kraken VSCode extension automatically uses this language server. Install it from the marketplace or configure manually:

```json
{
  "kraken.languageServer.path": "/path/to/kraken-lsp"
}
```

### Neovim

Using `nvim-lspconfig`:

```lua
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

if not configs.kraken_lsp then
  configs.kraken_lsp = {
    default_config = {
      cmd = { 'kraken-lsp' },
      filetypes = { 'kraken' },
      root_dir = lspconfig.util.root_pattern('Cargo.toml', '.git'),
      settings = {},
    },
  }
end

lspconfig.kraken_lsp.setup{}
```

### Helix

Add to your `languages.toml`:

```toml
[[language]]
name = "kraken"
scope = "source.kraken"
file-types = ["kr"]
roots = ["Cargo.toml"]
comment-token = "//"
language-servers = ["kraken-lsp"]

[language-server.kraken-lsp]
command = "kraken-lsp"
```

### Emacs (eglot)

```elisp
(add-to-list 'eglot-server-programs
             '(kraken-mode . ("kraken-lsp")))
```

### Sublime Text (LSP)

Add to LSP settings:

```json
{
  "clients": {
    "kraken-lsp": {
      "enabled": true,
      "command": ["kraken-lsp"],
      "selector": "source.kraken"
    }
  }
}
```

## Usage

The language server communicates via JSON-RPC over standard input/output. Most editors handle this automatically through their LSP client.

### Manual Testing

You can test the server manually using any LSP client or by sending JSON-RPC messages:

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | kraken-lsp
```

### Logging

Enable detailed logging with the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug kraken-lsp
```

Log levels: `error`, `warn`, `info`, `debug`, `trace`

## Supported LSP Capabilities

### Text Document Synchronization
- Full document sync
- Incremental updates (planned)

### Language Features
- **textDocument/completion**: Auto-completion suggestions
- **textDocument/hover**: Symbol information on hover
- **textDocument/definition**: Go to definition
- **textDocument/references**: Find all references
- **textDocument/documentSymbol**: Document outline
- **textDocument/semanticTokens**: Semantic highlighting

### Workspace Features
- **workspace/symbol**: Workspace-wide symbol search (planned)
- **workspace/didChangeConfiguration**: Dynamic configuration updates (planned)

## Architecture

The server is built with a modular architecture:

```
src/
├── main.rs           # Entry point and server initialization
├── server.rs         # Core LSP server implementation
├── handlers/         # LSP request handlers
│   ├── completion.rs
│   ├── diagnostics.rs
│   ├── hover.rs
│   ├── definition.rs
│   ├── references.rs
│   └── symbols.rs
└── analysis/         # Code analysis infrastructure
    └── document.rs
```

### Key Components

- **Server**: Manages client communication and document state
- **Handlers**: Process specific LSP requests (completion, hover, etc.)
- **Analysis**: Document parsing and semantic analysis
- **Document**: Represents open files with efficient text access

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
cargo clippy
cargo fmt
```

### Development Mode

Run with debug logging:

```bash
RUST_LOG=debug cargo run
```

## Performance

The server is designed for high performance:

- Async I/O with Tokio for non-blocking operations
- Concurrent document access with DashMap
- Incremental parsing (planned)
- Lazy analysis to minimize latency

Typical response times:
- Completion: < 10ms
- Hover: < 5ms
- Diagnostics: < 50ms (depends on file size)

## Roadmap

### Near-term
- Integration with Kraken compiler for full semantic analysis
- Signature help for function parameters
- Code actions (quick fixes, refactorings)
- Document formatting

### Long-term
- Workspace-wide analysis and caching
- Advanced refactoring operations
- Inlay hints for type information
- Call hierarchy and type hierarchy
- Debug adapter protocol support

## Compatibility

This language server tracks the Kraken compiler version. Version 0.8.42 supports all language features in Kraken v0.8.42.

## Contributing

Contributions are welcome. Please ensure:

- Code passes `cargo clippy` with no warnings
- All tests pass with `cargo test`
- New features include tests
- Code is formatted with `cargo fmt`

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

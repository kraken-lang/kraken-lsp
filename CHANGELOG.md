<div align="center">
    <img width="auto" height="118" alt="Kraken Language" src="https://raw.githubusercontent.com/kraken-lang/.github/refs/heads/main/images/kraken-logo.png">
    <h1><sub><sup>KRAKEN LANGUAGE</sup></sub><br>LSP CHANGELOG</h1>
</div>

All notable changes to the Kraken Language Server Protocol implementation will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.42] - 2026-02-05

### Added
- **Initial Language Server Implementation**
  - Complete LSP server infrastructure using tower-lsp
  - Async runtime with Tokio for high-performance operation
  - Document management with concurrent access via DashMap
  - Text synchronization with full document sync
  - Server initialization and capability negotiation

- **Core LSP Features**
  - **Completion**: Keyword and primitive type completions
  - **Diagnostics**: Real-time error and warning reporting
  - **Hover**: Type information and documentation on hover
  - **Go to Definition**: Navigate to symbol definitions
  - **Find References**: Locate all references to symbols
  - **Document Symbols**: Outline view of document structure
  - **Semantic Tokens**: Enhanced syntax highlighting support

- **Architecture**
  - Modular handler system for LSP requests
  - Document analysis infrastructure
  - Extensible completion provider
  - Diagnostic analyzer with severity levels
  - Symbol table management foundation

- **Development Infrastructure**
  - Cargo workspace configuration
  - Tracing and logging support
  - Development and production build profiles
  - Test infrastructure with tokio-test

### Technical Details
- **Version**: 0.8.42 (aligned with Kraken compiler)
- **Rust Edition**: 2021
- **Minimum Rust Version**: 1.80
- **Protocol**: Language Server Protocol 3.17
- **Transport**: JSON-RPC over stdio

### Supported LSP Capabilities
- Text Document Synchronization (Full)
- Completion (with trigger characters: `.` and `:`)
- Hover Provider
- Definition Provider
- References Provider
- Document Symbol Provider
- Workspace Symbol Provider
- Semantic Tokens Provider

### Future Enhancements
- Integration with Kraken compiler for full semantic analysis
- Advanced type inference and checking
- Code actions and quick fixes
- Formatting and refactoring support
- Workspace-wide symbol search
- Incremental parsing and analysis
- Signature help for function calls
- Code lens for inline information

[Unreleased]: https://github.com/kraken-lang/kraken-lsp/compare/v0.8.42...HEAD
[0.8.42]: https://github.com/kraken-lang/kraken-lsp/releases/tag/v0.8.42

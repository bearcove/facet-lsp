# facet-lsp

`facet-lsp` provides a deliberately small, Facet-derived subset of the Language Server Protocol 3.17 data model.

It includes:

- the LSP types needed by small language servers;
- JSON-RPC 2.0 messages and stdio `Content-Length` framing;
- byte-offset conversion to LSP UTF-16 positions and ranges;
- LSP semantic-token delta encoding.

The crate uses `facet` and `facet-json` directly. It does not depend on Serde, `tower-lsp`, `lsp-server`, Vox, or compiler-specific types, and it is not intended to mirror the complete LSP specification or provide a language-server runtime.

## License

Licensed under either of Apache License, Version 2.0 or the MIT license, at your option.

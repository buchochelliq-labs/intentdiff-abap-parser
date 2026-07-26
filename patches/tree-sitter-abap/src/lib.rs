//! Tree-sitter grammar for SAP ABAP (sqry fork), patched for this workspace.
//!
//! Source grammar: <https://github.com/mkoval1/tree-sitter-abap>
//! sqry source commit: c7604df9e25d56ae879fa25694fd9f2ddbab05d8

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_abap() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for SAP ABAP.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_abap) };

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &str = include_str!("node-types.json");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_types_not_empty() {
        assert!(!NODE_TYPES.is_empty());
    }

    #[test]
    fn language_loads() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&LANGUAGE.into())
            .expect("ABAP grammar should load");
    }
}

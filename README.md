# Rust Html Doc Parse

Parser for documentation you see on [docs.rs](https://docs.rs/).  
Generates an abstract syntax tree of the document (`vec![Title(x), Section(x), ...]`)
and parses some meta information (authors, repository links, version, coverage, ...).

One big html in, one big struct out, parsed by best effort, because
some fields might not exist (usually only outside of docs.rs).  
Take a look at a test, to see how it works.

This crate came into existence for [Ruder](https://github.com/julianbuettner/ruder).

## Note: unfinished product

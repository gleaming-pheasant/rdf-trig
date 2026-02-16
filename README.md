# rdf-lite
A crate for quick parsing of RDF triples to and from Rust native types.

## No Blank Nodes
This crate does not implement blank nodes. Implementations may be developed in 
future, but because it extends beyond current use cases, it is not on any 
roadmap.

## `Cow`s, everywhere...
For all `str` types, this crate defines `Cow<'static, str>`, and constructor 
methods assign parameters with `Into<Cow<'static, str>>`. This is to improve 
performance and decrease memory consumption when using RDF Nodes which are known 
at compile time; static str references can be used.

If you are dynamically defining all - or most - nodes and their namespaces, do 
not use this crate; the overhead of using `Cow` would be wasteful.
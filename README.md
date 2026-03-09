# rdf-trig
A crate for quick formatting of RDF triples in 
[TriG](https://en.wikipedia.org/wiki/TriG_(syntax)) from Rust native types.

Provides verified `Graph`, `Triple`, `Subject`, `Predicate` and `Object` types 
for adding to a `DataStore`. It also provides traits for writing the `DataStore` 
and individual elements to TriG.

The main impetus of this crate is speed. It uses types and methods explicitly 
for reading and writing types with as little reallocation, copying and memory 
peaking as possible.

Please see the crate documentation in `./src/lib.rs` for usage examples.

## Incomplete Escape Sequences
For the purposes of speed (and given this crate's limited functionality), no 
types are rejected on input. For example, a local name (generally the prefix for 
a namespace) can be passed to this crate with non-printables, whitespace, or 
other characters which need escaping.

Rather than rejecting these inputs and creating a bottleneck, the crate simply 
escapes - and in some cases refuses to write - characters on creating the TriG 
output. E.g. a local name (such as a prefix) declared with a line break, will be 
accepted, but the line break will simply be removed on the output.

__The escape sequences are also not completely valid!__ The crate doesn't 
exclude some of the random characters that the TriG specification excludes, such 
as this weird exclusion of the multiplication sign:

 > [#0370-#037D] | [#037F-#1FFF]

Instead, the crate hopes to not encounter them, trusts that users will exclude 
them manually, or that users' graph databases will tolerate the invalid 
characters.

Again, this is to improve speed; parsing single bytes rather than verifying 
unicode characters.

## Panic! Don't create too many triples!
__Warning!__ When stored in a `DataStore`, this crate interns every element 
that makes up its structure; `Namespace`s, `Graph`s, `Triple`s, `Quad`s and 
all of their `nodes`.

It maintains a group of collections for each specific type, and everything is 
represented with an index to prevent duplication.

This index is converted to a `u32` to be more cache friendly on 64-bit systems, 
but be warned, any collection with a quantity greater than maximum `u32` max 
(4,294,967,295) will cause applications to panic.

The crate also appends numbers to `Namespace` prefixes if you've used the same 
`prefix` for two different `iri`s. The limit for "suffixed prefixes" is the 
`u8` maximum (255). Exceeding this number of duplicated prefixes will cause 
applications to panic.

E.g. declaring *"owl"* as the prefix for both *"http://www.w3.org/2002/07/owl#"* 
and *"http://www.w3.org/2002/07/owl"* (anchor removed) will provide prefixes of 
*"owl"* and *"owl0"* respectively.

If you've messed up, and you're declaring each endpoint as its own namespace 
with a shared prefix, this crate will panic!

### So many Ids...
On that note... this crate may seem overkill initially. Interning every element 
has added a lot of boilerplate to the source code. However, this hasn't been 
done to gain minor performance wins, it has been done out of necessity.

A graph will have its own namespace, a node (if it is IRI) will have its own 
namespace. In order to limit the repeated declaration of some very lengthy IRIs, 
you already have to intern each graph's namespace in the same master store as 
each IRIs namespace (whether the node is related to a graph or not).

Given RDF's propensity to use a very small number of namespaces a very large 
number of times, it makes sense to use statics and intern everything in order to 
make parsing large graphs feasible.

## `Cow`s, everywhere...
For all `str` types, this crate defines a `Cow<'a, str>`, and constructor 
methods assign parameters with the dynamic `Into<Cow<'a, str>>` type.

This is to improve performance and decrease memory consumption when using RDF 
"nodes" which are known at compile time; `&'static str` references can be used 
in place of dynamically defined `String`s.

Despite the propaganda, RDF isn't actually "machine understandable", so 
hardcoding namespaces and iris is very common practice (see the de-facto 
standard [RDF4J](https://rdf4j.org/documentation/tutorials/getting-started/)). 
This crate even provides many `const` namespaces out of the box to assist with 
this (dcterms, foaf, owl, rdf, rdfs, skos, etc.). If you use any of these, it 
makes sense to stick with the `Cow`s.

If, however, you are dynamically defining all - or most - nodes and their 
namespaces, do not use this crate; the overhead of using `Cow` would be 
wasteful.

## Not Suitable for Broadcast
To assist in speed, this crate implements the 
[aHash](https://github.com/tkaitchuck/ahash) hashing algorithm. The algorithm is 
liable to change, may produce different hashes based on the platform and is not 
HashDOS resistant, so do not use this crate for any distributed platforms.

Additionally, this crate does not offer any means to index `Triple`s or `Quads`. 
It should simply be used for casting to RDF types and writing out to TriG.
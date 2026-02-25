# rdf-lite
A crate for quick parsing of RDF triples to and from Rust native types.

Provides verified `Graph`, `Triple`, `Subject`, `Predicate` and `Object` types 
for adding to a `DataStore`. It also provides traits for writing the `DataStore` 
and individual elements to [TriG](https://en.wikipedia.org/wiki/TriG_(syntax)), 
with more RDF formats planned.

# TODO!
 - Tests, tests and more tests!
 - Update Namespace store to return error if prefix exists for different IRI.
 - Add escape encodings to IRIs and literals.
 - Add validation to existing literal types.

## Panic! Don't create too many triples!
__Warning!__ When stored in a `DataStore`, this crate interns every element 
that makes up its structure; `Graph`s, `Triple`s, `Quad`s and `nodes`.

It maintains a group of hashing collections, and everything is represented with 
an index to prevent duplication. This index is converted to a `u32` to be more 
cache friendly on 64-bit systems, but be warned, any collection which reaches 
over the maximum `u32` size (4,294,967,295) will cause applications to panic.

### So many Ids...
On that note... this crate may seem overkill initially. Interning every element 
has added a lot of boilerplate to the source code. However, this hasn't been 
done to gain minor performance wins, it has been done out of necessity.

A graph will have its own namespace, a node - if it is an IRI node - will have 
its own namespace. In order to limit the repeated declaration of some very 
lengthy IRIs, you already have to intern each graph's namespace in the same 
master store as each IRIs namespace (whether the node is related to a graph or 
not).

Given RDF's propensity to use a very small number of namespaces a very large 
number of times, it was a no brainer to use statics and intern everything in 
order to make large graph parsing feasible.

## `Cow`s, everywhere...
For all `str` types, this crate defines a `Cow<'static, str>`, and constructor 
methods assign parameters with the `Into<Cow<'static, str>>` dynamic type.

This is to improve performance and decrease memory consumption when using RDF 
"nodes" which are known at compile time; static str references can be used.

Despite the propaganda, RDF isn't actually "machine understandable", so 
hardcoding namespaces and iris is very common practice (see the de-facto standard 
[RDF4J](https://rdf4j.org/documentation/tutorials/getting-started/)). So this 
crate even provides many `const` namespaces out of the box to assist with this 
(aocat, dcterms, foaf, owl, rdf, rdfs, skos, etc.). If you use any of these, it 
makes sense to stick with the `Cow`s.

If, however, you are dynamically defining all - or most - nodes and their 
namespaces, do not use this crate; the overhead of using `Cow` would be wasteful.

## Not Suitable for Broadcast
To assist in speed, this crate implements the 
[aHash](https://github.com/tkaitchuck/ahash) hashing algorithm. The algorithm is 
liable to change (and may produce different hashes based on the platform) and 
not HashDOS resistant, so do not use this crate for any distributed platforms.

Additionally, this crate does not offer any means to index `Triple`s or `Quads`. 
It should simply be used for casting to RDF types and writing out RDF formats.
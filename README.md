# rdf-trig
A crate for quick formatting of RDF triples in 
[TriG](https://en.wikipedia.org/wiki/TriG_(syntax)) from Rust native types.

Provides verified `Graph`, `Triple`, `Subject`, `Predicate` and `Object` types 
for adding to a `TripleStore`. It also provides traits for writing the 
`TripleStore` and individual elements to TriG on any `impl std::io::Write`.

The main impetus of this crate is speed. It uses types and methods explicitly 
for reading and writing types with as little reallocation or copying as 
possible.

__Warning__ This crate is pre-stable release. All releases (regardless of minor 
or major), will probably have breaking changes.

## Usage Examples
### Add Multiple Triples to a Graph
```rust
use rdf_trig::{
    IriNode, LangStringLiteral, Namespace, TripleStore, Triple, WriteTriG
};
use rdf_trig::namespaces::statics::OWL;
use rdf_trig::nodes::predicate::RDF_TYPE;

let mut store = TripleStore::new();

let my_schema = Namespace::new("schema", "http://www.example.com/ontology#")
    .unwrap();

let my_objects = Namespace::new("nodes", "http://www.example.com/")
    .unwrap();

let graph = IriNode::new(&my_objects, "MyGraph");

// Using a reference to graph uses Clone internally, but as all str values are 
// Cows, and no allocation is done except on first insertion of a value into the 
// TripleStore, this overhead is virtually free.
let type_triple = Triple::new_with_graph(
    &graph,
    IriNode::new(&my_objects, "Object123"),
    RDF_TYPE,
    IriNode::new(OWL, "Thing")
);

store.add_triple(type_triple);

let label_triple = Triple::new_with_graph(
    &graph,
    IriNode::new(&my_objects, "Object123"),
    IriNode::new(my_schema, "hasCustomLabel"),
    LangStringLiteral::new("is a Thing", "en").unwrap()
);

store.add_triple(label_triple);

let mut buf = vec![];

store.write_trig(&mut buf).unwrap();

let string_output = String::from_utf8(buf).unwrap();

// There is no guarantee of the output order here, hence .contains() rather than 
// assert_eq!().
assert!(string_output.contains(
    "@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .\n"
));
assert!(string_output.contains(
    "@prefix owl: <http://www.w3.org/2002/07/owl#> .\n"
));
assert!(string_output.contains(
    "@prefix schema: <http://www.example.com/ontology#> .\n"
));
assert!(string_output.contains(
    "<http://www.example.com/MyGraph> {"
));
assert!(string_output.contains(
    "nodes:Object123 a owl:Thing ."
));
assert!(string_output.contains(
    "nodes:Object123 schema:hasCustomLabel \"is a Thing\"@en ."
));

```

## A Note on Accepted Values
### *All Escaped Values*
For the purposes of speed (and given this crate's limited functionality), no 
types are rejected on input. A local name can be passed to this crate with 
whitespace, or characters in need of escaping, etc.

Rather than rejecting these inputs and creating a bottleneck, the crate simply 
escapes - and in some cases refuses to write - characters in the TriG output. 
For instance, a local name (such as a prefix) declared with a line break (\r\n 
or \n), will be accepted, but the line break will simply be removed on the 
output.

__But, the escape sequences are also not completely valid!__ The crate doesn't 
exclude some of the random characters that the TriG specification excludes, 
such as this exclusion of the multiplication sign:

 > [#0370-#037D] | [#037F-#1FFF]

Instead, the crate hopes to not encounter them, trusts that users will exclude 
them manually, or that users' graph databases will tolerate the invalid 
characters.

Again, this is to improve speed; parsing bytes based on their base2 value, 
rather than verifying individual characters.

### *IRIs*
This crate - as with most implementations of RDF - has an awkward relationship 
with IRIs.

To a certain degree, it has to trust that param separators, path separators, 
etc. are where they should. Using an example from the 
[TriG specification](https://www.w3.org/TR/trig/#sec-escapes) to explain:

> %-encoded sequences are in the character range for IRIs and are explicitly 
> allowed in local names. These appear as a '%' followed by two hex characters 
> and represent that same sequence of three characters. These sequences are not 
> decoded during processing. A term written as <http://a.example/%66oo-bar> in 
> TriG designates the IRI http://a.example/%66oo-bar and not IRI 
> http://a.example/foo-bar. A term written as ex:%66oo-bar with a prefix @prefix 
> ex: <http://a.example/> also designates the IRI http://a.example/%66oo-bar.

Therefore, the only verification that this crate does is on namespace IRIs 
(the base IRI, and not any local names). Once those are verified, any appended 
local_names are simply trusted to be in a valid format. This crate only assumes 
that if a local name as a [PLX](https://www.w3.org/TR/trig/#grammar-production-PLX) 
would need any characters escaping, the full escaped URL should be output (e.g. 
`<https://www.example.com/Not%20Valid%20PLX>`).

### *gYears*
To align with common practices - but, ironically, not the XML Schema - this 
crate does not output valid `xsd:gYear`s. As with most modern software - 
including GraphDB which accepts gYears in this format - a year is taken as 
entered. The [XML Schema](https://www.w3.org/TR/xmlschema11-2/#gYear) specifies 
that a year shorter than 4 digits is pre-padded with zeroes (e.g. the year 
"*123*" should become "*0123*"). The less said about this the better.

This crate also doesn't allow storage of timezone offsets with gYears. Again, 
purely for speed and practicality.

## Don't create too many triples!
__Warning!__ When stored in a `TripleStore`, this crate interns every element 
that makes up its structure; `Namespace`s, `Triple`s, all of their `nodes`.

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

If you've messed up, and you're declaring each local name as its own namespace 
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
methods assign parameters with the impl `Into<Cow<'a, str>>` trait (which allows 
acceptance of `String` and `&'a str` types).

This is to improve performance and decrease memory consumption when using RDF 
"nodes" which are known at compile time; `&'static str` references can be used 
in place of dynamically defined `String`s, and dynamically defined nodes which 
are used repeatedly in different `Triple`s can be shared.

As all values are interned, any newly encountered `&'a str` references are 
elided to `String`s for permanent storage, but then references can simply be 
dismissed if a `&'a str` or `String` is encountered.

RDF isn't actually "machine understandable", so hardcoding namespaces and iris 
is very common practice (see 
[RDF4J](https://rdf4j.org/documentation/tutorials/getting-started/)). This crate 
even provides many `const` namespaces out of the box to assist with this (dcterms, 
foaf, owl, rdf, rdfs, skos, etc.). If you use any of these, it makes sense to 
stick with the `Cow`s.

If, however, you are dynamically defining all - or most - nodes and their 
namespaces, do not use this crate; the overhead of using `Cow` would be 
wasteful.

## Not Suitable for Broadcast
To assist in speed, this crate implements the 
[aHash](https://github.com/tkaitchuck/ahash) hashing algorithm. The algorithm is 
liable to change, may produce different hashes based on the platform and is not 
HashDOS resistant, so do not use this crate for any distributed platforms.

Additionally, this crate does not offer any means to index `Triple`s. It should 
simply be used for casting to RDF types and writing out to TriG.`
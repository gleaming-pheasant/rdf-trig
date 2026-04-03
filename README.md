# rdf-trig
A crate for quick formatting of RDF triples in [N-Quads](https://www.w3.org/TR/n-quads/)
and [TriG](https://www.w3.org/TR/trig/) from Rust native types.

Provides verified `Graph`, `Triple`, `Subject`, `Predicate` and `Object` types 
for adding to a `TripleStore`. It also provides traits for writing the entire 
`TripleStore` and/or any individual elements to TriG on any 
`impl std::io::Write` or `impl tokio::io::AsyncWrite` (with the `tokio` feature 
flag enabled).

The main impetus of this crate is speed. It uses types and methods explicitly 
for reading and writing types with as little reallocation or copying as 
possible.

__Warning__ This crate is pre-stable release. All releases (regardless of minor 
or major), will probably have breaking changes.

## `TriG` or `N-Quads`
When writing RDF triples, there is always a dilemma between choosing long 
strings/excessive memory consumption, and excessive CPU usage. IRIs are long and 
common, but shortening them takes a lot of parsing...
 - Use `N-Quads` when you want to preserve CPU and care less about buffer and 
 output sizes. `N-Quads` prints every triple/quad in full but does so quickly.
 - Use `TriG` when buffer/output size is more of a concern. This implementation 
 of `TriG` groups triples so that reused graphs/subjects/predicates are only 
 output once. But, creating and sorting these groups is expensive.
    - To assist with this trade-off, this implemention of `TriG` only (and 
    automatically) prefixes the XML Schema (`xsd`). All other IRIs are still 
    output in full.

## Usage Examples
### Write `N-Quads`
```rust
use rdf_trig::{Triple, TripleStore};
use rdf_trig::nodes::{DateTimeLiteral, NamedNode, StringLiteral};
use rdf_trig::nodes::statics::{aocat, owl, rdf, rdfs};
use rdf_trig::traits::WriteNQuads;

// Create the master 
let mut ts = TripleStore::new();

let subject_iri = "urn:uuid:29d82556-7fac-4ab8-b1a1-a652d4b1ee36";

ts.add_triple(Triple::new(
    // Cloning existing Nodes only clones references to contained strings.
    NamedNode::new(subject_iri).unwrap().into(),
    rdf::Property::Type.into(),
    owl::Class::Thing.into()
));

ts.add_triple(Triple::new(
    NamedNode::new(subject_iri).unwrap().into(),
    rdfs::Property::Label.into(),
    // Only 2- or 3-digit ASCII alpha language codes are allowed.
    StringLiteral::new("L'étiquette de ma ressource", Some("fr")).unwrap().into()
));

ts.add_triple(Triple::new_with_graph(
    // Add a Graph
    NamedNode::new("https://www.example.com/MyGraph").unwrap().into(),
    NamedNode::new(subject_iri).unwrap().into(),
    aocat::Property::WasCreatedOn.into(),
    DateTimeLiteral::try_from_str("1969-12-13T12:59:30Z").unwrap().into()
));

// Write... traits implement Write, so we need to write to a Vec<u8> and parse.
let mut buf = vec![];
ts.write_nquads(&mut buf).unwrap();
let nquads_string = String::from_utf8(buf).unwrap();

// N-Quads prints the repeated resource in full, and the xsd type in full.
// The `TripleStore` still interns the `NamedNode` only once.
assert_eq!(
    nquads_string,
    "<urn:uuid:29d82556-7fac-4ab8-b1a1-a652d4b1ee36> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Thing> .\n\
    <urn:uuid:29d82556-7fac-4ab8-b1a1-a652d4b1ee36> <http://www.w3.org/2000/01/rdf-schema#label> \"L\\'étiquette de ma ressource\"@fr .\n\
    <urn:uuid:29d82556-7fac-4ab8-b1a1-a652d4b1ee36> <https://www.ariadne-infrastructure.eu/resource/ao/cat/1.1/was_created_on> \"1969-12-13T12:59:30Z\"^^<http://www.w3.org/2001/XMLSchema#dateTime> <https://www.example.com/MyGraph> .\n"
);
```

### Write `TriG`
```rust
use rdf_trig::{Triple, TripleStore};
use rdf_trig::nodes::{DateTimeLiteral, NamedNode, StringLiteral};
use rdf_trig::nodes::statics::{aocat, owl, rdf, skos};
use rdf_trig::traits::WriteTriG;

// Create the master 
let mut ts = TripleStore::new();

let graph_iri = "https://www.example.com/MyGraph";
let subject = NamedNode::new("urn:uuid:29d82556-7fac-4ab8-b1a1-a652d4b1ee36")
    .unwrap();

ts.add_triple(Triple::new_with_graph(
    NamedNode::new(graph_iri).unwrap().into(),
    (&subject).into(),
    rdf::Property::Type.into(),
    owl::Class::Thing.into()
));

ts.add_triple(Triple::new_with_graph(
    NamedNode::new(graph_iri).unwrap().into(),
    (&subject).into(),
    skos::Property::PrefLabel.into(),
    StringLiteral::new_en("My resource's label").into() // English language
));

ts.add_triple(Triple::new(
    // Using the same subject, but without a Graph.
    (&subject).into(),
    aocat::Property::WasCreatedOn.into(),
    DateTimeLiteral::try_from_str("1969-12-13T12:59:30Z").unwrap().into()
));

let mut buf = vec![];
ts.write_trig(&mut buf).unwrap();
let trig_string = String::from_utf8(buf).unwrap();

// TriG has a smaller output, but costs to sort the input in order to generate 
// the output.
assert_eq!(
    trig_string,
    "@prefix xsd: <http://www.w3.org/2001/XMLSchema#> . <urn:uuid:29d82556-7fac-4ab8-b1a1-a652d4b1ee36> <https://www.ariadne-infrastructure.eu/resource/ao/cat/1.1/was_created_on> \"1969-12-13T12:59:30Z\"^^xsd:dateTime . <https://www.example.com/MyGraph> { <urn:uuid:29d82556-7fac-4ab8-b1a1-a652d4b1ee36> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://www.w3.org/2002/07/owl#Thing> ; <http://www.w3.org/2004/02/skos/core#prefLabel> \"My resource\\'s label\"@en . } "
);
```

## Feature Flags
 - `tokio`: Enables async versions of `WriteTriG` (`WriteTriGAsync`) and 
 `WriteNQuads` (`WriteNQuadsAsync`) utilising `tokio::AsyncWrite`,
 - `chrono`: Allows creation of `DateTimeLiteral`s from `chrono::DateTime<_>` 
 types,
 - `time`: Allows creation of `DateTimeLiteral`s from `time::OffsetDateTime` and 
 `time::PrimitiveDateTime` types.

## A Note on Accepted Values
### *All Escaped Values*
For the purposes of speed (and given this crate's limited functionality), no 
types are rejected on input. A local name can be passed to this crate with 
whitespace, or characters in need of escaping, etc.

Rather than rejecting these inputs and creating a bottleneck, the crate simply 
escapes - and in some cases refuses to write - characters in the TriG output. 
For instance, a local name (generally a blank node's label) declared with a line 
break (\r\n or \n), will be accepted, but the line break will simply be removed 
on output to `N-Quads` or `TriG`.

__Warning: The escape sequences are also not completely valid!__ The crate 
doesn't exclude some of the random characters that the TriG specification 
excludes, such as this exclusion of the multiplication sign:

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

Therefore, the crate doesn't automaticaly escape non-ASCII characters or 
whitespace. Instead, it keeps international UTF-8 characters in their original 
format (using the [`fluent_uri` crate](https://github.com/yescallop/fluent-uri-rs), 
not converting to Punycode like other crates and browsers do). Otherwise it 
simply validates IRIs in `NamedNode`s and provides no character encoding.

### *gYears*
To align with common practices - but not the XML Schema - this crate does not 
output valid `xsd:gYear`s. As with most modern software - including GraphDB 
which accepts gYears in this format - a year is taken as entered. The 
[XML Schema](https://www.w3.org/TR/xmlschema11-2/#gYear) specifies that a year 
shorter than 4 digits is pre-padded with zeroes (e.g. the year "*123*" should 
become "*0123*"). The less said about this the better.

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

### So many Ids...
On that note... this crate may seem overkill initially. Interning every element 
has added a lot of boilerplate to the source code. This hasn't been done to gain 
performance wins.

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
elided to owned `Cow`s for permanent storage, but references are simply 
dismissed if an existing `Deref<Target = str>` is encountered.

RDF isn't actually "machine understandable", so hardcoding namespaces and IRIs 
is very common practice (see RDF4J and ResearchSpace as core examples). This 
crate even provides many `const` namespaces out of the box to assist with this 
(owl, rdf, rdfs, etc.). If you use any of these, it makes sense to stick with 
the `Cow`s.

If, however, you are dynamically defining all - or most - nodes, do not use this 
crate; the overhead of using `Cow` would be wasteful.

## Not Suitable for Broadcast
To assist in speed, this crate implements the 
[aHash](https://github.com/tkaitchuck/ahash) hashing algorithm. The algorithm is 
liable to change, may produce different hashes based on the platform and is not 
HashDOS resistant, so do not use this crate for any distributed platforms.

Additionally, this crate does not offer any means to index `Triple`s. It should 
simply be used for casting to RDF types and writing out to TriG.`
---
id: oxixml-xml-rdf-stack
source: oxixml
axes: [Data, Verify, Query, Packaging]
usefulness: high
optimality: high
demand: high
confidence: code
layer5: "Legal/science: Akoma Ntoso / statute XML and RDF export; Schematron/XSD validate the document, XMLDSig signs meaning — none of that is the event-log SoT"
status: closed
channels_failed: []
---

# W3C XML/RDF stack is an ingest pack — SPARQL syntax is not SPARQL execution

User URL: https://github.com/cool-japan/oxixml (cloned `/tmp/cool-japan/oxixml`, not CBM-indexed). README: pure-Rust workspace claiming Xerces+Xalan+FOP+Saxon+Oxigraph *capabilities*; 46 crates; SPARQL **query execution**, SHACL validation, FO→PDF, OWL/RDFS stay in host projects (`oxirs-arq`, `oxirs-shacl`, `oxirs`). Read: `sparql/oxixml-sparql-syntax/src/lib.rs` (SPARQL 1.1 Query/Update → algebra, `spargebra`-shaped, optional `sparql-12` / SPARQL-star); `rdf/oxixml-model/src/lib.rs` (`Graph` / `Dataset` / `GraphName` / `Quad`, oxrdf API parity, RDFC `try_canonicalize`); `validation/oxixml-schematron/src/lib.rs` (ISO Schematron compile → XPath tests → SVRL); `security/oxixml-dsig/src/lib.rs` (XMLDSig Core 1.1: URI→transforms→c14n→digest, then sign `SignedInfo`). Distinct from `paper-named-graphs-rdf-dataset` (Space *noun*), `paper-shacl-sparql-compile` (SHACL→SPARQL complexity), `paper-federated-sparql-query` (`SERVICE` across endpoints), `paper-iso-gql-gpml` (GQL surface).

## 1. Raw idea

One foundation crate family: byte-level XML up through XSD 1.1 / RELAX NG / Schematron / XSLT 3 / XQuery, plus RDF 1.2 terms, Turtle/RDF-XML/JSON-LD/RDFa/HDT, dataset canonicalization, SPARQL **syntax and results** (not ARQ). Schematron is rule/XPath assertions, not a grammar [schematron lib]. XMLDSig signs the *meaning pipeline*, not the file bytes [dsig lib]. `Dataset` is an in-memory interned quad store with named-graph `GraphName` — a *model*, not a WAL.

## 2. STCA applicability

Packaging 090/093: legal and scientific corpora arrive as XML/RDF; this is the **ingest/validate/export port**. Verify: Schematron/XSD/XMLDSig are document receipts, not quantum receipts on the event log. Query: SPARQL algebra parse can feed a compiler; execution must be Kutha leapfrog/homomorphism or an explicit `oxirs` *remote*, not a second SoT. Data: `Graph`/`Dataset` map to named-graph slices already closed as a noun — do not stand up Oxigraph/Jena. SPARQL-star is an export dialect (queued skip), now with a Rust parser.

## 3. Quality / cost

Usefulness high: Kutha will ingest XML/RDF whether or not it speaks SPARQL as SoT. Optimality high *for the pack* (W3C-suite-gated parsers). Cost: depend on `oxixml-xml` / `-schema` / `-schematron` / `-model` / `-dsig` as **libraries**; do not vendor the 46-crate workspace or claim SPARQL exec. HDT is a compression cousin of k² — skip unless a second card.

## 4. Demand

Statutes, dockets, papers, and RDF dumps are files before they are events. Engine demand: fail-closed XML/RDF ingest with signatures and Schematron, then append events.

## 5. Niche → effect

Legal/science: Akoma Ntoso / statute XML and RDF export; Schematron/XSD validate the document, XMLDSig signs meaning — none of that is the event-log SoT.

# Query-relation accumulation research

Start with [the protocol and measured gates](AKITA_RELATION_ACCUMULATION_PROTOCOL.md). It is the current design. The companion certificates, reviews and predecessor notes preserve the research record; historical alternatives and measurements are not new implementation, numerical security, zero-knowledge or post-quantum security claims.

The original workspace research paths are relative symlinks to these canonical files. Edit the versioned files here; do not create a second active protocol or companion-note copy in Jolt.

## Paired source checkouts

These experimental branches require sibling directories named `akita` and `jolt`. The shared transcript, field and polynomial source graph and the embedded trusted schedule use this layout. Neither repository is an independent standalone checkout of this experiment.

```bash
git clone --branch akita-query-relation-accumulation-v8 git@github.com:Loris-EPFL/akita.git akita
git clone --branch akita-query-relation-accumulation-v8 git@github.com:Loris-EPFL/jolt.git jolt
bash akita/autoresearch.sh
```

Use the pinned Rust toolchains and Cargo lockfiles. The harness uses `--offline --locked`; populate the matching dependency caches before an offline run. It runs the retained Dense and genuine Jolt legal-cut matrices at 1, 2, 3 and 256, including complete independent verification and mutation rejection. Its final primary metric is emitted only after both matrices pass. The protocol records which runs have actually completed; the existence of a command is not a passing result.

## Sources and provenance

The original workspace paper corpus, parsed papers, QMD database and embeddings remain local. They are not redistributed in these code forks: redistribution permission and the authenticity of every local paper revision have not been established. [Source provenance](SOURCE_PROVENANCE.md) records the local identities and public references without claiming that an online revision is identical to a local snapshot.

Textual `akita/...` and `jolt/...` citations use the paired workspace root. Crate-relative names, `A:`/`J:` revision aliases and historical line numbers retain their definitions in the cited note; they are not refreshed source anchors. Temporary paths and session-only evidence are historical provenance, not files supplied by a fork checkout. The protocol's explicit assumptions and acceptance limits take precedence over older exploratory notes.

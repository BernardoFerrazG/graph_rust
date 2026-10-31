# Immune System Interaction Graph

[![CI](https://github.com/BernardoFerrazG/graph_rust/actions/workflows/ci.yml/badge.svg)](https://github.com/BernardoFerrazG/graph_rust/actions/workflows/ci.yml)

A Rust library for modelling **immune-system dynamics as a signed, directed interaction graph**, developed as part of an undergraduate research project (*iniciação científica*).

Nodes are biological entities: cells, pathogens and molecules. Edges are interactions such as *Activate*, *Kill*, *Secrete* or *Differentiate*. Each interaction carries a **signal for each endpoint** (`+`, `−` or none), so the graph records not only who interacts with whom but also who benefits and who is harmed. The library answers structural questions about the network and enumerates the interaction **cascades** that connect two entities.

## Model

| Interaction | Effect on source | Effect on target |
| --- | --- | --- |
| Activate, Secrete, Replicate, Recruit | none | `+` |
| Apoptosis, Consume, Decay, Infect, Inhibit, Kill, Phagocytosis | none | `−` |
| Differentiate, Polarize | `−` (the source is converted) | `+` |

## Queries

| Function | Question it answers |
| --- | --- |
| `negative_effectors()` | Which entities harm at least one other entity? |
| `negatively_affected()` | Which entities suffer a negative effect, as a harmed target or as a converted source? |
| `never_negatively_affected()` | Which entities are never harmed by the network? |
| `PathBank::simple_paths(from, to, k)` | Which interaction cascades of at most `k` steps lead from one entity to another? |
| `with_negative_step()` / `without_negative_step()` | Which of those cascades contain a harmful step, and which are purely activating? |

## Example

[`src/main.rs`](src/main.rs) builds a simplified antiviral response. It is an illustrative network, not a curated biological model: a virus infects epithelial cells, macrophages engulf the virus and secrete cytokines that recruit T helper cells, and those activate cytotoxic T cells and B cells. B cells differentiate into plasma cells, which secrete antibodies that inhibit the virus.

```bash
cargo run
```

```
Entities that harm others:
  Virus, Macrophage, Cytotoxic T, Antibody

Entities harmed somewhere in the network:
  Virus, Epithelial cell, B cell

Entities never harmed:
  Macrophage, Cytokine, T helper, Cytotoxic T, Plasma cell, Antibody

Cascades from Macrophage to Virus (up to 8 steps): 2
  [-] Macrophage -Secrete-> Cytokine -Recruit-> T helper -Activate-> B cell -Differentiate-> Plasma cell -Secrete-> Antibody -Inhibit-> Virus (6 steps)
  [-] Macrophage -Phagocytosis-> Virus (1 steps)

2 with a harmful step, 0 purely activating.
```

## Using the library

```rust
use grafo_ic::{Interaction::*, InteractionGraph, PathBank};

let mut g = InteractionGraph::new();
let t = g.add_entity("Cytotoxic T");
let cell = g.add_entity("Infected cell");
g.add_interaction(t, cell, Kill);

assert!(g.negatively_affected().contains(&cell));
let cascades = PathBank::simple_paths(&g, t, cell, 3);
```

## Project structure

```
src/
├── model.rs   # Signal, Interaction and the signal table, Link (edge weight)
├── graph.rs   # InteractionGraph (wrapper over petgraph::DiGraph) and node queries
├── paths.rs   # Path, PathBank, simple-path enumeration and signal filters
├── lib.rs     # Public API
└── main.rs    # Demo network
```

## Development

```bash
cargo test                                    # unit tests
cargo clippy --all-targets -- -D warnings     # lints
cargo fmt
```

CI runs formatting, Clippy (warnings as errors) and the tests on every push.

## Changes from the first version

- The single `main.rs` was split into a library with `model`, `graph` and `paths` modules, plus a demo binary and unit tests.
- **"Input" queries now look at incoming edges.** The old `non_positive_input` and `non_negative_input` iterated over each node's *outgoing* edges. They are replaced by `negatively_affected()`, which considers every effect a node receives: as the target of incoming edges and as the source of outgoing ones.
- Paths are enumerated from the graph (`all_simple_paths`) instead of being built from arbitrary slices of the edge list.
- The compiled `target/` directory was removed from the repository, and a `.gitignore` was added.

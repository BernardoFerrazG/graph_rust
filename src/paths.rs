//! Interaction paths (cascades) and filters over collections of them.

use petgraph::algo::all_simple_paths;
use petgraph::graph::{EdgeIndex, NodeIndex};

use crate::graph::InteractionGraph;

/// A cascade of interactions, e.g. `T helper -Activate-> B cell -Differentiate-> Plasma cell`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub edges: Vec<EdgeIndex>,
}

impl Path {
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    /// True if some step of the cascade has a negative effect on its target.
    pub fn has_negative_step(&self, graph: &InteractionGraph) -> bool {
        self.edges
            .iter()
            .any(|&e| graph.link(e).on_target.is_negative())
    }

    pub fn describe(&self, graph: &InteractionGraph) -> String {
        let mut text = String::new();
        for (i, &edge) in self.edges.iter().enumerate() {
            let (source, target) = graph.endpoints(edge);
            if i == 0 {
                text.push_str(graph.name(source));
            }
            text.push_str(&format!(
                " -{}-> {}",
                graph.link(edge).interaction,
                graph.name(target)
            ));
        }
        text
    }
}

/// A collection of paths that can be queried by signal.
#[derive(Debug, Default, Clone)]
pub struct PathBank {
    paths: Vec<Path>,
}

impl PathBank {
    pub fn new() -> Self {
        Self::default()
    }

    /// All simple paths from `from` to `to` with at most `max_steps` interactions.
    pub fn simple_paths(
        graph: &InteractionGraph,
        from: NodeIndex,
        to: NodeIndex,
        max_steps: usize,
    ) -> Self {
        let inner = graph.inner();
        let mut bank = PathBank::new();
        let max_intermediate = max_steps.saturating_sub(1);
        for nodes in all_simple_paths::<Vec<_>, _>(inner, from, to, 0, Some(max_intermediate)) {
            let edges = nodes
                .windows(2)
                .map(|pair| {
                    inner
                        .find_edge(pair[0], pair[1])
                        .expect("consecutive nodes are linked")
                })
                .collect();
            bank.add(Path { edges });
        }
        bank
    }

    pub fn add(&mut self, path: Path) {
        self.paths.push(path);
    }

    pub fn len(&self) -> usize {
        self.paths.len()
    }

    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Path> {
        self.paths.iter()
    }

    /// Paths in which at least one step harms its target.
    pub fn with_negative_step<'a>(
        &'a self,
        graph: &'a InteractionGraph,
    ) -> impl Iterator<Item = &'a Path> {
        self.paths
            .iter()
            .filter(move |p| p.has_negative_step(graph))
    }

    /// Paths in which no step harms its target (purely activating cascades).
    pub fn without_negative_step<'a>(
        &'a self,
        graph: &'a InteractionGraph,
    ) -> impl Iterator<Item = &'a Path> {
        self.paths
            .iter()
            .filter(move |p| !p.has_negative_step(graph))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Interaction;

    #[test]
    fn enumerates_and_filters_paths() {
        let mut g = InteractionGraph::new();
        let a = g.add_entity("A");
        let b = g.add_entity("B");
        let c = g.add_entity("C");
        g.add_interaction(a, b, Interaction::Activate);
        g.add_interaction(b, c, Interaction::Activate);
        g.add_interaction(a, c, Interaction::Inhibit);

        let bank = PathBank::simple_paths(&g, a, c, 3);
        assert_eq!(bank.len(), 2);
        assert_eq!(bank.with_negative_step(&g).count(), 1);
        assert_eq!(bank.without_negative_step(&g).count(), 1);

        let direct = bank.with_negative_step(&g).next().unwrap();
        assert_eq!(direct.describe(&g), "A -Inhibit-> C");
    }

    #[test]
    fn respects_max_steps() {
        let mut g = InteractionGraph::new();
        let nodes: Vec<_> = (0..5).map(|i| g.add_entity(format!("N{i}"))).collect();
        for pair in nodes.windows(2) {
            g.add_interaction(pair[0], pair[1], Interaction::Activate);
        }
        assert_eq!(PathBank::simple_paths(&g, nodes[0], nodes[4], 3).len(), 0);
        assert_eq!(PathBank::simple_paths(&g, nodes[0], nodes[4], 4).len(), 1);
    }
}

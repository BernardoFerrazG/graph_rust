//! The interaction graph and the structural queries run on it.

use std::collections::BTreeSet;

use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;

use crate::model::{Interaction, Link, Signal};

/// Directed graph whose nodes are biological entities (cells, pathogens,
/// molecules) and whose edges are signed interactions between them.
#[derive(Debug, Default, Clone)]
pub struct InteractionGraph {
    graph: DiGraph<String, Link>,
}

impl InteractionGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entity(&mut self, name: impl Into<String>) -> NodeIndex {
        self.graph.add_node(name.into())
    }

    pub fn add_interaction(
        &mut self,
        source: NodeIndex,
        target: NodeIndex,
        interaction: Interaction,
    ) -> EdgeIndex {
        self.graph.add_edge(source, target, Link::from(interaction))
    }

    pub fn name(&self, node: NodeIndex) -> &str {
        &self.graph[node]
    }

    pub fn link(&self, edge: EdgeIndex) -> Link {
        self.graph[edge]
    }

    pub fn endpoints(&self, edge: EdgeIndex) -> (NodeIndex, NodeIndex) {
        self.graph
            .edge_endpoints(edge)
            .expect("edge belongs to this graph")
    }

    pub fn entity_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn interaction_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn edges(&self) -> impl Iterator<Item = EdgeIndex> + '_ {
        self.graph.edge_indices()
    }

    pub(crate) fn inner(&self) -> &DiGraph<String, Link> {
        &self.graph
    }

    /// Effects a node *receives*: the target-side signal of every incoming edge
    /// plus the source-side signal of every outgoing edge.
    fn received_signals(&self, node: NodeIndex) -> impl Iterator<Item = Signal> + '_ {
        let incoming = self
            .graph
            .edges_directed(node, Direction::Incoming)
            .map(|e| e.weight().on_target);
        let outgoing = self
            .graph
            .edges_directed(node, Direction::Outgoing)
            .map(|e| e.weight().on_source);
        incoming.chain(outgoing)
    }

    /// Entities that negatively affect at least one other entity (e.g. killers, inhibitors).
    pub fn negative_effectors(&self) -> BTreeSet<NodeIndex> {
        self.graph
            .edge_references()
            .filter(|e| e.weight().on_target.is_negative())
            .map(|e| e.source())
            .collect()
    }

    /// Entities that suffer at least one negative effect, either as the target
    /// of a harmful interaction or as the source of a costly one (e.g. a cell
    /// that differentiates into another type).
    pub fn negatively_affected(&self) -> BTreeSet<NodeIndex> {
        self.graph
            .node_indices()
            .filter(|&n| self.received_signals(n).any(Signal::is_negative))
            .collect()
    }

    /// Entities that never suffer a negative effect in the network.
    pub fn never_negatively_affected(&self) -> BTreeSet<NodeIndex> {
        let affected = self.negatively_affected();
        self.graph
            .node_indices()
            .filter(|n| !affected.contains(n))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small() -> (InteractionGraph, [NodeIndex; 4]) {
        let mut g = InteractionGraph::new();
        let virus = g.add_entity("Virus");
        let cell = g.add_entity("Cell");
        let tcell = g.add_entity("T cell");
        let bcell = g.add_entity("B cell");
        g.add_interaction(virus, cell, Interaction::Infect);
        g.add_interaction(tcell, cell, Interaction::Kill);
        g.add_interaction(tcell, bcell, Interaction::Activate);
        (g, [virus, cell, tcell, bcell])
    }

    #[test]
    fn negative_effectors_are_sources_of_harmful_edges() {
        let (g, [virus, _, tcell, _]) = small();
        assert_eq!(g.negative_effectors(), BTreeSet::from([virus, tcell]));
    }

    #[test]
    fn affected_and_unaffected_partition_the_nodes() {
        let (g, [virus, cell, tcell, bcell]) = small();
        assert_eq!(g.negatively_affected(), BTreeSet::from([cell]));
        assert_eq!(
            g.never_negatively_affected(),
            BTreeSet::from([virus, tcell, bcell])
        );
    }

    #[test]
    fn differentiation_counts_against_the_source() {
        let mut g = InteractionGraph::new();
        let b = g.add_entity("B cell");
        let plasma = g.add_entity("Plasma cell");
        g.add_interaction(b, plasma, Interaction::Differentiate);
        assert_eq!(g.negatively_affected(), BTreeSet::from([b]));
    }
}

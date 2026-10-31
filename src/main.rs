//! Demo: a simplified antiviral immune response modelled as an interaction graph.
//! The network is illustrative, not a curated biological model.

use grafo_ic::{Interaction::*, InteractionGraph, PathBank};
use petgraph::graph::NodeIndex;
use std::collections::BTreeSet;

fn print_set(title: &str, graph: &InteractionGraph, nodes: &BTreeSet<NodeIndex>) {
    let names: Vec<_> = nodes.iter().map(|&n| graph.name(n)).collect();
    println!("{title}:\n  {}\n", names.join(", "));
}

fn main() {
    let mut g = InteractionGraph::new();
    let virus = g.add_entity("Virus");
    let cell = g.add_entity("Epithelial cell");
    let macrophage = g.add_entity("Macrophage");
    let cytokine = g.add_entity("Cytokine");
    let t_helper = g.add_entity("T helper");
    let t_killer = g.add_entity("Cytotoxic T");
    let b_cell = g.add_entity("B cell");
    let plasma = g.add_entity("Plasma cell");
    let antibody = g.add_entity("Antibody");

    g.add_interaction(virus, cell, Infect);
    g.add_interaction(virus, virus, Replicate);
    g.add_interaction(macrophage, virus, Phagocytosis);
    g.add_interaction(macrophage, cytokine, Secrete);
    g.add_interaction(cytokine, t_helper, Recruit);
    g.add_interaction(t_helper, t_killer, Activate);
    g.add_interaction(t_helper, b_cell, Activate);
    g.add_interaction(t_killer, cell, Kill);
    g.add_interaction(b_cell, plasma, Differentiate);
    g.add_interaction(plasma, antibody, Secrete);
    g.add_interaction(antibody, virus, Inhibit);

    println!(
        "Network: {} entities, {} interactions\n",
        g.entity_count(),
        g.interaction_count()
    );
    for edge in g.edges() {
        let (s, t) = g.endpoints(edge);
        let link = g.link(edge);
        println!(
            "  {:<16} -{:<13}-> {:<16} effect on source: {:?}, on target: {:?}",
            g.name(s),
            link.interaction.to_string(),
            g.name(t),
            link.on_source,
            link.on_target
        );
    }
    println!();

    print_set("Entities that harm others", &g, &g.negative_effectors());
    print_set(
        "Entities harmed somewhere in the network",
        &g,
        &g.negatively_affected(),
    );
    print_set("Entities never harmed", &g, &g.never_negatively_affected());

    let cascades = PathBank::simple_paths(&g, macrophage, virus, 8);
    println!(
        "Cascades from Macrophage to Virus (up to 8 steps): {}",
        cascades.len()
    );
    for path in cascades.iter() {
        let kind = if path.has_negative_step(&g) { "-" } else { "+" };
        println!("  [{kind}] {} ({} steps)", path.describe(&g), path.len());
    }
    println!(
        "\n{} with a harmful step, {} purely activating.",
        cascades.with_negative_step(&g).count(),
        cascades.without_negative_step(&g).count()
    );
}

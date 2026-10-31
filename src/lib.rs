//! Signed interaction graphs for modelling immune-system dynamics.
//!
//! Entities (cells, pathogens, molecules) are nodes; interactions such as
//! *Activate*, *Kill* or *Differentiate* are directed edges that carry a signal
//! (`+`, `-` or none) for each endpoint. The crate offers structural queries
//! (who harms whom, who is never harmed) and enumeration of interaction cascades.

pub mod graph;
pub mod model;
pub mod paths;

pub use graph::InteractionGraph;
pub use model::{Interaction, Link, Signal};
pub use paths::{Path, PathBank};

//! Domain model: biological interactions and the signal each one carries.

use std::fmt;

/// Effect of an interaction on one of its endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Signal {
    None,
    Plus,
    Minus,
}

impl Signal {
    pub fn is_negative(self) -> bool {
        self == Signal::Minus
    }
}

/// Kinds of interaction between two entities of the immune system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interaction {
    Activate,
    Apoptosis,
    Consume,
    Decay,
    Differentiate,
    Infect,
    Inhibit,
    Kill,
    Phagocytosis,
    Polarize,
    Secrete,
    Replicate,
    Recruit,
}

impl Interaction {
    pub const ALL: [Interaction; 13] = [
        Interaction::Activate,
        Interaction::Apoptosis,
        Interaction::Consume,
        Interaction::Decay,
        Interaction::Differentiate,
        Interaction::Infect,
        Interaction::Inhibit,
        Interaction::Kill,
        Interaction::Phagocytosis,
        Interaction::Polarize,
        Interaction::Secrete,
        Interaction::Replicate,
        Interaction::Recruit,
    ];

    /// `(effect on the source, effect on the target)` of this interaction.
    pub fn signals(self) -> (Signal, Signal) {
        use Interaction::*;
        match self {
            Activate | Secrete | Replicate | Recruit => (Signal::None, Signal::Plus),
            Apoptosis | Consume | Decay | Infect | Inhibit | Kill | Phagocytosis => {
                (Signal::None, Signal::Minus)
            }
            // The source turns into / is converted towards the target.
            Differentiate | Polarize => (Signal::Minus, Signal::Plus),
        }
    }
}

impl fmt::Display for Interaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Edge weight of the interaction graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Link {
    pub interaction: Interaction,
    pub on_source: Signal,
    pub on_target: Signal,
}

impl From<Interaction> for Link {
    fn from(interaction: Interaction) -> Self {
        let (on_source, on_target) = interaction.signals();
        Link {
            interaction,
            on_source,
            on_target,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_interaction_affects_its_target() {
        for interaction in Interaction::ALL {
            assert_ne!(interaction.signals().1, Signal::None, "{interaction}");
        }
    }

    #[test]
    fn conversions_cost_the_source() {
        assert_eq!(
            Interaction::Differentiate.signals(),
            (Signal::Minus, Signal::Plus)
        );
        assert_eq!(
            Interaction::Polarize.signals(),
            (Signal::Minus, Signal::Plus)
        );
        assert_eq!(Interaction::Kill.signals(), (Signal::None, Signal::Minus));
    }

    #[test]
    fn link_copies_signals_from_interaction() {
        let link = Link::from(Interaction::Inhibit);
        assert_eq!(
            (link.on_source, link.on_target),
            Interaction::Inhibit.signals()
        );
    }
}

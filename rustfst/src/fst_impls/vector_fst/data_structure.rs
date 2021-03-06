use crate::algorithms::arc_filters::ArcFilter;
use crate::algorithms::arc_filters::{InputEpsilonArcFilter, OutputEpsilonArcFilter};
use crate::arc::Arc;
use crate::semirings::Semiring;
use crate::StateId;

/// Simple concrete, mutable FST whose states and arcs are stored in standard vectors.
///
/// All states are stored in a vector of states.
/// In each state, there is a vector of arcs containing the outgoing transitions.
#[derive(Debug, PartialEq, Clone)]
pub struct VectorFst<W: Semiring> {
    pub(crate) states: Vec<VectorFstState<W>>,
    pub(crate) start_state: Option<StateId>,
}

// In my opinion, it is not a good idea to store values like num_arcs, num_input_epsilons
// and num_output_epsilons inside the data structure as it would mean having to maintain them
// when the object is modified. Which is not trivial with the MutableArcIterator API for instance.
// Same goes for ArcMap. For not-mutable fst however, it is usefull.
#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct VectorFstState<W: Semiring> {
    pub(crate) final_weight: Option<W>,
    pub(crate) arcs: Vec<Arc<W>>,
}

impl<W: Semiring> VectorFstState<W> {
    pub fn num_arcs(&self) -> usize {
        self.arcs.len()
    }
}

impl<W: Semiring> VectorFstState<W> {
    pub fn num_input_epsilons(&self) -> usize {
        let filter = InputEpsilonArcFilter {};
        self.arcs.iter().filter(|v| filter.keep(v)).count()
    }

    pub fn num_output_epsilons(&self) -> usize {
        let filter = OutputEpsilonArcFilter {};
        self.arcs.iter().filter(|v| filter.keep(v)).count()
    }
}

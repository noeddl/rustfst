use failure::Fallible;
use unsafe_unwrap::UnsafeUnwrap;

use crate::algorithms::dfs_visit::{dfs_visit, Visitor};
use crate::fst_traits::Fst;
use crate::fst_traits::{CoreFst, ExpandedFst, MutableFst};
use crate::Arc;
use crate::StateId;
use crate::NO_STATE_ID;

/// This operation trims an FST, removing states and arcs that are not on successful paths.
///
/// # Example
/// ```
/// # #[macro_use] extern crate rustfst;
/// # use rustfst::utils::transducer;
/// # use rustfst::semirings::{Semiring, IntegerWeight};
/// # use rustfst::fst_impls::VectorFst;
/// # use rustfst::algorithms::connect;
/// # use rustfst::fst_traits::MutableFst;
/// let fst : VectorFst<IntegerWeight> = fst![2 => 3];
///
/// // Add a state not on a successful path
/// let mut no_connected_fst = fst.clone();
/// no_connected_fst.add_state();
///
/// let mut connected_fst = no_connected_fst.clone();
/// connect(&mut connected_fst);
///
/// assert_eq!(connected_fst, fst);
/// ```
pub fn connect<F: ExpandedFst + MutableFst>(fst: &mut F) -> Fallible<()> {
    let mut visitor = ConnectVisitor::new(fst);
    dfs_visit(fst, &mut visitor, false);
    let mut dstates = Vec::with_capacity(visitor.access.len());
    for s in 0..visitor.access.len() {
        if !visitor.access[s] || !visitor.coaccess[s] {
            dstates.push(s);
        }
    }
    fst.del_states(dstates)?;
    Ok(())
}

struct ConnectVisitor<'a, F: Fst> {
    access: Vec<bool>,
    coaccess: Vec<bool>,
    start: i32,
    fst: &'a F,
    nstates: usize,
    dfnumber: Vec<i32>,
    lowlink: Vec<i32>,
    onstack: Vec<bool>,
    scc_stack: Vec<StateId>,
}

impl<'a, F: 'a + Fst + ExpandedFst> ConnectVisitor<'a, F> {
    pub fn new(fst: &'a F) -> Self {
        let n = fst.num_states();
        Self {
            access: vec![false; n],
            coaccess: vec![false; n],
            start: fst.start().map(|v| v as i32).unwrap_or(NO_STATE_ID),
            fst,
            nstates: 0,
            dfnumber: vec![-1; n],
            lowlink: vec![-1; n],
            onstack: vec![false; n],
            scc_stack: vec![],
        }
    }
}

impl<'a, F: 'a + ExpandedFst> Visitor<'a, F> for ConnectVisitor<'a, F> {
    fn init_visit(&mut self, _fst: &'a F) {}

    fn init_state(&mut self, s: usize, root: usize) -> bool {
        self.scc_stack.push(s);
        self.dfnumber[s] = self.nstates as i32;
        self.lowlink[s] = self.nstates as i32;
        self.onstack[s] = true;
        self.access[s] = root as i32 == self.start;
        self.nstates += 1;
        true
    }

    fn tree_arc(&mut self, _s: usize, _arc: &Arc<<F as CoreFst>::W>) -> bool {
        true
    }

    fn back_arc(&mut self, s: usize, arc: &Arc<<F as CoreFst>::W>) -> bool {
        let t = arc.nextstate;
        if self.dfnumber[t] < self.lowlink[s] {
            self.lowlink[s] = self.dfnumber[t];
        }
        if self.coaccess[t] {
            self.coaccess[s] = true;
        }
        true
    }

    fn forward_or_cross_arc(&mut self, s: usize, arc: &Arc<<F as CoreFst>::W>) -> bool {
        let t = arc.nextstate;
        if self.dfnumber[t] < self.dfnumber[s]
            && self.onstack[t]
            && self.dfnumber[t] < self.lowlink[s]
        {
            self.lowlink[s] = self.dfnumber[t];
        }
        if self.coaccess[t] {
            self.coaccess[s] = true;
        }
        true
    }

    #[inline]
    fn finish_state(
        &mut self,
        s: usize,
        parent: Option<usize>,
        _arc: Option<&Arc<<F as CoreFst>::W>>,
    ) {
        if unsafe { self.fst.is_final_unchecked(s) } {
            self.coaccess[s] = true;
        }
        if self.dfnumber[s] == self.lowlink[s] {
            let mut scc_coaccess = false;
            let mut i = self.scc_stack.len();
            let mut t;
            loop {
                i -= 1;
                t = self.scc_stack[i];
                if self.coaccess[t] {
                    scc_coaccess = true;
                }
                if s == t {
                    break;
                }
            }
            loop {
                t = unsafe { *self.scc_stack.last().unsafe_unwrap() };
                if scc_coaccess {
                    self.coaccess[t] = true;
                }
                self.onstack[t] = false;
                self.scc_stack.pop();
                if s == t {
                    break;
                }
            }
        }
        if let Some(_p) = parent {
            if self.coaccess[s] {
                self.coaccess[_p] = true;
            }
            if self.lowlink[s] < self.lowlink[_p] {
                self.lowlink[_p] = self.lowlink[s];
            }
        }
    }

    #[inline]
    fn finish_visit(&mut self) {}
}

#[cfg(test)]
mod tests {
    use crate::test_data::vector_fst::get_vector_fsts_for_tests;

    use crate::proptest_fst::proptest_fst;

    use crate::fst_properties::FstProperties;

    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn test_connect_proptest(mut fst in proptest_fst()) {
            connect(&mut fst).unwrap();
            prop_assume!(fst.properties().unwrap().intersects(
                FstProperties::ACCESSIBLE | FstProperties::COACCESSIBLE
            ));
        }
    }

    #[test]
    fn test_connect_generic() -> Fallible<()> {
        for data in get_vector_fsts_for_tests() {
            let fst = &data.fst;

            let mut connect_fst = fst.clone();
            connect(&mut connect_fst)?;

            assert_eq!(
                connect_fst, data.connected_fst,
                "Connect test fail for fst : {:?}",
                &data.name
            );
        }
        Ok(())
    }
}

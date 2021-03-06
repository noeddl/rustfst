use crate::algorithms::dfs_visit::Visitor;
use crate::fst_traits::{CoreFst, ExpandedFst, Fst};
use crate::Arc;
use crate::{StateId, NO_STATE_ID};

use unsafe_unwrap::UnsafeUnwrap;

pub struct SccVisitor<'a, F: Fst> {
    pub scc: Option<Vec<i32>>,
    pub access: Option<Vec<bool>>,
    pub coaccess: Vec<bool>,
    start: i32,
    fst: &'a F,
    nstates: usize,
    dfnumber: Vec<i32>,
    lowlink: Vec<i32>,
    onstack: Vec<bool>,
    scc_stack: Vec<StateId>,
    pub nscc: i32,
}

impl<'a, F: 'a + Fst + ExpandedFst> SccVisitor<'a, F> {
    pub fn new(fst: &'a F, compute_scc: bool, compute_acess: bool) -> Self {
        let n = fst.num_states();
        Self {
            scc: if compute_scc { Some(vec![-1; n]) } else { None },
            access: if compute_acess {
                Some(vec![false; n])
            } else {
                None
            },
            coaccess: vec![false; n],
            start: fst.start().map(|v| v as i32).unwrap_or(NO_STATE_ID),
            fst,
            nstates: 0,
            dfnumber: vec![-1; n],
            lowlink: vec![-1; n],
            onstack: vec![false; n],
            scc_stack: vec![],
            nscc: 0,
        }
    }
}

impl<'a, F: 'a + ExpandedFst> Visitor<'a, F> for SccVisitor<'a, F> {
    fn init_visit(&mut self, _fst: &'a F) {}

    fn init_state(&mut self, s: usize, root: usize) -> bool {
        self.scc_stack.push(s);
        self.dfnumber[s] = self.nstates as i32;
        self.lowlink[s] = self.nstates as i32;
        self.onstack[s] = true;
        if let Some(ref mut access) = self.access {
            access[s] = root as i32 == self.start;
        }
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
                if let Some(ref mut scc) = self.scc {
                    scc[t] = self.nscc;
                }
                if scc_coaccess {
                    self.coaccess[t] = true;
                }
                self.onstack[t] = false;
                self.scc_stack.pop();
                if s == t {
                    break;
                }
            }
            self.nscc += 1;
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
    fn finish_visit(&mut self) {
        if let Some(ref mut scc) = self.scc {
            for s in 0..scc.len() {
                scc[s] = self.nscc - 1 - scc[s];
            }
        }
    }
}

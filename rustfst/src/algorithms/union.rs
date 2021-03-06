use std::collections::HashMap;

use failure::{format_err, Fallible};

use crate::arc::Arc;
use crate::fst_traits::{CoreFst, ExpandedFst, FinalStatesIterator, MutableFst};
use crate::semirings::Semiring;
use crate::StateId;

/// Performs the union of two wFSTs. If A transduces string `x` to `y` with weight `a`
/// and `B` transduces string `w` to `v` with weight `b`, then their union transduces `x` to `y`
/// with weight `a` and `w` to `v` with weight `b`.
///
/// # Example
/// ```
/// # #[macro_use] extern crate rustfst;
/// # use failure::Fallible;
/// # use rustfst::utils::transducer;
/// # use rustfst::semirings::{Semiring, IntegerWeight};
/// # use rustfst::fst_impls::VectorFst;
/// # use rustfst::fst_traits::PathsIterator;
/// # use rustfst::FstPath;
/// # use rustfst::algorithms::union;
/// # use std::collections::HashSet;
/// # fn main() -> Fallible<()> {
/// let fst_a : VectorFst<IntegerWeight> = fst![2 => 3];
/// let fst_b : VectorFst<IntegerWeight> = fst![6 => 5];
///
/// let fst_res : VectorFst<IntegerWeight> = union(&fst_a, &fst_b)?;
/// let paths : HashSet<_> = fst_res.paths_iter().collect();
///
/// let mut paths_ref = HashSet::<FstPath<IntegerWeight>>::new();
/// paths_ref.insert(fst_path![2 => 3]);
/// paths_ref.insert(fst_path![6 => 5]);
///
/// assert_eq!(paths, paths_ref);
/// # Ok(())
/// # }
/// ```
pub fn union<W, F1, F2, F3>(fst_1: &F1, fst_2: &F2) -> Fallible<F3>
where
    W: Semiring,
    F1: ExpandedFst<W = W>,
    F2: ExpandedFst<W = W>,
    F3: MutableFst<W = W>,
{
    let mut fst_out = F3::new();

    let start_state = fst_out.add_state();
    fst_out.set_start(start_state)?;

    let mapping_states_fst_1 = fst_out.add_fst(fst_1)?;
    let mapping_states_fst_2 = fst_out.add_fst(fst_2)?;

    add_epsilon_arc_to_initial_state(fst_1, &mapping_states_fst_1, &mut fst_out)?;
    add_epsilon_arc_to_initial_state(fst_2, &mapping_states_fst_2, &mut fst_out)?;

    set_new_final_states(fst_1, &mapping_states_fst_1, &mut fst_out)?;
    set_new_final_states(fst_2, &mapping_states_fst_2, &mut fst_out)?;

    Ok(fst_out)
}

fn add_epsilon_arc_to_initial_state<F1, F2>(
    fst: &F1,
    mapping: &HashMap<StateId, StateId>,
    fst_out: &mut F2,
) -> Fallible<()>
where
    F1: ExpandedFst,
    F2: MutableFst,
{
    let start_state = fst_out.start().unwrap();
    if let Some(old_start_state_fst) = fst.start() {
        fst_out.add_arc(
            start_state,
            Arc::new(
                0,
                0,
                <F2 as CoreFst>::W::one(),
                *mapping.get(&old_start_state_fst).unwrap(),
            ),
        )?;
    }
    Ok(())
}

fn set_new_final_states<W, F1, F2>(
    fst: &F1,
    mapping: &HashMap<StateId, StateId>,
    fst_out: &mut F2,
) -> Fallible<()>
where
    W: Semiring,
    F1: ExpandedFst<W = W>,
    F2: MutableFst<W = W>,
{
    for old_final_state in fst.final_states_iter() {
        let final_state = mapping.get(&old_final_state.state_id).ok_or_else(|| {
            format_err!(
                "Key {:?} doesn't exist in mapping",
                old_final_state.state_id
            )
        })?;
        fst_out.set_final(*final_state, old_final_state.final_weight.clone())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use counter::Counter;
    use failure::ResultExt;
    use itertools::Itertools;

    use crate::fst_impls::VectorFst;
    use crate::fst_traits::PathsIterator;
    use crate::semirings::IntegerWeight;
    use crate::test_data::vector_fst::get_vector_fsts_for_tests;

    #[test]
    fn test_union_generic() -> Fallible<()> {
        for data in get_vector_fsts_for_tests().combinations(2) {
            let fst_1 = &data[0].fst;
            let fst_2 = &data[1].fst;

            let mut paths_ref: Counter<_> = fst_1.paths_iter().collect();
            paths_ref.update(fst_2.paths_iter());

            let union_fst: VectorFst<IntegerWeight> = union(fst_1, fst_2).with_context(|_| {
                format_err!(
                    "Error when performing union operation between {:?} and {:?}",
                    &data[0].name,
                    &data[1].name
                )
            })?;
            let paths: Counter<_> = union_fst.paths_iter().collect();

            assert_eq!(
                paths, paths_ref,
                "Test failing for union between {:?} and {:?}",
                &data[0].name, &data[1].name
            );
        }
        Ok(())
    }
}

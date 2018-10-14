use fst_traits::Fst;
use path::Path;
use semirings::Semiring;
use std::collections::VecDeque;
use StateId;

/// Trait to iterate over the paths accepted by an FST
pub trait PathsIterator<'a> {
    type W: Semiring;
    type Iter: Iterator<Item = Path<Self::W>>;
    fn paths_iter(&'a self) -> Self::Iter;
}

impl<'a, F> PathsIterator<'a> for F
where
    F: 'a + Fst,
{
    type W = F::W;
    type Iter = StructPathsIterator<'a, F>;
    fn paths_iter(&'a self) -> Self::Iter {
        StructPathsIterator::new(&self)
    }
}

pub struct StructPathsIterator<'a, F>
where
    F: 'a + Fst,
{
    fst: &'a F,
    queue: VecDeque<(StateId, Path<F::W>)>,
}

impl<'a, F> StructPathsIterator<'a, F>
where
    F: 'a + Fst,
{
    pub fn new(fst: &'a F) -> Self {
        let mut queue = VecDeque::new();

        if let Some(state_start) = fst.start() {
            queue.push_back((state_start, Path::default()));
        }

        StructPathsIterator { fst, queue }
    }
}

impl<'a, F> Iterator for StructPathsIterator<'a, F>
where
    F: 'a + Fst,
{
    type Item = Path<F::W>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.queue.is_empty() {
            let (state_id, mut path) = self.queue.pop_front().unwrap();

            for arc in self.fst.arcs_iter(&state_id).unwrap() {
                let mut new_path = path.clone();
                new_path.add_to_path(arc.ilabel, arc.olabel, arc.weight.clone());
                self.queue.push_back((arc.nextstate, new_path));
            }

            if let Some(final_weight) = self.fst.final_weight(&state_id) {
                path.add_weight(final_weight);
                return Some(path);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arc::Arc;
    use fst_impls::VectorFst;
    use fst_traits::MutableFst;
    use semirings::{IntegerWeight, Semiring};
    use std::collections::HashSet;
    use utils::acceptor;

    #[test]
    fn test_paths_iterator_linear_fst() {
        let labels = vec![153, 45, 96];

        let fst: VectorFst<IntegerWeight> = acceptor(labels.clone().into_iter()).unwrap();

        assert_eq!(fst.paths_iter().count(), 1);

        for path in fst.paths_iter() {
            assert_eq!(
                path,
                Path::new(labels.clone(), labels.clone(), IntegerWeight::one())
            );
        }
    }

    #[test]
    fn test_paths_iterator_small_fst_one_final_state() {
        let mut fst: VectorFst<IntegerWeight> = VectorFst::new();

        let s1 = fst.add_state();
        let s2 = fst.add_state();
        let s3 = fst.add_state();
        let s4 = fst.add_state();

        fst.set_start(&s1).unwrap();
        fst.set_final(&s4, IntegerWeight::new(18)).unwrap();

        fst.add_arc(&s1, Arc::new(1, 1, IntegerWeight::new(1), s2))
            .unwrap();
        fst.add_arc(&s1, Arc::new(2, 2, IntegerWeight::new(2), s3))
            .unwrap();
        fst.add_arc(&s1, Arc::new(3, 3, IntegerWeight::new(3), s4))
            .unwrap();
        fst.add_arc(&s2, Arc::new(4, 4, IntegerWeight::new(4), s4))
            .unwrap();
        fst.add_arc(&s3, Arc::new(5, 5, IntegerWeight::new(5), s4))
            .unwrap();

        assert_eq!(fst.paths_iter().count(), 3);

        let mut paths_ref = HashSet::new();
        paths_ref.insert(Path::new(
            vec![1, 4],
            vec![1, 4],
            IntegerWeight::new(4 * 18),
        ));
        paths_ref.insert(Path::new(
            vec![2, 5],
            vec![2, 5],
            IntegerWeight::new(10 * 18),
        ));
        paths_ref.insert(Path::new(vec![3], vec![3], IntegerWeight::new(3 * 18)));

        let paths: HashSet<_> = fst.paths_iter().collect();

        assert_eq!(paths_ref, paths);
    }

    #[test]
    fn test_paths_iterator_small_fst_multiple_final_states() {
        let mut fst: VectorFst<IntegerWeight> = VectorFst::new();

        let s1 = fst.add_state();
        let s2 = fst.add_state();
        let s3 = fst.add_state();
        let s4 = fst.add_state();

        fst.set_start(&s1).unwrap();
        fst.set_final(&s1, IntegerWeight::new(38)).unwrap();
        fst.set_final(&s2, IntegerWeight::new(41)).unwrap();
        fst.set_final(&s3, IntegerWeight::new(53)).unwrap();
        fst.set_final(&s4, IntegerWeight::new(185)).unwrap();

        fst.add_arc(&s1, Arc::new(1, 1, IntegerWeight::new(1), s2))
            .unwrap();
        fst.add_arc(&s1, Arc::new(2, 2, IntegerWeight::new(2), s3))
            .unwrap();
        fst.add_arc(&s1, Arc::new(3, 3, IntegerWeight::new(3), s4))
            .unwrap();
        fst.add_arc(&s2, Arc::new(4, 4, IntegerWeight::new(4), s4))
            .unwrap();
        fst.add_arc(&s3, Arc::new(5, 5, IntegerWeight::new(5), s4))
            .unwrap();

        assert_eq!(fst.paths_iter().count(), 6);

        let mut paths_ref = HashSet::new();
        paths_ref.insert(Path::new(vec![], vec![], IntegerWeight::new(38)));
        paths_ref.insert(Path::new(vec![1], vec![1], IntegerWeight::new(1 * 41)));
        paths_ref.insert(Path::new(vec![2], vec![2], IntegerWeight::new(2 * 53)));
        paths_ref.insert(Path::new(
            vec![1, 4],
            vec![1, 4],
            IntegerWeight::new(4 * 185),
        ));
        paths_ref.insert(Path::new(
            vec![2, 5],
            vec![2, 5],
            IntegerWeight::new(10 * 185),
        ));
        paths_ref.insert(Path::new(vec![3], vec![3], IntegerWeight::new(3 * 185)));

        let paths: HashSet<_> = fst.paths_iter().collect();

        assert_eq!(paths_ref, paths);
    }
}
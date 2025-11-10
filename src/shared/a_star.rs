use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

pub(crate) mod maze;

pub(crate) trait State: Sized + Clone + Ord + Hash {
    type Position: Eq + Hash;

    fn cost(&self) -> u64;

	#[allow(unused)]
    fn heuristic(&self) -> u64;

	#[allow(unused)]
    fn total_cost(&self) -> u64 {
        self.heuristic() + self.cost()
    }

    fn position(&self) -> Self::Position;

    fn is_final(&self) -> bool;

    fn neighbours(&self) -> Vec<Self>;

    fn path(&self) -> Vec<Self::Position>;
}

pub(crate) fn a_star<T: State>(initial: T) -> Option<T> {
    let mut open_set: BinaryHeap<T> = BinaryHeap::from([initial.clone()]);
    let mut best_cost: HashMap<T::Position, u64> =
        HashMap::from([(initial.position(), initial.cost())]);

    while let Some(node) = open_set.pop() {
        if node.is_final() {
            return Some(node);
        }

        for neighbour in node.neighbours() {
            let previous_best = best_cost.get(&neighbour.position()).copied();
            if previous_best.is_none_or(|previous_best| previous_best > neighbour.cost()) {
                best_cost.insert(neighbour.position(), neighbour.cost());
                open_set.push(neighbour);
            }
        }
    }

    None
}

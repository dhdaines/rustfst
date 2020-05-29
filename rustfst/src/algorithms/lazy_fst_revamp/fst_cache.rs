use std::fmt::Debug;

use crate::semirings::Semiring;
use crate::{StateId, TrsVec};

pub trait FstCache<W: Semiring>: Debug + Default {
    fn get_start(&self) -> Option<Option<StateId>>;
    fn insert_start(&self, id: Option<StateId>);

    fn get_trs(&self, id: StateId) -> Option<TrsVec<W>>;
    fn insert_trs(&self, id: StateId, trs: TrsVec<W>);

    fn get_final_weight(&self, id: StateId) -> Option<Option<W>>;
    fn insert_final_weight(&self, id: StateId, weight: Option<W>);

    fn num_known_states(&self) -> usize;
    fn num_trs(&self, id: StateId) -> Option<usize>;

    fn num_input_epsilons(&self, id: usize) -> Option<usize>;
    fn num_output_epsilons(&self, id: usize) -> Option<usize>;
}

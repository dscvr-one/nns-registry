use crate::prelude::*;

#[derive(Serialize, CandidType)]
pub struct Stats {
    pub(crate) whitelisted: usize,
    pub(crate) claimed_neurons: usize,
    pub(crate) available_neurons: usize,
}

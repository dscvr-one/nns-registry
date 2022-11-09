use super::State;
use crate::prelude::*;
use crate::service_controller::ServiceControllers;
use ic_cdk::api::stable;

#[derive(Serialize, Deserialize, Clone, CandidType)]
pub struct StableStorage {
    pub(crate) max_neurons: usize,
    pub(crate) nns_principals: Vec<Principal>,
    pub(crate) whitelist: Vec<(Principal, bool)>,
    pub(crate) controllers: ServiceControllers,
}

impl From<&mut State> for StableStorage {
    fn from(state: &mut State) -> Self {
        Self {
            max_neurons: std::mem::take(&mut state.max_neurons),
            nns_principals: std::mem::take(&mut state.nns_principals.iter().cloned().collect()),
            whitelist: std::mem::take(&mut state.whitelist.iter().map(|(k, v)| (*k, *v)).collect()),
            controllers: std::mem::take(&mut state.controllers),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, CandidType)]
pub struct StableStorageChunk<T> {
    pub chunk: Vec<T>,
    pub start: Option<usize>,
}

#[allow(dead_code)]
pub(crate) fn stable_save<T>(t: T) -> Result<(), rmp_serde::encode::Error>
where
    T: serde::Serialize,
{
    let mut storage = stable::StableWriter::default();
    rmp_serde::encode::write(&mut storage, &t)?;
    rmp_serde::encode::write(&mut storage, &ic_cdk::api::instruction_counter())
}

#[allow(dead_code)]
pub(crate) fn stable_restore<T1, T2>() -> (
    Result<T1, rmp_serde::decode::Error>,
    Result<T2, rmp_serde::decode::Error>,
)
where
    T1: for<'de> serde::Deserialize<'de>,
    T2: for<'de> serde::Deserialize<'de>,
{
    let mut reader = stable::StableReader::default();
    let t1 = rmp_serde::decode::from_read(&mut reader);
    let t2 = rmp_serde::decode::from_read(&mut reader);
    (t1, t2)
}

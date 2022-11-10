mod guards;
mod service_controller;
mod state;

pub mod prelude {
    pub use ic_cdk::export::candid::{CandidType, Principal};
    pub use ic_cdk_macros::*;
    pub use serde::{Deserialize, Serialize};
}

use std::collections::HashSet;

use crate::guards::*;
use crate::service_controller::ServiceControllerKind;
use crate::state::State;
use prelude::*;
use state::Status;

fn main() {}

#[init]
fn init() {
    State::mutate_state(|state| {
        // Owner Service Account
        state.add_owner(ic_cdk::api::caller());
    });
}

#[query]
fn get_status() -> Status {
    State::read_state(|state| state.get_status(&ic_cdk::api::caller()))
}

#[update(guard = "is_owner")]
fn whitelist_principals(principals: Vec<Principal>) -> Vec<Principal> {
    let mut already_whitelisted = HashSet::<Principal>::default();
    State::mutate_state(|state| {
        for principal in principals.into_iter() {
            if state.whitelist_principal(principal).is_err() {
                already_whitelisted.insert(principal);
            }
        }
    });
    already_whitelisted.into_iter().collect::<Vec<_>>()
}

#[update]
fn add_nns_principal(nns_principal: Principal) -> Result<(), String> {
    State::mutate_state(|state| state.add_nns_principal(ic_cdk::api::caller(), nns_principal))
        .map_err(|e| e.to_string())
}

#[query(guard = "is_admin")]
fn get_nns_principals() -> Vec<Principal> {
    State::read_state(|state| state.get_nns_principals())
}

#[query(guard = "is_admin")]
fn set_max_neurons(max_neurons: usize) -> Result<(), String> {
    State::mutate_state(|state| state.set_max_neurons(max_neurons)).map_err(|e| e.to_string())
}

#[query]
fn get_admins() -> Vec<Principal> {
    State::read_state(|state| state.get_admins())
}

#[update(guard = "is_owner")]
fn add_admin(principal: Principal) -> Result<(), String> {
    if State::mutate_state(|state| state.add_admin(principal)) {
        Ok(())
    } else {
        Err(format!(
            "The pair Principal: {:?}, ServiceControllerKind: {:?} already exists.  Failed to add.",
            principal,
            ServiceControllerKind::Admin
        ))
    }
}

#[update(guard = "is_owner")]
fn remove_admin(principal: Principal) -> Result<(), String> {
    if State::mutate_state(|state| state.remove_admin(&principal)) {
        Ok(())
    } else {
        Err(format!(
            "The pair Principal: {:?}, ServiceControllerKind: {:?} already exists.  Failed to add.",
            principal,
            ServiceControllerKind::Admin
        ))
    }
}

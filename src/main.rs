mod guards;
mod service_controller;
mod state;

pub mod prelude {
    pub use ic_cdk::export::candid::{CandidType, Principal};
    pub use ic_cdk_macros::*;
    pub use serde::{Deserialize, Serialize};
}

use crate::guards::*;
use crate::service_controller::ServiceControllerKind;
use crate::state::State;
use prelude::*;

fn main() {}

#[query]
fn test() -> String {
    let s = String::from("Hello, world!");
    ic_cdk::println!("{}", s);
    s
}

#[init]
fn init() {
    State::mutate_state(|state| {
        // Owner Service Account
        state.add_owner(ic_cdk::api::caller());
    });
}

#[query]
fn whitelist_contains(principal: Principal) -> bool {
    State::read_state(|state| state.whitelist_contains(&principal))
}

#[update(guard = "is_owner")]
fn whitelist_principal(principal: Principal) -> Result<(), String> {
    State::mutate_state(|state| state.whitelist_principal(principal))
}

#[update]
fn add_nns_principal(nns_principal: Principal) -> Result<(), String> {
    State::mutate_state(|state| state.add_nns_principal(ic_cdk::api::caller(), nns_principal))
}

#[update]
fn add_non_dscvr_nns_principal(site_principal: Principal, nns_principal: Principal) -> Result<(), String> {
    State::mutate_state(|state| state.add_nns_principal(site_principal, nns_principal))
}

#[query(guard = "is_admin")]
fn get_nns_principals() -> Vec<Principal> {
    State::read_state(|state| state.get_nns_principals())
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

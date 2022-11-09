pub(crate) mod stable_storage;

use crate::prelude::*;
use crate::service_controller::{ServiceControllerKind, ServiceControllers};
use crate::state::stable_storage::StableStorage;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

pub struct State {
    max_neurons: usize,
    nns_principals: HashSet<Principal>,
    whitelist: HashMap<Principal, bool>,
    controllers: ServiceControllers,
}

impl Default for State {
    fn default() -> Self {
        let max_neurons = 1_000;
        Self {
            max_neurons,
            nns_principals: HashSet::with_capacity(max_neurons),
            whitelist: HashMap::with_capacity(max_neurons),
            controllers: Default::default(),
        }
    }
}

impl From<StableStorage> for State {
    fn from(storage: StableStorage) -> Self {
        Self {
            max_neurons: storage.max_neurons,
            nns_principals: HashSet::from_iter(storage.nns_principals),
            controllers: storage.controllers,
            whitelist: HashMap::from_iter(storage.whitelist),
        }
    }
}

impl State {
    thread_local! {
        pub static STATE: RefCell<State> = RefCell::default();
    }

    pub fn read_state<F: FnOnce(&Self) -> R, R>(f: F) -> R {
        State::STATE.with(|s| f(&s.borrow()))
    }

    pub fn mutate_state<F: FnOnce(&mut Self) -> R, R>(f: F) -> R {
        State::STATE.with(|s| f(&mut s.borrow_mut()))
    }

    pub fn get_admins(&self) -> Vec<Principal> {
        self.controllers
            .ref_values()
            .iter()
            .filter_map(|controller| {
                if controller.kind == ServiceControllerKind::Admin {
                    Some(controller.controller_id)
                } else {
                    None
                }
            })
            .collect::<Vec<Principal>>()
    }

    pub fn add_owner(&mut self, principal: Principal) -> bool {
        self.controllers.add(ServiceControllerKind::Owner, principal)
    }

    pub fn add_admin(&mut self, principal: Principal) -> bool {
        self.controllers.add(ServiceControllerKind::Admin, principal)
    }

    pub fn remove_admin(&mut self, principal: &Principal) -> bool {
        self.controllers.remove(principal, ServiceControllerKind::Admin)
    }

    pub fn has_access(&self, kind: ServiceControllerKind, principal: Principal) -> bool {
        self.controllers.has_access(kind, principal)
    }

    pub fn add_nns_principal(&mut self, caller: Principal, nns_principal: Principal) -> Result<(), String> {
        if self.nns_principals.len() == self.max_neurons {
            return Err(format!("Maximum number of neurons ({:?}) have been claimed.", self.max_neurons))
        }
        if let Some(is_used) = self
            .whitelist
            .iter_mut()
            .find_map(|(user, is_used)| if *user == caller { Some(*is_used) } else { None })
        {
            if is_used {
                self.nns_principals.insert(nns_principal);
                self.whitelist.insert(caller, true);
                Ok(())
            } else {
                Err(String::from(
                    "The Caller is in the Whitelist but has already assigned their NNS Principal",
                ))
            }
        } else {
            Err(String::from("Caller is not an approved member of the whitelist"))
        }
    }

    pub fn whitelist_contains(&self, principal: &Principal) -> bool {
        self.whitelist.iter().any(|(key, _)| principal == key)
    }

    pub fn whitelist_principal(&mut self, principal: Principal) -> Result<(), String> {
        match self.whitelist.entry(principal) {
            Entry::Occupied(_) => Err(format!("The Principal: {:?} is already whitelisted.", principal)),
            Entry::Vacant(entry) => {
                entry.insert(false);
                Ok(())
            }
        }
    }

    pub fn get_nns_principals(&self) -> Vec<Principal> {
        self.nns_principals.iter().cloned().collect()
    }

    pub fn set_max_neurons(&mut self, max_neurons: usize) {
        self.max_neurons = max_neurons;
    }
}

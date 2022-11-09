use crate::prelude::*;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, CandidType, Deserialize, Serialize, Hash)]
pub enum ServiceControllerKind {
    Backup,
    Restore,
    Admin,
    Owner,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize, Serialize)]
pub struct ServiceControllers(Vec<ServiceController>);

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct ServiceController {
    pub kind: ServiceControllerKind,
    pub controller_id: Principal,
    pub created_at: u64,
}

impl PartialEq<ServiceController> for ServiceController {
    fn eq(&self, other: &Self) -> bool {
        self.controller_id.to_string() == other.controller_id.to_string() && self.kind == other.kind
    }
}

impl ServiceControllers {
    pub fn has_access(&self, kind: ServiceControllerKind, controller_id: Principal) -> bool {
        return if let Some(pair) = self.0.iter().find(|p| p.kind == kind) {
            controller_id == pair.controller_id
        } else {
            false
        };
    }

    pub fn ref_values(&self) -> &Vec<ServiceController> {
        &self.0
    }

    pub fn add(&mut self, kind: ServiceControllerKind, controller_id: Principal) -> bool {
        if !self
            .0
            .iter()
            .any(|r| r.kind == kind && r.controller_id == controller_id)
        {
            self.0.push(ServiceController {
                kind,
                controller_id,
                created_at: ic_cdk::api::time(),
            });
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, principal: &Principal, kind: ServiceControllerKind) -> bool {
        let mut removed = false;

        if let Some(index) = self
            .0
            .iter()
            .position(|r| r.kind == kind && r.controller_id == *principal)
        {
            self.0.remove(index);
            removed = true;
        }

        removed
    }
}

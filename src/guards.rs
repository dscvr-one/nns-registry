use crate::service_controller::ServiceControllerKind;
use crate::State;

pub fn is_owner() -> Result<(), String> {
    if State::read_state(|state| state.has_access(ServiceControllerKind::Owner, ic_cdk::api::caller())) {
        Ok(())
    } else {
        Err("Proper service account require to make this request".to_string())
    }
}

pub fn is_admin() -> Result<(), String> {
    if State::read_state(|state| state.has_access(ServiceControllerKind::Admin, ic_cdk::api::caller())) {
        Ok(())
    } else {
        Err("Admin Service Account required to make this request".to_string())
    }
}

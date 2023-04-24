use std::collections::HashMap;

use candid::{export_service, CandidType, Deserialize, Principal};
use serde::Serialize;

#[cfg(test)]
mod test;

#[ic_cdk::init]
#[candid::candid_method(init)]
fn init(init_args: UserIndexInitArgs) {
    init_args
        .known_principal_ids
        .iter()
        .for_each(|(_principal_type, principal_value)| {
            ic_cdk::print(format!(
                "🥫 Principal received: {}",
                principal_value.to_text()
            ));
        });
}

#[derive(Deserialize, CandidType, Default, Clone)]
pub struct UserIndexInitArgs {
    pub known_principal_ids: KnownPrincipalMap,
}

pub type KnownPrincipalMap = HashMap<KnownPrincipalType, Principal>;

#[derive(CandidType, Deserialize, PartialEq, Eq, Hash, Clone, Serialize)]
pub enum KnownPrincipalType {
    UserIdGlobalSuperAdmin,
    CanisterIdConfiguration,
    CanisterIdDataBackup,
    CanisterIdPostCache,
    CanisterIdProjectMemberIndex,
    CanisterIdRootCanister,
    CanisterIdSNSController,
    CanisterIdTopicCacheIndex,
    CanisterIdUserIndex,
}

#[derive(PartialEq, Eq, Debug, CandidType, Deserialize, Clone, Serialize)]
pub enum UserAccessRole {
    /// User has canister WASM install/uninstall/delete capabilities
    CanisterController,
    /// User has edit access to all data residing in the canister
    CanisterAdmin,
    /// Data in this canister is the data of this user
    ProfileOwner,
    /// This principal is for a canister part of this project
    ProjectCanister,
}

#[ic_cdk::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

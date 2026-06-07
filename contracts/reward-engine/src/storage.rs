use soroban_sdk::{contracttype, Address, Env, String};

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub enum VerificationStatus {
    Pending,
    Approved,
    Rejected,
    Disputed,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Verification {
    pub task_id: u64,
    pub user: Address,
    pub proof_cid: String,
    pub reward_amount: i128,
    pub status: VerificationStatus,
    pub submitted_at: u64,
    pub resolved_at: Option<u64>,
    pub oracle: Address,
}

#[derive(Clone, Debug)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Registry,
    Oracle,
    Verification(u64, Address),
}

pub fn write_admin(e: &Env, admin: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, admin);
}

pub fn read_admin(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().instance().get(&key).unwrap()
}

pub fn has_admin(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn write_token(e: &Env, token: &Address) {
    let key = DataKey::Token;
    e.storage().instance().set(&key, token);
}

pub fn read_token(e: &Env) -> Address {
    let key = DataKey::Token;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_registry(e: &Env, registry: &Address) {
    let key = DataKey::Registry;
    e.storage().instance().set(&key, registry);
}

pub fn read_registry(e: &Env) -> Address {
    let key = DataKey::Registry;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_oracle(e: &Env, oracle: &Address) {
    let key = DataKey::Oracle;
    e.storage().instance().set(&key, oracle);
}

pub fn read_oracle(e: &Env) -> Address {
    let key = DataKey::Oracle;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_verification(e: &Env, task_id: u64, user: &Address, v: &Verification) {
    let key = DataKey::Verification(task_id, user.clone());
    e.storage().persistent().set(&key, v);
}

pub fn read_verification(e: &Env, task_id: u64, user: &Address) -> Option<Verification> {
    let key = DataKey::Verification(task_id, user.clone());
    e.storage().persistent().get(&key)
}

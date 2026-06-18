use crate::storage;
use soroban_sdk::{Address, Env};

pub fn require_admin(e: &Env, addr: &Address) {
    let admin = storage::read_admin(e);
    if addr != &admin {
        panic!("unauthorized");
    }
}

pub fn require_sponsor(e: &Env, addr: &Address) {
    let admin = storage::read_admin(e);
    if addr == &admin {
        return;
    }
    if !storage::is_sponsor(e, addr) {
        panic!("unauthorized");
    }
}

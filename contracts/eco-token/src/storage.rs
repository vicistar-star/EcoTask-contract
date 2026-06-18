use soroban_sdk::{symbol_short, Address, Env, String};

pub fn write_balance(e: &Env, addr: &Address, amount: i128) {
    let key = (symbol_short!("balance"), addr.clone());
    e.storage().persistent().set(&key, &amount);
}

pub fn read_balance(e: &Env, addr: &Address) -> i128 {
    let key = (symbol_short!("balance"), addr.clone());
    e.storage().persistent().get(&key).unwrap_or(0)
}

pub fn write_admin(e: &Env, admin: &Address) {
    e.storage().instance().set(&symbol_short!("admin"), admin);
}

pub fn read_admin(e: &Env) -> Address {
    e.storage().instance().get(&symbol_short!("admin")).unwrap()
}

pub fn has_admin(e: &Env) -> bool {
    e.storage().instance().has(&symbol_short!("admin"))
}

pub fn write_metadata(e: &Env, name: &String, symbol: &String, decimal: &u32) {
    e.storage().instance().set(&symbol_short!("name"), name);
    e.storage().instance().set(&symbol_short!("symbol"), symbol);
    e.storage()
        .instance()
        .set(&symbol_short!("decimal"), decimal);
}

pub fn read_name(e: &Env) -> String {
    e.storage().instance().get(&symbol_short!("name")).unwrap()
}

pub fn read_symbol(e: &Env) -> String {
    e.storage()
        .instance()
        .get(&symbol_short!("symbol"))
        .unwrap()
}

pub fn read_decimal(e: &Env) -> u32 {
    e.storage()
        .instance()
        .get(&symbol_short!("decimal"))
        .unwrap()
}

pub fn write_supply(e: &Env, amount: i128) {
    e.storage()
        .instance()
        .set(&symbol_short!("supply"), &amount);
}

pub fn read_supply(e: &Env) -> i128 {
    e.storage()
        .instance()
        .get(&symbol_short!("supply"))
        .unwrap_or(0)
}

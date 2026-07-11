use crate::storage;
use soroban_sdk::{contract, contractevent, contractimpl, Address, Env, String};

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintEvent {
    #[topic]
    pub admin: Address,
    #[topic]
    pub to: Address,
    pub amount: i128,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferEvent {
    #[topic]
    pub from: Address,
    #[topic]
    pub to: Address,
    pub amount: i128,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BurnEvent {
    #[topic]
    pub from: Address,
    pub amount: i128,
}

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn initialize(e: Env, admin: Address, name: String, symbol: String, decimal: u32) {
        if storage::has_admin(&e) {
            panic!("already initialized");
        }
        storage::write_admin(&e, &admin);
        storage::write_metadata(&e, &name, &symbol, &decimal);
        storage::write_supply(&e, 0);
    }

    pub fn mint(e: Env, to: Address, amount: i128) {
        let admin = storage::read_admin(&e);
        admin.require_auth();

        if amount <= 0 {
            panic!("amount must be positive");
        }

        let balance = storage::read_balance(&e, &to);
        storage::write_balance(
            &e,
            &to,
            balance.checked_add(amount).expect("balance overflow"),
        );

        let supply = storage::read_supply(&e);
        storage::write_supply(&e, supply.checked_add(amount).expect("supply overflow"));

        MintEvent {
            admin,
            to: to.clone(),
            amount,
        }
        .publish(&e);
    }

    pub fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        if amount <= 0 {
            panic!("amount must be positive");
        }

        let from_balance = storage::read_balance(&e, &from);
        if from_balance < amount {
            panic!("insufficient balance");
        }

        storage::write_balance(
            &e,
            &from,
            from_balance.checked_sub(amount).expect("balance underflow"),
        );

        let to_balance = storage::read_balance(&e, &to);
        storage::write_balance(
            &e,
            &to,
            to_balance.checked_add(amount).expect("balance overflow"),
        );

        TransferEvent {
            from: from.clone(),
            to: to.clone(),
            amount,
        }
        .publish(&e);
    }

    pub fn balance(e: Env, id: Address) -> i128 {
        storage::read_balance(&e, &id)
    }

    pub fn total_supply(e: Env) -> i128 {
        storage::read_supply(&e)
    }

    pub fn name(e: Env) -> String {
        storage::read_name(&e)
    }

    pub fn symbol(e: Env) -> String {
        storage::read_symbol(&e)
    }

    pub fn decimal(e: Env) -> u32 {
        storage::read_decimal(&e)
    }

    pub fn admin(e: Env) -> Address {
        storage::read_admin(&e)
    }

    pub fn burn(e: Env, from: Address, amount: i128) {
        from.require_auth();

        if amount <= 0 {
            panic!("amount must be positive");
        }

        let balance = storage::read_balance(&e, &from);
        if balance < amount {
            panic!("insufficient balance");
        }

        storage::write_balance(
            &e,
            &from,
            balance.checked_sub(amount).expect("balance underflow"),
        );

        let supply = storage::read_supply(&e);
        storage::write_supply(&e, supply.checked_sub(amount).expect("supply underflow"));

        BurnEvent {
            from: from.clone(),
            amount,
        }
        .publish(&e);
    }
}

#[cfg(test)]
mod test {
    use crate::{TokenContract, TokenContractClient};
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{Address, Env, String};

    #[test]
    fn test_initialize_and_metadata() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let contract_id = e.register(TokenContract, ());
        let client = TokenContractClient::new(&e, &contract_id);

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );

        assert_eq!(client.name(), String::from_str(&e, "ECO"));
        assert_eq!(client.symbol(), String::from_str(&e, "ECO"));
        assert_eq!(client.decimal(), 7);
        assert_eq!(client.total_supply(), 0);
    }

    #[test]
    fn test_mint_and_balance() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let user = Address::generate(&e);
        let contract_id = e.register(TokenContract, ());
        let client = TokenContractClient::new(&e, &contract_id);

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );

        e.mock_all_auths();
        client.mint(&user, &1000);

        assert_eq!(client.balance(&user), 1000);
        assert_eq!(client.total_supply(), 1000);
    }

    #[test]
    fn test_transfer() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let from = Address::generate(&e);
        let to = Address::generate(&e);
        let contract_id = e.register(TokenContract, ());
        let client = TokenContractClient::new(&e, &contract_id);

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );

        e.mock_all_auths();
        client.mint(&from, &500);
        client.transfer(&from, &to, &300);

        assert_eq!(client.balance(&from), 200);
        assert_eq!(client.balance(&to), 300);
    }

    #[test]
    #[should_panic(expected = "amount must be positive")]
    fn test_mint_zero_amount() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let user = Address::generate(&e);
        let contract_id = e.register(TokenContract, ());
        let client = TokenContractClient::new(&e, &contract_id);

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );

        e.mock_all_auths();
        client.mint(&user, &0);
    }

    #[test]
    #[should_panic(expected = "insufficient balance")]
    fn test_transfer_insufficient_balance() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let from = Address::generate(&e);
        let to = Address::generate(&e);
        let contract_id = e.register(TokenContract, ());
        let client = TokenContractClient::new(&e, &contract_id);

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );

        e.mock_all_auths();
        client.transfer(&from, &to, &100);
    }

    #[test]
    #[should_panic]
    fn test_mint_only_admin() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let user = Address::generate(&e);
        let contract_id = e.register(TokenContract, ());
        let client = TokenContractClient::new(&e, &contract_id);

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );

        client.mint(&user, &1000);
    }

    #[test]
    #[should_panic(expected = "already initialized")]
    fn test_double_initialize_fails() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let contract_id = e.register(TokenContract, ());
        let client = TokenContractClient::new(&e, &contract_id);

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );
    }

    #[test]
    fn test_burn() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let user = Address::generate(&e);
        let contract_id = e.register(TokenContract, ());
        let client = TokenContractClient::new(&e, &contract_id);

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );

        e.mock_all_auths();
        client.mint(&user, &1000);
        client.burn(&user, &400);

        assert_eq!(client.balance(&user), 600);
        assert_eq!(client.total_supply(), 600);
    }

    #[test]
    #[should_panic(expected = "insufficient balance")]
    fn test_burn_more_than_balance() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let user = Address::generate(&e);
        let contract_id = e.register(TokenContract, ());
        let client = TokenContractClient::new(&e, &contract_id);

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );

        e.mock_all_auths();
        client.mint(&user, &100);
        client.burn(&user, &200);
    }

    #[test]
    #[should_panic(expected = "amount must be positive")]
    fn test_burn_zero_amount() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let user = Address::generate(&e);
        let contract_id = e.register(TokenContract, ());
        let client = TokenContractClient::new(&e, &contract_id);

        client.initialize(
            &admin,
            &String::from_str(&e, "ECO"),
            &String::from_str(&e, "ECO"),
            &7,
        );

        e.mock_all_auths();
        client.burn(&user, &0);
    }
}

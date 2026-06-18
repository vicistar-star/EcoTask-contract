use crate::storage;
use soroban_sdk::{contract, contractimpl, contractevent, vec, Address, Env, IntoVal, String, Symbol, Val};
pub use storage::{Verification, VerificationStatus};

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RewardEvent {
    ProofSubmitted(Address, Address, u64),
    RewardPaid(Address, Address, u64, i128),
    ProofRejected(Address, Address, u64),
    DisputeRaised(Address, u64),
}

#[contract]
pub struct RewardEngine;

#[contractimpl]
impl RewardEngine {
    pub fn initialize(e: Env, admin: Address, token: Address, registry: Address, oracle: Address) {
        if storage::has_admin(&e) {
            panic!("already initialized");
        }
        if admin == oracle {
            panic!("oracle must be different from admin");
        }
        storage::write_admin(&e, &admin);
        storage::write_token(&e, &token);
        storage::write_registry(&e, &registry);
        storage::write_oracle(&e, &oracle);
    }

    pub fn set_oracle(e: Env, caller: Address, new_oracle: Address) {
        caller.require_auth();
        let admin = storage::read_admin(&e);
        if caller != admin {
            panic!("unauthorized");
        }
        storage::write_oracle(&e, &new_oracle);
    }

    pub fn submit_proof(e: Env, oracle: Address, user: Address, task_id: u64, proof_cid: String) {
        oracle.require_auth();
        let stored_oracle = storage::read_oracle(&e);
        if oracle != stored_oracle {
            panic!("unauthorized");
        }

        if storage::read_verification(&e, task_id, &user).is_some() {
            panic!("proof already submitted");
        }

        let verification = Verification {
            task_id,
            user: user.clone(),
            proof_cid,
            reward_amount: 0,
            status: VerificationStatus::Pending,
            submitted_at: e.ledger().timestamp(),
            resolved_at: None,
            oracle: oracle.clone(),
        };

        storage::write_verification(&e, task_id, &user, &verification);

        e.events().publish(
            (),
            RewardEvent::ProofSubmitted(oracle, user, task_id),
        );
    }

    pub fn approve_proof(
        e: Env,
        oracle: Address,
        user: Address,
        task_id: u64,
        reward_amount: i128,
    ) {
        oracle.require_auth();
        let stored_oracle = storage::read_oracle(&e);
        if oracle != stored_oracle {
            panic!("unauthorized");
        }

        let mut verification = match storage::read_verification(&e, task_id, &user) {
            Some(v) => v,
            None => panic!("verification not found"),
        };

        if verification.status != VerificationStatus::Pending {
            panic!("verification is not pending");
        }

        verification.status = VerificationStatus::Approved;
        verification.reward_amount = reward_amount;
        verification.resolved_at = Some(e.ledger().timestamp());
        storage::write_verification(&e, task_id, &user, &verification);

        let registry_id = storage::read_registry(&e);
        e.invoke_contract::<Val>(
            &registry_id,
            &Symbol::new(&e, "complete_task"),
            vec![
                &e,
                e.current_contract_address().into_val(&e),
                (task_id as u64).into_val(&e),
                user.clone().into_val(&e),
            ],
        );

        let token_id = storage::read_token(&e);
        e.invoke_contract::<Val>(
            &token_id,
            &Symbol::new(&e, "mint"),
            vec![&e, user.clone().into_val(&e), reward_amount.into_val(&e)],
        );

        e.events().publish(
            (),
            RewardEvent::RewardPaid(oracle, user, task_id, reward_amount),
        );
    }

    pub fn reject_proof(e: Env, oracle: Address, user: Address, task_id: u64) {
        oracle.require_auth();
        let stored_oracle = storage::read_oracle(&e);
        if oracle != stored_oracle {
            panic!("unauthorized");
        }

        let mut verification = match storage::read_verification(&e, task_id, &user) {
            Some(v) => v,
            None => panic!("verification not found"),
        };

        if verification.status != VerificationStatus::Pending {
            panic!("verification is not pending");
        }

        verification.status = VerificationStatus::Rejected;
        verification.resolved_at = Some(e.ledger().timestamp());
        storage::write_verification(&e, task_id, &user, &verification);

        e.events().publish(
            (),
            RewardEvent::ProofRejected(oracle, user, task_id),
        );
    }

    pub fn dispute_proof(e: Env, caller: Address, user: Address, task_id: u64) {
        caller.require_auth();
        let admin = storage::read_admin(&e);
        if caller != admin {
            panic!("unauthorized");
        }

        let mut verification = match storage::read_verification(&e, task_id, &user) {
            Some(v) => v,
            None => panic!("verification not found"),
        };

        verification.status = VerificationStatus::Disputed;
        storage::write_verification(&e, task_id, &user, &verification);

        e.events().publish(
            (),
            RewardEvent::DisputeRaised(user, task_id),
        );
    }

    pub fn get_verification(e: Env, task_id: u64, user: Address) -> Verification {
        match storage::read_verification(&e, task_id, &user) {
            Some(v) => v,
            None => panic!("verification not found"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{RewardEngine, RewardEngineClient, VerificationStatus};
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::testutils::BytesN;
    use soroban_sdk::{Address, Env, String};

    fn deploy_token(e: &Env, admin: &Address) -> Address {
        let token_id = e.register(None, eco_token::TokenContract);
        let token_client = eco_token::TokenContractClient::new(e, &token_id);
        token_client.initialize(
            admin,
            &String::from_str(e, "ECO"),
            &String::from_str(e, "ECO"),
            &7,
        );
        token_id
    }

    fn deploy_registry(e: &Env, admin: &Address) -> Address {
        let reg_id = e.register(None, task_registry::RegistryContract);
        let reg_client = task_registry::RegistryContractClient::new(e, &reg_id);
        reg_client.initialize(admin);
        reg_id
    }

    fn setup() -> (
        Env,
        Address,
        Address,
        Address,
        u64,
        RewardEngineClient<'static>,
    ) {
        let e = Env::default();
        let admin = Address::generate(&e);
        let oracle = Address::generate(&e);
        let user = Address::generate(&e);

        let token_id = deploy_token(&e, &admin);
        let reg_id = deploy_registry(&e, &admin);

        let engine_id = e.register(None, RewardEngine);
        let engine_client = RewardEngineClient::new(&e, &engine_id);

        e.mock_all_auths_allowing_non_root_auth();
        let reg_client = task_registry::RegistryContractClient::new(&e, &reg_id);
        reg_client.add_sponsor(&admin, &engine_id);

        engine_client.initialize(&admin, &token_id, &reg_id, &oracle);

        let loc_hash = soroban_sdk::BytesN::<32>::random(&e);
        let task_id = reg_client.create_task(
            &admin,
            &String::from_str(&e, "tree-planting"),
            &loc_hash,
            &1000,
            &1,
            &(e.ledger().timestamp() + 10000),
        );

        (e, admin, oracle, user, task_id, engine_client)
    }

    #[test]
    fn test_submit_and_approve() {
        let (e, _admin, oracle, user, task_id, client) = setup();
        e.mock_all_auths_allowing_non_root_auth();

        let proof_cid = String::from_str(&e, "QmTest123");
        client.submit_proof(&oracle, &user, &task_id, &proof_cid);

        client.approve_proof(&oracle, &user, &task_id, &1000);

        let verification = client.get_verification(&task_id, &user);
        assert_eq!(verification.status, VerificationStatus::Approved);
        assert_eq!(verification.reward_amount, 1000);
        assert!(verification.resolved_at.is_some());
    }

    #[test]
    fn test_submit_and_reject() {
        let (e, _admin, oracle, user, task_id, client) = setup();
        e.mock_all_auths_allowing_non_root_auth();

        let proof_cid = String::from_str(&e, "QmTest456");
        client.submit_proof(&oracle, &user, &task_id, &proof_cid);

        client.reject_proof(&oracle, &user, &task_id);

        let verification = client.get_verification(&task_id, &user);
        assert_eq!(verification.status, VerificationStatus::Rejected);
        assert!(verification.resolved_at.is_some());
    }

    #[test]
    #[should_panic(expected = "oracle must be different from admin")]
    fn test_initialize_oracle_same_as_admin() {
        let e = Env::default();
        let admin = Address::generate(&e);
        let token = Address::generate(&e);
        let registry = Address::generate(&e);

        let engine_id = e.register(None, RewardEngine);
        let engine_client = RewardEngineClient::new(&e, &engine_id);

        engine_client.initialize(&admin, &token, &registry, &admin);
    }

    #[test]
    #[should_panic(expected = "unauthorized")]
    fn test_unauthorized_oracle_cannot_submit() {
        let (e, _admin, _oracle, user, _task_id, client) = setup();
        e.mock_all_auths_allowing_non_root_auth();

        let fake_oracle = Address::generate(&e);
        let proof_cid = String::from_str(&e, "QmBad");
        client.submit_proof(&fake_oracle, &user, &1, &proof_cid);
    }

    #[test]
    #[should_panic(expected = "verification is not pending")]
    fn test_double_approve_fails() {
        let (e, _admin, oracle, user, task_id, client) = setup();
        e.mock_all_auths_allowing_non_root_auth();

        let proof_cid = String::from_str(&e, "QmTest789");
        client.submit_proof(&oracle, &user, &task_id, &proof_cid);
        client.approve_proof(&oracle, &user, &task_id, &1000);
        client.approve_proof(&oracle, &user, &task_id, &1000);
    }

    #[test]
    fn test_dispute_flow() {
        let (e, admin, oracle, user, task_id, client) = setup();
        e.mock_all_auths_allowing_non_root_auth();

        let proof_cid = String::from_str(&e, "QmDispute");
        client.submit_proof(&oracle, &user, &task_id, &proof_cid);

        client.dispute_proof(&admin, &user, &task_id);

        let verification = client.get_verification(&task_id, &user);
        assert_eq!(verification.status, VerificationStatus::Disputed);
    }

    #[test]
    fn test_full_integration() {
        let e = Env::default();
        e.mock_all_auths_allowing_non_root_auth();

        let admin = Address::generate(&e);
        let oracle = Address::generate(&e);
        let user = Address::generate(&e);

        let token_id = deploy_token(&e, &admin);
        let reg_id = deploy_registry(&e, &admin);

        let engine_id = e.register(None, RewardEngine);
        let engine_client = RewardEngineClient::new(&e, &engine_id);

        let reg_client = task_registry::RegistryContractClient::new(&e, &reg_id);
        reg_client.add_sponsor(&admin, &engine_id);

        engine_client.initialize(&admin, &token_id, &reg_id, &oracle);
        let loc_hash = soroban_sdk::BytesN::<32>::random(&e);
        let task_id = reg_client.create_task(
            &admin,
            &String::from_str(&e, "tree-planting"),
            &loc_hash,
            &1000,
            &1,
            &(e.ledger().timestamp() + 10000),
        );

        let proof_cid = String::from_str(&e, "QmIntegration");
        engine_client.submit_proof(&oracle, &user, &task_id, &proof_cid);
        engine_client.approve_proof(&oracle, &user, &task_id, &1000);

        let token_client = eco_token::TokenContractClient::new(&e, &token_id);
        assert_eq!(token_client.balance(&user), 1000);

        assert!(reg_client.is_task_completed(&task_id, &user));
    }
}

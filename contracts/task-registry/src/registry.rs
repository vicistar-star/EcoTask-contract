use crate::{access, storage};
use soroban_sdk::{contract, contractimpl, contractevent, Address, BytesN, Env, String, Symbol};
pub use storage::{Task, TaskStatus};

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegistryEvent {
    TaskCreated(Address, u64),
    TaskCompleted(Address, u64),
    TaskExpired(u64),
    SponsorAdded(Address),
}

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
    pub fn initialize(e: Env, admin: Address) {
        if storage::has_admin(&e) {
            panic!("already initialized");
        }
        storage::write_admin(&e, &admin);
    }

    pub fn add_sponsor(e: Env, caller: Address, sponsor: Address) {
        caller.require_auth();
        access::require_admin(&e, &caller);
        storage::add_sponsor(&e, &sponsor);
    }

    pub fn create_task(
        e: Env,
        creator: Address,
        task_type: String,
        location_hash: BytesN<32>,
        reward_amount: i128,
        max_completions: u32,
        expires_at: u64,
    ) -> u64 {
        creator.require_auth();
        access::require_sponsor(&e, &creator);

        if reward_amount <= 0 {
            panic!("reward must be positive");
        }
        if max_completions == 0 {
            panic!("max completions must be positive");
        }
        if expires_at <= e.ledger().timestamp() {
            panic!("expiry must be in the future");
        }

        let task_id = storage::next_task_id(&e);

        let task = Task {
            id: task_id,
            creator: creator.clone(),
            task_type,
            location_hash,
            reward_amount,
            max_completions,
            completions: 0,
            status: TaskStatus::Active,
            created_at: e.ledger().timestamp(),
            expires_at,
        };

        storage::write_task(&e, &task);

        e.events()
            .publish((), RegistryEvent::TaskCreated(creator, task_id));

        task_id
    }

    pub fn get_task(e: Env, task_id: u64) -> Task {
        match storage::read_task(&e, task_id) {
            Some(task) => task,
            None => panic!("task not found"),
        }
    }

    pub fn complete_task(e: Env, caller: Address, task_id: u64, user: Address) {
        caller.require_auth();
        access::require_sponsor(&e, &caller);

        let mut task = match storage::read_task(&e, task_id) {
            Some(task) => task,
            None => panic!("task not found"),
        };

        if task.status != TaskStatus::Active {
            panic!("task is not active");
        }
        if task.expires_at < e.ledger().timestamp() {
            panic!("task expired");
        }
        if storage::is_completed(&e, task_id, &user) {
            panic!("already completed");
        }
        if task.completions >= task.max_completions {
            panic!("max completions reached");
        }

        task.completions += 1;
        if task.completions >= task.max_completions {
            task.status = TaskStatus::Completed;
        }

        storage::write_task(&e, &task);
        storage::mark_completed(&e, task_id, &user);

        e.events()
            .publish((), RegistryEvent::TaskCompleted(user, task_id));
    }

    pub fn expire_task(e: Env, caller: Address, task_id: u64) {
        caller.require_auth();
        access::require_admin(&e, &caller);

        let mut task = match storage::read_task(&e, task_id) {
            Some(task) => task,
            None => panic!("task not found"),
        };

        if task.status != TaskStatus::Active {
            panic!("task is not active");
        }

        task.status = TaskStatus::Expired;
        storage::write_task(&e, &task);
    }

    pub fn task_count(e: Env) -> u64 {
        storage::next_task_id(&e)
    }

    pub fn is_task_completed(e: Env, task_id: u64, user: Address) -> bool {
        storage::is_completed(&e, task_id, &user)
    }
}

#[cfg(test)]
mod test {
    use crate::{RegistryContract, RegistryContractClient, TaskStatus};
    use soroban_sdk::testutils::{Address as _, BytesN as _, Ledger as _};
    use soroban_sdk::{Address, BytesN, Env, String};

    fn setup() -> (Env, Address, RegistryContractClient<'static>) {
        let e = Env::default();
        let admin = Address::generate(&e);
        let contract_id = e.register(None, RegistryContract);
        let client = RegistryContractClient::new(&e, &contract_id);

        client.initialize(&admin);
        (e, admin, client)
    }

    fn create_test_task(
        client: &RegistryContractClient<'static>,
        creator: &Address,
        task_type: &String,
        max_completions: u32,
        expires_in: u64,
    ) -> u64 {
        let loc_hash: BytesN<32> = BytesN::random(&client.env);
        client.create_task(
            creator,
            task_type,
            &loc_hash,
            &1000,
            &max_completions,
            &(client.env.ledger().timestamp() + expires_in),
        )
    }

    #[test]
    fn test_create_and_get_task() {
        let (e, _admin, client) = setup();
        e.mock_all_auths();

        let task_type = String::from_str(&e, "tree-planting");
        let task_id = create_test_task(&client, &admin, &task_type, 1, 1000);

        let task = client.get_task(&task_id);
        assert_eq!(task.id, task_id);
        assert_eq!(task.creator, admin);
        assert_eq!(task.task_type, task_type);
        assert_eq!(task.reward_amount, 1000);
        assert_eq!(task.max_completions, 1);
        assert_eq!(task.completions, 0);
        assert_eq!(task.status, TaskStatus::Active);
    }

    #[test]
    fn test_complete_task() {
        let (e, admin, client) = setup();
        e.mock_all_auths();

        let user = Address::generate(&e);
        let task_id = create_test_task(
            &client,
            &admin,
            &String::from_str(&e, "trash-collection"),
            1,
            1000,
        );

        client.complete_task(&admin, &task_id, &user);

        let task = client.get_task(&task_id);
        assert_eq!(task.completions, 1);
        assert_eq!(task.status, TaskStatus::Completed);
        assert!(client.is_task_completed(&task_id, &user));
    }

    #[test]
    #[should_panic(expected = "already completed")]
    fn test_double_claim_prevention() {
        let (e, admin, client) = setup();
        e.mock_all_auths();

        let user = Address::generate(&e);
        let task_id = create_test_task(
            &client,
            &admin,
            &String::from_str(&e, "ocean-cleanup"),
            2,
            1000,
        );

        client.complete_task(&admin, &task_id, &user);
        client.complete_task(&admin, &task_id, &user);
    }

    #[test]
    fn test_max_completions() {
        let (e, admin, client) = setup();
        e.mock_all_auths();

        let user1 = Address::generate(&e);
        let user2 = Address::generate(&e);
        let task_id = create_test_task(
            &client,
            &admin,
            &String::from_str(&e, "tree-planting"),
            2,
            1000,
        );

        client.complete_task(&admin, &task_id, &user1);
        let task = client.get_task(&task_id);
        assert_eq!(task.completions, 1);
        assert_eq!(task.status, TaskStatus::Active);

        client.complete_task(&admin, &task_id, &user2);
        let task = client.get_task(&task_id);
        assert_eq!(task.completions, 2);
        assert_eq!(task.status, TaskStatus::Completed);
    }

    #[test]
    fn test_expire_task() {
        let (e, admin, client) = setup();
        e.mock_all_auths();

        let task_id = create_test_task(
            &client,
            &admin,
            &String::from_str(&e, "tree-planting"),
            1,
            1000,
        );

        client.expire_task(&admin, &task_id);

        let task = client.get_task(&task_id);
        assert_eq!(task.status, TaskStatus::Expired);
    }

    #[test]
    #[should_panic(expected = "unauthorized")]
    fn test_unauthorized_creator() {
        let (e, _admin, client) = setup();
        e.mock_all_auths();

        let attacker = Address::generate(&e);

        let loc_hash: BytesN<32> = BytesN::random(&e);
        client.create_task(
            &attacker,
            &String::from_str(&e, "tree-planting"),
            &loc_hash,
            &1000,
            &1,
            &(e.ledger().timestamp() + 1000),
        );
    }

    #[test]
    #[should_panic(expected = "unauthorized")]
    fn test_unauthorized_expire() {
        let (e, _admin, client) = setup();
        e.mock_all_auths();

        let attacker = Address::generate(&e);
        client.expire_task(&attacker, &0);
    }

    #[test]
    fn test_add_sponsor() {
        let (e, admin, client) = setup();
        e.mock_all_auths();

        let sponsor = Address::generate(&e);
        client.add_sponsor(&admin, &sponsor);

        let loc_hash: BytesN<32> = BytesN::random(&e);
        let task_id = client.create_task(
            &sponsor,
            &String::from_str(&e, "tree-planting"),
            &loc_hash,
            &1000,
            &1,
            &(e.ledger().timestamp() + 1000),
        );

        let task = client.get_task(&task_id);
        assert_eq!(task.creator, sponsor);
    }

    #[test]
    #[should_panic(expected = "task is not active")]
    fn test_expire_already_expired_task() {
        let (e, admin, client) = setup();
        e.mock_all_auths();

        let task_id = create_test_task(
            &client,
            &admin,
            &String::from_str(&e, "tree-planting"),
            1,
            1000,
        );

        client.expire_task(&admin, &task_id);
        client.expire_task(&admin, &task_id);
    }

    #[test]
    #[should_panic(expected = "task expired")]
    fn test_expired_task_cannot_be_completed() {
        let (e, admin, client) = setup();
        e.mock_all_auths();

        let user = Address::generate(&e);

        e.ledger().set_timestamp(1000);
        let task_id = create_test_task(
            &client,
            &admin,
            &String::from_str(&e, "tree-planting"),
            1,
            1000,
        );

        e.ledger().set_timestamp(3000);
        client.complete_task(&admin, &task_id, &user);
    }
}

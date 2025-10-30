use soroban_sdk::{Env, Address, String};
use crate::{AgroCoinContract, AgroCoinContractClient};

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{symbol_short, testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation}, Address, Env};

    #[test]
    fn test_initialize_contract() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AgroCoinContract);
        let client = AgroCoinContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        env.mock_all_auths();

        client.initialize(&admin);

        assert_eq!(client.get_project_count(), 0);
    }

    #[test]
    fn test_create_project() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AgroCoinContract);
        let client = AgroCoinContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let producer = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&admin);

        let project_id = client.create_project(
            &producer,
            &String::from_str(&env, "Cultivos Orgánicos"),
            &String::from_str(&env, "Proyecto de agricultura orgánica sostenible"),
            &1000000i128, // 100 XLM
            &10000i128,   // 1 XLM mínimo
            &1800u32,     // 18% ROI
            &12u32,       // 12 meses
        );

        assert_eq!(project_id, 1);
        assert_eq!(client.get_project_count(), 1);

        let project = client.get_project(&project_id);
        assert_eq!(project.owner, producer);
        assert_eq!(project.funding_goal, 1000000i128);
        assert_eq!(project.current_funding, 0);
        assert_eq!(project.is_active, true);
        assert_eq!(project.is_funded, false);
    }

    #[test]
    fn test_invest_in_project() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AgroCoinContract);
        let client = AgroCoinContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let producer = Address::generate(&env);
        let investor = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&admin);

        let project_id = client.create_project(
            &producer,
            &String::from_str(&env, "Test Project"),
            &String::from_str(&env, "Test Description"),
            &1000000i128,
            &10000i128,
            &1800u32,
            &12u32,
        );

        client.invest(&investor, &project_id, &500000i128);

        let project = client.get_project(&project_id);
        assert_eq!(project.current_funding, 500000i128);
        assert_eq!(project.is_funded, false);

        let investment = client.get_investment(&investor, &project_id);
        assert_eq!(investment.amount, 500000i128);
        assert_eq!(investment.investor, investor);
    }

    #[test]
    fn test_full_funding() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AgroCoinContract);
        let client = AgroCoinContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let producer = Address::generate(&env);
        let investor = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&admin);

        let project_id = client.create_project(
            &producer,
            &String::from_str(&env, "Test Project"),
            &String::from_str(&env, "Test Description"),
            &1000000i128,
            &10000i128,
            &1800u32,
            &12u32,
        );

        client.invest(&investor, &project_id, &1000000i128);

        let project = client.get_project(&project_id);
        assert_eq!(project.current_funding, 1000000i128);
        assert_eq!(project.is_funded, true);
    }

    #[test]
    #[should_panic(expected = "Investment exceeds funding goal")]
    fn test_over_invest() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AgroCoinContract);
        let client = AgroCoinContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let producer = Address::generate(&env);
        let investor = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&admin);

        let project_id = client.create_project(
            &producer,
            &String::from_str(&env, "Test Project"),
            &String::from_str(&env, "Test Description"),
            &1000000i128,
            &10000i128,
            &1800u32,
            &12u32,
        );

        client.invest(&investor, &project_id, &1500000i128);
    }

    #[test]
    fn test_pause_project() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AgroCoinContract);
        let client = AgroCoinContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let producer = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&admin);

        let project_id = client.create_project(
            &producer,
            &String::from_str(&env, "Test Project"),
            &String::from_str(&env, "Test Description"),
            &1000000i128,
            &10000i128,
            &1800u32,
            &12u32,
        );

        client.pause_project(&admin, &project_id);

        let project = client.get_project(&project_id);
        assert_eq!(project.is_active, false);
    }
}

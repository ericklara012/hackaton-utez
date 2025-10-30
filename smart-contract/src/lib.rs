#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, contractmeta,
    Address, Env, String, Vec, Symbol,
    symbol_short, log,
};


contractmeta!(
    key = "description",
    val = "AgroCoin - Smart Contract para inversiones agrícolas descentralizadas"
);

#[contract]
pub struct AgroCoinContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Project {
    pub id: u64,
    pub owner: Address,          
    pub name: String,             
    pub description: String,    
    pub funding_goal: i128,       
    pub current_funding: i128,   
    pub min_investment: i128,     
    pub expected_roi: u32,       
    pub duration_months: u32,    
    pub is_active: bool,         
    pub is_funded: bool,       
    pub created_at: u64,         
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Investment {
    pub investor: Address,
    pub project_id: u64,
    pub amount: i128,             
    pub timestamp: u64,           
    pub claimed_returns: i128,   
}


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectStats {
    pub total_investors: u32,
    pub funding_percentage: u32,
    pub days_remaining: u32,
}


const PROJECTS: Symbol = symbol_short!("PROJECTS");
const INVESTMENTS: Symbol = symbol_short!("INVESTS");
const PROJECT_COUNT: Symbol = symbol_short!("P_COUNT");
const USER_PROJECTS: Symbol = symbol_short!("U_PROJ");
const USER_INVESTMENTS: Symbol = symbol_short!("U_INV");
const ADMIN: Symbol = symbol_short!("ADMIN");

#[contractimpl]
impl AgroCoinContract {

    
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("Contract already initialized");
        }

        admin.require_auth();
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&PROJECT_COUNT, &0u64);

        log!(&env, "AgroCoin Contract initialized with admin: {}", admin);
    }

    
    pub fn create_project(
        env: Env,
        owner: Address,
        name: String,
        description: String,
        funding_goal: i128,
        min_investment: i128,
        expected_roi: u32,
        duration_months: u32,
    ) -> u64 {
        owner.require_auth();

        if funding_goal <= 0 {
            panic!("Funding goal must be positive");
        }
        if min_investment <= 0 || min_investment > funding_goal {
            panic!("Invalid minimum investment");
        }
        if expected_roi > 10000 {
            panic!("ROI too high");
        }
        if duration_months == 0 || duration_months > 60 {
            panic!("Invalid duration");
        }

        let mut project_count: u64 = env.storage().instance().get(&PROJECT_COUNT).unwrap_or(0);
        project_count += 1;

        let project = Project {
            id: project_count,
            owner: owner.clone(),
            name,
            description,
            funding_goal,
            current_funding: 0,
            min_investment,
            expected_roi,
            duration_months,
            is_active: true,
            is_funded: false,
            created_at: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&(PROJECTS, project_count), &project);
        env.storage().instance().set(&PROJECT_COUNT, &project_count);

        let mut user_projects: Vec<u64> = env.storage()
            .persistent()
            .get(&(USER_PROJECTS, &owner))
            .unwrap_or(Vec::new(&env));
        user_projects.push_back(project_count);
        env.storage().persistent().set(&(USER_PROJECTS, &owner), &user_projects);

        log!(&env, "Project created: {} by {}", project_count, owner);
        project_count
    }

    
    pub fn invest(env: Env, investor: Address, project_id: u64, amount: i128) {
        investor.require_auth();

        if amount <= 0 {
            panic!("Investment amount must be positive");
        }

        
        let mut project: Project = env.storage()
            .persistent()
            .get(&(PROJECTS, project_id))
            .unwrap_or_else(|| panic!("Project not found"));

        
        if !project.is_active {
            panic!("Project is not active");
        }
        if project.is_funded {
            panic!("Project already funded");
        }
        if amount < project.min_investment {
            panic!("Amount below minimum investment");
        }
        if project.current_funding + amount > project.funding_goal {
            panic!("Investment exceeds funding goal");
        }

        
        project.current_funding += amount;
        if project.current_funding >= project.funding_goal {
            project.is_funded = true;
        }

        
        let investment = Investment {
            investor: investor.clone(),
            project_id,
            amount,
            timestamp: env.ledger().timestamp(),
            claimed_returns: 0,
        };

        
        env.storage().persistent().set(&(PROJECTS, project_id), &project);

        let investment_key = (INVESTMENTS, &investor, project_id);
        let existing_investment_opt: Option<Investment> = env.storage()
            .persistent()
            .get(&investment_key);

        let final_investment = match existing_investment_opt {
            Some(mut existing) => {
                existing.amount += amount;
                existing
            },
            None => investment,
        };

        env.storage().persistent().set(&investment_key, &final_investment);


        let mut user_investments: Vec<u64> = env.storage()
            .persistent()
            .get(&(USER_INVESTMENTS, &investor))
            .unwrap_or(Vec::new(&env));

        if !user_investments.contains(&project_id) {
            user_investments.push_back(project_id);
            env.storage().persistent().set(&(USER_INVESTMENTS, &investor), &user_investments);
        }

        log!(&env, "Investment: {} invested {} in project {}", investor, amount, project_id);
    }

    pub fn claim_returns(env: Env, investor: Address, project_id: u64) -> i128 {
        investor.require_auth();

        let project: Project = env.storage()
            .persistent()
            .get(&(PROJECTS, project_id))
            .unwrap_or_else(|| panic!("Project not found"));

        if !project.is_funded {
            panic!("Project not yet funded");
        }

        let mut investment: Investment = env.storage()
            .persistent()
            .get(&(INVESTMENTS, &investor, project_id))
            .unwrap_or_else(|| panic!("Investment not found"));

        
        let total_returns = (investment.amount * project.expected_roi as i128) / 10000;
        let available_returns = total_returns - investment.claimed_returns;

        if available_returns <= 0 {
            panic!("No returns available");
        }

        let time_elapsed = env.ledger().timestamp() - investment.timestamp;
        let months_elapsed = time_elapsed / (30 * 24 * 3600);

        if months_elapsed < project.duration_months as u64 {
            panic!("Project duration not completed");
        }

        investment.claimed_returns += available_returns;
        env.storage().persistent().set(&(INVESTMENTS, &investor, project_id), &investment);

        log!(&env, "Returns claimed: {} claimed {} from project {}", investor, available_returns, project_id);
        available_returns
    }

    pub fn get_project(env: Env, project_id: u64) -> Project {
        env.storage()
            .persistent()
            .get(&(PROJECTS, project_id))
            .unwrap_or_else(|| panic!("Project not found"))
    }

    pub fn get_project_stats(env: Env, project_id: u64) -> ProjectStats {
        let project: Project = Self::get_project(env.clone(), project_id);

        
        let total_investors = 1u32;

        let funding_percentage = if project.funding_goal > 0 {
            ((project.current_funding * 10000) / project.funding_goal) as u32
        } else {
            0
        };

        let days_remaining = project.duration_months * 30;

        ProjectStats {
            total_investors,
            funding_percentage,
            days_remaining,
        }
    }

    pub fn get_investment(env: Env, investor: Address, project_id: u64) -> Investment {
        env.storage()
            .persistent()
            .get(&(INVESTMENTS, &investor, project_id))
            .unwrap_or_else(|| panic!("Investment not found"))
    }

    pub fn get_user_projects(env: Env, user: Address) -> Vec<u64> {
        env.storage()
            .persistent()
            .get(&(USER_PROJECTS, &user))
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_user_investments(env: Env, user: Address) -> Vec<u64> {
        env.storage()
            .persistent()
            .get(&(USER_INVESTMENTS, &user))
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_project_count(env: Env) -> u64 {
        env.storage().instance().get(&PROJECT_COUNT).unwrap_or(0)
    }

    pub fn pause_project(env: Env, admin: Address, project_id: u64) {
        admin.require_auth();

        let stored_admin: Address = env.storage()
            .instance()
            .get(&ADMIN)
            .unwrap_or_else(|| panic!("Contract not initialized"));

        if admin != stored_admin {
            panic!("Only admin can pause projects");
        }

        let mut project: Project = Self::get_project(env.clone(), project_id);
        project.is_active = false;
        env.storage().persistent().set(&(PROJECTS, project_id), &project);

        log!(&env, "Project {} paused by admin", project_id);
    }

    pub fn withdraw_funds(env: Env, owner: Address, project_id: u64) -> i128 {
        owner.require_auth();

        let project: Project = Self::get_project(env.clone(), project_id);

        if project.owner != owner {
            panic!("Only project owner can withdraw");
        }
        if !project.is_funded {
            panic!("Project not funded yet");
        }

        log!(&env, "Funds withdrawn: {} withdrew {} from project {}", owner, project.current_funding, project_id);

        project.current_funding
    }
}

#[cfg(test)]
mod test;

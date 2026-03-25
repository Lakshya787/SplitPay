#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, log, token, Address, Env, Map, String, Vec,
};

// ── Data Keys ──────────────────────────────────────────────────────
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Group(u64),
    GroupCount,
    GroupMembers(u64),
    GroupPayments(u64),
    MemberBalance(u64, Address), // (group_id, member)
}

// ── Group Info ─────────────────────────────────────────────────────
#[contracttype]
#[derive(Clone)]
pub struct Group {
    pub id: u64,
    pub name: String,
    pub creator: Address,
    pub token: Address,
    pub total_members: u32,
    pub total_paid: i128,
    pub is_settled: bool,
}

// ── Payment Record ─────────────────────────────────────────────────
#[contracttype]
#[derive(Clone)]
pub struct Payment {
    pub payer: Address,
    pub amount: i128,
    pub description: String,
    pub timestamp: u64,
}

#[contract]
pub struct SplitPayContract;

#[contractimpl]
impl SplitPayContract {
    // ── Initialize the contract ────────────────────────────────────
    pub fn initialize(env: Env, admin: Address) {
        // Ensure not already initialized
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::GroupCount, &0u64);
        log!(&env, "SplitPay initialized with admin: {}", admin);
    }

    // ── Create a new expense group ─────────────────────────────────
    pub fn create_group(
        env: Env,
        creator: Address,
        name: String,
        token: Address,
        members: Vec<Address>,
    ) -> u64 {
        creator.require_auth();

        // Ensure creator is in members list
        let mut creator_found = false;
        for i in 0..members.len() {
            if members.get(i).unwrap() == creator {
                creator_found = true;
                break;
            }
        }
        if !creator_found {
            panic!("Creator must be a member of the group");
        }

        // Get and increment group count
        let group_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::GroupCount)
            .unwrap_or(0);
        let new_count = group_id + 1;
        env.storage()
            .instance()
            .set(&DataKey::GroupCount, &new_count);

        // Create the group
        let group = Group {
            id: group_id,
            name,
            creator: creator.clone(),
            token,
            total_members: members.len(),
            total_paid: 0,
            is_settled: false,
        };

        // Store group data
        env.storage()
            .instance()
            .set(&DataKey::Group(group_id), &group);
        env.storage()
            .instance()
            .set(&DataKey::GroupMembers(group_id), &members);

        let empty_payments: Vec<Payment> = Vec::new(&env);
        env.storage()
            .instance()
            .set(&DataKey::GroupPayments(group_id), &empty_payments);

        // Initialize balances to 0
        for i in 0..members.len() {
            let member = members.get(i).unwrap();
            env.storage()
                .instance()
                .set(&DataKey::MemberBalance(group_id, member), &0i128);
        }

        log!(&env, "Group {} created by {}", group_id, creator);
        group_id
    }

    // ── Add an expense (payer pays, split among all members) ───────
    pub fn add_expense(env: Env, group_id: u64, payer: Address, amount: i128, description: String) {
        payer.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        // Load group
        let mut group: Group = env
            .storage()
            .instance()
            .get(&DataKey::Group(group_id))
            .expect("Group not found");

        if group.is_settled {
            panic!("Group is already settled");
        }

        // Verify payer is a member
        let members: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::GroupMembers(group_id))
            .unwrap();

        let mut is_member = false;
        for i in 0..members.len() {
            if members.get(i).unwrap() == payer {
                is_member = true;
                break;
            }
        }
        if !is_member {
            panic!("Payer is not a member of this group");
        }

        // Transfer tokens from payer to contract
        let contract_address = env.current_contract_address();
        let token_client = token::Client::new(&env, &group.token);
        token_client.transfer(&payer, &contract_address, &amount);

        // Calculate each member's share
        let num_members = members.len() as i128;
        let share_per_member = amount / num_members;
        let remainder = amount - (share_per_member * num_members);

        // Update balances:
        // Payer's balance increases (they are owed money)
        // Other members' balances decrease (they owe money)
        for i in 0..members.len() {
            let member = members.get(i).unwrap();
            let key = DataKey::MemberBalance(group_id, member.clone());
            let current_balance: i128 = env.storage().instance().get(&key).unwrap_or(0);

            if member == payer {
                // Payer is owed (amount - their_share)
                let payer_credit = amount - share_per_member - if i == 0 { remainder } else { 0 };
                env.storage()
                    .instance()
                    .set(&key, &(current_balance + payer_credit));
            } else {
                // Member owes their share
                let member_debt = share_per_member + if i == 0 { remainder } else { 0 };
                env.storage()
                    .instance()
                    .set(&key, &(current_balance - member_debt));
            }
        }

        // Record payment
        let payment = Payment {
            payer: payer.clone(),
            amount,
            description,
            timestamp: env.ledger().timestamp(),
        };

        let mut payments: Vec<Payment> = env
            .storage()
            .instance()
            .get(&DataKey::GroupPayments(group_id))
            .unwrap_or(Vec::new(&env));
        payments.push_back(payment);
        env.storage()
            .instance()
            .set(&DataKey::GroupPayments(group_id), &payments);

        // Update group total
        group.total_paid += amount;
        env.storage()
            .instance()
            .set(&DataKey::Group(group_id), &group);

        log!(
            &env,
            "Expense of {} added by {} to group {}",
            amount,
            payer,
            group_id
        );
    }

    // ── Settle the group — distribute funds based on balances ──────
    pub fn settle_group(env: Env, group_id: u64, caller: Address) {
        caller.require_auth();

        let mut group: Group = env
            .storage()
            .instance()
            .get(&DataKey::Group(group_id))
            .expect("Group not found");

        if group.is_settled {
            panic!("Group is already settled");
        }

        if caller != group.creator {
            panic!("Only the group creator can settle");
        }

        let members: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::GroupMembers(group_id))
            .unwrap();

        let token_client = token::Client::new(&env, &group.token);
        let contract_address = env.current_contract_address();

        // Calculate what each person should receive back
        // Each member's fair share of total_paid
        let num_members = members.len() as i128;
        let fair_share = group.total_paid / num_members;

        // Send each member their fair share from the contract
        for i in 0..members.len() {
            let member = members.get(i).unwrap();
            let balance: i128 = env
                .storage()
                .instance()
                .get(&DataKey::MemberBalance(group_id, member.clone()))
                .unwrap_or(0);

            // If balance > 0, member is owed money (they overpaid)
            // If balance < 0, member owes money (they underpaid)
            // Everyone gets fair_share, adjusted by their balance
            let payout = fair_share + balance;

            if payout > 0 {
                token_client.transfer(&contract_address, &member, &payout);
            }

            // Reset balance
            env.storage()
                .instance()
                .set(&DataKey::MemberBalance(group_id, member), &0i128);
        }

        group.is_settled = true;
        env.storage()
            .instance()
            .set(&DataKey::Group(group_id), &group);

        log!(&env, "Group {} settled", group_id);
    }

    // ── Direct split pay (one-time, no group needed) ───────────────
    pub fn split_payment(
        env: Env,
        payer: Address,
        token: Address,
        amount: i128,
        recipients: Vec<Address>,
    ) {
        payer.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }
        if recipients.is_empty() {
            panic!("Must have at least one recipient");
        }

        let token_client = token::Client::new(&env, &token);
        let num_recipients = recipients.len() as i128;
        let share = amount / num_recipients;
        let remainder = amount - (share * num_recipients);

        for i in 0..recipients.len() {
            let recipient = recipients.get(i).unwrap();
            let mut payment = share;
            // Give remainder to first recipient
            if i == 0 {
                payment += remainder;
            }
            token_client.transfer(&payer, &recipient, &payment);
        }

        log!(
            &env,
            "Split payment of {} from {} to {} recipients",
            amount,
            payer,
            num_recipients
        );
    }

    // ── View Functions ─────────────────────────────────────────────

    pub fn get_group(env: Env, group_id: u64) -> Group {
        env.storage()
            .instance()
            .get(&DataKey::Group(group_id))
            .expect("Group not found")
    }

    pub fn get_members(env: Env, group_id: u64) -> Vec<Address> {
        env.storage()
            .instance()
            .get(&DataKey::GroupMembers(group_id))
            .expect("Group not found")
    }

    pub fn get_balance(env: Env, group_id: u64, member: Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::MemberBalance(group_id, member))
            .unwrap_or(0)
    }

    pub fn get_payments(env: Env, group_id: u64) -> Vec<Payment> {
        env.storage()
            .instance()
            .get(&DataKey::GroupPayments(group_id))
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_group_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::GroupCount)
            .unwrap_or(0)
    }
}

// ── Tests ──────────────────────────────────────────────────────────
#[cfg(test)]
mod test;

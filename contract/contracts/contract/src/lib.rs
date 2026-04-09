#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String
};

// =====================
// 📦 DATA STRUCTURES
// =====================

#[contracttype]
#[derive(Clone)]
pub struct CarbonCredit {
    pub id: u64,
    pub creator: Address,
    pub metadata: String,
    pub price: i128,
    pub is_listed: bool,
    pub is_retired: bool,
}

#[contracttype]
pub enum DataKey {
    Credit(u64),
    Counter,
}

// =====================
// 🚀 CONTRACT
// =====================

#[contract]
pub struct CarbonContract;

// =====================
// 🔧 IMPLEMENTATION
// =====================

#[contractimpl]
impl CarbonContract {

    // 🌱 CREATE CREDIT (permissionless mint)
    pub fn create_credit(env: Env, creator: Address, metadata: String) -> u64 {
        creator.require_auth();

        let mut counter: u64 = env
            .storage()
            .instance()
            .get(&DataKey::Counter)
            .unwrap_or(0);

        counter += 1;

        let credit = CarbonCredit {
            id: counter,
            creator: creator.clone(),
            metadata,
            price: 0,
            is_listed: false,
            is_retired: false,
        };

        env.storage().instance().set(&DataKey::Credit(counter), &credit);
        env.storage().instance().set(&DataKey::Counter, &counter);

        counter
    }

    // 💰 LIST CREDIT FOR SALE
    pub fn list_credit(env: Env, owner: Address, id: u64, price: i128) {
        owner.require_auth();

        let mut credit: CarbonCredit = env
            .storage()
            .instance()
            .get(&DataKey::Credit(id))
            .expect("Credit not found");

        if owner != credit.creator {
            panic!("Not owner");
        }

        if credit.is_retired {
            panic!("Cannot list retired credit");
        }

        credit.price = price;
        credit.is_listed = true;

        env.storage().instance().set(&DataKey::Credit(id), &credit);
    }

    // 🛒 BUY CREDIT (ownership transfer)
    pub fn buy_credit(env: Env, buyer: Address, id: u64) {
        buyer.require_auth();

        let mut credit: CarbonCredit = env
            .storage()
            .instance()
            .get(&DataKey::Credit(id))
            .expect("Credit not found");

        if !credit.is_listed {
            panic!("Not listed for sale");
        }

        if credit.is_retired {
            panic!("Already retired");
        }

        // NOTE: Payment logic will be added later

        credit.creator = buyer;
        credit.is_listed = false;
        credit.price = 0;

        env.storage().instance().set(&DataKey::Credit(id), &credit);
    }

    // 🔥 RETIRE CREDIT
    pub fn retire_credit(env: Env, owner: Address, id: u64) {
        owner.require_auth();

        let mut credit: CarbonCredit = env
            .storage()
            .instance()
            .get(&DataKey::Credit(id))
            .expect("Credit not found");

        if owner != credit.creator {
            panic!("Not owner");
        }

        if credit.is_retired {
            panic!("Already retired");
        }

        credit.is_retired = true;
        credit.is_listed = false;

        env.storage().instance().set(&DataKey::Credit(id), &credit);
    }

    // 👀 GET CREDIT
    pub fn get_credit(env: Env, id: u64) -> CarbonCredit {
        env.storage()
            .instance()
            .get(&DataKey::Credit(id))
            .expect("Credit not found")
    }

    // 📊 TOTAL COUNT
    pub fn get_total_credits(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::Counter)
            .unwrap_or(0)
    }
}
#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String, Symbol};
use soroban_sdk::token::{self, Interface as _};

const ADMIN: Symbol = symbol_short!("ADMIN");
const NAME: Symbol = symbol_short!("NAME");
const SYMBOL: Symbol = symbol_short!("SYMBOL");

#[contract]
pub struct Token;

fn write_admin(e: &Env, name: &Address) {
    e.storage().instance().set(&ADMIN, name);
}

fn read_admin(e: &Env) -> Address {
    e.storage().instance().get(&ADMIN).unwrap()
}

fn write_name(e: &Env, name: &String) {
    e.storage().instance().set(&NAME, name);
}

fn read_name(e: &Env) -> String {
    e.storage().instance().get(&NAME).unwrap()
}

fn write_symbol(e: &Env, symbol: &String) {
    e.storage().instance().set(&SYMBOL, symbol);
}

fn read_symbol(e: &Env) -> String {
    e.storage().instance().get(&SYMBOL).unwrap()
}

#[contractimpl]
impl Token {
    pub fn __constructor(e: Env, admin: Address, name: String, symbol: String) {
        write_admin(&e, &admin);
        write_name(&e, &name);
        write_symbol(&e, &symbol);
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin = read_admin(&env);
        admin.require_auth();
        if amount <= 0 {
            panic!("Amount must be greater than zero");
        }
        let mut to_bal: i128 = env.storage().persistent().get(&to).unwrap_or_default();
        if to_bal + amount > i128::MAX {
            panic!("Exceeds i128 max value");
        }

        to_bal += amount;
        env.storage().persistent().set(&to, &to_bal);
        env.events().publish((symbol_short!("mint"), &to), amount);
    }
}

#[contractimpl]
impl token::Interface for Token {
    fn allowance(_e: Env, _from: Address, _spender: Address) -> i128 {
        0
    }

    fn approve(_e: Env, _from: Address, _spender: Address, _amount: i128, _expiration_ledger: u32) {}

    fn balance(env: Env, id: Address) -> i128 {
        env.storage().persistent().get(&id).unwrap_or_default()
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        let mut from_bal: i128 = env.storage().persistent().get(&from).unwrap_or_default();
        let mut to_bal: i128 = env.storage().persistent().get(&to).unwrap_or_default();
        from_bal -= amount;
        to_bal += amount;
        env.storage().persistent().set(&from, &from_bal);
        env.storage().persistent().set(&to, &to_bal);
        env.events().publish((symbol_short!("transfer"), &from, &to), amount);
    }

    fn transfer_from(_e: Env, _spender: Address, _from: Address, _to: Address, _amount: i128) {}

    fn burn(_e: Env, _from: Address, _amount: i128) {}

    fn burn_from(_e: Env, _spender: Address, _from: Address, _amount: i128) {}

    fn decimals(_e: Env) -> u32 {
        0
    }

    fn name(e: Env) -> String {
        read_name(&e)
    }

    fn symbol(e: Env) -> String {
        read_symbol(&e)
    }
}
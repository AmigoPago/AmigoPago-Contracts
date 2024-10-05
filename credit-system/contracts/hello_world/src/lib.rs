#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, vec, Address, BytesN, Env, IntoVal, Vec, log, symbol_short, Symbol, map};



mod usdc_contract 
{
    soroban_sdk::contractimport!(file = "../../../usdc-mockup/target/wasm32-unknown-unknown/release/usdc_mockup.wasm");
}

mod AMI_contract 
{
    soroban_sdk::contractimport!(file = "../../../AMI_token/target/wasm32-unknown-unknown/release/wrapped_lumen_mockup.wasm");
}




#[contract]
pub struct CreditSystem;

#[contractimpl]
impl CreditSystem 
{
    pub fn get_micro_credit(env: Env, ami_contract_address: Address,  usdc_contract_address: Address,AMI_to_use: u32, caller: Address)
    {
        caller.require_auth();

        let client = AMI_contract::Client::new(&env, &ami_contract_address);
        let caller_balance = client.balance_of(&caller);

        assert!(caller_balance >= AMI_to_use);

        let actual_contract_address = env.current_contract_address();

        client.transfer_from(&actual_contract_address, &caller, &actual_contract_address, &AMI_to_use);


        let client_two = usdc_contract::Client::new(&env, &usdc_contract_address);
        client_two.transfer(&actual_contract_address, &caller, &AMI_to_use);
    }
}

mod test;
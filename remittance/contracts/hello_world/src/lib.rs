#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};



mod usdc_contract 
{
    soroban_sdk::contractimport!(file = "../../../usdc-mockup/target/wasm32-unknown-unknown/release/usdc_mockup.wasm");
}

mod AMI_contract 
{
    soroban_sdk::contractimport!(file = "../../../AMI_token/target/wasm32-unknown-unknown/release/ami_token.wasm");
}




#[contract]
pub struct Remittance;

#[contractimpl]
impl Remittance 
{
    pub fn sendPayment(env: Env, caller: Address, usd_to_send: u32, address_to_send: Address, usdc_contract_address: Address, ami_contract_address: Address)
    {
        caller.require_auth();

        let units: u32 = usd_to_send / 100;
        let fee: u32 = units * 4;
        let amount_to_send: u32 = usd_to_send - fee;

        let actual_contract_address = env.current_contract_address();

        let client = usdc_contract::Client::new(&env, &usdc_contract_address);
        client.transfer_from(&actual_contract_address, &caller, &address_to_send, &amount_to_send);
        client.transfer_from(&actual_contract_address, &caller, &actual_contract_address, &fee);

        let client_two = AMI_contract::Client::new(&env, &ami_contract_address);
        client_two.mint(&caller, &units);
    }
}

mod test;









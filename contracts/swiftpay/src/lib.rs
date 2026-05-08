#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct SwiftPay;

#[contractimpl]
impl SwiftPay {
    pub fn hello(env: Env, to: Symbol) -> Symbol {
        env.storage().instance().set(&symbol_short!("Hello"), &to);
        to
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_hello() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SwiftPay);
        let client = SwiftPayClient::new(&env, &contract_id);

        let res = client.hello(&symbol_short!("Dev"));
        assert_eq!(res, symbol_short!("Dev"));
    }
}
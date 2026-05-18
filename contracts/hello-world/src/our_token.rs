use soroban_sdk::{Env, contract, contractimpl, Address, IntoVal, String};

use crate::{error::ContractError, storage::{ AllowanceKey, DataKey}, event::{Approval, Transfer, Burn, Mint},};

#[contract]
pub struct SibToken;

#[contractimpl]
impl SibToken {

    /// Returns the balance of `id`.
    pub fn balance(env: Env, id: Address)-> i128 {
        env.storage().persistent().get(&DataKey::Balance(id)).unwrap_or(0)
    }

    // total tokens 
    pub fn total_supply(env: Env) -> i128 {
        env.storage().persistent().get(&DataKey::TotalSupply).unwrap_or(0)
    }

    /// Returns the allowance for `spender` to transfer from `from`.
    pub fn allowance(env: Env, from:Address, spender:Address) -> i128 {
        env.storage().persistent().get(&DataKey::Allowance(AllowanceKey {from, spender})).unwrap_or(0)
    }

    /// Set the allowance by `amount` for `spender` to transfer/burn from
    pub fn approve(env: Env, from: Address, spender: Address, amount: i128, live_until_ledger: u32) -> Result<(), ContractError> {
        from.require_auth();
        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }
        let key = DataKey::Allowance(AllowanceKey {from: from.clone(), spender: spender.clone()});
        env.storage().persistent().set(&key, &amount);

        Approval{
            from, spender, amount:amount.try_into().unwrap(), live_until_ledger: live_until_ledger.into_val(&env),
        }
        .publish(&env);
        Ok(())
    }

    /// Transfer `amount` from `from` to `to`.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) ->Result<(), ContractError> {
        from.require_auth();
        let sender_balance = Self::balance(env.clone(), from.clone());
        let receiver_balance = Self::balance(env.clone(), to.clone());

        if sender_balance < amount {
        return Err(ContractError::InsufficientFunds);
        }
        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(sender_balance - amount));
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(receiver_balance + amount));

        Transfer {from, to, amount:amount.try_into().unwrap(),}
        .publish(&env);

        Ok(())
    }

     /// Transfer `amount` from `from` to `to`, consuming the allowance of
    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) -> Result<(), ContractError> {
        spender.require_auth();
        if amount <= 0 {
            return Err(ContractError::InsufficientFunds);
        }

        let current_allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        if current_allowance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        let sender_balance = Self::balance(env.clone(), from.clone());
        if sender_balance <= amount {
            return Err(ContractError::InsufficientFunds);
        }

        let allowance_key = DataKey::Allowance(AllowanceKey { from: from.clone(), spender: spender.clone() });
        env.storage().persistent().set(&allowance_key, &(current_allowance - amount));
        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(sender_balance - amount));

        let to_balance = Self::balance(env.clone(), to.clone());
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(&to_balance + amount));

        Transfer {from, to, amount:amount.try_into().unwrap(),}
        .publish(&env);

        Ok(())
    }

      /// Burn `amount` from `from`
    pub fn burn(env: Env, from: Address, amount: i128) -> Result<(), ContractError> {
        from.require_auth();

        if amount < 0 {
            return Err(ContractError::InsufficientFunds);
        }

        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance <= 0 {
            return Err(ContractError::InsufficientFunds);
        }

        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(&from_balance - amount));

        let total = Self::total_supply(env.clone());
        env.storage().persistent().set(&DataKey::TotalSupply, &(total - amount));

        Burn {
            from,
            amount: amount.try_into().unwrap(),
        }
        .publish(&env);

        Ok(())
    }

    /// Burn `amount` from `from`, consuming the allowance of `spender`.
    pub fn burn_from(env: Env, spender: Address, from: Address, amount: i128) -> Result<(), ContractError> {

        spender.require_auth();
        if amount < 0 {
            return Err(ContractError::InsufficientFunds);
        }

       let current_allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        if current_allowance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        let allowance_key = DataKey::Allowance(AllowanceKey { from: from.clone(), spender: spender.clone() });
        env.storage().persistent().set(&allowance_key, &(current_allowance - amount));

        let total = Self::total_supply(env.clone());
        env.storage().persistent().set(&DataKey::TotalSupply, &(total - amount));

        Burn {
            from,
            amount: amount.try_into().unwrap(),
        }
        .publish(&env);

        Ok(())
    }

    pub fn mint(env:Env, to: Address, amount: i128) -> Result<(), ContractError> {
        if amount < 0 {
            return Err(ContractError::InsufficientFunds);
        }

        let to_balance = Self::balance(env.clone(), to.clone());
        env.storage().persistent().set(&DataKey::Balance(to.clone()),&(to_balance + amount));

        let total = Self::total_supply(env.clone());
        env.storage().persistent().set(&DataKey::TotalSupply,&(total + amount));

        Mint {
            to,
            amount: amount.try_into().unwrap()
        }
        .publish(&env);
        Ok(())
    }

    pub fn name(env: Env) -> String {
        String::from_str(&env, "SibToken")
    }
    pub fn symbol(env: Env) -> String{
        String::from_str(&env, "SIB")
    }
    pub fn decimals(env: Env) -> u32 {
        18
    }

}
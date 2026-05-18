use soroban_sdk::{contractevent, Address};

#[contractevent]
#[derive(Debug,Clone,Eq,PartialEq)]
pub struct Transfer {
    #[topic]
    pub from: Address,
    #[topic]
    pub to: Address,
    pub amount: u32,
}


#[contractevent]
#[derive(Debug,Clone,Eq,PartialEq)]
pub struct Approval {
    #[topic]
    pub from: Address,
    #[topic]
    pub spender: Address,
    pub amount: u32,
    pub live_until_ledger: u32,
}

#[contractevent]
#[derive(Debug,Clone,Eq,PartialEq)]
pub struct Burn {
    #[topic]
    pub from: Address,
    pub amount: u32,
}

#[contractevent]
#[derive(Debug,Clone,Eq,PartialEq)]
pub struct Mint {
    #[topic]
    pub to: Address,
    pub amount: u32,
}
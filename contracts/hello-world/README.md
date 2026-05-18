# SibToken — SEP-41 Fungible Token on Stellar Soroban
 
A fully implemented SEP-41 compliant fungible token smart contract built on the [Stellar](https://stellar.org) blockchain using the [Soroban](https://soroban.stellar.org) SDK. This contract covers the complete token lifecycle — minting, transferring, approving, burning, and delegated spending via allowances.



 
## What the Project Does
 
SibToken is a fungible token contract that follows the SEP-41 token standard. It allows:
 
- An admin to **mint** new tokens into circulation
- Token holders to **transfer** tokens directly to other addresses
- Token holders to **approve** a third-party spender to move tokens on their behalf
- Approved spenders to **transfer** or **burn** tokens using their allowance
- Token holders to **burn** their own tokens, permanently removing them from supply
The contract tracks every wallet's individual balance, every active allowance between pairs of addresses, and the running total supply — all stored persistently on the Stellar ledger.
 
## Project Structure
 
```
contracts/sep41-token/
└── src/
    ├── lib.rs          # Crate root — declares all modules
    ├── our_token.rs    # Core contract logic — all public functions live here
    ├── storage.rs      # DataKey enum and AllowanceKey struct for ledger keys
    ├── error.rs        # ContractError enum — all possible failure reasons
    ├── token_traits.rs  
    ├── event.rs       # Event structs — Approval, Transfer, Mint, Burn
    └── test.rs         # Full test suite — 22 tests covering all functions
```
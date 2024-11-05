# ICP Token Wallet

The **ICP Token Wallet** is a decentralized token wallet implemented in Rust, designed for managing token balances on the Internet Computer (IC) blockchain. It enables users to securely transfer and receive tokens while maintaining their balances in a ledger.

## Features

- **Initialize the Token Ledger:** Set up the token ledger with an owner.
- **Transfer Tokens:** Facilitate the transfer of tokens between principals.
- **Receive Tokens:** Allow principals to receive tokens from other users.
- **Check Balances:** Query the balance of the caller or any specific principal.

## Technologies Used

- **Rust**
- **Internet Computer SDK**
- **Candid** for type definitions

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [dfx](https://internetcomputer.org/docs/current/developers-guide/install-upgrade-ic/#install-dfx) (Internet Computer SDK)

### Setup

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/YashK-194/ICP-Token-Wallet.git
   cd icp_token_wallet
2. **Install Dependencies:** Ensure you have the dfx command-line tool set up.

3. **Build the Project:**
```bash
cargo build
```
4. **Deploy the Project Locally:**

```bash
dfx start --background
dfx deploy
```
5. **Interact with the Wallet:** Use the dfx command-line tool or create a frontend application to interact with your wallet's canister.

### Testing
To run the tests included in this project, execute:

``` bash
cargo test
``` 

## Code Explanation
### Main Logic
The wallet uses a TokenLedger struct to manage token balances. Key functions include:

init: Initializes the ledger with an owner.
transfer: Allows one principal to send tokens to another, checking for sufficient balance.
receive_tokens: Updates the caller's balance upon receiving tokens.
balance and balance_of: Query current balances.

### Error Handling
An enum, WalletError, manages errors related to:

Insufficient balance
Unauthorized access
Invalid amounts

### Testing
The module includes tests for:

Initial balance verification.
Correct token reception.
Successful token transfers and balance updates.
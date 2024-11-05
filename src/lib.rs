use candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::*;
use std::collections::HashMap;
use std::cell::RefCell;

#[cfg(not(test))]
use ic_cdk::api::caller;  // Only use the actual caller in production

// Token ledger structure to store balances
#[derive(CandidType, Deserialize)]
struct TokenLedger {
    balances: HashMap<Principal, u64>,
    owner: Principal,
}

// Initialize token ledger with thread-safe interior mutability
thread_local! {
    static LEDGER: RefCell<TokenLedger> = RefCell::new(TokenLedger {
        balances: HashMap::new(),
        owner: Principal::anonymous(),
    });
}

// Error types for wallet operations
#[derive(CandidType, Deserialize, Debug)]
enum WalletError {
    InsufficientBalance,
    Unauthorized,
    InvalidAmount,
}

type WalletResult<T> = Result<T, WalletError>;

// Define get_caller function for both production and testing
#[cfg(not(test))]
fn get_caller() -> Principal {
    caller()
}

#[cfg(test)]
thread_local! {
    static MOCK_CALLER: RefCell<Principal> = RefCell::new(Principal::anonymous());
}

#[cfg(test)]
fn get_caller() -> Principal {
    MOCK_CALLER.with(|mock_caller| *mock_caller.borrow())
}

#[cfg(test)]
fn set_caller(principal: Principal) {
    MOCK_CALLER.with(|mock_caller| {
        *mock_caller.borrow_mut() = principal;
    });
}

// Initialize the token ledger with the owner
#[init]
fn init(owner: Principal) {
    LEDGER.with(|ledger| {
        let mut l = ledger.borrow_mut();
        l.owner = owner;
    });
}

// Transfer tokens from the caller to another principal
#[update]
fn transfer(to: Principal, amount: u64) -> WalletResult<()> {
    if amount == 0 {
        return Err(WalletError::InvalidAmount);
    }

    LEDGER.with(|ledger| {
        let mut l = ledger.borrow_mut();

        let sender_balance = *l.balances.get(&get_caller()).unwrap_or(&0);
        if sender_balance < amount {
            return Err(WalletError::InsufficientBalance);
        }

        l.balances.insert(get_caller(), sender_balance - amount);

        let recipient_balance = *l.balances.get(&to).unwrap_or(&0);
        l.balances.insert(to, recipient_balance + amount);

        Ok(())
    })
}

// Receive tokens from another principal
#[update]
fn receive_tokens(_from: Principal, amount: u64) -> WalletResult<()> {
    if amount == 0 {
        return Err(WalletError::InvalidAmount);
    }

    LEDGER.with(|ledger| {
        let mut l = ledger.borrow_mut();

        let current_balance = *l.balances.get(&get_caller()).unwrap_or(&0);
        l.balances.insert(get_caller(), current_balance + amount);

        Ok(())
    })
}

// Check the balance of the caller
#[query]
fn balance() -> u64 {
    LEDGER.with(|ledger| {
        let l = ledger.borrow();
        *l.balances.get(&get_caller()).unwrap_or(&0)
    })
}

// Check the balance of a specific principal
#[query]
fn balance_of(account: Principal) -> u64 {
    LEDGER.with(|ledger| {
        let l = ledger.borrow();
        *l.balances.get(&account).unwrap_or(&0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_cdk::export::Principal;

    #[test]
    fn test_initial_balance() {
        let owner = Principal::anonymous();
        init(owner);

        LEDGER.with(|ledger| {
            let l = ledger.borrow();
            assert_eq!(*l.balances.get(&owner).unwrap_or(&0), 0);
        });
    }

    #[test]
    fn test_receive_tokens() {
        let owner = Principal::anonymous();
        init(owner);

        let amount = 1000;

        set_caller(owner);
        receive_tokens(owner, amount).unwrap();

        LEDGER.with(|ledger| {
            let l = ledger.borrow();
            assert_eq!(*l.balances.get(&owner).unwrap_or(&0), amount);
        });
    }
}
    
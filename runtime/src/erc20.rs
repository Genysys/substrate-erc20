use rstd::prelude::*;
use srml_support::{dispatch::Result, StorageMap, StorageValue};
use {balances, system::ensure_signed};

// trait for this module
// contains type definitions
pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// public interface for this runtime module
// these functions can't be accessed by other modules
// they will be accessible only to the outer world
decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
      // initialize the default event for this module
      fn deposit_event<T>() = default;

      fn init(_origin) -> Result {
          let sender = ensure_signed(_origin)?;
          ensure!(Self::is_init() == false, "Already Initialized");

          <Balances<T>>::insert(sender.clone(), Self::total_supply());
          <Owner<T>>::put(sender);
          <Init<T>>::put(true);

          Ok(())
      }

      // transfer tokens from one account to another
      fn transfer(_origin, to: T::AccountId, value: T::Balance) -> Result {
          let sender = ensure_signed(_origin)?;
          Self::_transfer(sender, to, value)
      }

      // approve token transfer from one account to another
      // if this is done, then transfer_from can be called with corresponding values
      fn approve(_origin, spender: T::AccountId, value: T::Balance) -> Result {
          let sender = ensure_signed(_origin)?;
          ensure!(<Balances<T>>::exists(&sender), "Account does not own this token");
          Self::deposit_event(RawEvent::Approval(sender.clone(), spender.clone(), value));

          if <Allowed<T>>::exists((sender.clone(), spender.clone())) {
              <Allowed<T>>::mutate((sender, spender), |allowance| *allowance += value);
          } else {
              <Allowed<T>>::insert((sender, spender), value);
          }

          Ok(())
      }

      // if approved, transfer from an account to another account without needed owner's signature
      fn transfer_from(_origin, from: T::AccountId, to: T::AccountId, value: T::Balance) -> Result {
          Self::_transfer_from(from, to, value)
      }
  }
}

// storage for this runtime module
decl_storage! {
  trait Store for Module<T: Trait> as Erc20 {
    Init get(is_init): bool;
    TotalSupply get(total_supply) config(): T::Balance;
    Owner get(owner): T::AccountId;
    Name get(name) config(): Vec<u8>;
    Ticker get (ticker) config(): Vec<u8>;
    Balances get(balance_of): map T::AccountId => T::Balance;
    Allowed get(allowance): map (T::AccountId, T::AccountId) => T::Balance;
  }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId, Balance = <T as balances::Trait>::Balance {
        // event for transfer of tokens
        // from, to, value
        Transfer(AccountId, AccountId, Balance),
        // event when an approval is made
        // owner, spender, value
        Approval(AccountId, AccountId, Balance),
    }
);

// implementation of mudule
// utility and private functions
// if marked public, accessible by other modules
impl<T: Trait> Module<T> {
    // transfer from will be needed by other modules too
    // hence implemented here
    pub fn _transfer_from(
        from: T::AccountId,
        to: T::AccountId,
        value: T::Balance,
    ) -> Result {
        ensure!(<Allowed<T>>::exists((from.clone(), to.clone())), "Allowance does not exist.");
        ensure!(Self::allowance((from.clone(), to.clone())) >= value, "Not enough allowance.");

        <Allowed<T>>::mutate((from.clone(), to.clone()), |allowance| *allowance -= value);
        Self::deposit_event(RawEvent::Approval(from.clone(), to.clone(), value));

        Self::_transfer(from, to, value)
    }

    fn _transfer(
        from: T::AccountId,
        to: T::AccountId,
        value: T::Balance,
    ) -> Result {
        ensure!(<Balances<T>>::exists(&from), "Account does not own this token");

        let sender_balance = Self::balance_of(&from);
        ensure!(sender_balance > value, "Not enough balance.");

        Self::deposit_event(RawEvent::Transfer(from.clone(), to.clone(), value));
        
        // reduce sender's balance
        <Balances<T>>::mutate(from, |balance| *balance -= value);

        // increase receiver's balance
        if <Balances<T>>::exists(to.clone()) {
            <Balances<T>>::mutate(to, |balance| *balance += value);
        } else {
            <Balances<T>>::insert(to, value);
        }

        Ok(())
    }
}

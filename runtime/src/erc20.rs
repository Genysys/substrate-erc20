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
      
      // transfer tokens from one account to another
      fn transfer(_origin, to: T::AccountId, value: T::Balance) {

      }

      // approve token transfer from one account to another
      // if this is done, then transfer_from can be called with corresponding values
      fn approve(_origin, spender: T::AccountId, value: T::Balance) {

      }

      // if approved, transfer from an account to another account without needed owner's signature
      fn transfer_from(_origin, from: T::AccountId, to: T::AccountId, value: T::Balance) {
          Self::transfer_from_internal(_origin, from, to, value);
      }

      // get the total supply of the token
      fn totalSupply(_origin) {

      }

      // get the balance of an account
      fn balance_of(_origin, who: T::AccountId) {

      }

      // get the allowance of an account
      // this is teh value approved by owner for the spender using the approve function above
      fn allowance(_origin, owner: T::AccountId, spender: T::AccountId) {

      }
  }
}

// storage for this runtime module
decl_storage! {
  trait Store for Module<T: Trait> as ERC20 {
    
  }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as system::Trait>::AccountId, Balance = <T as balances::Trait>::Balance  {
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
    pub fn transfer_from_internal(origin: T::Origin, from: T::AccountId, to: T::AccountId, value: T::Balance) {

    }
}

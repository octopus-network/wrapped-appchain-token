use std::ops::Mul;

use near_contract_standards::fungible_token::core::ext_ft_core;
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{
    assert_one_yocto, assert_self, env, near_bindgen, AccountId, Gas, PanicOnDefault, Promise,
    PromiseOrValue,
};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct WrappedAppchainToken {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
    owner_id: AccountId,
}

#[near_bindgen]
impl WrappedAppchainToken {
    #[init]
    pub fn new(
        owner_id: AccountId,
        premined_beneficiary: AccountId,
        premined_balance: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized.");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
            owner_id: owner_id.clone().into(),
        };
        this.token
            .internal_register_account(&env::current_account_id());
        this.token.internal_register_account(&premined_beneficiary);
        this.internal_mint(premined_beneficiary.clone(), premined_balance);
        // sync state to corresponding appchain anchor contract
        #[derive(near_sdk::serde::Serialize)]
        #[serde(crate = "near_sdk::serde")]
        struct Args {
            metadata: FungibleTokenMetadata,
            premined_beneficiary: AccountId,
            premined_balance: U128,
        }
        let args = Args {
            metadata,
            premined_beneficiary,
            premined_balance,
        };
        let args = near_sdk::serde_json::to_vec(&args)
            .expect("Failed to serialize the cross contract args using JSON.");
        Promise::new(owner_id).function_call(
            "sync_basedata_of_wrapped_appchain_token".to_string(),
            args,
            0,
            Gas::ONE_TERA.mul(50),
        );
        this
    }
    ///
    #[payable]
    pub fn mint(&mut self, account_id: AccountId, amount: U128) {
        self.assert_owner();
        self.storage_deposit(Some(account_id.clone()), None);
        self.internal_mint(account_id, amount);
    }
    //
    fn internal_mint(&mut self, account_id: AccountId, amount: U128) {
        self.token
            .internal_deposit(&env::current_account_id(), amount.into());
        ext_ft_core::ext(env::current_account_id())
            .with_attached_deposit(1)
            .with_static_gas(Gas::ONE_TERA.mul(10))
            .ft_transfer(account_id, amount, None);
    }
    ///
    #[payable]
    pub fn burn(&mut self, account_id: AccountId, amount: U128) {
        assert_one_yocto();
        self.assert_owner();
        self.token.internal_withdraw(&account_id, amount.into());
    }
    ///
    pub fn set_icon(&mut self, icon: String) {
        assert_self();
        let mut metadata = self.metadata.get().unwrap();
        metadata.icon = Some(icon);
        self.metadata.set(&metadata);
    }
}

pub trait Ownable {
    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.get_owner(),
            "Only owner can call mint."
        );
    }
    fn get_owner(&self) -> AccountId;
    fn set_owner(&mut self, owner: AccountId);
}

#[near_bindgen]
impl Ownable for WrappedAppchainToken {
    fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    fn set_owner(&mut self, owner_id: AccountId) {
        self.assert_owner();
        self.owner_id = owner_id;
    }
}

near_contract_standards::impl_fungible_token_core!(WrappedAppchainToken, token);
near_contract_standards::impl_fungible_token_storage!(WrappedAppchainToken, token);

#[near_bindgen]
impl FungibleTokenMetadataProvider for WrappedAppchainToken {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

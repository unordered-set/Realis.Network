#![cfg_attr(not(feature = "std"), no_std)]

// 1. Imports and Dependencies
pub use frame_support::traits::Currency;
pub use pallet::*;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

pub use weights::WeightInfoRealis;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::sp_runtime::traits::AccountIdConversion;
    use frame_support::traits::Imbalance;
    use frame_support::traits::{Currency, ExistenceRequirement, WithdrawReasons};
    use frame_support::weights::Pays;
    use frame_support::PalletId;
    use frame_system::pallet_prelude::*;
    use marketplace;
    use node_primitives::Balance;
    use sp_runtime::traits::Saturating;

    use pallet_nft as NFT;
    use realis_primitives::{Rarity, Status, String, TokenId, TokenType};

    type BalanceOf<T> =
        <<T as Config>::ApiCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_nft::Config
        + pallet_staking::Config
        + pallet_balances::Config
        + marketplace::Config
        + pallet_nft_delegate::Config
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type PalletId: Get<PalletId>;

        type ApiCurrency: Currency<Self::AccountId, Balance = Self::Balance>;

        type StakingPoolId: From<<Self as pallet_staking::Config>::PalletId>;

        type WeightInfoRealis: WeightInfoRealis;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// NFT was minted in game
        NftMinted(T::AccountId, TokenId, String),
        /// NFT was transfered from player to player
        NftTransferred(T::AccountId, T::AccountId, TokenId),
        /// NFT was burned by player
        NftBurned(T::AccountId, TokenId),
        /// LIS was transfered from player to player
        FundsTransferred(T::AccountId, T::AccountId, BalanceOf<T>),
        /// User was spended tokens in game
        SpendInGame(T::AccountId, BalanceOf<T>),
        /// Pallet Balance
        Balance(T::AccountId, BalanceOf<T>),
        ///
        AddToWhiteList(T::AccountId),
        /// NFT delegated from player to player
        NftDelegated(T::AccountId, T::AccountId, TokenId),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        ///
        TokenExist,
        ///
        NotTokenOwner,
        ///
        NonExistentToken,

        NotApiMaster,

        ApiMasterWasAddedEarly,

        UserNotFoundInWhitelist,

        AccountAlreadyInWhitelist,

        CannotTransferNftBecauseThisNftOnAnotherUser,

        CannotTransferNftBecauseThisNftInMarketplace,

        CannotBuyOwnNft,
    }

    #[pallet::storage]
    #[pallet::getter(fn api_masters)]
    pub type ApiMasters<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn whitelist)]
    pub type Whitelist<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, (), ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn validator_whitelist)]
    pub type ValidatorWhitelist<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub api_masters: Vec<T::AccountId>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                api_masters: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            ApiMasters::<T>::put(&self.api_masters);
        }
    }

    #[cfg(feature = "std")]
    impl<T: Config> GenesisConfig<T> {
        /// Direct implementation of `GenesisBuild::build_storage`.
        ///
        /// Kept in order not to break dependency.
        pub fn build_storage(&self) -> Result<sp_runtime::Storage, std::string::String> {
            <Self as GenesisBuild<T>>::build_storage(self)
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // 7. Extrinsics
    // Functions that are callable from outside the runtime.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight((T::WeightInfoRealis::mint_nft(), Pays::No))]
        pub fn mint_nft(
            origin: OriginFor<T>,
            target_account: T::AccountId,
            token_id: TokenId,
            mint_id: u32,
            name: String,
            rarity: Rarity,
            id: String,
            link: String,
        ) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);
            // ensure!(
            //     Whitelist::<T>::contains_key(&target_account),
            //     Error::<T>::UserNotFoundInWhitelist
            // );

            ensure!(
                !NFT::AccountForToken::<T>::contains_key(token_id),
                Error::<T>::TokenExist
            );

            NFT::Pallet::<T>::mint(
                origin.clone(),
                target_account.clone(),
                name,
                token_id,
                mint_id,
                rarity,
                link,
            )?;
            Self::deposit_event(Event::<T>::NftMinted(target_account.clone(), token_id, id));
            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::burn_nft(), Pays::No))]
        pub fn burn_nft(
            origin: OriginFor<T>,
            from: T::AccountId,
            token_id: TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);
            ensure!(
                Whitelist::<T>::contains_key(&from),
                Error::<T>::UserNotFoundInWhitelist
            );
            let tokens = NFT::TokensList::<T>::get(from.clone());
            for token in tokens {
                if token.0.id == token_id {
                    ensure!(
                        token.1 != Status::OnSell,
                        Error::<T>::CannotTransferNftBecauseThisNftInMarketplace
                    );
                    ensure!(
                        token.1 != Status::InDelegation,
                        Error::<T>::CannotTransferNftBecauseThisNftOnAnotherUser
                    );
                    ensure!(
                        token.1 != Status::OnDelegateSell,
                        Error::<T>::CannotTransferNftBecauseThisNftOnAnotherUser
                    );
                };
            }
            NFT::Pallet::<T>::burn_nft(token_id, &from)?;
            Self::deposit_event(Event::<T>::NftBurned(from, token_id));
            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::transfer_nft(), Pays::No))]
        pub fn transfer_nft(
            origin: OriginFor<T>,
            from: T::AccountId,
            dest: T::AccountId,
            token_id: TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);
            ensure!(
                Whitelist::<T>::contains_key(&from),
                Error::<T>::UserNotFoundInWhitelist
            );
            // ensure!(
            //     Self::whitelist().contains(&dest),
            //     Error::<T>::UserNotFoundInWhitelist
            // );
            let tokens = NFT::TokensList::<T>::get(from.clone());
            for token in tokens {
                if token.0.id == token_id {
                    ensure!(
                        token.1 != Status::OnSell,
                        Error::<T>::CannotTransferNftBecauseThisNftInMarketplace
                    );
                    ensure!(
                        token.1 != Status::InDelegation,
                        Error::<T>::CannotTransferNftBecauseThisNftOnAnotherUser
                    );
                    ensure!(
                        token.1 != Status::OnDelegateSell,
                        Error::<T>::CannotTransferNftBecauseThisNftOnAnotherUser
                    );
                };
            }

            NFT::Pallet::<T>::transfer_nft(&dest, &from, token_id)?;
            Self::deposit_event(Event::<T>::NftTransferred(from, dest, token_id));
            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::transfer_from_pallet(), Pays::No))]
        pub fn transfer_from_pallet(
            origin: OriginFor<T>,
            dest: T::AccountId,
            #[pallet::compact] amount: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);
            // ensure!(
            //     Whitelist::<T>::contains_key(&dest),
            //     Error::<T>::UserNotFoundInWhitelist
            // );
            let pallet_id = Self::account_id();
            <T as Config>::ApiCurrency::transfer(
                &pallet_id,
                &dest,
                amount,
                ExistenceRequirement::KeepAlive,
            )?;
            Self::deposit_event(Event::<T>::FundsTransferred(pallet_id, dest, amount));
            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::transfer_to_pallet(), Pays::No))]
        pub fn transfer_to_pallet(
            origin: OriginFor<T>,
            from: T::AccountId,
            #[pallet::compact] amount: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);
            ensure!(
                Whitelist::<T>::contains_key(&from),
                Error::<T>::UserNotFoundInWhitelist
            );
            let pallet_id = Self::account_id();
            <T as Config>::ApiCurrency::transfer(
                &from,
                &pallet_id,
                amount,
                ExistenceRequirement::KeepAlive,
            )?;
            Self::deposit_event(Event::<T>::FundsTransferred(from, pallet_id, amount));
            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::transfer_from_ptp(), Pays::No))]
        pub fn transfer_from_ptp(
            origin: OriginFor<T>,
            from: T::AccountId,
            dest: T::AccountId,
            #[pallet::compact] amount: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);
            ensure!(
                Whitelist::<T>::contains_key(&from),
                Error::<T>::UserNotFoundInWhitelist
            );
            <T as Config>::ApiCurrency::transfer(
                &from,
                &dest,
                amount,
                ExistenceRequirement::KeepAlive,
            )?;
            Self::deposit_event(Event::<T>::FundsTransferred(from, dest, amount));
            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::spend_in_game(), Pays::No))]
        pub fn spend_in_game(
            origin: OriginFor<T>,
            dest: T::AccountId,
            #[pallet::compact] amount: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);
            ensure!(
                Whitelist::<T>::contains_key(&who),
                Error::<T>::UserNotFoundInWhitelist
            );
            let imbalance = <T as Config>::ApiCurrency::withdraw(
                &dest,
                amount,
                WithdrawReasons::all(),
                ExistenceRequirement::KeepAlive,
            )?;
            let pallet_id_staking = Self::account_id_staking();
            let pallet_id_game_api = Self::account_id();
            // for fees, 80% to treasury, 20% to author
            let (to_game_api, to_staking) = imbalance.ration(80, 20);
            <T as Config>::ApiCurrency::resolve_creating(&pallet_id_game_api, to_game_api);
            <T as Config>::ApiCurrency::resolve_creating(&pallet_id_staking, to_staking);
            Self::deposit_event(Event::<T>::SpendInGame(dest, amount));
            Ok(())
        }

        #[pallet::weight((90_000_000, Pays::No))]
        pub fn balance_pallet(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);
            let account_id = Self::account_id();
            let balance = <T as Config>::ApiCurrency::free_balance(&account_id)
                .saturating_sub(<T as Config>::ApiCurrency::minimum_balance());
            Self::deposit_event(Event::Balance(account_id, balance));
            Ok(())
        }

        #[pallet::weight((90_000_000, Pays::No))]
        pub fn add_api_master(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            // Check is signed correct
            let who = ensure_signed(origin)?;
            // Check if account that signed operation have permission for this operation
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);
            ensure!(
                !Self::api_masters().contains(&account),
                Error::<T>::ApiMasterWasAddedEarly
            );

            ApiMasters::<T>::mutate(|nft_masters| {
                nft_masters.push(account);
            });
            Ok(())
        }

        /// Remove api_master
        #[pallet::weight((90_000_000, Pays::No))]
        pub fn remove_api_master(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            // Check is signed correct
            let who = ensure_signed(origin)?;
            // Check if account that signed operation have permission for this operation
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);

            ApiMasters::<T>::mutate(|api_masters| {
                let index = api_masters.iter().position(|token| *token == account);
                api_masters.remove(index.unwrap())
            });
            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::add_to_whitelist(), Pays::No))]
        pub fn add_to_whitelist(origin: OriginFor<T>) -> DispatchResult {
            // Check is signed correct
            let who = ensure_signed(origin)?;
            // Check if account that signed operation have permission for this operation
            ensure!(
                !Whitelist::<T>::contains_key(&who),
                Error::<T>::AccountAlreadyInWhitelist
            );

            Whitelist::<T>::insert(who.clone(), ());

            Self::deposit_event(Event::AddToWhiteList(who));

            Ok(())
        }

        /// Remove api_master
        #[pallet::weight(T::WeightInfoRealis::remove_from_whitelist())]
        pub fn remove_from_whitelist(origin: OriginFor<T>) -> DispatchResult {
            // Check is signed correct
            let who = ensure_signed(origin)?;
            // Check if account that signed operation have permission for this operation
            Whitelist::<T>::remove(&who);
            Ok(())
        }

        #[pallet::weight(T::WeightInfoRealis::add_to_validator_whitelist())]
        pub fn add_to_validator_whitelist(
            origin: OriginFor<T>,
            account_id: T::AccountId,
        ) -> DispatchResult {
            // Check is signed correct
            let who = ensure_signed(origin)?;
            // Check if account that signed operation have permission for this operation
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);

            ValidatorWhitelist::<T>::mutate(|member_whitelist| {
                member_whitelist.push(account_id);
            });
            Ok(())
        }

        #[pallet::weight(T::WeightInfoRealis::remove_from_validator_whitelist())]
        pub fn remove_from_validator_whitelist(
            origin: OriginFor<T>,
            account_id: T::AccountId,
        ) -> DispatchResult {
            // Check is signed correct
            let _who = ensure_signed(origin)?;
            // Check if account that signed operation have permission for this operation
            ValidatorWhitelist::<T>::mutate(|member_whitelist| {
                let index = member_whitelist.iter().position(|user| *user == account_id);
                member_whitelist.remove(index.unwrap())
            });
            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::sell_nft(), Pays::No))]
        pub fn sell_nft(
            origin: OriginFor<T>,
            account_id: T::AccountId,
            token_id: TokenId,
            amount: u128,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);

            ensure!(
                Whitelist::<T>::contains_key(&account_id),
                Error::<T>::UserNotFoundInWhitelist
            );

            let tokens = NFT::TokensList::<T>::get(account_id.clone());
            let mut rarity = vec![];
            for token in tokens {
                if token.0.id == token_id {
                    ensure!(
                        token.1 == Status::Free,
                        Error::<T>::CannotTransferNftBecauseThisNftInMarketplace
                    );

                    let TokenType::Basic(rare, _, _, _) = token.0.token_type;
                    rarity.insert(0, rare);
                };
            }
            marketplace::Pallet::<T>::sell(account_id.clone(), token_id, rarity[0], amount);

            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::buy_nft(), Pays::No))]
        pub fn buy_nft(
            origin: OriginFor<T>,
            account_id: T::AccountId,
            token_id: TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);

            ensure!(
                Whitelist::<T>::contains_key(&account_id),
                Error::<T>::UserNotFoundInWhitelist
            );

            marketplace::Pallet::<T>::buy(account_id, token_id)?;
            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::change_price_nft(), Pays::No))]
        pub fn change_price_nft(
            origin: OriginFor<T>,
            account_id: T::AccountId,
            token_id: TokenId,
            amount: u128,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);

            ensure!(
                Whitelist::<T>::contains_key(&account_id),
                Error::<T>::UserNotFoundInWhitelist
            );

            marketplace::Pallet::<T>::change_price(account_id, token_id, amount);
            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::remove_nft(), Pays::No))]
        pub fn remove_nft(
            origin: OriginFor<T>,
            account_id: T::AccountId,
            token_id: TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);

            ensure!(
                Whitelist::<T>::contains_key(&account_id),
                Error::<T>::UserNotFoundInWhitelist
            );

            marketplace::Pallet::<T>::remove(account_id, token_id);
            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::delegate_nft(), Pays::No))]
        pub fn delegate_nft(
            origin: OriginFor<T>,
            from: T::AccountId,
            to: T::AccountId,
            token_id: TokenId,
            delegated_time: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);
            ensure!(
                Whitelist::<T>::contains_key(&from),
                Error::<T>::UserNotFoundInWhitelist
            );
            let owner =
                NFT::AccountForToken::<T>::get(token_id).ok_or(Error::<T>::NonExistentToken)?;
            ensure!(from == owner, Error::<T>::NotTokenOwner);

            pallet_nft_delegate::Pallet::<T>::check_time(delegated_time)?;
            pallet_nft_delegate::Pallet::<T>::can_delegate_nft(token_id)?;

            pallet_nft_delegate::Pallet::<T>::delegate_nft(owner, to, token_id, delegated_time);

            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::sell_delegate_nft(), Pays::No))]
        pub fn sell_delegate_nft(
            origin: OriginFor<T>,
            seller: T::AccountId,
            token_id: TokenId,
            delegated_time: u32,
            price: Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);
            ensure!(
                Whitelist::<T>::contains_key(&seller),
                Error::<T>::UserNotFoundInWhitelist
            );
            let owner =
                NFT::AccountForToken::<T>::get(token_id).ok_or(Error::<T>::NonExistentToken)?;
            ensure!(seller == owner, Error::<T>::NotTokenOwner);

            pallet_nft_delegate::Pallet::<T>::check_time(delegated_time)?;
            pallet_nft_delegate::Pallet::<T>::can_delegate_nft(token_id)?;

            pallet_nft_delegate::Pallet::<T>::sale_delegate_nft(
                owner,
                token_id,
                delegated_time,
                price,
            );

            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::buy_delegate_nft(), Pays::No))]
        pub fn buy_delegate_nft(
            origin: OriginFor<T>,
            buyer: T::AccountId,
            token_id: TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);
            ensure!(
                Whitelist::<T>::contains_key(&buyer),
                Error::<T>::UserNotFoundInWhitelist
            );
            let owner =
                NFT::AccountForToken::<T>::get(token_id).ok_or(Error::<T>::NonExistentToken)?;
            ensure!(buyer != owner, Error::<T>::CannotBuyOwnNft);

            pallet_nft_delegate::Pallet::<T>::buy_delegate_nft(buyer, token_id)
        }

        #[pallet::weight((T::WeightInfoRealis::change_price_delegate_nft(), Pays::No))]
        pub fn change_price_delegate_nft(
            origin: OriginFor<T>,
            seller: T::AccountId,
            token_id: TokenId,
            new_price: Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);
            ensure!(
                Whitelist::<T>::contains_key(&seller),
                Error::<T>::UserNotFoundInWhitelist
            );
            let owner =
                NFT::AccountForToken::<T>::get(token_id).ok_or(Error::<T>::NonExistentToken)?;
            ensure!(seller == owner, Error::<T>::NotTokenOwner);

            pallet_nft_delegate::Pallet::<T>::change_price_delegate_nft(token_id, new_price);

            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::change_delegate_nft_time_on_sale(), Pays::No))]
        pub fn change_delegate_nft_time_on_sale(
            origin: OriginFor<T>,
            seller: T::AccountId,
            token_id: TokenId,
            new_time: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);
            ensure!(
                Whitelist::<T>::contains_key(&seller),
                Error::<T>::UserNotFoundInWhitelist
            );
            let owner =
                NFT::AccountForToken::<T>::get(token_id).ok_or(Error::<T>::NonExistentToken)?;
            ensure!(seller == owner, Error::<T>::NotTokenOwner);

            pallet_nft_delegate::Pallet::<T>::check_time(new_time)?;

            pallet_nft_delegate::Pallet::<T>::change_delegate_nft_time_on_sale(token_id, new_time);

            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::remove_from_sell(), Pays::No))]
        pub fn remove_from_sell(
            origin: OriginFor<T>,
            seller: T::AccountId,
            token_id: TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);
            ensure!(
                Whitelist::<T>::contains_key(&seller),
                Error::<T>::UserNotFoundInWhitelist
            );
            let owner =
                NFT::AccountForToken::<T>::get(token_id).ok_or(Error::<T>::NonExistentToken)?;
            ensure!(seller == owner, Error::<T>::NotTokenOwner);

            pallet_nft_delegate::Pallet::<T>::remove_nft_from_sell(token_id);

            Ok(())
        }

        #[pallet::weight((T::WeightInfoRealis::remove_delegate(), Pays::No))]
        pub fn remove_delegate(
            origin: OriginFor<T>,
            delegator: T::AccountId,
            token_id: TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;

            ensure!(Self::api_masters().contains(&who), Error::<T>::NotApiMaster);
            ensure!(
                Whitelist::<T>::contains_key(&delegator),
                Error::<T>::UserNotFoundInWhitelist
            );

            let owner =
                NFT::AccountForToken::<T>::get(token_id).ok_or(Error::<T>::NonExistentToken)?;

            ensure!(delegator == owner, Error::<T>::NotTokenOwner);

            pallet_nft_delegate::Pallet::<T>::remove_delegate(origin, token_id)?;

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn account_id() -> T::AccountId {
            <T as Config>::PalletId::get().into_account()
        }

        pub fn account_id_staking() -> T::AccountId {
            <T as pallet_staking::Config>::PalletId::get().into_account()
        }
    }
}

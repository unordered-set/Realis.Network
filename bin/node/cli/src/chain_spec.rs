// This file is part of Substrate.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use grandpa_primitives::AuthorityId as GrandpaId;
use hex_literal::hex;
use node_runtime::{
    wasm_binary_unwrap, AuthorityDiscoveryConfig, BabeConfig,
    BalancesConfig, /*CouncilConfig,*/
    /*DemocracyConfig,*/ /*ElectionsConfig,*/ GrandpaConfig, ImOnlineConfig, IndicesConfig,
    NftConfig, RealisBridgeConfig, RealisGameApiConfig, SessionConfig, SessionKeys,
    /*SocietyConfig,*/ StakerStatus, StakingConfig, SudoConfig, SystemConfig,
    /*TechnicalCommitteeConfig,*/ MAX_NOMINATIONS,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};

pub use node_primitives::{AccountId, Balance, Signature};
use node_runtime::constants::currency::DOLLARS;
use node_runtime::pallet_staking;
use node_runtime::realis_game_api;
use node_runtime::Runtime;
pub use node_runtime::{Block, GenesisConfig};
use sc_telemetry::serde_json::Map;

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<Block>,
    /// The light sync state extension used by the sync-state rpc.
    pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;
/// Flaming Fir testnet generator
pub fn flaming_fir_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/flaming-fir.json")[..])
}

fn session_keys(
    grandpa: GrandpaId,
    babe: BabeId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
    }
}

fn staging_testnet_config_genesis() -> GenesisConfig {
    #[rustfmt::skip]
        // stash, controller, session-key
        // generated with secret:
        // for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
        //
        // and
        //
        // for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done

        let initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )> = vec![
        (
            // 5Fbsd6WXDGiLTxunqeK5BATNiocfCqu9bS1yArVjCgeBLkVy
            hex!["9c7a2ee14e565db0c69f78c7b4cd839fbf52b607d867e9e9c5a79042898a0d12"].into(),
            // 5EnCiV7wSHeNhjW3FSUwiJNkcc2SBkPLn5Nj93FmbLtBjQUq
            hex!["781ead1e2fa9ccb74b44c19d29cb2a7a4b5be3972927ae98cd3877523976a276"].into(),
            // 5Fb9ayurnxnaXj56CjmyQLBiadfRCqUbL2VWNbbe1nZU6wiC
            hex!["9becad03e6dcac03cee07edebca5475314861492cdfc96a2144a67bbe9699332"]
                .unchecked_into(),
            // 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
            hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"]
                .unchecked_into(),
            // 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
            hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"]
                .unchecked_into(),
            // 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
            hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"]
                .unchecked_into(),
        ),
        (
            // 5ERawXCzCWkjVq3xz1W5KGNtVx2VdefvZ62Bw1FEuZW4Vny2
            hex!["68655684472b743e456907b398d3a44c113f189e56d1bbfd55e889e295dfde78"].into(),
            // 5Gc4vr42hH1uDZc93Nayk5G7i687bAQdHHc9unLuyeawHipF
            hex!["c8dc79e36b29395413399edaec3e20fcca7205fb19776ed8ddb25d6f427ec40e"].into(),
            // 5EockCXN6YkiNCDjpqqnbcqd4ad35nU4RmA1ikM4YeRN4WcE
            hex!["7932cff431e748892fa48e10c63c17d30f80ca42e4de3921e641249cd7fa3c2f"]
                .unchecked_into(),
            // 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
            hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"]
                .unchecked_into(),
            // 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
            hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"]
                .unchecked_into(),
            // 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
            hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"]
                .unchecked_into(),
        ),
        (
            // 5DyVtKWPidondEu8iHZgi6Ffv9yrJJ1NDNLom3X9cTDi98qp
            hex!["547ff0ab649283a7ae01dbc2eb73932eba2fb09075e9485ff369082a2ff38d65"].into(),
            // 5FeD54vGVNpFX3PndHPXJ2MDakc462vBCD5mgtWRnWYCpZU9
            hex!["9e42241d7cd91d001773b0b616d523dd80e13c6c2cab860b1234ef1b9ffc1526"].into(),
            // 5E1jLYfLdUQKrFrtqoKgFrRvxM3oQPMbf6DfcsrugZZ5Bn8d
            hex!["5633b70b80a6c8bb16270f82cca6d56b27ed7b76c8fd5af2986a25a4788ce440"]
                .unchecked_into(),
            // 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
            hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"]
                .unchecked_into(),
            // 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
            hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"]
                .unchecked_into(),
            // 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
            hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"]
                .unchecked_into(),
        ),
        (
            // 5HYZnKWe5FVZQ33ZRJK1rG3WaLMztxWrrNDb1JRwaHHVWyP9
            hex!["f26cdb14b5aec7b2789fd5ca80f979cef3761897ae1f37ffb3e154cbcc1c2663"].into(),
            // 5EPQdAQ39WQNLCRjWsCk5jErsCitHiY5ZmjfWzzbXDoAoYbn
            hex!["66bc1e5d275da50b72b15de072a2468a5ad414919ca9054d2695767cf650012f"].into(),
            // 5DMa31Hd5u1dwoRKgC4uvqyrdK45RHv3CpwvpUC1EzuwDit4
            hex!["3919132b851ef0fd2dae42a7e734fe547af5a6b809006100f48944d7fae8e8ef"]
                .unchecked_into(),
            // 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
            hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"]
                .unchecked_into(),
            // 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
            hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"]
                .unchecked_into(),
            // 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
            hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"]
                .unchecked_into(),
        ),
    ];

    // generated with secret: subkey inspect "$secret"/fir
    let root_key: AccountId = hex![
        // 5Ff3iXP75ruzroPWRP2FYBHWnmGGBSb63857BgnzCoXNxfPo
        "9ee5e5bdc0ec239eb164f865ecc345ce4c88e76ee002e0f7e318097347471809"
    ]
    .into();

    let nft_master: Vec<AccountId> = vec![hex![
        // 5Ff3iXP75ruzroPWRP2FYBHWnmGGBSb63857BgnzCoXNxfPo
        "9ee5e5bdc0ec239eb164f865ecc345ce4c88e76ee002e0f7e318097347471809"
    ]
    .into()];

    let api_master: Vec<AccountId> = vec![hex![
        // 5Ff3iXP75ruzroPWRP2FYBHWnmGGBSb63857BgnzCoXNxfPo
        "9ee5e5bdc0ec239eb164f865ecc345ce4c88e76ee002e0f7e318097347471809"
    ]
    .into()];

    let bridge_master: Vec<AccountId> = vec![hex![
        // 5Ff3iXP75ruzroPWRP2FYBHWnmGGBSb63857BgnzCoXNxfPo
        "9ee5e5bdc0ec239eb164f865ecc345ce4c88e76ee002e0f7e318097347471809"
    ]
    .into()];

    let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

    testnet_genesis(
        initial_authorities,
        vec![],
        root_key,
        nft_master,
        api_master,
        bridge_master,
        Some(endowed_accounts),
    )
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
    let boot_nodes = vec![];
    ChainSpec::from_genesis(
        "Staging Testnet",
        "staging_testnet",
        ChainType::Live,
        staging_testnet_config_genesis,
        boot_nodes,
        Some(
            TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
                .expect("Staging telemetry url is valid; qed"),
        ),
        None,
        None,
        Default::default(),
    )
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
    )
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )>,
    initial_nominators: Vec<AccountId>,
    root_key: AccountId,
    nft_master: Vec<AccountId>,
    api_master: Vec<AccountId>,
    bridge_master: Vec<AccountId>,
    endowed_accounts: Option<Vec<AccountId>>,
) -> GenesisConfig {
    let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
            realis_game_api::Pallet::<Runtime>::account_id(),
            pallet_staking::Pallet::<Runtime>::account_id(),
        ]
    });
    // endow all authorities and nominators.
    initial_authorities
        .iter()
        .map(|x| &x.0)
        .chain(initial_nominators.iter())
        .for_each(|x| {
            if !endowed_accounts.contains(&x) {
                endowed_accounts.push(x.clone())
            }
        });

    // stakers: all validators and nominators.
    let mut rng = rand::thread_rng();
    let stakers = initial_authorities
        .iter()
        .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
        .chain(initial_nominators.iter().map(|x| {
            use rand::{seq::SliceRandom, Rng};
            let limit = (MAX_NOMINATIONS as usize).min(initial_authorities.len());
            let count = rng.gen::<usize>() % limit;
            let nominations = initial_authorities
                .as_slice()
                .choose_multiple(&mut rng, count)
                .into_iter()
                .map(|choice| choice.0.clone())
                .collect::<Vec<_>>();
            (
                x.clone(),
                x.clone(),
                STASH,
                StakerStatus::Nominator(nominations),
            )
        }))
        .collect::<Vec<_>>();

    let _num_endowed_accounts = endowed_accounts.len();

    const ENDOWMENT: Balance = 30_000 * DOLLARS / 10;
    const GAME_WALLET: Balance = 10_000 * DOLLARS / 10;
    const STAKING_POOL: Balance = 30_000 * DOLLARS / 10;
    const STASH: Balance = ENDOWMENT / 1000;

    let pallet_id_staking = pallet_staking::Pallet::<Runtime>::account_id();
    let game_wallet = realis_game_api::Pallet::<Runtime>::account_id();

    GenesisConfig {
        system: SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
            changes_trie_config: Default::default(),
        },
        balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|x| {
                    if x == pallet_id_staking {
                        (x, STAKING_POOL)
                    } else if x == game_wallet {
                        (x, GAME_WALLET)
                    } else {
                        (x, ENDOWMENT)
                    }
                })
                .collect(),
        },
        indices: IndicesConfig { indices: vec![] },
        session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        },
        staking: StakingConfig {
            validator_count: initial_authorities.len() as u32,
            minimum_validator_count: initial_authorities.len() as u32,
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            stakers,
            ..Default::default()
        },
        // democracy: DemocracyConfig::default(),
        // elections: ElectionsConfig {
        //     members: endowed_accounts
        //         .iter()
        //         .take((num_endowed_accounts + 1) / 2)
        //         .cloned()
        //         .map(|member| (member, STASH))
        //         .collect(),
        // },
        // council: CouncilConfig::default(),
        // technical_committee: TechnicalCommitteeConfig {
        //     members: endowed_accounts
        //         .iter()
        //         .take((num_endowed_accounts + 1) / 2)
        //         .cloned()
        //         .collect(),
        //     phantom: Default::default(),
        // },
        sudo: SudoConfig { key: root_key },
        babe: BabeConfig {
            authorities: vec![],
            epoch_config: Some(node_runtime::BABE_GENESIS_EPOCH_CONFIG),
        },
        im_online: ImOnlineConfig { keys: vec![] },
        authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
        grandpa: GrandpaConfig {
            authorities: vec![],
        },
        // technical_membership: Default::default(),
        // treasury: Default::default(),
        // society: SocietyConfig {
        //     members: endowed_accounts
        //         .iter()
        //         .take((num_endowed_accounts + 1) / 2)
        //         .cloned()
        //         .collect(),
        //     pot: 0,
        //     max_members: 999,
        // },
        vesting: Default::default(),
        gilt: Default::default(),
        nft: NftConfig {
            nft_masters: nft_master,
        },
        realis_game_api: RealisGameApiConfig {
            api_masters: api_master,
        },
        realis_bridge: RealisBridgeConfig {
            bridge_masters: bridge_master,
        },
    }
}

///Realis chain-spec
pub fn realis_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )>,
    initial_nominators: Vec<AccountId>,
    root_key: AccountId,
    nft_master: Vec<AccountId>,
    api_master: Vec<AccountId>,
    bridge_master: Vec<AccountId>,
    endowed_accounts: Option<Vec<AccountId>>,
) -> GenesisConfig {
    let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            hex!["781f4331933557680355932ef7f39b88e938fcb4338cc0e03edb3c523e47fd09"].into(),
            hex!["fe8823fb870f61eed24638a228adbe5885de6e945bb1375ca6a7415a4824756e"].into(),
            hex!["bc95bdafa3582b0ecbf5caf1e30b00412fa7c2dfbccd518f3b842c63890cc979"].into(),
            hex!["08bdc3547dc26a647391b509960b00adafa550496e9a95339a2faa02343be20f"].into(),
            hex!["d4c2ffb1322efb7fe78463ad6f24301751454685edd96640197cab2c44e1b16c"].into(),
            hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"].into(),
            hex!["d671cde125c8b7f42afbf40fb9d0d93d4d80c888cd34824c99ab292b589dbe75"].into(),
            hex!["d4c2ffb1322efb7fe78463ad6f24301751454685edd96640197cab2c44e1b16c"].into(),
            pallet_staking::Pallet::<Runtime>::account_id(),
            realis_game_api::Pallet::<Runtime>::account_id(),
        ]
    });
    // endow all authorities and nominators.
    initial_authorities
        .iter()
        .map(|x| &x.0)
        .chain(initial_nominators.iter())
        .for_each(|x| {
            if !endowed_accounts.contains(&x) {
                endowed_accounts.push(x.clone())
            }
        });

    // stakers: all validators and nominators.
    let mut rng = rand::thread_rng();
    let stakers = initial_authorities
        .iter()
        .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
        .chain(initial_nominators.iter().map(|x| {
            use rand::{seq::SliceRandom, Rng};
            let limit = (MAX_NOMINATIONS as usize).min(initial_authorities.len());
            let count = rng.gen::<usize>() % limit;
            let nominations = initial_authorities
                .as_slice()
                .choose_multiple(&mut rng, count)
                .into_iter()
                .map(|choice| choice.0.clone())
                .collect::<Vec<_>>();
            (
                x.clone(),
                x.clone(),
                STASH,
                StakerStatus::Nominator(nominations),
            )
        }))
        .collect::<Vec<_>>();
    let _num_endowed_accounts = endowed_accounts.len();

    const ENDOWMENT: Balance = 900_000 * DOLLARS / 12 * 100;
    const GAME_WALLET: Balance = 10_000 * DOLLARS / 10;
    const STAKING_POOL: Balance = 30_000 * DOLLARS / 10;
    const STASH: Balance = ENDOWMENT / 1000;

    let pallet_id_staking = pallet_staking::Pallet::<Runtime>::account_id();
    let game_wallet = realis_game_api::Pallet::<Runtime>::account_id();

    GenesisConfig {
        system: SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
            changes_trie_config: Default::default(),
        },
        balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|x| {
                    if x == pallet_id_staking {
                        (x, STAKING_POOL)
                    } else if x == game_wallet {
                        (x, GAME_WALLET)
                    } else {
                        (x, ENDOWMENT)
                    }
                })
                .collect(),
        },
        indices: IndicesConfig { indices: vec![] },
        session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        },
        staking: StakingConfig {
            validator_count: initial_authorities.len() as u32,
            minimum_validator_count: initial_authorities.len() as u32,
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            stakers,
            ..Default::default()
        },
        // democracy: DemocracyConfig::default(),
        // elections: ElectionsConfig {
        //     members: endowed_accounts
        //         .iter()
        //         .take((num_endowed_accounts + 1) / 2)
        //         .cloned()
        //         .map(|member| (member, STASH))
        //         .collect(),
        // },
        // council: CouncilConfig::default(),
        // technical_committee: TechnicalCommitteeConfig {
        //     members: endowed_accounts
        //         .iter()
        //         .take((num_endowed_accounts + 1) / 2)
        //         .cloned()
        //         .collect(),
        //     phantom: Default::default(),
        // },
        sudo: SudoConfig { key: root_key },
        babe: BabeConfig {
            authorities: vec![],
            epoch_config: Some(node_runtime::BABE_GENESIS_EPOCH_CONFIG),
        },
        im_online: ImOnlineConfig { keys: vec![] },
        authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
        grandpa: GrandpaConfig {
            authorities: vec![],
        },
        // technical_membership: Default::default(),
        // treasury: Default::default(),
        // society: SocietyConfig {
        //     members: endowed_accounts
        //         .iter()
        //         .take((num_endowed_accounts + 1) / 2)
        //         .cloned()
        //         .collect(),
        //     pot: 0,
        //     max_members: 999,
        // },
        vesting: Default::default(),
        gilt: Default::default(),
        nft: NftConfig {
            nft_masters: nft_master,
        },
        realis_game_api: RealisGameApiConfig {
            api_masters: api_master,
        },
        realis_bridge: RealisBridgeConfig {
            bridge_masters: bridge_master,
        },
    }
}

///Realis test chain-spec
pub fn realis_testnet_config() -> ChainSpec {
    let mut properties = Map::new();
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("tokenSymbol".into(), "LIS".into());
    properties.insert("ss58Format".into(), 42.into());

    ChainSpec::from_genesis(
        "Realis Network",
        "realis",
        ChainType::Live,
        realis_testnet_genesis,
        vec![],
        None,
        None,
        Some(properties),
        Default::default(),
    )
}

///Realis ctestnet genesis
pub fn realis_testnet_genesis() -> GenesisConfig {
    let initial_authorities = vec![
        (
            hex!["1aa0d5c594a4581ec17069ec9631cd6225d5fb403fe4d85c8ec8aa51833fdf7f"].into(),
            hex!["d671cde125c8b7f42afbf40fb9d0d93d4d80c888cd34824c99ab292b589dbe75"].into(),
            hex!["b7606f13fb700cdabffd98bf466557a9faeb68bc773ef6e2bf681b9913079d37"]
                .unchecked_into(),
            hex!["d671cde125c8b7f42afbf40fb9d0d93d4d80c888cd34824c99ab292b589dbe75"]
                .unchecked_into(),
            hex!["d671cde125c8b7f42afbf40fb9d0d93d4d80c888cd34824c99ab292b589dbe75"]
                .unchecked_into(),
            hex!["d671cde125c8b7f42afbf40fb9d0d93d4d80c888cd34824c99ab292b589dbe75"]
                .unchecked_into(),
        ),
        (
            hex!["cc32b24b66c8636b31394dce95949a27022c901d2597c5584554aa5d81db7416"].into(),
            hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"].into(),
            hex!["4a9e6cc2606a74d65ee2ba026e986024de8b60a22890023552b6cf6c977c8420"]
                .unchecked_into(),
            hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"]
                .unchecked_into(),
            hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"]
                .unchecked_into(),
            hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"]
                .unchecked_into(),
        ),
        (
            hex!["24c42c17c4f95987c9916fc7e9bcd0c9385b6724f72658d943b643b6c3d83b73"].into(),
            hex!["dc869f188c87d823da3d8e6b069a2688d0772d2dc3f09d8dfa96b8551a601513"].into(),
            hex!["32e610d5ed216b2681ba9ad4907f05220ef9b81edf7049dd73c732a670c14379"]
                .unchecked_into(),
            hex!["dc869f188c87d823da3d8e6b069a2688d0772d2dc3f09d8dfa96b8551a601513"]
                .unchecked_into(),
            hex!["dc869f188c87d823da3d8e6b069a2688d0772d2dc3f09d8dfa96b8551a601513"]
                .unchecked_into(),
            hex!["dc869f188c87d823da3d8e6b069a2688d0772d2dc3f09d8dfa96b8551a601513"]
                .unchecked_into(),
        ),
    ];
    //sudo account
    let root_key = hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"].into();
    //NFT Master
    let nft_master =
        vec![hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"].into()];
    let api_master =
        vec![hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"].into()];
    let bridge_master =
        vec![hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"].into()];
    let endowed_accounts =
        vec![hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"].into()];
    realis_genesis(
        initial_authorities,
        vec![],
        root_key,
        nft_master,
        api_master,
        bridge_master,
        Some(endowed_accounts),
    )
}

///Realis chain-spec from realis.json
pub fn realis_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../../../../docker/realis.json")[..])
}

fn development_config_genesis() -> GenesisConfig {
    let sudo_1: AccountId =
        hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"].into();
    // let sudo_2: AccountId =
    // let sudo_3: AccountId =
    // let sudo_4: AccountId =
    // let sudo_5: AccountId =

    let nft_master = vec![sudo_1.clone()];
    let api_master = vec![sudo_1.clone()];
    let bridge_master = vec![sudo_1.clone()];

    testnet_genesis(
        vec![authority_keys_from_seed("Alice")],
        vec![],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        nft_master,
        api_master,
        bridge_master,
        None,
    )
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
    let mut properties = Map::new();
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("tokenSymbol".into(), "LIS".into());
    properties.insert("ss58Format".into(), 42.into());

    ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        development_config_genesis,
        vec![],
        None,
        None,
        Some(properties),
        Default::default(),
    )
}

fn local_testnet_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            authority_keys_from_seed("Alice"),
            authority_keys_from_seed("Bob"),
        ],
        vec![],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
        vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
        vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
        None,
    )
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        ChainType::Local,
        local_testnet_genesis,
        vec![],
        None,
        None,
        None,
        Default::default(),
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::service::{new_full_base, new_light_base, NewFullBase};
    use sc_service_test;
    use sp_runtime::BuildStorage;

    fn local_testnet_genesis_instant_single() -> GenesisConfig {
        testnet_genesis(
            vec![authority_keys_from_seed("Alice")],
            vec![],
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
            vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
            vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
            None,
        )
    }

    /// Local testnet config (single validator - Alice)
    pub fn integration_test_config_with_single_authority() -> ChainSpec {
        ChainSpec::from_genesis(
            "Integration Test",
            "test",
            ChainType::Development,
            local_testnet_genesis_instant_single,
            vec![],
            None,
            None,
            None,
            Default::default(),
        )
    }

    /// Local testnet config (multivalidator Alice + Bob)
    pub fn integration_test_config_with_two_authorities() -> ChainSpec {
        ChainSpec::from_genesis(
            "Integration Test",
            "test",
            ChainType::Development,
            local_testnet_genesis,
            vec![],
            None,
            None,
            None,
            Default::default(),
        )
    }

    #[test]
    #[ignore]
    fn test_connectivity() {
        sc_service_test::connectivity(
            integration_test_config_with_two_authorities(),
            |config| {
                let NewFullBase {
                    task_manager,
                    client,
                    network,
                    transaction_pool,
                    ..
                } = new_full_base(config, |_, _| ())?;
                Ok(sc_service_test::TestNetComponents::new(
                    task_manager,
                    client,
                    network,
                    transaction_pool,
                ))
            },
            |config| {
                let (keep_alive, _, client, network, transaction_pool) = new_light_base(config)?;
                Ok(sc_service_test::TestNetComponents::new(
                    keep_alive,
                    client,
                    network,
                    transaction_pool,
                ))
            },
        );
    }

    #[test]
    fn test_create_development_chain_spec() {
        development_config().build_storage().unwrap();
    }

    #[test]
    fn test_create_local_testnet_chain_spec() {
        local_testnet_config().build_storage().unwrap();
    }

    #[test]
    fn test_staging_test_net_chain_spec() {
        staging_testnet_config().build_storage().unwrap();
    }
}

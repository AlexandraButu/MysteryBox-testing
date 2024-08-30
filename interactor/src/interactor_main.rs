#![allow(non_snake_case)]
#[warn(dead_code)]
mod proxy;
mod testcontract_proxy;

use multiversx_sc_snippets::imports::*;
use multiversx_sc_snippets::sdk;
//use mystery_box::__wasm__endpoints__::set_roles;
use core::task;
use serde::{Deserialize, Serialize};
use std::collections;
use std::f64::consts::E;
use std::{
    io::{Read, Write},
    path::Path,
};

//use mystery_box::config::RewardType;
use crate::proxy::RewardType;

//use async_std::task;
use tokio::task::spawn;
use tokio::time::sleep;
use tokio::time::Duration;

const GATEWAY: &str = sdk::gateway::DEVNET_GATEWAY;
const STATE_FILE: &str = "state.toml";
const TOKEN_IDENTIFIER2: &str = "TST-c0986b";
const TOKEN_IDENTIFIER3: &str = "BSK-476470";

const TOKEN_IDENTIFIER: &str = "TTO-281def";
const INVALID_TOKEN_ID: &str = "xyz";
const TOKEN_NONCE: u64 = 1;
const TOKEN_SFT_AMOUNT: u64 = 1;

const MB_TOKEN_IDENTIFIER: &str = "TTO-959e6e";

const MB_TOKEN_IDENTIFIER_NONCE_1: &str = "TTO-959e6e-01";
const MB_TOKEN_IDENTIFIER_NONCE_2: &str = "TTO-959e6e-02";

const MB_TOKEN_IDENTIFIER_FAIL: &str = "TTO-5c8209-02";

const WRONG: &str = "TTO-5c8209";

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    // let cmd = args.next().expect("at least one argument required");
    // let mut interact = ContractInteract::new().await;
    // match cmd.as_str() {
    //     "deploy" => interact.deploy().await,
    //     "setupMysteryBox" => interact.setup_mystery_box().await,
    //     "updateMysteryBoxUris" => interact.update_mystery_box_uris().await,
    //     "createMysteryBox" => interact.create_mystery_box().await,
    //     "openMysteryBox" => interact.open_mystery_box().await,
    //     "getMysteryBoxTokenIdentifier" => interact.mystery_box_token_id().await,
    //     "getGlobalCooldownEpoch" => interact.global_cooldown_epoch().await,
    //   "getWinningRates" => interact.winning_rates().await,
    //     "getMysteryBoxUris" => interact.mystery_box_uris().await,
    //     "isAdmin" => interact.is_admin().await,
    //    // "addAdmin" => interact.add_admin().await,
    //     "removeAdmin" => interact.remove_admin().await,
    //    "getAdmins" => interact.admins().await,
    //     _ => panic!("unknown command: {}", &cmd),
    // }
    /*
    let mut interact = ContractInteract::new().await;
    let token_amount = RustBigUint::from(10000000000000000000u128);
    interact.deploy().await;
    interact.issue_token_set_all_roles().await;
    interact.setup_mystery_box1().await;
    interact.create_mystery_box(managed_biguint!(1000)).await; //1000
    interact.mystery_box_token_id().await; // get mystery box token id != payment token_id
    interact
        .open_mystery_box_wrong_token(
            TOKEN_IDENTIFIER2,
            TOKEN_NONCE,
            token_amount,
            ExpectError(4, "Bad payment token"),
        )
        .await;
    */
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct State {
    contract_address: Option<Bech32Address>,
    collection: Option<String>,
}

impl State {
    // Deserializes state from file
    pub fn load_state() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    /// Sets the contract address
    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = Some(address);
    }

    /// Returns the contract address
    pub fn current_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known contract, deploy first")
    }

    pub fn set_collection(&mut self, collection: String) {
        self.collection = Some(collection);
    }

    /// Returns the collection
    pub fn get_collection(&self) -> &String {
        self.collection
            .as_ref()
            .expect("no known collection, create first")
    }
}

impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}

struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_code: BytesValue,
    test_contract_code: BytesValue,
    state: State,
    user1_address: Address,
    user2_address: Address,
}

impl ContractInteract {
    async fn new() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(test_wallets::alice());
        let user1_address = interactor.register_wallet(test_wallets::dan());
        let user2_address = interactor.register_wallet(test_wallets::frank());

        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/mystery-box.mxsc.json",
            &InterpreterContext::default(),
        );

        let test_contract_code = BytesValue::interpret_from(
            "mxsc:../contracttest/output/contracttest.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            wallet_address,
            user1_address,
            user2_address,
            contract_code,
            test_contract_code,
            state: State::load_state(),
        }
    }

    async fn send_nft_to_user(&mut self) {
        let _x = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::MysteryBoxProxy)
            .send_nft(&self.user1_address)
            .single_esdt(
                &TokenIdentifier::from(MB_TOKEN_IDENTIFIER),
                1,
                &BigUint::from(10u64),
            )
            .prepare_async()
            .run()
            .await;
    }

    async fn send_nft_to_user1(&mut self) {
        let _x = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.user1_address)
            .typed(proxy::MysteryBoxProxy)
            .send_nft(&self.user1_address)
            .single_esdt(
                &TokenIdentifier::from(MB_TOKEN_IDENTIFIER),
                7,
                &BigUint::from(2u64),
            )
            .prepare_async()
            .run()
            .await;
    }

    async fn send_nft_to_user2(&mut self) {
        let _x = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(&self.user2_address)
            .typed(proxy::MysteryBoxProxy)
            .send_nft(&self.user2_address)
            .single_esdt(
                &TokenIdentifier::from(MB_TOKEN_IDENTIFIER),
                2,
                &BigUint::from(10u64),
            )
            .prepare_async()
            .run()
            .await;
    }

    async fn deploy(&mut self) {
        let mystery_box_token_id = TokenIdentifier::from_esdt_bytes(TOKEN_IDENTIFIER);

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(70_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .init(mystery_box_token_id)
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state.set_address(Bech32Address::from_bech32_string(
            new_address_bech32.clone(),
        ));

        println!("new address: {new_address_bech32}");
    }

    async fn deploy_contract_test(&mut self) -> Address {
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(70_000_000u64)
            .typed(testcontract_proxy::ContracttestProxy)
            .init()
            .code(&self.test_contract_code)
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);

        println!("new address test contract: {new_address_bech32}");
        new_address
    }

    //send nft to smart contract
    async fn send_nft_to_sc(&mut self, to: Address, expected_result: ExpectError<'_>) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(to)
            .typed(testcontract_proxy::ContracttestProxy)
            .send_nft(&self.state.current_address().to_address())
            .payment((
                TokenIdentifier::from(MB_TOKEN_IDENTIFIER),
                1,
                BigUint::from(1u64),
            ))
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;
    }

    async fn deploy_fail(&mut self, expected_result: ExpectError<'_>) {
        let mystery_box_token_id = TokenIdentifier::from_esdt_bytes(INVALID_TOKEN_ID);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .gas(50_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .init(mystery_box_token_id)
            .code(&self.contract_code)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;
        //    let new_address_bech32 = bech32::encode(&new_address);
        //    self.state
        //    .set_address(Bech32Address::from_bech32_string(new_address_bech32.clone()));
        //    println!("new address: {new_address_bech32}");
    }

    /////////////////////////////////////////////////////////          SETUP MYSTERYBOX            //////////////////////////////////////////////

    async fn setup_mystery_box_reward_None(&mut self) {
        let mut winning_rates_list = MultiValueVec::<
            MultiValue6<
                RewardType,
                EgldOrEsdtTokenIdentifier<StaticApi>,
                BigUint<StaticApi>,
                ManagedBuffer<StaticApi>,
                u64,
                u64,
            >,
        >::new();

        let mut reward = (
            RewardType::None,
            EgldOrEsdtTokenIdentifier::esdt(managed_token_id!(TOKEN_IDENTIFIER2)),
            BigUint::from(0u64),
            managed_buffer!(b"None"),
            10_000,
            0,
        )
            .into();
        winning_rates_list.push(reward);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address) // Verifică dacă această adresă are permisiuni de admin
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn setup_mystery_box_reward_SFT_None_50_50(&mut self) {
        let mut winning_rates_list = MultiValueVec::<
            MultiValue6<
                RewardType,
                EgldOrEsdtTokenIdentifier<StaticApi>,
                BigUint<StaticApi>,
                ManagedBuffer<StaticApi>,
                u64,
                u64,
            >,
        >::new();

        let mut reward1 = (
            RewardType::SFT,
            EgldOrEsdtTokenIdentifier::esdt(managed_token_id!(TOKEN_IDENTIFIER2)),
            BigUint::from(5u64),
            managed_buffer!(b"SFT"),
            5_000,
            0,
        )
            .into();
        winning_rates_list.push(reward1);

        let mut reward = (
            RewardType::None,
            EgldOrEsdtTokenIdentifier::esdt(managed_token_id!(TOKEN_IDENTIFIER2)),
            BigUint::from(0u64),
            managed_buffer!(b"None"),
            5_000,
            0,
        )
            .into();
        winning_rates_list.push(reward);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address) // Verifică dacă această adresă are permisiuni de admin
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn setup_mystery_box_reward_SFT3(&mut self) {
        let mut winning_rates_list = MultiValueVec::<
            MultiValue6<
                RewardType,
                EgldOrEsdtTokenIdentifier<StaticApi>,
                BigUint<StaticApi>,
                ManagedBuffer<StaticApi>,
                u64,
                u64,
            >,
        >::new();

        let mut reward = (
            RewardType::SFT,
            EgldOrEsdtTokenIdentifier::esdt(managed_token_id!(MB_TOKEN_IDENTIFIER)),
            BigUint::from(2u64),
            managed_buffer!(b"SFT"),
            10_000,
            0,
        )
            .into();
        winning_rates_list.push(reward);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address) // Verifică dacă această adresă are permisiuni de admin
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn setup_mystery_box_reward_SFT_FixedValue_50_50(&mut self) {
        let mut winning_rates_list = MultiValueVec::<
            MultiValue6<
                RewardType,
                EgldOrEsdtTokenIdentifier<StaticApi>,
                BigUint<StaticApi>,
                ManagedBuffer<StaticApi>,
                u64,
                u64,
            >,
        >::new();

        let mut reward1 = (
            RewardType::SFT,
            EgldOrEsdtTokenIdentifier::esdt(managed_token_id!(TOKEN_IDENTIFIER2)),
            BigUint::from(2u64),
            managed_buffer!(b"SFT"),
            4_000,
            0,
        )
            .into();
        winning_rates_list.push(reward1);

        let mut reward2 = (
            RewardType::FixedValue,
            EgldOrEsdtTokenIdentifier::esdt(managed_token_id!(TOKEN_IDENTIFIER2)),
            BigUint::from(5u128),
            managed_buffer!(b"FixedValue"),
            6_000,
            0,
        )
            .into();
        winning_rates_list.push(reward2);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address) // Verifică dacă această adresă are permisiuni de admin
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn setup_mystery_box1(&mut self) {
        let mut winning_rates_list = MultiValueVec::<
            MultiValue6<
                RewardType,
                EgldOrEsdtTokenIdentifier<StaticApi>,
                BigUint<StaticApi>,
                ManagedBuffer<StaticApi>,
                u64,
                u64,
            >,
        >::new();

        let mut reward1 = (
            RewardType::FixedValue,
            EgldOrEsdtTokenIdentifier::esdt(managed_token_id!(TOKEN_IDENTIFIER2)),
            BigUint::from(5u128),
            managed_buffer!(b"FixedValue"),
            3_000,
            1,
        )
            .into();
        winning_rates_list.push(reward1);

        let mut reward2 = (
            RewardType::CustomReward,
            EgldOrEsdtTokenIdentifier::esdt(managed_token_id!(TOKEN_IDENTIFIER2)),
            BigUint::from(5u128),
            managed_buffer!(b"CustomText"),
            7_000,
            2,
        )
            .into();
        winning_rates_list.push(reward2);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address) // Verifică dacă această adresă are permisiuni de admin
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn setup_mystery_box_fail_percentage(&mut self, expected_result: ExpectError<'_>) {
        let mut winning_rates_list = MultiValueVec::<
            MultiValue6<
                RewardType,
                EgldOrEsdtTokenIdentifier<StaticApi>,
                BigUint<StaticApi>,
                ManagedBuffer<StaticApi>,
                u64,
                u64,
            >,
        >::new();

        let mut reward1 = (
            RewardType::FixedValue,
            EgldOrEsdtTokenIdentifier::esdt(managed_token_id!(TOKEN_IDENTIFIER)),
            BigUint::from(5u128),
            managed_buffer!(b"FixedValue"),
            2_000,
            1,
        )
            .into();
        winning_rates_list.push(reward1);

        let mut reward2 = (
            RewardType::CustomReward,
            EgldOrEsdtTokenIdentifier::esdt(managed_token_id!(TOKEN_IDENTIFIER)),
            BigUint::from(5u128),
            managed_buffer!(b"CustomText"),
            7_000,
            2,
        )
            .into();
        winning_rates_list.push(reward2);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address) // Verifică dacă această adresă are permisiuni de admin
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn setup_mystery_box_one_reward(&mut self) {
        let mut winning_rates_list = MultiValueVec::<
            MultiValue6<
                RewardType,
                EgldOrEsdtTokenIdentifier<StaticApi>,
                BigUint<StaticApi>,
                ManagedBuffer<StaticApi>,
                u64,
                u64,
            >,
        >::new();

        let mut reward1 = (
            RewardType::ExperiencePoints,
            EgldOrEsdtTokenIdentifier::esdt(managed_token_id!(TOKEN_IDENTIFIER)),
            BigUint::from(1u128),
            managed_buffer!(b"ExperiencePoints"),
            10_000,
            2,
        )
            .into();
        winning_rates_list.push(reward1);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address) // Verifică dacă această adresă are permisiuni de admin
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn setup_mystery_box_reward_mystery_box(&mut self) {
        let mut winning_rates_list = MultiValueVec::<
            MultiValue6<
                RewardType,
                EgldOrEsdtTokenIdentifier<StaticApi>,
                BigUint<StaticApi>,
                ManagedBuffer<StaticApi>,
                u64,
                u64,
            >,
        >::new();

        let mut reward1 = (
            RewardType::MysteryBox,
            EgldOrEsdtTokenIdentifier::esdt(managed_token_id!(TOKEN_IDENTIFIER)),
            BigUint::from(100u128),
            managed_buffer!(b"MysteryBox"),
            10_000,
            0,
        )
            .into();
        winning_rates_list.push(reward1);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address) // Verifică dacă această adresă are permisiuni de admin
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    ////////////////////////////////////////////////////////////////////           FAIL URI REWARD URI    //////////////////////////////////////////////

    async fn setup_mystery_box_reward_mystery_box_fail(
        &mut self,
        expected_result: ExpectError<'_>,
    ) {
        let mut winning_rates_list = MultiValueVec::<
            MultiValue6<
                RewardType,
                EgldOrEsdtTokenIdentifier<StaticApi>,
                BigUint<StaticApi>,
                ManagedBuffer<StaticApi>,
                u64,
                u64,
            >,
        >::new();

        let mut reward1 = (
            RewardType::MysteryBox,
            EgldOrEsdtTokenIdentifier::esdt(managed_token_id!(MB_TOKEN_IDENTIFIER_FAIL)),
            BigUint::from(1u128),
            managed_buffer!(b"MysteryBox"),
            10_000,
            0,
        )
            .into();
        winning_rates_list.push(reward1);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address) // Verifică dacă această adresă are permisiuni de admin
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn setup_mystery_box_reward_ExperiencePoints_fail(
        &mut self,
        expected_result: ExpectError<'_>,
    ) {
        let mut winning_rates_list = MultiValueVec::<
            MultiValue6<
                RewardType,
                EgldOrEsdtTokenIdentifier<StaticApi>,
                BigUint<StaticApi>,
                ManagedBuffer<StaticApi>,
                u64,
                u64,
            >,
        >::new();

        let mut reward1 = (
            RewardType::ExperiencePoints,
            EgldOrEsdtTokenIdentifier::esdt(managed_token_id!(MB_TOKEN_IDENTIFIER)),
            BigUint::from(0u128),
            managed_buffer!(b"ExperiencePoints"),
            10_000,
            0,
        )
            .into();
        winning_rates_list.push(reward1);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address) // Verifică dacă această adresă are permisiuni de admin
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn setup_mystery_box_reward_SFT_fail(&mut self, expected_result: ExpectError<'_>) {
        let mut winning_rates_list = MultiValueVec::<
            MultiValue6<
                RewardType,
                EgldOrEsdtTokenIdentifier<StaticApi>,
                BigUint<StaticApi>,
                ManagedBuffer<StaticApi>,
                u64,
                u64,
            >,
        >::new();

        let mut reward1 = (
            RewardType::SFT,
            EgldOrEsdtTokenIdentifier::egld(),
            BigUint::from(1u128),
            managed_buffer!(b"SFT"),
            10_000,
            0,
        )
            .into();
        winning_rates_list.push(reward1);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address) // Verifică dacă această adresă are permisiuni de admin
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn setup_mystery_box_reward_PercentValue_fail(
        &mut self,
        expected_result: ExpectError<'_>,
    ) {
        let mut winning_rates_list = MultiValueVec::<
            MultiValue6<
                RewardType,
                EgldOrEsdtTokenIdentifier<StaticApi>,
                BigUint<StaticApi>,
                ManagedBuffer<StaticApi>,
                u64,
                u64,
            >,
        >::new();

        let mut reward1 = (
            RewardType::PercentValue,
            EgldOrEsdtTokenIdentifier::egld(),
            BigUint::from(0u128),
            managed_buffer!(b"Percent"),
            10_000,
            0,
        )
            .into();
        winning_rates_list.push(reward1);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address) // Verifică dacă această adresă are permisiuni de admin
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn setup_mystery_box_reward_FixedValue_fail(&mut self, expected_result: ExpectError<'_>) {
        let mut winning_rates_list = MultiValueVec::<
            MultiValue6<
                RewardType,
                EgldOrEsdtTokenIdentifier<StaticApi>,
                BigUint<StaticApi>,
                ManagedBuffer<StaticApi>,
                u64,
                u64,
            >,
        >::new();

        let mut reward1 = (
            RewardType::FixedValue,
            EgldOrEsdtTokenIdentifier::egld(),
            BigUint::from(0u128),
            managed_buffer!(b"FixedValue"),
            10_000,
            0,
        )
            .into();
        winning_rates_list.push(reward1);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address) // Verifică dacă această adresă are permisiuni de admin
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn setup_mystery_box_reward_CustomReward_fail(
        &mut self,
        expected_result: ExpectError<'_>,
    ) {
        let mut winning_rates_list = MultiValueVec::<
            MultiValue6<
                RewardType,
                EgldOrEsdtTokenIdentifier<StaticApi>,
                BigUint<StaticApi>,
                ManagedBuffer<StaticApi>,
                u64,
                u64,
            >,
        >::new();

        let mut reward1 = (
            RewardType::CustomReward,
            EgldOrEsdtTokenIdentifier::egld(),
            BigUint::from(0u128),
            managed_buffer!(b""),
            10_000,
            0,
        )
            .into();
        winning_rates_list.push(reward1);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address) // Verifică dacă această adresă are permisiuni de admin
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn setup_mystery_box(&mut self) {
        let winning_rates_list = MultiValueVec::<
            MultiValue6<
                RewardType,
                EgldOrEsdtTokenIdentifier<StaticApi>,
                BigUint<StaticApi>,
                ManagedBuffer<StaticApi>,
                u64,
                u64,
            >,
        >::new();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    async fn update_mystery_box_uris(&mut self) {
        let uris = MultiValueVec::from(vec![ManagedBuffer::new_from_bytes(&b""[..])]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .update_mystery_box_uris(uris)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn create_mystery_box(&mut self, amount: BigUint<StaticApi>) {
        //  let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .create_mystery_box(amount)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn create_mystery_box_fail(
        &mut self,
        amount: BigUint<StaticApi>,
        expected_result: ExpectError<'_>,
    ) {
        //  let amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .create_mystery_box(amount)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn open_mystery_box(
        &mut self,
        token_id: &str,
        token_nonce: u64,
        token_amount: BigUint<StaticApi>,
    ) {
        let response = self
            .interactor
            .tx()
            .from(&self.user1_address)
            .to(self.state.current_address())
            .gas(50_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .open_mystery_box()
            .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn open_mystery_box_user1(
        &mut self,
        token_id: &str,
        token_nonce: u64,
        token_amount: BigUint<StaticApi>,
    ) {
        let response = self
            .interactor
            .tx()
            .from(&self.user1_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .open_mystery_box()
            .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn open_mystery_box_user2(
        &mut self,
        token_id: &str,
        token_nonce: u64,
        token_amount: BigUint<StaticApi>,
    ) {
        //   let token_id = String::new();
        //   let token_nonce = 0u64;
        //   let token_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.user2_address)
            .to(self.state.current_address())
            .gas(50_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .open_mystery_box()
            .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn open_mystery_box_by_user(
        &mut self,
        adresa: Address,
        token_id: &str,
        token_nonce: u64,
        token_amount: BigUint<StaticApi>,
    ) {
        //   let token_id = String::new();
        //   let token_nonce = 0u64;
        //   let token_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(adresa)
            .to(self.state.current_address())
            .gas(200_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .open_mystery_box()
            .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn open_mystery_box_by_sc(
        &mut self,
        sc_account: Address,
        token_id: &str,
        token_nonce: u64,
        token_amount: BigUint<StaticApi>,
        expected_result: ExpectError<'_>,
    ) {
        let response = self
            .interactor
            .tx()
            .from(sc_account)
            .to(self.state.current_address())
            // .from(&self.wallet_address)
            // .from(sc_account.clone())
            // .to(sc_account)
            .gas(50_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .open_mystery_box()
            .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn open_mystery_box_wrong_token(
        &mut self,
        token_id: &str,
        token_nonce: u64,
        token_amount: BigUint<StaticApi>,
        expected_result: ExpectError<'_>,
    ) {
        let payment = EsdtTokenPayment::new(
            TokenIdentifier::from(token_id),
            token_nonce,
            token_amount.into(),
        );
        //  println!("wallet_addres {:?}", self.wallet_address);
        // println!("current address {:?}", self.state.current_address());
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(50_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .open_mystery_box()
            .esdt(payment)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn open_mystery_box_wrong_amount(
        &mut self,
        token_id: &str,
        token_nonce: u64,
        token_amount: BigUint<StaticApi>,
        expected_result: ExpectError<'_>,
    ) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(50_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .open_mystery_box()
            .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn feed_contract_egld(
        &mut self,
        token_id: &str,
        token_nonce: u64,
        token_amount: BigUint<StaticApi>,
    ) {
        let payment =
            EsdtTokenPayment::new(TokenIdentifier::from(token_id), token_nonce, token_amount);

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            // .egld(NumExpr("0,050000000000000000"))
            // .esdt(payment)
            .single_esdt(
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            )
            .prepare_async()
            .run()
            .await;
    }

    ////////////////////////////////////////////////    TRANSFER DE TOKENS DE LA ADMIN CATRE USERI ////////////////////////////////////////////////////////////////

    async fn send_from_admin_to_user1(
        &mut self,
        token_id: &str,
        token_nonce: u64,
        token_amount: BigUint<StaticApi>,
    ) {
        let payment =
            EsdtTokenPayment::new(TokenIdentifier::from(token_id), token_nonce, token_amount);
        self.interactor
            .tx()
            .from(&self.wallet_address) // Adresa de la care se trimite (admin)
            .to(self.state.current_address()) // Adresa de destinație (user)
            .gas(50_000_000u64) // Setare gas limit
            .typed(proxy::MysteryBoxProxy)
            .update_attributes(payment, &self.user1_address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;
    }

    async fn send_from_admin_to_user2(
        &mut self,
        token_id: &str,
        token_nonce: u64,
        token_amount: BigUint<StaticApi>,
    ) {
        let payment =
            EsdtTokenPayment::new(TokenIdentifier::from(token_id), token_nonce, token_amount);
        self.interactor
            .tx()
            .from(&self.wallet_address) // Adresa de la care se trimite (admin)
            .to(self.state.current_address()) // Adresa de destinație (user)
            .gas(50_000_000u64) // Setare gas limit
            .typed(proxy::MysteryBoxProxy)
            .update_attributes(payment, &self.user2_address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;
    }

    /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    async fn mystery_box_token_id(&mut self) -> TokenIdentifier<StaticApi> {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MysteryBoxProxy)
            .mystery_box_token_id()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
        result_value
    }

    async fn token_issued(&mut self) -> TokenIdentifier<StaticApi> {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MysteryBoxProxy)
            .get_token_issued()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
        result_value
    }

    async fn global_cooldown_epoch(&mut self, reward: RewardType) -> u64 {
        // async fn global_cooldown_epoch(&mut self, reward : RewardType )  {

        // let reward = RewardType::None;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MysteryBoxProxy)
            .global_cooldown_epoch(reward)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
        result_value
    }

    async fn winning_rates(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MysteryBoxProxy)
            .winning_rates()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {:#?}", result_value);
    }

    async fn mystery_box_uris(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MysteryBoxProxy)
            .mystery_box_uris()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn is_admin(&mut self, adresa: Bech32Address) {
        //   let address = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MysteryBoxProxy)
            .is_admin(adresa)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn add_admin(&mut self, new_admin: Bech32Address) {
        //  let address = bech32::decode("");

        //  let address = &self.user_address;
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .add_admin(new_admin)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Add admin: {response:?}");
    }

    async fn remove_admin(&mut self, admin: Bech32Address) {
        //   let address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .remove_admin(admin)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Remove admin: {response:?}");
    }

    async fn remove_admin_by_user_fail(
        &mut self,
        admin: Bech32Address,
        expected_result: ExpectError<'_>,
    ) {
        //   let address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.user2_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .remove_admin(admin)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Remove admin: {response:?}");
    }

    async fn admins(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::MysteryBoxProxy)
            .admins()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Admins: {result_value:?}");
    }

    async fn issue_token_set_all_roles(&mut self) {
        let token_name = managed_buffer!(b"Nume");
        let token_ticker = managed_buffer!(b"TTO");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.contract_address.clone().unwrap()) // adresa contractului meu
            .gas(70_000_000u64)
            .typed(proxy::MysteryBoxProxy)
            .issue(token_name, token_ticker)
            .egld(NumExpr("0,050000000000000000"))
            .prepare_async()
            .run()
            .await;
    }
}

/////////////////////////////////////////////////////////////////          TESTE              ////////////////////////////////////////////////////////////////////

#[tokio::test]
async fn test_deploy() {
    let mut interact = ContractInteract::new().await;

    interact.deploy().await;
}

#[tokio::test]
async fn test_add_admin() {
    let mut interact = ContractInteract::new().await;

    let admin_nou: Bech32Address = interact.user1_address.clone().into();

    interact.add_admin(admin_nou).await;
}

#[tokio::test]
async fn test_remove_admin() {
    let mut interact = ContractInteract::new().await;

    let admin_de_sters: Bech32Address = interact.user1_address.clone().into();
    interact.is_admin(admin_de_sters.clone()).await;
    interact.remove_admin(admin_de_sters).await;
}

#[tokio::test]
async fn test_get_admins() {
    let mut interact = ContractInteract::new().await;
    interact.admins().await;
}

#[tokio::test]
async fn test_verify_is_admin() {
    let mut interact = ContractInteract::new().await;

    let adresa: Bech32Address = interact.wallet_address.clone().into();
    let adresa2: Bech32Address = interact.user1_address.clone().into();
    interact.is_admin(adresa).await;
    interact.is_admin(adresa2).await;
}

#[tokio::test]
async fn test_setup_mysterybox_success() {
    let mut interact = ContractInteract::new().await;
    interact.setup_mystery_box1().await;
}

#[tokio::test]
async fn test_setup_mysterybox_one_reward() {
    let mut interact = ContractInteract::new().await;
    interact.setup_mystery_box_one_reward().await;
}

#[tokio::test]
async fn test_get_winning_rates() {
    let mut interact = ContractInteract::new().await;
    interact.winning_rates().await;
}

#[tokio::test]
async fn test_get_global_cooldown_epoch() {
    let mut interact = ContractInteract::new().await;

    let reward1 = RewardType::FixedValue;
    let reward2 = RewardType::CustomReward;
    let reward3 = RewardType::ExperiencePoints;
    interact.global_cooldown_epoch(reward1).await;
}

#[tokio::test]
async fn test_mystery_box_get_uris() {
    let mut interact = ContractInteract::new().await;
    interact.mystery_box_uris().await;
}

//Returneaza tokenul mb ului
#[tokio::test]
async fn test_mystery_box_get_token_id() {
    let mut interact = ContractInteract::new().await;
    interact.mystery_box_token_id().await;
}

#[tokio::test]
async fn test_generate_sft_collection() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    //  interact.issue_sft_collection().await;
}

#[tokio::test]
async fn test_create_mystery_box() {
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    let amount = BigUint::<StaticApi>::from(0u128);

    //  interact.issue_sft_collection().await;
    //    interact.setup_mystery_box1().await;

    //      interact.set_roles().await;
    //  interact.create_mystery_box(amount).await;
}

//SETUP

#[tokio::test]
async fn test_setup() {
    let mut interact = ContractInteract::new().await;

    interact.deploy().await;
    interact.issue_token_set_all_roles().await;
    interact.token_issued().await;
    interact.mystery_box_token_id().await;
}

// CREARE MYSTERY BOX
#[tokio::test]
async fn test_setup_create_mystery_box() {
    let mut interact = ContractInteract::new().await;

    let amount = BigUint::<StaticApi>::from(1000u128);
    interact.setup_mystery_box1().await;
    interact.create_mystery_box(amount).await;
}

//OBTINERE TOKEN CREAT
#[tokio::test]
async fn test_getting_token_issued() {
    let mut interact = ContractInteract::new().await;

    interact.token_issued().await;
}

//OBTINERE TOKEN CREAT
#[tokio::test]
async fn test_getting_token_mystery_box() {
    let mut interact = ContractInteract::new().await;

    interact.mystery_box_token_id().await;
}

//Dan deschide mb ul
#[tokio::test]
async fn test_open_mystery_box_by_user() {
    let mut interact = ContractInteract::new().await;

    // let amount = BigUint::<StaticApi>::from(1000u128);
    // interact.setup_mystery_box1().await;
    //interact.create_mystery_box(amount).await;

    let tokensft = interact.mystery_box_token_id().await;
    let tokensft_str = tokensft.to_string();
    let token_amount = BigUint::<StaticApi>::from(1u128);

    // let token_amount: BigUint<StaticApi> = TOKEN_SFT_AMOUNT.into();
    interact
        .open_mystery_box(&tokensft_str, TOKEN_NONCE, token_amount)
        .await;
}

// CREARE MYSTERY BOX care are ca reward un mystery box                  OK
#[tokio::test]
async fn test_setup_create_mystery_box_with_mystery_box_as_reward() {
    let mut interact = ContractInteract::new().await;

    let amount = BigUint::<StaticApi>::from(1000u128);
    interact.setup_mystery_box_reward_mystery_box().await;
    interact.create_mystery_box(amount).await;
}

//De la ALICE trimit la DAN
#[tokio::test]
async fn test_transfer_tokens_from_admin_to_user1() {
    let mut interact = ContractInteract::new().await;

    /*
    // interact.deploy().await;
    interact.issue_token_set_all_roles().await;
    //  interact.setup_mystery_box_reward_mystery_box().await;
    interact.create_mystery_box(managed_biguint!(1000)).await; //1000

    let tokensft = interact.mystery_box_token_id().await; //Intoarce token ul creat care e setat ca token id ul mystery box ului
    let tokensft_str = tokensft.to_string();
    let token_amount = BigUint::<StaticApi>::from(1u64);

    interact
        .send_from_admin_to_user1(MB_TOKEN_IDENTIFIER, TOKEN_NONCE, token_amount)
        .await;*/

    interact.send_nft_to_user1().await;
}

//De la ALICE trimit la FRANK
#[tokio::test]
async fn test_transfer_tokens_from_admin_to_user2() {
    let mut interact = ContractInteract::new().await;

    interact.send_nft_to_user2().await;
}

//OPEN MYSTERY BOX BY 2 USERS
#[tokio::test]
async fn test_open_mystery_box_by_multiple_users() {
    let mut interact = ContractInteract::new().await;

    let tokensft = interact.token_issued().await;
    let tokensft_str = tokensft.to_string();

    let token_amount1 = BigUint::<StaticApi>::from(1u128);
    let token_amount2 = BigUint::<StaticApi>::from(1u128);
    /*
    let user1_address = interact.user1_address.clone();
    interact.open_mystery_box_by_user(user1_address, &tokensft_str, TOKEN_NONCE, token_amount1).await;
    let user2_address = interact.user2_address.clone();
    interact.open_mystery_box_by_user(user2_address, &tokensft_str, TOKEN_NONCE,token_amount2).await;*/

    interact
        .open_mystery_box_user1(&tokensft_str, TOKEN_NONCE, token_amount1)
        .await;

    // interact
    // .open_mystery_box_user2(&tokensft_str, TOKEN_NONCE, token_amount2)
    // .await;
}

#[tokio::test]
async fn test_open_mb_with_reward_None() {
    let mut interact = ContractInteract::new().await;

    //   let amount = BigUint::<StaticApi>::from(10u128);
    let amount2 = BigUint::<StaticApi>::from(1u128);
    //interact.setup_mystery_box_reward_None().await;
    // interact.create_mystery_box(amount).await;
    interact
        .open_mystery_box_user2(MB_TOKEN_IDENTIFIER, 2, amount2)
        .await;
}

#[tokio::test]
async fn test_open_mb_with_reward_SFT() {
    let mut interact = ContractInteract::new().await;

    let amount = BigUint::<StaticApi>::from(6u128);
    let amount2 = BigUint::<StaticApi>::from(1u128);

    //interact.setup_mystery_box_reward_SFT3().await;
    //interact.create_mystery_box(amount).await;
    //interact.send_nft_to_user1().await;
    interact
        .open_mystery_box_user1(MB_TOKEN_IDENTIFIER, 7, amount2)
        .await;
}

#[tokio::test]
async fn test_open_mb_with_reward_SFT_None_50_50() {
    let mut interact = ContractInteract::new().await;

    let amount = BigUint::<StaticApi>::from(3u128);
    let amount2 = BigUint::<StaticApi>::from(1u128);
    interact.setup_mystery_box_reward_SFT_None_50_50().await;

    interact.create_mystery_box(amount).await;
    interact
        .open_mystery_box_user1(MB_TOKEN_IDENTIFIER, 3, amount2)
        .await;
}

#[tokio::test]
async fn test_open_mb_with_reward_SFT_FixedValue_50_50() {
    let mut interact = ContractInteract::new().await;

    let amount = BigUint::<StaticApi>::from(3u128);
    let amount2 = BigUint::<StaticApi>::from(1u128);
    //  interact.setup_mystery_box_reward_SFT_FixedValue_50_50().await;

    //interact.create_mystery_box(amount).await;
    interact
        .open_mystery_box_user1(MB_TOKEN_IDENTIFIER, 1, amount2)
        .await;
}

///////////////////////////////////////////////////////////       FAIL TESTS         //////////////////////////////////////////////////////////////////////////////////

//DEPLOY WITH WRONG ID
#[tokio::test]
async fn test_deploy_fail() {
    let mut interact = ContractInteract::new().await;
    interact
        .deploy_fail(ExpectError(4, "Invalid token ID"))
        .await;
}

#[tokio::test]
async fn test_remove_admin_by_user_fail() {
    let mut interact = ContractInteract::new().await;

    let admin_de_sters: Bech32Address = interact.user1_address.clone().into();
    interact.is_admin(admin_de_sters.clone()).await;
    interact
        .remove_admin_by_user_fail(
            admin_de_sters,
            ExpectError(4, "You can not remove an admin if you are an user"),
        )
        .await;
}

#[tokio::test]
async fn test_create_mystery_box_without_setup_fail() {
    let mut interact = ContractInteract::new().await;
    let amount = BigUint::<StaticApi>::from(1u128);
    interact
        .create_mystery_box_fail(
            amount,
            ExpectError(4, "The Mystery Box must be set up first"),
        )
        .await;
}

#[tokio::test]
async fn test_setup_mysterybox_fail_percentage_not_100() {
    let mut interact = ContractInteract::new().await;
    interact
        .setup_mystery_box_fail_percentage(ExpectError(4, "The total percentage must be 100%"))
        .await;
}

//Fac deploy pt al doilea sc, ii trimit un token mysterybox pentru a putea face open mysterybox, dar nu poate pentru ca e sc -> EROARE
#[tokio::test]
async fn test_open_mb_by_sc() {
    let mut interact = ContractInteract::new().await;

    let new_address = interact.deploy_contract_test().await;
    interact
        .send_nft_to_sc(
            new_address,
            ExpectError(4, "Only user accounts can open mystery boxes"),
        )
        .await;
}

//OPEN MYSTERY BOX WRONG AMOUNT -> ERROR : "Bad payment amount"                 OK
#[tokio::test]
async fn test_open_mystery_box_with_wrong_amount() {
    let mut interact = ContractInteract::new().await;

    let tokensft = interact.token_issued().await;
    let tokensft_str = tokensft.to_string();

    // let tokensft_str = "TTO-5c8209";
    let wrong_token_amount = BigUint::<StaticApi>::from(2u128);

    interact
        .open_mystery_box_wrong_amount(
            &tokensft_str,
            TOKEN_NONCE,
            wrong_token_amount,
            ExpectError(4, "Bad payment amount"),
        )
        .await;
}

//OPEN MYSTERY BOX WRONG TOKEN ID -> ERROR :  "Bad payment token"                                 OK
#[tokio::test]

async fn test_open_mystery_box_with_wrong_token_id() {
    let mut interact = ContractInteract::new().await;

    let token_amount = BigUint::<StaticApi>::from(1u64);

    let x = interact.mystery_box_token_id().await; // get mystery box token id != payment token_id
    println!("MB token: {}", x);
    println!("Payment token: {}", WRONG);
    interact
        .open_mystery_box_wrong_token(
            WRONG,
            TOKEN_NONCE,
            token_amount,
            ExpectError(4, "Bad payment token"),
        )
        .await;
}

//CREATE MYSTERYBOX
//TRIMITEM UN MYSTERYBOX LA CONTRACT
// TRIMIT LA BOB PRIN CONTRACT
//BOB VINE SI DESCHIDE MYSTERYBOX

// CREARE MYSTERY BOX care are ca reward un mystery box dar cu un ID gresit         OK
#[tokio::test]
async fn test_setup_mb_with_mystery_box_as_reward_fail() {
    let mut interact = ContractInteract::new().await;

    let amount = BigUint::<StaticApi>::from(1000u128);
    interact
        .setup_mystery_box_reward_mystery_box_fail(ExpectError(
            4,
            "The reward token id must be the same as the mystery box",
        ))
        .await;
}

// CREARE MYSTERY BOX care are ca reward ExperiencePoints cu valoare gresita          OK
#[tokio::test]
async fn test_setup_mb_ExperiencePoints_fail() {
    let mut interact = ContractInteract::new().await;

    interact
        .setup_mystery_box_reward_ExperiencePoints_fail(ExpectError(
            4,
            "The experience points amount must be greater than 0",
        ))
        .await;
}

// CREARE MYSTERY BOX care are ca reward SFT cu id gresit                  OK
#[tokio::test]
async fn test_setup_mb_SFT_fail() {
    let mut interact = ContractInteract::new().await;

    interact
        .setup_mystery_box_reward_SFT_fail(ExpectError(4, "The reward token id must be an ESDT"))
        .await;
}

// CREARE MYSTERY BOX care are ca reward PercentValue cu procentaj gresit                OK
#[tokio::test]
async fn test_setup_mb_PercentValue_fail() {
    let mut interact = ContractInteract::new().await;

    interact
        .setup_mystery_box_reward_PercentValue_fail(ExpectError(
            4,
            "The reward percentage must be positive and <= 100%",
        ))
        .await;
}

// CREARE MYSTERY BOX care are ca reward FixedValue cu valoare gresita              OK
#[tokio::test]
async fn test_setup_mb_FixedValue_fail() {
    let mut interact = ContractInteract::new().await;

    interact
        .setup_mystery_box_reward_FixedValue_fail(ExpectError(
            4,
            "The reward amount must be greater than 0",
        ))
        .await;
}

// CREARE MYSTERY BOX care are ca reward CustomReward fara descriere                 OK
#[tokio::test]
async fn test_setup_mb_CustomReward_fail() {
    let mut interact = ContractInteract::new().await;

    interact
        .setup_mystery_box_reward_CustomReward_fail(ExpectError(
            4,
            "The custom reward needs to have a description",
        ))
        .await;
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

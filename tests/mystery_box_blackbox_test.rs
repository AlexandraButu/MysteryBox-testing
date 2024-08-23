use std::time::UNIX_EPOCH;

use multiversx_sc_scenario::{imports::*, ScenarioWorld};
use mystery_box::*;
mod mysterybox_proxy;
use crate::mysterybox_proxy::RewardType;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const USER1_ADDRESS: TestAddress = TestAddress::new("user1");
const USER2_ADDRESS: TestAddress = TestAddress::new("user2");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("mystery-box");

const CODE_PATH: MxscPath = MxscPath::new("output/mystery-box.mxsc.json");
const BALANCE: u64 = 2_000;

const TOKEN_ID_TTO: TestTokenIdentifier = TestTokenIdentifier::new("TTO-281def");
const TOKEN_ID_COLECTIE: TestTokenIdentifier = TestTokenIdentifier::new("CLC-203e07");
const WRONG_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("WRONG_TOKEN");
const MYSTERYBOX_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("CLC-203e07");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(CODE_PATH, mystery_box::ContractBuilder);
    blockchain
}

struct MysteryBoxTestState {
    world: ScenarioWorld,
}

impl MysteryBoxTestState {
    fn new() -> Self {
        let mut world = world();

        world.start_trace();

        world
            .account(OWNER_ADDRESS)
            .nonce(1)
            .esdt_balance(TOKEN_ID_TTO, BALANCE)
            .esdt_balance(MYSTERYBOX_TOKEN_ID, BALANCE) // ????????? ESDT
            .balance(BALANCE)
            .account(USER1_ADDRESS)
            .nonce(1)
            .esdt_balance(TOKEN_ID_TTO, BALANCE)
            .esdt_balance(MYSTERYBOX_TOKEN_ID, BALANCE)
            .balance(BALANCE)
            .account(USER2_ADDRESS)
            .nonce(1)
            .esdt_balance(TOKEN_ID_TTO, BALANCE)
            .esdt_balance(MYSTERYBOX_TOKEN_ID, BALANCE)
            .balance(BALANCE);

        Self { world }
    }

    ///////
    fn write_scenario_trace(&mut self, filename: &str) {
        self.world.write_scenario_trace(filename);
    }
    ///////

    /*fn set_esdt_local_roles(&mut self, sc_address: TestSCAddress, token_id: TestTokenIdentifier, roles: &[EsdtLocalRole])  {


      let roles = vec![
          EsdtLocalRole::Mint,
          EsdtLocalRole::Burn,
         EsdtLocalRole::NftCreate,
          EsdtLocalRole::NftAddQuantity,
          EsdtLocalRole::NftBurn,
          EsdtLocalRole::NftAddUri,
         EsdtLocalRole::NftUpdateAttributes,
          EsdtLocalRole::Transfer

      ];

          self.world
          .tx()
          .from(OWNER_ADDRESS)
          .to(SC_ADDRESS)
          .typed(mysterybox_proxy::MysteryBoxProxy)
     //     .set_special_roles(SC_ADDRESS, TOKEN_ID_COLECTIE ,roles.into_iter())
           .set_roles()
            .run();


    }   */

    fn deploy_mysterybox_contract(&mut self) -> &mut Self {
        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .typed(mysterybox_proxy::MysteryBoxProxy)
            .init(MYSTERYBOX_TOKEN_ID)
            .code(CODE_PATH)
            .new_address(SC_ADDRESS)
            .run();
        self
    }

    fn set_time_block(&mut self, timestamp: u64) {
        self.world.current_block().block_timestamp(timestamp);
    }

    fn setup_mystery_box_contract_one_reward(&mut self) {
        let token_id: TokenIdentifier<StaticApi> = TOKEN_ID_TTO.into();

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

        let reward1 = (
            RewardType::FixedValue,
            EgldOrEsdtTokenIdentifier::esdt(token_id),
            BigUint::from(5u128),
            managed_buffer!(b"FixedValue"),
            10_000,
            1,
        )
            .into();
        winning_rates_list.push(reward1);

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .gas(30_000_000u64)
            .typed(mysterybox_proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(ReturnsResultUnmanaged)
            .run();
    }

    fn setup_mystery_box_contract_all_rewards(&mut self) {
        let token_id: TokenIdentifier<StaticApi> = TOKEN_ID_TTO.into();
        let token_id: TokenIdentifier<StaticApi> = MYSTERYBOX_TOKEN_ID.into();

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

        let reward1 = (
            RewardType::FixedValue,
            EgldOrEsdtTokenIdentifier::esdt(token_id),
            BigUint::from(5u128),
            managed_buffer!(b"FixedValue"),
            1_000,
            1,
        )
            .into();
        winning_rates_list.push(reward1);

        self.world
            .tx()
            .from(OWNER_ADDRESS)
            .to(SC_ADDRESS)
            .gas(30_000_000u64)
            .typed(mysterybox_proxy::MysteryBoxProxy)
            .setup_mystery_box(winning_rates_list)
            .returns(ReturnsResultUnmanaged)
            .run();
    }

    ////////// CHECKS //////////

    fn get_mystery_box_token_id(&mut self) -> TokenIdentifier<StaticApi> {
        self.world
            .query()
            .to(SC_ADDRESS)
            .typed(mysterybox_proxy::MysteryBoxProxy)
            .mystery_box_token_id()
            .returns(ReturnsResult)
            .run()
    }

    fn get_global_cooldown_epoch(&mut self, reward: RewardType) -> u64 {
        self.world
            .query()
            .to(SC_ADDRESS)
            .typed(mysterybox_proxy::MysteryBoxProxy)
            .global_cooldown_epoch(reward)
            .returns(ReturnsResult)
            .run()
    }
}

#[test]
fn test_deploy() {
    let mut world = MysteryBoxTestState::new();

    world.deploy_mysterybox_contract();

    world.write_scenario_trace("scenarios/init-mysterybox.scen.json");
}

#[test]
fn test_set_roles() {
    let mut world = MysteryBoxTestState::new();
    world.deploy_mysterybox_contract();

    // world.set_esdt_local_roles(SC_ADDRESS, TOKEN_ID_COLECTIE, &[EsdtLocalRole::Mint]);
}

#[test]
fn test_setup_mysterybox_one_reward() {
    let mut world = MysteryBoxTestState::new();
    world.deploy_mysterybox_contract();

    world.setup_mystery_box_contract_one_reward();

    world.write_scenario_trace("scenarios/setup_one_reward-mysterybox.scen.json");
}

#[test]
fn test_checks_mystery_box() {
    let mut world = MysteryBoxTestState::new();

    let reward1 = RewardType::FixedValue;

    world.deploy_mysterybox_contract();

    world.setup_mystery_box_contract_one_reward();

    world.get_mystery_box_token_id();

    //  world.set_time_block(30);

    world.get_global_cooldown_epoch(reward1);

    world.write_scenario_trace("scenarios/check_mystery_box5.scen.json");
}

//get ..
//assert_eq!(len, 0);

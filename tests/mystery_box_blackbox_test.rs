
use multiversx_sc_scenario::{imports::*, ScenarioWorld};
use mystery_box::*;
mod mysterybox_proxy;

const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const USER1_ADDRESS: TestAddress = TestAddress::new("user1");
const USER2_ADDRESS: TestAddress = TestAddress::new("user2");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("mystery-box");

const CODE_PATH: MxscPath = MxscPath::new("output/mystery-box.mxsc.json");
const BALANCE: u64 = 2_000;

const TOKEN_ID_TTO: TestTokenIdentifier = TestTokenIdentifier::new("TTO-281def");
const TOKEN_ID_COLECTIE : TestTokenIdentifier = TestTokenIdentifier::new("CLC-203e07");
const WRONG_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("WRONG_TOKEN");
const MYSTERYBOX_TOKEN_ID: TestTokenIdentifier = TestTokenIdentifier::new("TTO-281def");


fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

 
    blockchain.register_contract(CODE_PATH, mystery_box::ContractBuilder);
    blockchain
}


struct MysteryBoxTestState {
    world: ScenarioWorld,
}


impl MysteryBoxTestState{
    fn new() -> Self {

        let mut world = world();

        world.start_trace();

        world
        .account(OWNER_ADDRESS)
        .nonce(1)
        .esdt_balance(TOKEN_ID_TTO, BALANCE)    
        .esdt_balance(WRONG_TOKEN_ID, BALANCE)
        .balance(BALANCE)


        .account(USER1_ADDRESS)
        .nonce(1)
        .esdt_balance(TOKEN_ID_TTO, BALANCE)    
        .esdt_balance(WRONG_TOKEN_ID, BALANCE)
        .balance(BALANCE)


        .account(USER2_ADDRESS)
        .nonce(1)
        .esdt_balance(TOKEN_ID_TTO, BALANCE)    
        .esdt_balance(WRONG_TOKEN_ID, BALANCE)
        .balance(BALANCE);

       
    
        Self { world }
    }


    ///////
    fn write_scenario(&mut self, filename: &str) 
    {
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


    fn set_time_block(&mut self, timestamp: u64){
        self.world.current_block().block_timestamp(timestamp);
}

}



#[test]
fn test_deploy() {
    let mut world = MysteryBoxTestState::new();
    
    world.deploy_mysterybox_contract();


    world.write_scenario("scenarios/init-mysterybox.scen.json");
   
}

#[test]
fn test_set_roles() {
    let mut world = MysteryBoxTestState::new();
    world.deploy_mysterybox_contract();

   // world.set_esdt_local_roles(SC_ADDRESS, TOKEN_ID_COLECTIE, &[EsdtLocalRole::Mint]); 

   
}
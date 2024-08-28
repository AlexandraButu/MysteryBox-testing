use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/contracttest.mxsc.json", contracttest::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/contracttest.scen.json");
}

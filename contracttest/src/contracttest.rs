#![no_std]

#[allow(unused_imports)]


use multiversx_sc::imports::*;
mod mysterybox_proxy;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait Contracttest {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}


    #[payable("*")]
    #[endpoint]
    fn send_nft(&self, to: ManagedAddress) {
        let nft = self.call_value().single_esdt();

        self.tx()
            .to(to)
            .raw_call("openMysteryBox") // face un call pentru open mystery box 
            .payment(nft)
            .sync_call();
    }



}

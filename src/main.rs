extern crate rustc_serialize;

use keyring::AccountKeyring;
use primitives::hashing::blake2_256;
use rustc_serialize::hex::{FromHex, ToHex};
use sp_core::{blake2_128, sr25519};
use sp_core::crypto::Pair;
use substrate_api_client::{Api, compose_extrinsic, extrinsic::xt_primitives::UncheckedExtrinsicV4, node_metadata::Metadata, utils::hexstr_to_hash, XtStatus};

fn metadata_caller() {
    let demo = "Hello, Worldd".as_bytes();
    let a = blake2_128(&demo).to_vec();
    let c: Vec<u8> = vec![188, 252, 217, 14, 178, 234, 158, 162, 224, 107, 104, 34, 134, 34, 5, 104, 45];
    let meta = api.get_metadata();
    println!("Metadata:\n {}", Metadata::pretty_format(&meta).unwrap());
}

fn extensic_caller() {
    let xt: UncheckedExtrinsicV4<_> = compose_extrinsic!( api.clone(),"System", "create_claim","xxxxxxxxxx");
    println!("[+] Composed Extrinsic:\n {:?}\n", xt);
    let signer = AccountKeyring::Alice.pair();
    api.signer = Some(signer);
    let tx_hash = api.send_extrinsic(xt.hex_encode(), XtStatus::Finalized).unwrap();
    println!("[+] Transaction got finalized. Hash: {:?}", tx_hash);
}

fn main() {
    let url = "127.0.0.1:9944";
    let from = AccountKeyring::Alice.pair();
    let mut api = Api::new(format!("ws://{}", url)).set_signer(from);
    let x = api.clone();
    metadata_caller();
    extensic_caller();
}



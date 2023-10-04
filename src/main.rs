extern crate bitcoin;
extern crate secp256k1;

use std::{io, io::BufRead};
use std::str::FromStr;

use secp256k1::{Secp256k1, SecretKey};
use bitcoin::{network::constants::Network, PrivateKey, Address, key};

fn main() {
    println!("===========");
    println!("HEX to WIF");
    println!("============");
    println!("Input HEX, 64 digit (0123456789ABCDEF)");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match SecretKey::from_str(&line.unwrap().to_string()) {
            Ok(secret_key) => {
                let private_key_uncompressed = PrivateKey::new_uncompressed(secret_key, Network::Bitcoin);
                let private_key_compressed = PrivateKey::new(secret_key, Network::Bitcoin);
                let public_key_compressed = key::PublicKey::from_private_key(&Secp256k1::new(), &private_key_compressed);
                let public_key_uncompressed = key::PublicKey::from_private_key(&Secp256k1::new(), &private_key_uncompressed);
                let address_uncompressed = Address::p2pkh(&public_key_uncompressed, Network::Bitcoin);
                let address_compressed = Address::p2pkh(&public_key_compressed, Network::Bitcoin);
                let addres_49 = Address::p2shwpkh(&public_key_compressed, Network::Bitcoin).expect("p2shwpkh");
                let addres_84 = Address::p2wpkh(&public_key_compressed, Network::Bitcoin).expect("p2wpkh");

                println!("Adress_bip44(u):{} private_key(WIF): {}", address_uncompressed, private_key_uncompressed);
                println!("Adress_bip44(c):{} private_key(WIF): {}", address_compressed, private_key_compressed);
                println!("Adress_bip49:{} private_key(WIF): {}", addres_49, private_key_compressed);
                println!("Adress_bip84:{} private_key(WIF): {}", addres_84, private_key_compressed);

            }
            Err(_) =>
                {
                    println!("Invalid HEX, try again");
                }
        }
    }
}

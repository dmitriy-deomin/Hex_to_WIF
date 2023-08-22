extern crate bitcoin;
extern crate secp256k1;

use std::{io, io::BufRead};
use std::str::FromStr;

use secp256k1::{Secp256k1, SecretKey};
use bitcoin::{network::constants::Network, PrivateKey, Address, key};

fn main() {
    println!("Введите HEX");
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
                println!("Adress:{} Compressed WIF: {}", address_compressed, private_key_compressed);
                println!("Adress:{} Uncompressed WIF: {}", address_uncompressed, private_key_uncompressed);
            }
            Err(_) =>
                {
                    println!("Неверный HEX,попробуйте ещё раз");
                }
        }
    }
}

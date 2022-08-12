use secp256k1::rand::thread_rng;
use tapyrus::network::constants::Network;
use tapyrus::secp256k1::Secp256k1;
use tapyrus::{PrivateKey, PublicKey};

pub fn generate_key_pair() -> (PrivateKey, PublicKey) {
    let s = Secp256k1::new();
    let mut rng = thread_rng();
    let private_key = tapyrus::PrivateKey {
        compressed: true,
        network: Network::Prod,
        key: s.generate_keypair(&mut rng).0,
    };
    let public_key = PublicKey::from_private_key(&s, &private_key);
    (private_key, public_key)
}

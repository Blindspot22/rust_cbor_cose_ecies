use serde::{Serialize, Deserialize};

use cose::crypto::{Signer, Verifier};

use cose::sign::{COSESign1Builder, COSESign1};
use cose::CborSerializable;
use ecies::{encrypt, decrypt};
use rand::rngs::OsRng;
use rsa::{RSAPrivateKey, RSAPublicKey, PaddingScheme};
use rsa::pkcs1::ToRsaPrivateKey;
use secp256k1::{SecretKey, PublicKey};
use sha2::Sha256;

#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    name: String,
    value: u32,
}

fn sign_data(cbor_data: &[u8]) -> Vec<u8> {
    // Generate RSA key pair
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RSAPublicKey::from(&private_key);

    // Create a COSESign1Builder
    let mut builder = COSESign1Builder::new();

    // Sign the CBOR data
    let signer = Signer::new_rsa(&private_key.to_pkcs1_der().unwrap());
    builder.message(cbor_data).sign(&signer).unwrap();

    // Serialize the COSESign1 message
    builder.to_vec().unwrap()
}

fn verify_data(cose_data: &[u8], public_key: &RSAPublicKey) -> bool {
    // Parse the COSESign1 message
    let sign1 = COSESign1::from_slice(cose_data).unwrap();

    // Verify the signature
    let verifier = Verifier::new_rsa(public_key);
    sign1.verify(&verifier).is_ok()
}

fn ecies_encrypt_decrypt(cbor_data: &[u8]) -> Vec<u8> {
    // Generate a key pair
    let secret_key = SecretKey::new(&mut rand::thread_rng());
    let public_key = PublicKey::from_secret_key(&secret_key);

    // Encrypt the CBOR data
    let encrypted_data = encrypt(&public_key, cbor_data).unwrap();

    // Decrypt the data
    let decrypted_data = decrypt(&secret_key, &encrypted_data).unwrap();

    decrypted_data
}

fn main() {
    let data = MyData {
        name: String::from("example"),
        value: 42,
    };

    // Serialize to CBOR
    let cbor_data = serde_cbor::to_vec(&data).unwrap();
    println!("CBOR Data: {:?}", cbor_data);

    // Sign the CBOR data
    let cose_data = sign_data(&cbor_data);
    println!("COSE Data: {:?}", cose_data);

    // Verify the signed COSE data
    let public_key = RSAPublicKey::from_pkcs1_der(&cose_data).unwrap();
    let is_valid = verify_data(&cose_data, &public_key);
    println!("Signature valid: {:?}", is_valid);

    // Encrypt and then decrypt the CBOR data using ECIES
    let decrypted_data = ecies_encrypt_decrypt(&cbor_data);
    println!("Decrypted Data: {:?}", decrypted_data);

    // Deserialize the decrypted CBOR data
    let decoded: MyData = serde_cbor::from_slice(&decrypted_data).unwrap();
    println!("Decoded Data: {:?}", decoded);
}

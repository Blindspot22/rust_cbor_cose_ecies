# rust_cbor_cose_ecies

This Rust project demonstrates how to use CBOR, COSE, and ECIES in a single application.

## Dependencies

- `ciborium`: For CBOR serialization and deserialization.
- `cose`: For COSE signing and encryption.
- `ecies`: For ECIES encryption and decryption.
- `rand`: For random number generation.
- `serde` and `serde_cbor`: For serializing and deserializing Rust structs to and from CBOR.
- `rsa`: For RSA key generation and signing.
- `sha2`: For SHA-256 hashing.

## Usage

1. Clone the repository.
2. Run `cargo build` to build the project.
3. Run `cargo run` to execute the application.

## Example Output

CBOR Data: [<...>]

COSE Data: [<...>]

Signature valid: true

Decrypted Data: [<...>]

Decoded Data: MyData { name: "example", value: 42 }

## Explanation

- The application first serializes a Rust struct to CBOR.
- It then signs the CBOR data using COSE with RSA keys.
- It verifies the signed COSE data.
- It encrypts the CBOR data using ECIES and then decrypts it.
- Finally, it deserializes the decrypted CBOR data back into the original Rust struct.. 



use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    name: String,
    value: u32,
}

fn main() {
    let data = MyData {
        name: String::from("example"),
        value: 42,
    };

    // Serialize to CBOR
    let cbor_data = serde_cbor::to_vec(&data).unwrap();
    println!("CBOR Data: {:?}", cbor_data);

    // Deserialize from CBOR
    let decoded: MyData = serde_cbor::from_slice(&cbor_data).unwrap();
    println!("Decoded Data: {:?}", decoded);
}

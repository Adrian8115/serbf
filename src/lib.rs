use std::collections::HashMap;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};
use crate::ser::SerbfSerializer;

mod ser;
mod error;
mod de;

#[test]
fn test() {
    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        num_u8: u8,
        num_u16: u16,
        num_u32: u32,
        num_u64: u64,
        num_u128: u128,

        num_i8: i8,
        num_i16: i16,
        num_i32: i32,
        num_i64: i64,
        num_i128: i128,

        boolean: bool,
        char: char,
        str: &'static str,
        string: String,

        some: Option<u8>,

        vec: Vec<u8>,
        
        tuple: (u32, i32),

        map: HashMap<String, u32>,
    }

    let testy = TestStruct {
        num_u8: 42,
        num_u16: 42,
        num_u32: 42,
        num_u64: 42,
        num_u128: 42,
        num_i8: -42,
        num_i16: -42,
        num_i32: -42,
        num_i64: -42,
        num_i128: -42,
        boolean: true,
        char: 'C',
        str: "Halloea Friend",
        string: "Nuauan".to_string(),
        some: Some(42),
        vec: vec![42, 41, 67, 69, 1, 3, 5, 6, 99, 0, 255],
        tuple: (69, 69),
        map: HashMap::from([
            ("69".to_string(), 69),
            ("42".to_string(), 42),
            ("-1".to_string(), 1),
        ])
    };

    let mut serializer = SerbfSerializer::new();

    testy.serialize(&mut serializer).unwrap();

    let buf = serializer.into_inner();

    println!("{:?}", buf);
}

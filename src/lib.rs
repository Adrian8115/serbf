use std::collections::HashMap;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize};
use crate::de::SerbfDeserializer;
use crate::ser::SerbfSerializer;

mod ser;
mod error;
mod de;

#[test]
fn ser() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
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

    println!("{:?}", buf)
}

#[test]
fn de() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
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
        //string: String,
//
        //some: Option<u8>,
//
        //vec: Vec<u8>,
//
        //tuple: (u32, i32),
        //
        // map: HashMap<String, u32>,
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
        //string: "Nuauan".to_string(),
        //some: Some(42),
        //vec: vec![42, 41, 67, 69, 1, 3, 5, 6, 99, 0, 255],
        //tuple: (69, 69),
        // map: HashMap::from([
        //     ("69".to_string(), 69),
        //     ("42".to_string(), 42),
        //     ("-1".to_string(), 1),
        // ])
    };

    let mut deserializer = SerbfDeserializer::new(&[42, 42, 0, 42, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 214, 214, 255, 214, 255, 255, 255, 214, 255, 255, 255, 255, 255, 255, 255, 214, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 1, 1, 67, 14, 72, 97, 108, 108, 111, 101, 97, 32, 70, 114, 105, 101, 110, 100, 6, 78, 117, 97, 117, 97, 110, 1, 42, 11, 42, 41, 67, 69, 1, 3, 5, 6, 99, 0, 255, 2, 69, 0, 0, 0, 69, 0, 0, 0, 3, 2, 45, 49, 1, 0, 0, 0, 2, 52, 50, 42, 0, 0, 0, 2, 54, 57, 69, 0, 0, 0]);

    let res = TestStruct::deserialize(&mut deserializer).unwrap();

    assert_eq!(testy, res);
}

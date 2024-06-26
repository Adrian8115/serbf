use std::io::Write;
use std::sync::Arc;
use byteorder::{LittleEndian, WriteBytesExt};
use serde::ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};
use serde::{Serialize, Serializer};
use varint_rs::VarintWriter;
use crate::error::SerbfError;

#[derive(Clone)]
pub struct SerbfSerializer {
    buf: Vec<u8>,
}

impl SerbfSerializer {
    pub fn new() -> Self {
        Self {
            buf: Vec::new(),
        }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.buf
    }
}

impl<'a> Serializer for &'a mut SerbfSerializer {
    type Ok = ();
    type Error = SerbfError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        let res = match v {
            true => { self.buf.write_u8(1) }
            false => { self.buf.write_u8(0) }
        };

        match res {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(SerbfError::IOError(Arc::new(e))) }
        }
    }

    fn serialize_i8(mut self, v: i8) -> Result<Self::Ok, Self::Error> {
        match self.buf.write_i8(v) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(SerbfError::IOError(Arc::new(e))) }
        }
    }

    fn serialize_i16(mut self, v: i16) -> Result<Self::Ok, Self::Error> {
        match self.buf.write_i16::<LittleEndian>(v) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(SerbfError::IOError(Arc::new(e))) }
        }
    }

    fn serialize_i32(mut self, v: i32) -> Result<Self::Ok, Self::Error> {
        match self.buf.write_i32::<LittleEndian>(v) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(SerbfError::IOError(Arc::new(e))) }
        }
    }

    fn serialize_i64(mut self, v: i64) -> Result<Self::Ok, Self::Error> {
        match self.buf.write_i64::<LittleEndian>(v) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(SerbfError::IOError(Arc::new(e))) }
        }
    }

    fn serialize_i128(mut self, v: i128) -> Result<Self::Ok, Self::Error> {
        match self.buf.write_i128::<LittleEndian>(v) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(SerbfError::IOError(Arc::new(e))) }
        }
    }

    fn serialize_u8(mut self, v: u8) -> Result<Self::Ok, Self::Error> {
        match self.buf.write_u8(v) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(SerbfError::IOError(Arc::new(e))) }
        }
    }

    fn serialize_u16(mut self, v: u16) -> Result<Self::Ok, Self::Error> {
        match self.buf.write_u16::<LittleEndian>(v) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(SerbfError::IOError(Arc::new(e))) }
        }
    }

    fn serialize_u32(mut self, v: u32) -> Result<Self::Ok, Self::Error> {
        match self.buf.write_u32::<LittleEndian>(v) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(SerbfError::IOError(Arc::new(e))) }
        }
    }

    fn serialize_u64(mut self, v: u64) -> Result<Self::Ok, Self::Error> {
        match self.buf.write_u64::<LittleEndian>(v) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(SerbfError::IOError(Arc::new(e))) }
        }
    }

    fn serialize_u128(mut self, v: u128) -> Result<Self::Ok, Self::Error> {
        match self.buf.write_u128::<LittleEndian>(v) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(SerbfError::IOError(Arc::new(e))) }
        }
    }

    fn serialize_f32(mut self, v: f32) -> Result<Self::Ok, Self::Error> {
        match self.buf.write_f32::<LittleEndian>(v) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(SerbfError::IOError(Arc::new(e))) }
        }
    }

    fn serialize_f64(mut self, v: f64) -> Result<Self::Ok, Self::Error> {
        match self.buf.write_f64::<LittleEndian>(v) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(SerbfError::IOError(Arc::new(e))) }
        }
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0; 4];
        let str = v.encode_utf8(&mut buf);

        self.serialize_str(&str)
    }

    fn serialize_str(mut self, v: &str) -> Result<Self::Ok, Self::Error> {
        match self.buf.write_usize_varint(v.len()) {
            Ok(_) => {}
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        match self.buf.write_all(v.as_bytes()) {
            Ok(_) => {}
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        Ok(())
    }

    fn serialize_bytes(mut self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        match self.buf.write_usize_varint(v.len()) {
            Ok(_) => {}
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        match self.buf.write_all(v) {
            Ok(_) => {}
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        Ok(())
    }

    fn serialize_none(mut self) -> Result<Self::Ok, Self::Error> {
        match self.buf.write_u8(0) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(SerbfError::IOError(Arc::new(e))) }
        }
    }

    fn serialize_some<T>(mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize
    {
        match self.buf.write_u8(1) {
            Ok(_) => { }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        // ¯\_(:/)_/¯ nothing to do

        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(mut self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        match self.buf.write_u32_varint(variant_index) {
            Ok(_) => { Ok(()) }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        }
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(mut self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize
    {
        match self.buf.write_u32_varint(variant_index) {
            Ok(_) => {}
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        }

        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(self)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<'a> SerializeSeq for &'a mut SerbfSerializer {
    type Ok = ();
    type Error = SerbfError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeTuple for &'a mut SerbfSerializer {
    type Ok = ();
    type Error = SerbfError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeTupleStruct for &'a mut SerbfSerializer {
    type Ok = ();
    type Error = SerbfError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeTupleVariant for &'a mut SerbfSerializer {
    type Ok = ();
    type Error = SerbfError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeMap for &'a mut SerbfSerializer {
    type Ok = ();
    type Error = SerbfError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeStruct for &'a mut SerbfSerializer {
    type Ok = ();
    type Error = SerbfError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeStructVariant for &'a mut SerbfSerializer {
    type Ok = ();
    type Error = SerbfError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
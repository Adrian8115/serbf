use std::char::DecodeUtf16;
use std::io::{Cursor, Error, Read};
use std::ptr::read;
use std::slice::Iter;
use std::string::FromUtf8Error;
use std::sync::Arc;
use byteorder::{LittleEndian, ReadBytesExt};
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use varint_rs::VarintReader;
use crate::error::SerbfError;
use crate::ser::SerbfSerializer;

#[derive(Clone)]
pub struct SerbfDeserializer<'a> {
    buf: Cursor<&'a [u8]>,
}

impl<'a> SerbfDeserializer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            buf: Cursor::new(data),
        }
    }
}

impl<'de> Deserializer<'de> for &'de mut SerbfDeserializer<'de> {
    type Error = SerbfError;

    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let v = match self.buf.read_u8() {
            Ok(0) => { false }
            Ok(1) => { true }
            Ok(_) => { return Err(SerbfError::InvalidEnumID) }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_bool(v)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let v = match self.buf.read_i8() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_i8(v)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let v = match self.buf.read_i16::<LittleEndian>() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_i16(v)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let v = match self.buf.read_i32::<LittleEndian>() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_i32(v)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let v = match self.buf.read_i64::<LittleEndian>() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_i64(v)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let v = match self.buf.read_i128::<LittleEndian>() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_i128(v)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let v = match self.buf.read_u8() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_u8(v)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let v = match self.buf.read_u16::<LittleEndian>() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_u16(v)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let v = match self.buf.read_u32::<LittleEndian>() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_u32(v)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let v = match self.buf.read_u64::<LittleEndian>() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_u64(v)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let v = match self.buf.read_u128::<LittleEndian>() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_u128(v)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let v = match self.buf.read_f32::<LittleEndian>() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_f32(v)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let v = match self.buf.read_f64::<LittleEndian>() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_f64(v)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let len = match self.buf.read_usize_varint() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        let mut vec = Vec::with_capacity(len);

        match self.buf.read_exact(vec.as_mut_slice()) {
            Ok(_) => {}
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        let str = match String::from_utf8(vec) {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::UTF8Error(e)) }
        };

        match str.chars().nth(0) {
            Some(v) => { visitor.visit_char(v) }
            None => { Err(SerbfError::InvalidChar) }
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let len = match self.buf.read_usize_varint() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        let mut vec = Vec::with_capacity(len);

        match self.buf.read_exact(vec.as_mut_slice()) {
            Ok(_) => {}
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        match String::from_utf8(vec) {
            Ok(v) => { visitor.visit_str(v.as_str()) }
            Err(e) => { Err(SerbfError::UTF8Error(e)) }
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let len = match self.buf.read_usize_varint() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        let mut vec = Vec::with_capacity(len);

        match self.buf.read_exact(vec.as_mut_slice()) {
            Ok(_) => {}
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        match String::from_utf8(vec) {
            Ok(v) => { visitor.visit_string(v) }
            Err(e) => { Err(SerbfError::UTF8Error(e)) }
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let len = match self.buf.read_usize_varint() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        let mut vec = Vec::with_capacity(len);

        match self.buf.read_exact(vec.as_mut_slice()) {
            Ok(_) => {}
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_bytes(vec.as_slice())
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let len = match self.buf.read_usize_varint() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        let mut vec = Vec::with_capacity(len);

        match self.buf.read_exact(vec.as_mut_slice()) {
            Ok(_) => {}
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_byte_buf(vec)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        match self.buf.read_u8() {
            Ok(0) => { visitor.visit_some(self) }
            Ok(1) => { visitor.visit_none() }
            Ok(_) => { Err(SerbfError::InvalidEnumID) }
            Err(e) => { Err(SerbfError::IOError(Arc::new(e))) }
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        // ¯\_(:/)_/¯ nothing to do

        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let len = match self.buf.read_usize_varint() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_seq(SerbfSeqDeserializer::new(self, len))
    }

    fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let len = match self.buf.read_usize_varint() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_seq(&mut SerbfSeqDeserializer::new(self, len))
    }

    fn deserialize_tuple_struct<V>(mut self, name: &'static str, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        let len = match self.buf.read_usize_varint() {
            Ok(v) => { v }
            Err(e) => { return Err(SerbfError::IOError(Arc::new(e))) }
        };

        visitor.visit_seq(&mut SerbfSeqDeserializer::new(self, len))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        todo!()
    }

    fn deserialize_struct<V>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        todo!()
    }

    fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        todo!()
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

struct SerbfSeqDeserializer<'a> {
    serbf_deserializer: &'a mut SerbfDeserializer<'a>,

    len: usize,
    current_index: usize,
}

impl<'de, 'a> SerbfSeqDeserializer<'de> {
    pub fn new(serbf_deserializer: &'a mut SerbfDeserializer<'de>, len: usize) -> Self {
        Self {
            serbf_deserializer,
            len,
            current_index: 0,
        }
    }
}

impl<'de> SeqAccess<'de> for SerbfSeqDeserializer<'de> {
    type Error = SerbfError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>
    {
        self.current_index += 1;

        match self.current_index > self.len {
            false => { Ok(Some(seed.deserialize(self.serbf_deserializer)?)) }
            true => { Ok(None) }
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

struct SerbfMapDeserializer<'a> {
    serbf_deserializer: &'a mut SerbfDeserializer<'a>,

    len: usize,
    current_index: usize,
}

impl<'de> MapAccess<'de> for &'de mut SerbfMapDeserializer<'de> {
    type Error = SerbfError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>
    {
        self.current_index += 1;

        match self.current_index > self.len {
            false => { Ok(Some(seed.deserialize(self.serbf_deserializer)?)) }
            true => { Ok(None) }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>
    {
        seed.deserialize(self.serbf_deserializer)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}
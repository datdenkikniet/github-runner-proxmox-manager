use std::ops::{AddAssign, MulAssign, Neg};
use std::str::FromStr;

use percent_encoding::percent_decode_str;
use serde::de::{self, DeserializeSeed, IntoDeserializer, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;

use error::{Error, Result};

use crate::error;

pub struct Deserializer<'de> {
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::Message("WrongCharacter".into()))
    }
}
impl<'de> Deserializer<'de> {
    fn peek_char(&mut self) -> Result<char> {
        self.input.chars().next().ok_or(Error::Eof)
    }
    fn next_three_str(&mut self) -> Result<&'de str> {
        let s = &self.input[0..3];
        self.input = &self.input[s.len()..];
        Ok(s)
    }
    fn parse_bool(&mut self) -> Result<bool> {
        if self.input.starts_with("1") {
            self.input = &self.input["1".len()..];
            Ok(true)
        } else if self.input.starts_with("0") {
            self.input = &self.input["0".len()..];
            Ok(false)
        } else {
            Err(Error::Message("ExpectedBoolean".into()))
        }
    }
    fn parse_unsigned<T>(&mut self) -> Result<T>
    where
        T: AddAssign<T> + MulAssign<T> + From<u8> + FromStr,
    {
        match self.parse_string().unwrap().parse::<T>() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::Message("invalid number".into())),
        }
    }

    fn parse_signed<T>(&mut self) -> Result<T>
    where
        T: Neg<Output = T> + AddAssign<T> + MulAssign<T> + From<i8> + FromStr,
    {
        match self.parse_string().unwrap().parse::<T>() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::Message("invalid number".into())),
        }
    }

    fn next_is_value(&mut self) -> Result<bool> {
        let eq = self.input.find("%3D");
        let comma = self.input.find("%2C");
        println!("eq: {:?}, comma: {:?}", eq, comma);
        Ok(if eq.is_none() && comma.is_none() {
            true
        } else if eq.is_some() && comma.is_none() {
            if eq == Some(0){
                true
            } else {
                false
            }
        } else {
            if comma == Some(0) {
                false
            } else if eq == Some(0) {
                true
            } else if comma.unwrap() < eq.unwrap() {
                true
            } else {
                false
            }
        })
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        println!("parsing");
        let eq = self.input.find("%3D");
        let comma = self.input.find("%2C");
        let target = if eq.is_none() && comma.is_none() {
            self.input.len()
        } else if comma.is_none() || (eq.is_some() && (eq.unwrap() < comma.unwrap())) {
            eq.unwrap()
        } else {
            comma.unwrap()
        };
        let s = &self.input[..target];
        self.input = &self.input[target..];
        Ok(s)
    }
    fn parse_seq_element(&mut self) -> Result<&'de str> {
        let target = self.input.find("%3B").unwrap_or(self.input.len());
        let s = &self.input[..target];
        self.input = &self.input[target..];
        Ok(s)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let s = self.parse_string()?;
        visitor.visit_str(s)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.parse_signed()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.parse_signed()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.parse_signed()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.parse_signed()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_unsigned()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.parse_unsigned()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_unsigned()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.parse_unsigned()?)
    }

    // Float parsing is stupidly hard.
    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // Float parsing is stupidly hard.
    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(
            percent_decode_str(self.parse_string()?)
                .decode_utf8()
                .unwrap()
                .to_string(),
        )
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.input.starts_with("null") {
            self.input = &self.input["null".len()..];
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.input.starts_with("null") {
            self.input = &self.input["null".len()..];
            visitor.visit_unit()
        } else {
            Err(Error::Message("ExpectedNull".into()))
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(CommaSeparated::new(self))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    // Tuple structs look just like sequences in JSON.
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Ok(visitor.visit_map(CommaSeparated::new(self))?)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self.parse_string()?.into_deserializer())
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct CommaSeparated<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    first: bool,
}

impl<'a, 'de> CommaSeparated<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        CommaSeparated { de, first: true }
    }
}

impl<'de, 'a> SeqAccess<'de> for CommaSeparated<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.de.peek_char().is_err() {
            return Ok(None);
        }
        if !self.first && self.de.next_three_str()? != "%3B" {
            return Err(Error::Message("ExpectedArrayComma".into()));
        }
        self.first = false;
        let s = self.de.parse_seq_element()?;
        let mut de = Deserializer::from_str(s);
        seed.deserialize(&mut de).map(Some)
    }
}

impl<'de, 'a> MapAccess<'de> for CommaSeparated<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.de.peek_char().is_err() {
            return Ok(None);
        }
        println!("hit_key_seed: {}", self.de.input);
        if self.de.next_is_value().unwrap() {
            println!("next is value: {}", self.de.input);
            let mut de = Deserializer::from_str("");
            return seed.deserialize(&mut de).map(Some);
        }
        println!("next is not value: {}", self.de.input);

        if !self.first && self.de.next_three_str()? != "%2C" {
            return Err(Error::Message("ExpectedAnd".into()));
        }
        self.first = false;
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        println!("hit value: {}", self.de.input);
        if !self.first && self.de.next_three_str()? != "%3D" {
            return Err(Error::Message("ExpectedEqual".into()));
        }
        self.first = false;
        seed.deserialize(&mut *self.de)
    }

    fn next_key<K>(&mut self) -> std::prelude::v1::Result<Option<K>, Self::Error>
    where
        K: Deserialize<'de>,
    {
        self.next_key_seed(std::marker::PhantomData)
    }

    fn next_value<V>(&mut self) -> std::prelude::v1::Result<V, Self::Error>
    where
        V: Deserialize<'de>,
    {
        self.next_value_seed(std::marker::PhantomData)
    }

    fn next_entry<K, V>(&mut self) -> std::prelude::v1::Result<Option<(K, V)>, Self::Error>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        self.next_entry_seed(std::marker::PhantomData, std::marker::PhantomData)
    }

    fn size_hint(&self) -> Option<usize> {
        None
    }

    fn next_entry_seed<K, V>(
        &mut self,
        kseed: K,
        vseed: V,
    ) -> std::prelude::v1::Result<Option<(K::Value, V::Value)>, Self::Error>
    where
        K: DeserializeSeed<'de>,
        V: DeserializeSeed<'de>,
    {
        match self.next_key_seed(kseed)? {
            Some(key) => {
                let value = self.next_value_seed(vseed)?;
                Ok(Some((key, value)))
            }
            None => Ok(None),
        }
    }
}

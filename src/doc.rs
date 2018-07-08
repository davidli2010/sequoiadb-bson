// Copyright 2018 David Li
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use element::ElementType;
use std::io::Write;

#[derive(Debug)]
pub struct Doc {
    data: Vec<u8>,
}

impl Doc {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }
}

impl From<Vec<u8>> for Doc {
    fn from(v: Vec<u8>) -> Self {
        let len = v.as_slice().read_u32::<LittleEndian>().unwrap() as usize;
        assert_eq!(len, v.len(), "Invalid length");
        Doc { data: v }
    }
}

#[inline]
fn reserve_length(v: &mut Vec<u8>) {
    let buf = [0u8; 4];
    v.write_all(&buf).unwrap();
}

#[inline]
fn set_length(v: &mut Vec<u8>, length: usize) {
    v.as_mut_slice().write_u32::<LittleEndian>(length as u32).unwrap();
}

#[inline]
fn write_byte<W: Write + ?Sized>(w: &mut W, b: u8) {
    w.write_u8(b).unwrap()
}

#[inline]
fn write_type<W: Write + ?Sized>(w: &mut W, t: ElementType) {
    write_byte(w, t as u8);
}

#[inline]
fn write_cstring<W: Write + ?Sized>(w: &mut W, v: &str) {
    const EOS: u8 = 0x00;
    w.write_all(v.as_bytes()).unwrap();
    write_byte(w, EOS);
}

#[inline]
fn write_bytes<W: Write + ?Sized>(w: &mut W, v: &[u8]) {
    w.write_all(v).unwrap();
}

#[inline]
fn write_int32<W: Write + ?Sized>(w: &mut W, v: i32) {
    w.write_i32::<LittleEndian>(v).unwrap();
}

#[inline]
fn write_int64<W: Write + ?Sized>(w: &mut W, v: i64) {
    w.write_i64::<LittleEndian>(v).unwrap();
}

#[inline]
fn write_float64<W: Write + ?Sized>(w: &mut W, v: f64) {
    w.write_f64::<LittleEndian>(v).unwrap();
}

const INITIAL_BUFFER_SIZE: usize = 64;


pub struct DocBuilder {
    data: Option<Vec<u8>>,
}

impl DocBuilder {
    pub fn new() -> DocBuilder {
        let mut builder = DocBuilder {
            data: Some(Vec::new()),
        };
        builder.reserve_length();
        builder
    }

    pub fn with_capacity(capacity: usize) -> DocBuilder {
        let mut builder = DocBuilder {
            data: Some(Vec::with_capacity(capacity)),
        };
        builder.reserve_length();
        builder
    }

    fn reserve_length(&mut self) {
        if let Some(ref mut data) = self.data {
            reserve_length(data);
        }
    }

    pub fn append_float64(&mut self, key: &str, value: f64) -> &mut Self {
        if let Some(ref mut data) = self.data {
            write_type(data, ElementType::Float64);
            write_cstring(data, key);
            write_float64(data, value);
        } else {
            panic!("the DocBuilder is finished")
        }
        self
    }

    pub fn append_string(&mut self, key: &str, value: &str) -> &mut Self {
        if let Some(ref mut data) = self.data {
            write_type(data, ElementType::String);
            write_cstring(data, key);
            write_int32(data, value.len() as i32 + 1);
            write_cstring(data, value);
        } else {
            panic!("the DocBuilder is finished")
        }
        self
    }

    pub fn append_bool(&mut self, key: &str, value: bool) -> &mut Self {
        if let Some(ref mut data) = self.data {
            write_type(data, ElementType::Bool);
            write_cstring(data, key);
            if value {
                write_bytes(data, &[1u8]);
            } else {
                write_bytes(data, &[0u8]);
            }
        } else {
            panic!("the DocBuilder is finished")
        }
        self
    }

    pub fn append_null(&mut self, key: &str) -> &mut Self {
        if let Some(ref mut data) = self.data {
            write_type(data, ElementType::Null);
            write_cstring(data, key);
        } else {
            panic!("the DocBuilder is finished")
        }
        self
    }

    pub fn append_int32(&mut self, key: &str, value: i32) -> &mut Self {
        if let Some(ref mut data) = self.data {
            write_type(data, ElementType::Int32);
            write_cstring(data, key);
            write_int32(data, value);
        } else {
            panic!("the DocBuilder is finished")
        }
        self
    }

    pub fn append_int64(&mut self, key: &str, value: i64) -> &mut Self {
        if let Some(ref mut data) = self.data {
            write_type(data, ElementType::Int64);
            write_cstring(data, key);
            write_int64(data, value);
        } else {
            panic!("the DocBuilder is finished")
        }
        self
    }

    pub fn finish(&mut self) -> Doc {
        if self.data.is_some() {
            if let Some(ref mut data) = self.data {
                const EOD: u8 = 0x00;
                write_byte(data, EOD);
                let len = data.len();
                set_length(data, len);
            }
            Doc { data: self.data.take().unwrap() }
        } else {
            panic!("the DocBuilder is finished")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::*;
    use super::*;

    #[test]
    fn doc_builder() {
        let doc = DocBuilder::new()
            .append_string("str", "str")
            .append_bool("bool", true)
            .append_null("null")
            .append_int32("int32", 65536i32)
            .append_int64("int64", 64i64)
            .append_float64("float64", 64f64)
            .finish();
        println!("doc={:?}", doc);
        println!("doc.length={}", doc.len());
        println!("sizeof(Doc)={}", size_of::<Doc>());
    }
}

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

const ELEMENT_TYPE_EOD: u8 = 0x00;
const ELEMENT_TYPE_FLOAT64: u8 = 0x01;
const ELEMENT_TYPE_STRING: u8 = 0x02;
const ELEMENT_TYPE_BSON: u8 = 0x03;
const ELEMENT_TYPE_ARRAY: u8 = 0x04;
const ELEMENT_TYPE_BINARY: u8 = 0x05;
const ELEMENT_TYPE_UNDEFINED: u8 = 0x06;
const ELEMENT_TYPE_OBJECT_ID: u8 = 0x07;
const ELEMENT_TYPE_BOOL: u8 = 0x08;
const ELEMENT_TYPE_DATE: u8 = 0x09;
const ELEMENT_TYPE_NULL: u8 = 0x0A;
const ELEMENT_TYPE_REGEX: u8 = 0x0B;
const ELEMENT_TYPE_DB_POINTER: u8 = 0x0C;
const ELEMENT_TYPE_CODE: u8 = 0x0D;
const ELEMENT_TYPE_SYMBOL: u8 = 0x0E;
const ELEMENT_TYPE_CODE_WSCOPE: u8 = 0x0F;
const ELEMENT_TYPE_INT32: u8 = 0x10;
const ELEMENT_TYPE_TIMESTAMP: u8 = 0x11;
const ELEMENT_TYPE_INT64: u8 = 0x12;
const ELEMENT_TYPE_DECIMAL: u8 = 0x64;
const ELEMENT_TYPE_MAX_KEY: u8 = 0x7F;
const ELEMENT_TYPE_MIN_KEY: u8 = 0xFF;

#[repr(u8)]
#[derive(Debug)]
pub enum ElementType {
    Float64 = ELEMENT_TYPE_FLOAT64,
    String = ELEMENT_TYPE_STRING,
    Bson = ELEMENT_TYPE_BSON,
    Array = ELEMENT_TYPE_ARRAY,
    Binary = ELEMENT_TYPE_BINARY,
    Undefined = ELEMENT_TYPE_UNDEFINED,
    ObjectId = ELEMENT_TYPE_OBJECT_ID,
    Bool = ELEMENT_TYPE_BOOL,
    Date = ELEMENT_TYPE_DATE,
    Null = ELEMENT_TYPE_NULL,
    RegEx = ELEMENT_TYPE_REGEX,
    DBPointer = ELEMENT_TYPE_DB_POINTER,
    Code = ELEMENT_TYPE_CODE,
    Symbol = ELEMENT_TYPE_SYMBOL,
    CodeWScope = ELEMENT_TYPE_CODE_WSCOPE,
    Int32 = ELEMENT_TYPE_INT32,
    Timestamp = ELEMENT_TYPE_TIMESTAMP,
    Int64 = ELEMENT_TYPE_INT64,
    Decimal = ELEMENT_TYPE_DECIMAL,
    MaxKey = ELEMENT_TYPE_MAX_KEY,
    MinKey = ELEMENT_TYPE_MIN_KEY,
}

impl ElementType {
    /// Try to convert from a `u8`.
    #[inline]
    pub fn from(i: u8) -> Option<ElementType> {
        Some(match i {
            ELEMENT_TYPE_FLOAT64 => ElementType::Float64,
            ELEMENT_TYPE_STRING => ElementType::String,
            ELEMENT_TYPE_BSON => ElementType::Bson,
            ELEMENT_TYPE_ARRAY => ElementType::Array,
            ELEMENT_TYPE_BINARY => ElementType::Binary,
            ELEMENT_TYPE_UNDEFINED => ElementType::Undefined,
            ELEMENT_TYPE_OBJECT_ID => ElementType::ObjectId,
            ELEMENT_TYPE_BOOL => ElementType::Bool,
            ELEMENT_TYPE_DATE => ElementType::Date,
            ELEMENT_TYPE_NULL => ElementType::Null,
            ELEMENT_TYPE_REGEX => ElementType::RegEx,
            ELEMENT_TYPE_DB_POINTER => ElementType::DBPointer,
            ELEMENT_TYPE_CODE => ElementType::Code,
            ELEMENT_TYPE_SYMBOL => ElementType::Symbol,
            ELEMENT_TYPE_CODE_WSCOPE => ElementType::CodeWScope,
            ELEMENT_TYPE_INT32 => ElementType::Int32,
            ELEMENT_TYPE_TIMESTAMP => ElementType::Timestamp,
            ELEMENT_TYPE_INT64 => ElementType::Int64,
            ELEMENT_TYPE_DECIMAL => ElementType::Decimal,
            ELEMENT_TYPE_MAX_KEY => ElementType::MaxKey,
            ELEMENT_TYPE_MIN_KEY => ElementType::MinKey,
            _ => return None,
        })
    }
}

#[macro_use]
extern crate serde_derive;
extern crate bincode;

use bincode::{Infinite};

pub mod string_messages;

pub mod enums;
use enums::MessageType;

pub fn hello() {
    println!("Hello chunk-protocol");
}

pub fn serialize_float(x: &f32) -> Vec<u8> { bincode::serialize(x, Infinite).unwrap() }
pub fn deserialize_float(buf: &Vec<u8>) -> f32 { bincode::deserialize(&buf[..]).unwrap() }

pub fn pack(msg: &MessageType) -> Vec<u8> {
    bincode::serialize(msg, Infinite).unwrap()
}

pub fn unpack(buf: &Vec<u8>) -> MessageType {
    bincode::deserialize(&buf[..]).unwrap()
}

extern crate bincode;
#[macro_use]
extern crate serde_derive;

use bincode::Infinite;

pub mod string_messages;

pub mod enums;
use enums::MessageType;

const SEQUENCE_HEADER: u8 = 1;

pub fn hello() {
    println!("Hello chunk-protocol");
}

pub fn serialize_float(x: &f32) -> Vec<u8> {
    bincode::serialize(x, Infinite).unwrap()
}
pub fn deserialize_float(buf: &Vec<u8>) -> f32 {
    bincode::deserialize(&buf[..]).unwrap()
}

pub fn pack(msg: &MessageType) -> Vec<u8> {
    bincode::serialize(msg, Infinite).unwrap()
}

pub fn unpack(buf: &Vec<u8>) -> MessageType {
    bincode::deserialize(&buf[..]).unwrap()
}

pub struct Sequence {
    seq: Vec<u8>,
}

impl Sequence {
    pub fn new() -> Sequence {
        let mut seq: Vec<u8> = Vec::with_capacity(512);
        seq.push(SEQUENCE_HEADER);

        Sequence { seq: seq }
    }

    pub fn parse(seq: &Vec<u8>) -> Vec<MessageType> {
        let mut result: Vec<MessageType> = Vec::new();
        let len = seq.len();
        let mut offset = 1;
        while offset < len {
            let size: u16 = bincode::deserialize(&seq[offset..offset + 2]).unwrap();
            offset += 2;
            result.push(unpack(&seq[offset..offset + size as usize].to_vec()));
            offset += size as usize;
        }
        result
    }

    pub fn seq(&self) -> &Vec<u8> {
        &self.seq
    }

    pub fn add(&mut self, msg: &MessageType) {
        let mut buf = pack(msg);
        let len = buf.len() as u16;
        self.seq
            .append(&mut bincode::serialize(&len, Infinite).unwrap());
        self.seq.append(&mut buf);
    }
}

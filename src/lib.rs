#[macro_use]
extern crate serde_derive;
extern crate bincode;

use bincode::{Infinite};

pub mod enums;
use enums::MessageType;

pub trait BaseMessage {
    fn id(&self) -> MessageType;
    fn pack(&self) -> Vec<u8>;
    fn unpack(buf: &Vec<u8>) -> Box<Self>;
}

pub fn hello() {
    println!("Hello chunk-protocol");
}

pub fn serialize_float(x: &f32) -> Vec<u8> { bincode::serialize(x, Infinite).unwrap() }
pub fn deserialize_float(buf: &Vec<u8>) -> f32 { bincode::deserialize(&buf[..]).unwrap() }

#[derive(Debug)]
pub struct StringMessage {
    id: u8,
    text_size: u64,
    text: String
}

impl StringMessage {
    pub fn text(&self) -> &String { &self.text }
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(256);
        buf.push(self.id);
        buf.append(&mut bincode::serialize(&self.text_size, Infinite).unwrap());
        buf.append(&mut bincode::serialize(&self.text, Infinite).unwrap());
        buf
    }
}

pub fn new_string_message(text: String) -> Box<StringMessage> {
    Box::new(StringMessage {
        id: 0,
        text_size: bincode::serialized_size(&text),
        text: text
    })
}

pub fn unpack_string_message(buf: &Vec<u8>) -> Box<StringMessage> {
    let id = buf[0];
    let text_size: u64 = bincode::deserialize(&buf[1..9]).unwrap();
    let range = 9 + text_size as usize;
    let text: String = bincode::deserialize(&buf[9..range]).unwrap();
    Box::new(StringMessage {
        id: id,
        text_size: text_size,
        text: text
    })
}

pub struct RoomStatusMessage {
    _id: MessageType,
    _number: u8,
    _is_active: bool
}

impl RoomStatusMessage {
    pub fn new(number: u8, is_active: bool) -> Box<RoomStatusMessage> {
        Box::new(RoomStatusMessage {
            _id: MessageType::RoomStatus,
            _number: number,
            _is_active: is_active
        })
    }

    pub fn number(&self) -> &u8 { &self._number }
    pub fn is_active(&self) -> bool { self._is_active }
}

impl BaseMessage for RoomStatusMessage {
    fn id(&self) -> MessageType { self._id.clone() }

    fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(256);
        buf.append(&mut bincode::serialize(&self._id, Infinite).unwrap());
        buf.push(self._number);
        buf.append(&mut bincode::serialize(&self._is_active, Infinite).unwrap());
        buf
    }

    fn unpack(buf: &Vec<u8>) -> Box<RoomStatusMessage> {
        let number = buf[4];
        let is_active: bool = bincode::deserialize(&buf[5..6]).unwrap();
        RoomStatusMessage::new(number, is_active)
    }
}

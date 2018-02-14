use bincode::{Infinite, serialize, serialized_size, deserialize};

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
        buf.append(&mut serialize(&self.text_size, Infinite).unwrap());
        buf.append(&mut serialize(&self.text, Infinite).unwrap());
        buf
    }
}

pub fn new_string_message(text: String) -> Box<StringMessage> {
    Box::new(StringMessage {
        id: 0,
        text_size: serialized_size(&text),
        text: text
    })
}

pub fn unpack_string_message(buf: &Vec<u8>) -> Box<StringMessage> {
    let id = buf[0];
    let text_size: u64 = deserialize(&buf[1..9]).unwrap();
    let range = 9 + text_size as usize;
    let text: String = deserialize(&buf[9..range]).unwrap();
    Box::new(StringMessage {
        id: id,
        text_size: text_size,
        text: text
    })
}

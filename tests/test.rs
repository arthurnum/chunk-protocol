extern crate chunk_protocol as lib;

use lib::enums::MessageType;
use lib::BaseMessage;

#[test]
fn hello() {
    lib::hello();
}

#[test]
fn float() {
    let x: f32 = 0.5;
    let buf = lib::serialize_float(&x);
    let y = lib::deserialize_float(&buf);
    assert_eq!(x, y);
}

#[test]
fn string_message() {
    let msg = lib::new_string_message("Test message".to_string());
    let buf = msg.pack();
    let msg_unpacked = lib::unpack_string_message(&buf);
    assert_eq!(msg.text(), msg_unpacked.text());
}

#[test]
fn room_status_message() {
    let msg = lib::RoomStatusMessage::new(4, true);
    let buf = msg.pack();
    let msg_unpacked = lib::RoomStatusMessage::unpack(&buf);

    assert_eq!(msg.id(), MessageType::RoomStatus);
    assert_eq!(msg.id(), msg_unpacked.id());
    assert_eq!(msg.number(), msg_unpacked.number());
    assert_eq!(msg.is_active(), msg_unpacked.is_active());
}

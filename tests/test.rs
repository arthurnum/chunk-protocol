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

#[test]
fn add_to_listeners_request_message() {
    let msg = lib::AddToListenersRequestMessage::new();
    let buf = msg.pack();
    let msg_unpacked = lib::AddToListenersRequestMessage::unpack(&buf);

    assert_eq!(msg.id(), MessageType::AddToListenersRequest);
    assert_eq!(msg.id(), msg_unpacked.id());
}

#[test]
fn remove_from_listeners_message() {
    let msg = lib::RemoveFromListenersMessage::new();
    let buf = msg.pack();
    let msg_unpacked = lib::RemoveFromListenersMessage::unpack(&buf);

    assert_eq!(msg.id(), MessageType::RemoveFromListeners);
    assert_eq!(msg.id(), msg_unpacked.id());
}

#[test]
fn get_message_type() {
    let buf: Vec<u8> = vec![0, 0, 0, 0];
    let subject = lib::get_message_type(&buf);
    assert_eq!(true, subject.is_ok());
    assert_eq!(MessageType::AddToListenersRequest, subject.unwrap());

    let buf: Vec<u8> = vec![0, 0, 0, 255];
    let subject = lib::get_message_type(&buf);
    assert_eq!(true, subject.is_err());

    let buf: Vec<u8> = vec![];
    let subject = lib::get_message_type(&buf);
    assert_eq!(true, subject.is_err());
}

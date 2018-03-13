extern crate chunk_protocol as lib;
extern crate bincode;

use lib::enums::{MessageType};

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
    let msg = lib::string_messages::new_string_message("Test message".to_string());
    let buf = msg.pack();
    let msg_unpacked = lib::string_messages::unpack_string_message(&buf);
    assert_eq!(msg.text(), msg_unpacked.text());
}

#[test]
fn sequence() {
    let mut seq = lib::Sequence::new();
    seq.add(&MessageType::AddToListenersRequest);
    seq.add(&MessageType::RoomStatus { number: 1, is_active: true });
    let buf = seq.seq();

    let subject = lib::Sequence::parse(buf);
    assert_eq!(subject[0], MessageType::AddToListenersRequest);
    assert_eq!(subject[1], MessageType::RoomStatus { number: 1, is_active: true });
}

#[test]
fn add_to_listeners_request_message() {
    let msg = MessageType::AddToListenersRequest;
    let buf = lib::pack(&msg);
    let msg_unpacked = lib::unpack(&buf);

    assert_eq!(msg_unpacked, MessageType::AddToListenersRequest);
    assert_eq!(msg, msg_unpacked);
}

#[test]
fn member_in_message() {
    let msg = MessageType::MemberIn(1);
    let buf = lib::pack(&msg);
    let msg_unpacked = lib::unpack(&buf);

    assert_eq!(msg_unpacked, MessageType::MemberIn(1));
    assert_eq!(msg, msg_unpacked);
}

#[test]
fn member_out_message() {
    let msg = MessageType::MemberOut(1);
    let buf = lib::pack(&msg);
    let msg_unpacked = lib::unpack(&buf);

    assert_eq!(msg_unpacked, MessageType::MemberOut(1));
    assert_eq!(msg, msg_unpacked);
}

#[test]
fn remove_from_listeners_message() {
    let msg = MessageType::RemoveFromListeners;
    let buf = lib::pack(&msg);
    let msg_unpacked = lib::unpack(&buf);

    assert_eq!(msg_unpacked, MessageType::RemoveFromListeners);
    assert_eq!(msg, msg_unpacked);
}

#[test]
fn room_status_message() {
    let msg = MessageType::RoomStatus { number: 1, is_active: true };
    let buf = lib::pack(&msg);
    let msg_unpacked = lib::unpack(&buf);

    assert_eq!(msg_unpacked, MessageType::RoomStatus { number: 1, is_active: true });
    assert_eq!(msg, msg_unpacked);
}

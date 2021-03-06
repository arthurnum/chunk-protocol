extern crate bincode;
extern crate chunk_protocol as lib;

use lib::enums::MessageType;

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
    seq.add(&MessageType::ServerOn);
    let buf = seq.seq();

    let subject = lib::Sequence::parse(buf);
    assert_eq!(subject[0], MessageType::AddToListenersRequest);
    assert_eq!(subject[1], MessageType::ServerOn);
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
    let msg = MessageType::MemberIn;
    let buf = lib::pack(&msg);
    let msg_unpacked = lib::unpack(&buf);

    assert_eq!(msg_unpacked, MessageType::MemberIn);
    assert_eq!(msg, msg_unpacked);
}

#[test]
fn member_move_message() {
    let msg = MessageType::MemberMove(0.5, 0.5);
    let buf = lib::pack(&msg);
    let msg_unpacked = lib::unpack(&buf);

    assert_eq!(msg_unpacked, MessageType::MemberMove(0.5, 0.5));
    assert_eq!(msg, msg_unpacked);
}

#[test]
fn member_out_message() {
    let msg = MessageType::MemberOut;
    let buf = lib::pack(&msg);
    let msg_unpacked = lib::unpack(&buf);

    assert_eq!(msg_unpacked, MessageType::MemberOut);
    assert_eq!(msg, msg_unpacked);
}

#[test]
fn member_stop_move_message() {
    let msg = MessageType::MemberStopMove;
    let buf = lib::pack(&msg);
    let msg_unpacked = lib::unpack(&buf);

    assert_eq!(msg_unpacked, MessageType::MemberStopMove);
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
fn server_on_message() {
    let msg = MessageType::ServerOn;
    let buf = lib::pack(&msg);
    let msg_unpacked = lib::unpack(&buf);

    assert_eq!(msg_unpacked, MessageType::ServerOn);
    assert_eq!(msg, msg_unpacked);
}

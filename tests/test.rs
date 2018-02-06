extern crate chunk_protocol as lib;

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

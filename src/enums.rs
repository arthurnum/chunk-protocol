#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MessageType {
    AddToListenersRequest,
    MemberIn(u8),
    MemberOut(u8),
    RemoveFromListeners,
    RoomStatus { number: u8, is_active: bool }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MessageType {
    AddToListenersRequest,
    MemberIn,
    MemberOut,
    RemoveFromListeners,
    RoomStatus { number: u8, is_active: bool }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MessageType {
    AddToListenersRequest,
    RemoveFromListeners,
    RoomStatus
}

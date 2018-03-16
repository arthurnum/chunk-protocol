#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MessageType {
    AddToListenersRequest,
    MemberIn,
    MemberMove(f32, f32),
    MemberOut,
    MemberStopMove,
    RemoveFromListeners,
    ServerOn,
}
